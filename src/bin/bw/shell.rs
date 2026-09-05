use super::error::{
    CliError, map_anchor_error, map_data_error, map_edit_error, map_pick_error,
    map_session_replace_content_error, map_store_error, session_error_status,
};
use super::output::{
    write_batch_check, write_check, write_data_list, write_data_value, write_pick, write_search,
    write_session_check, write_session_refs, write_session_replace, write_session_status,
    write_session_view, write_view,
};
use super::{
    open_runtime, parse_search, prepare_replace_content, required_token, run_check,
    run_check_batch, run_search, run_view,
};
use backwriter::backwriter::anchor::{Anchedress, AnchorOutcome};
use backwriter::backwriter::anddress::{Anddress, AnddressTarget};
use backwriter::backwriter::apply::{ApplyError, EditReceipt};
use backwriter::backwriter::check::{CheckOutcome, CheckStatus};
use backwriter::backwriter::data::{DataKind, DataName, DataStore};
use backwriter::backwriter::edit::{Edit, Position};
use backwriter::backwriter::pick::{PickOutcome, PickPredicate, PickTargetKind, pick};
use backwriter::backwriter::search::SearchOutcome;
use backwriter::backwriter::view::ViewOutcome;
use backwriter::runtime::{AdmissionRoot, WorkspaceRuntime};
use std::{
    io::{self, BufRead, BufWriter},
    path::PathBuf,
    process::ExitCode,
};

pub(super) enum SessionValue {
    Search(SearchOutcome),
    Pick(PickOutcome),
    Anddress(Anddress),
    Anchedress {
        handle: Anchedress,
        target: AnddressTarget,
    },
    Edit(Edit),
    View(ViewOutcome),
    CheckAnddress(CheckOutcome<Option<Anddress>>),
    CheckSearch(CheckOutcome<SearchOutcome>),
    CheckPick(CheckOutcome<PickOutcome>),
}

struct SessionBinding {
    name: String,
    value: SessionValue,
}

enum SessionControl {
    Continue,
    Exit,
}

pub(super) fn execute_shell(
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<ExitCode, CliError> {
    let mut runtime = open_runtime(workspace, admissions, None)?;
    let mut data = DataStore::new();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut bindings = Vec::new();
    let mut refs = Vec::new();
    let mut line = String::new();
    let mut highest_error = 0_u8;

    loop {
        line.clear();
        let read = reader
            .read_line(&mut line)
            .map_err(|error| CliError::stream(error.to_string()))?;
        if read == 0 {
            break;
        }
        trim_physical_line(&mut line);
        let tokens = match lex_line(&line) {
            Ok(tokens) => tokens,
            Err(error) => {
                error.report();
                highest_error = highest_error.max(session_error_status(&error));
                continue;
            }
        };
        if tokens.is_empty() {
            continue;
        }
        match execute_session_command(&mut runtime, &mut data, &mut bindings, &mut refs, &tokens) {
            Ok(SessionControl::Continue) => {}
            Ok(SessionControl::Exit) => break,
            Err(error @ CliError::Stream(_)) => return Err(error),
            Err(error) => {
                error.report();
                highest_error = highest_error.max(session_error_status(&error));
            }
        }
    }

    Ok(ExitCode::from(highest_error))
}

fn trim_physical_line(line: &mut String) {
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
}

fn execute_session_command(
    runtime: &mut WorkspaceRuntime,
    data: &mut DataStore,
    bindings: &mut Vec<SessionBinding>,
    refs: &mut Vec<Anddress>,
    tokens: &[String],
) -> Result<SessionControl, CliError> {
    match tokens[0].as_str() {
        "search" => {
            execute_session_search(runtime, refs, &tokens[1..])?;
            Ok(SessionControl::Continue)
        }
        "pick" => {
            let outcome = run_pick(bindings, &tokens[1..])?;
            write_pick(&outcome)?;
            Ok(SessionControl::Continue)
        }
        "let" => {
            execute_let(runtime, data, bindings, refs, tokens)?;
            Ok(SessionControl::Continue)
        }
        "view" => {
            execute_session_view(runtime, bindings, refs, tokens)?;
            Ok(SessionControl::Continue)
        }
        "check" => {
            execute_session_check(runtime, bindings, refs, tokens)?;
            Ok(SessionControl::Continue)
        }
        "anchor" => {
            execute_session_anchor(runtime, tokens)?;
            Ok(SessionControl::Continue)
        }
        "apply" => {
            execute_session_apply(runtime, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "data" => {
            execute_session_data(data, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "replace" => {
            execute_session_replace(runtime, refs, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "exit" if tokens.len() == 1 => Ok(SessionControl::Exit),
        "exit" => Err(CliError::usage("exit accepts no operands")),
        capability => Err(CliError::usage(format!(
            "unsupported Session command: {capability}"
        ))),
    }
}

fn execute_session_search(
    runtime: &WorkspaceRuntime,
    refs: &mut Vec<Anddress>,
    arguments: &[String],
) -> Result<(), CliError> {
    let outcome = run_search(runtime, parse_search(arguments)?)?;
    let SearchOutcome::Found { anddresses } = outcome else {
        return Ok(());
    };
    let start = reserve_session_refs(refs, anddresses.len())?;
    refs.extend(anddresses);
    write_session_refs(start, &refs[start..])
}

fn execute_session_replace(
    runtime: &mut WorkspaceRuntime,
    refs: &mut Vec<Anddress>,
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<(), CliError> {
    if tokens.len() != 3 {
        return Err(CliError::usage(
            "replace requires exactly one reference and Content",
        ));
    }
    let target = resolve_session_ref(bindings, refs, &tokens[1])?;
    let content = prepare_replace_content(&target, tokens[2].clone())
        .map_err(map_session_replace_content_error)?;
    let edit = Edit::Replace { target, content };
    edit.validate().map_err(map_edit_error)?;
    let slot = reserve_session_refs(refs, 1)?;
    let receipt = runtime
        .apply_replace(&edit)
        .map_err(|error: ApplyError| CliError::execution(error.to_string()))?;
    match receipt {
        EditReceipt::Unchanged { anddress } => {
            refs.push(anddress);
            write_session_replace(slot, "Unchanged", &refs[slot])
        }
        EditReceipt::Changed {
            anddress: Some(anddress),
        } => {
            refs.push(anddress);
            write_session_replace(slot, "Changed", &refs[slot])
        }
        EditReceipt::Changed { anddress: None } => write_session_status("Changed\tNone"),
    }
}

fn execute_session_ref_view(
    runtime: &mut WorkspaceRuntime,
    bindings: &[SessionBinding],
    refs: &mut Vec<Anddress>,
    tokens: &[String],
) -> Result<(), CliError> {
    let (references, projection) = parse_session_ref_view(tokens)?;
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(references.len())
        .map_err(|_| CliError::execution("Session View allocation failed"))?;
    for reference in references {
        inputs.push(resolve_session_ref(bindings, refs, reference)?);
    }

    let outcomes = if inputs.len() == 1 {
        let mut outcomes = Vec::new();
        outcomes
            .try_reserve_exact(1)
            .map_err(|_| CliError::execution("Session View allocation failed"))?;
        let input = &inputs[0];
        outcomes.push(run_view(
            runtime,
            input,
            projection.unwrap_or_else(|| input.target()),
        )?);
        outcomes
    } else {
        runtime
            .view_batch(&inputs, projection)
            .map_err(|error| CliError::execution(error.to_string()))?
    };
    write_session_view(
        BufWriter::new(io::stdout().lock()),
        refs,
        references,
        outcomes,
    )
}

fn parse_session_ref_view(
    tokens: &[String],
) -> Result<(&[String], Option<AnddressTarget>), CliError> {
    let Some(first) = tokens.first() else {
        return Err(CliError::usage("view requires at least one reference"));
    };
    if !first.starts_with('@') {
        return Err(CliError::usage("view requires an Anddress reference"));
    }
    let Some(as_index) = tokens.iter().position(|token| token == "--as") else {
        return Ok((tokens, None));
    };
    if as_index == 0 || as_index + 2 != tokens.len() {
        return Err(CliError::usage(
            "view --as requires references followed by one target kind",
        ));
    }
    let projection = match tokens[as_index + 1].as_str() {
        "line" => AnddressTarget::Line,
        "paragraph" => AnddressTarget::Paragraph,
        "file" => AnddressTarget::File,
        _ => return Err(CliError::usage("invalid view target kind")),
    };
    Ok((&tokens[..as_index], Some(projection)))
}

pub(super) fn reserve_session_refs(
    refs: &mut Vec<Anddress>,
    count: usize,
) -> Result<usize, CliError> {
    refs.try_reserve_exact(count)
        .map_err(|_| CliError::execution("Session reference allocation failed"))?;
    Ok(refs.len())
}

fn execute_let(
    runtime: &mut WorkspaceRuntime,
    data: &mut DataStore,
    bindings: &mut Vec<SessionBinding>,
    refs: &[Anddress],
    tokens: &[String],
) -> Result<(), CliError> {
    let name = required_token(tokens, 1, "let name")?;
    validate_binding_name(name)?;
    if required_token(tokens, 2, "let assignment")? != "=" {
        return Err(CliError::usage("let requires a standalone = token"));
    }
    if binding(bindings, name).is_some() {
        return Err(CliError::usage(format!("binding already exists: {name}")));
    }
    let right_hand_side = required_token(tokens, 3, "let value")?;
    if right_hand_side == "data" {
        if tokens.len() != 7 || tokens[4] != "get" {
            return Err(CliError::usage("let data requires get, kind, and name"));
        }
        let value = data_get(data, &tokens[5], &tokens[6])?;
        write_data_value(&value)?;
        return store_binding(bindings, name, value);
    }
    if right_hand_side == "anchor" {
        if required_token(tokens, 4, "anchor operation")? != "create" {
            return Err(CliError::usage("let anchor requires the create operation"));
        }
        let operand = required_token(tokens, 5, "anchor Anddress reference")?;
        if tokens.len() != 6 {
            return Err(CliError::usage(
                "let anchor create accepts exactly one Anddress reference",
            ));
        }
        let anddress = resolve_anddress(bindings, operand)?;
        let target = anddress.target();
        return match runtime.anchor(&anddress).map_err(map_anchor_error)? {
            AnchorOutcome::Anchored(handle) => {
                write_session_status("Anchored")?;
                store_binding(bindings, name, SessionValue::Anchedress { handle, target })
            }
            AnchorOutcome::AlreadyLive => write_session_status("AlreadyLive"),
        };
    }
    if right_hand_side == "edit" {
        let edit = parse_session_edit(bindings, &tokens[4..])?;
        edit.validate().map_err(map_edit_error)?;
        return store_binding(bindings, name, SessionValue::Edit(edit));
    }
    if right_hand_side == "view" {
        let form = required_token(tokens, 4, "view input form")?;
        let outcome = match form {
            "anddress" => {
                if tokens.len() != 6 {
                    return Err(CliError::usage(
                        "view anddress accepts exactly one reference",
                    ));
                }
                let anddress = resolve_anddress(bindings, &tokens[5])?;
                run_view(runtime, &anddress, anddress.target())?
            }
            "anchored" => {
                if tokens.len() != 6 {
                    return Err(CliError::usage(
                        "view anchored accepts exactly one handle binding",
                    ));
                }
                let (handle, target) = resolve_anchedress(bindings, &tokens[5])?;
                runtime
                    .view_anchored(handle, target)
                    .map_err(|error| CliError::execution(error.to_string()))?
            }
            _ => {
                return Err(CliError::usage(
                    "view requires the anddress or anchored input form",
                ));
            }
        };
        write_view(&outcome)?;
        return store_binding(bindings, name, SessionValue::View(outcome));
    }
    if right_hand_side == "check" {
        let form = required_token(tokens, 4, "check input form")?;
        return match form {
            "anddress" => {
                if tokens.len() != 6 {
                    return Err(CliError::usage(
                        "check anddress accepts exactly one reference",
                    ));
                }
                let outcome = run_check(runtime, resolve_anddress(bindings, &tokens[5])?)?;
                write_check(&outcome)?;
                store_binding(bindings, name, SessionValue::CheckAnddress(outcome))
            }
            "search" | "pick" => {
                if tokens.len() != 6 {
                    return Err(CliError::usage(format!(
                        "check {form} accepts exactly one binding"
                    )));
                }
                let value = resolve_binding_value(bindings, &tokens[5])?;
                match (form, value) {
                    ("search", SessionValue::Search(input)) => {
                        let outcome = runtime
                            .check_search(input)
                            .map_err(|error| CliError::execution(error.to_string()))?;
                        write_batch_check(&outcome.report)?;
                        store_binding(bindings, name, SessionValue::CheckSearch(outcome))
                    }
                    ("pick", SessionValue::Pick(input)) => {
                        let outcome = runtime
                            .check_pick(input)
                            .map_err(|error| CliError::execution(error.to_string()))?;
                        write_batch_check(&outcome.report)?;
                        store_binding(bindings, name, SessionValue::CheckPick(outcome))
                    }
                    ("search", _) => Err(CliError::usage("check search requires a Search binding")),
                    ("pick", _) => Err(CliError::usage("check pick requires a Pick binding")),
                    _ => unreachable!(),
                }
            }
            _ => Err(CliError::usage(
                "check requires the anddress, search, or pick input form",
            )),
        };
    }
    let value = if right_hand_side == "search" {
        let outcome = run_search(runtime, parse_search(&tokens[4..])?)?;
        write_search(&outcome)?;
        SessionValue::Search(outcome)
    } else if right_hand_side == "pick" {
        let outcome = run_pick(bindings, &tokens[4..])?;
        write_pick(&outcome)?;
        SessionValue::Pick(outcome)
    } else {
        if tokens.len() != 4 {
            return Err(CliError::usage("let reference accepts exactly one operand"));
        }
        if numeric_session_ref(right_hand_side)?.is_some() {
            SessionValue::Anddress(resolve_session_ref(bindings, refs, right_hand_side)?)
        } else if right_hand_side.contains('[') || right_hand_side.contains(']') {
            SessionValue::Anddress(resolve_anddress(bindings, right_hand_side)?)
        } else {
            resolve_binding_value(bindings, right_hand_side)?
        }
    };
    store_binding(bindings, name, value)
}

fn execute_session_data(
    data: &mut DataStore,
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<(), CliError> {
    match required_token(tokens, 1, "data operation")? {
        "store" => {
            if tokens.len() != 5 {
                return Err(CliError::usage("data store requires kind, name, and value"));
            }
            data_store(data, bindings, &tokens[2], &tokens[3], &tokens[4])
        }
        "get" => {
            if tokens.len() != 4 {
                return Err(CliError::usage("data get requires kind and name"));
            }
            let value = data_get(data, &tokens[2], &tokens[3])?;
            write_data_value(&value)
        }
        "list" if tokens.len() == 2 => write_data_list(data),
        "rename" => {
            if tokens.len() != 5 {
                return Err(CliError::usage(
                    "data rename requires kind, old name, and new name",
                ));
            }
            data.rename(
                parse_data_kind(&tokens[2])?,
                &data_name(&tokens[3])?,
                &data_name(&tokens[4])?,
            )
            .map_err(map_data_error)?;
            write_session_status("OK")
        }
        "remove" => {
            if tokens.len() != 4 {
                return Err(CliError::usage("data remove requires kind and name"));
            }
            data.remove(parse_data_kind(&tokens[2])?, &data_name(&tokens[3])?)
                .map_err(map_data_error)?;
            write_session_status("OK")
        }
        _ => Err(CliError::usage("unsupported data command")),
    }
}

fn data_name(value: &str) -> Result<DataName, CliError> {
    DataName::new(value.to_owned()).map_err(|error| CliError::usage(error.to_string()))
}

fn parse_data_kind(value: &str) -> Result<DataKind, CliError> {
    match value {
        "anddress" => Ok(DataKind::Anddress),
        "search" => Ok(DataKind::Search),
        "pick" => Ok(DataKind::Pick),
        "view" => Ok(DataKind::View),
        "check-anddress" => Ok(DataKind::CheckAnddress),
        "check-search" => Ok(DataKind::CheckSearch),
        "check-pick" => Ok(DataKind::CheckPick),
        _ => Err(CliError::usage("unknown Data kind")),
    }
}

fn data_store(
    data: &mut DataStore,
    bindings: &[SessionBinding],
    kind: &str,
    name: &str,
    reference: &str,
) -> Result<(), CliError> {
    let kind = parse_data_kind(kind)?;
    let name = data_name(name)?;
    let value = if kind == DataKind::Anddress {
        SessionValue::Anddress(resolve_anddress(bindings, reference)?)
    } else {
        resolve_binding_value(bindings, reference)?
    };
    match (kind, value) {
        (DataKind::Anddress, SessionValue::Anddress(v)) => {
            data.store_anddress(&name, v).map_err(map_store_error)
        }
        (DataKind::Search, SessionValue::Search(v)) => {
            data.store_search(&name, v).map_err(map_store_error)
        }
        (DataKind::Pick, SessionValue::Pick(v)) => {
            data.store_pick(&name, v).map_err(map_store_error)
        }
        (DataKind::View, SessionValue::View(v)) => {
            data.store_view(&name, v).map_err(map_store_error)
        }
        (DataKind::CheckAnddress, SessionValue::CheckAnddress(v)) => {
            data.store_check_anddress(&name, v).map_err(map_store_error)
        }
        (DataKind::CheckSearch, SessionValue::CheckSearch(v)) => {
            data.store_check_search(&name, v).map_err(map_store_error)
        }
        (DataKind::CheckPick, SessionValue::CheckPick(v)) => {
            data.store_check_pick(&name, v).map_err(map_store_error)
        }
        _ => Err(CliError::usage("Data kind does not match binding")),
    }?;
    write_session_status("OK")
}

fn data_get(data: &DataStore, kind: &str, name: &str) -> Result<SessionValue, CliError> {
    let name = data_name(name)?;
    match parse_data_kind(kind)? {
        DataKind::Anddress => data
            .get_anddress(&name)
            .cloned()
            .map(SessionValue::Anddress),
        DataKind::Search => data.get_search(&name).cloned().map(SessionValue::Search),
        DataKind::Pick => data.get_pick(&name).cloned().map(SessionValue::Pick),
        DataKind::View => data.get_view(&name).cloned().map(SessionValue::View),
        DataKind::CheckAnddress => data
            .get_check_anddress(&name)
            .cloned()
            .map(SessionValue::CheckAnddress),
        DataKind::CheckSearch => data
            .get_check_search(&name)
            .cloned()
            .map(SessionValue::CheckSearch),
        DataKind::CheckPick => data
            .get_check_pick(&name)
            .cloned()
            .map(SessionValue::CheckPick),
    }
    .ok_or_else(|| CliError::usage("Data entry was not found"))
}

fn parse_session_edit(bindings: &[SessionBinding], arguments: &[String]) -> Result<Edit, CliError> {
    match required_token(arguments, 0, "edit operation")? {
        "insert" => {
            if arguments.len() != 4 {
                return Err(CliError::usage(
                    "edit insert requires a position and content",
                ));
            }
            Ok(Edit::Insert {
                position: parse_session_position(bindings, &arguments[1..3])?,
                content: arguments[3].clone(),
            })
        }
        "replace" => {
            if arguments.len() != 3 {
                return Err(CliError::usage(
                    "edit replace requires a target and content",
                ));
            }
            Ok(Edit::Replace {
                target: resolve_anddress(bindings, &arguments[1])?,
                content: arguments[2].clone(),
            })
        }
        "delete" => {
            if arguments.len() != 2 {
                return Err(CliError::usage("edit delete requires exactly one target"));
            }
            Ok(Edit::Delete {
                target: resolve_anddress(bindings, &arguments[1])?,
            })
        }
        "move" | "copy" => {
            if arguments.len() != 4 {
                return Err(CliError::usage(format!(
                    "edit {} requires a target and position",
                    arguments[0]
                )));
            }
            let target = resolve_anddress(bindings, &arguments[1])?;
            let position = parse_session_position(bindings, &arguments[2..4])?;
            Ok(if arguments[0] == "move" {
                Edit::Move { target, position }
            } else {
                Edit::Copy { target, position }
            })
        }
        _ => Err(CliError::usage("unsupported edit operation")),
    }
}

fn parse_session_position(
    bindings: &[SessionBinding],
    arguments: &[String],
) -> Result<Position, CliError> {
    if arguments.len() != 2 {
        return Err(CliError::usage(
            "position requires exactly one Anddress reference",
        ));
    }
    let target = resolve_anddress(bindings, &arguments[1])?;
    match arguments[0].as_str() {
        "before" => Ok(Position::Before(target)),
        "after" => Ok(Position::After(target)),
        "start-of" => Ok(Position::StartOf(target)),
        "end-of" => Ok(Position::EndOf(target)),
        _ => Err(CliError::usage("unsupported position")),
    }
}

fn run_pick(bindings: &[SessionBinding], arguments: &[String]) -> Result<PickOutcome, CliError> {
    let candidates =
        resolve_pick_candidates(bindings, required_token(arguments, 0, "pick candidates")?)?;
    let predicate_tokens = split_pick_parentheses(&arguments[1..])?;
    let predicate = parse_pick_predicate(bindings, &predicate_tokens)?;
    pick(candidates, &predicate).map_err(map_pick_error)
}

fn resolve_pick_candidates(
    bindings: &[SessionBinding],
    token: &str,
) -> Result<Vec<Anddress>, CliError> {
    let name = token
        .strip_prefix('@')
        .ok_or_else(|| CliError::usage("Pick candidates require a binding reference"))?;
    if name.contains('[') || name.contains(']') {
        return Err(CliError::usage(
            "Pick candidates require a Search or Pick binding without an index",
        ));
    }
    validate_binding_name(name)?;
    let source = match binding(bindings, name) {
        Some(SessionValue::Search(SearchOutcome::Empty))
        | Some(SessionValue::Pick(PickOutcome::Empty)) => {
            return Ok(Vec::new());
        }
        Some(SessionValue::Search(SearchOutcome::Found { anddresses })) => {
            let mut candidates = Vec::new();
            candidates
                .try_reserve_exact(anddresses.len())
                .map_err(|_| CliError::execution("Pick candidate allocation failed"))?;
            candidates.extend(anddresses.iter().cloned());
            return Ok(candidates);
        }
        Some(SessionValue::Pick(PickOutcome::Selected { anddresses })) => anddresses,
        Some(SessionValue::Anddress(_)) => {
            return Err(CliError::usage(format!(
                "Pick candidates require a Search or Pick binding: {name}"
            )));
        }
        Some(SessionValue::Anchedress { .. }) => {
            return Err(CliError::usage(format!(
                "Pick candidates require a Search or Pick binding: {name}"
            )));
        }
        Some(SessionValue::Edit(_)) => {
            return Err(CliError::usage(format!(
                "Pick candidates require a Search or Pick binding: {name}"
            )));
        }
        Some(SessionValue::View(_))
        | Some(SessionValue::CheckAnddress(_))
        | Some(SessionValue::CheckSearch(_))
        | Some(SessionValue::CheckPick(_)) => {
            return Err(CliError::usage(format!(
                "Pick candidates require a Search or Pick binding: {name}"
            )));
        }
        None => return Err(CliError::usage(format!("unknown binding: {name}"))),
    };
    let mut candidates = Vec::new();
    candidates
        .try_reserve_exact(source.len())
        .map_err(|_| CliError::execution("Pick candidate allocation failed"))?;
    candidates.extend(source.iter().cloned());
    Ok(candidates)
}

enum PickFrameKind {
    Group,
    Not,
    AllOf,
    AnyOf,
}

struct PickFrame {
    kind: PickFrameKind,
    predicates: Vec<PickPredicate>,
}

fn split_pick_parentheses(tokens: &[String]) -> Result<Vec<String>, CliError> {
    let mut split = Vec::new();
    split
        .try_reserve(tokens.len())
        .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
    for token in tokens {
        let mut start = 0;
        for (offset, character) in token.char_indices() {
            if !matches!(character, '(' | ')') {
                continue;
            }
            if start != offset {
                push_pick_token(&mut split, &token[start..offset])?;
            }
            push_pick_token(&mut split, &token[offset..offset + character.len_utf8()])?;
            start = offset + character.len_utf8();
        }
        if start != token.len() {
            push_pick_token(&mut split, &token[start..])?;
        }
    }
    Ok(split)
}

fn push_pick_token(tokens: &mut Vec<String>, value: &str) -> Result<(), CliError> {
    tokens
        .try_reserve(1)
        .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
    let mut token = String::new();
    token
        .try_reserve_exact(value.len())
        .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
    token.push_str(value);
    tokens.push(token);
    Ok(())
}

fn parse_pick_predicate(
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<PickPredicate, CliError> {
    let mut position = 0;
    let mut frames = Vec::new();
    let mut root = None;
    while let Some(token) = tokens.get(position) {
        finish_pick_operators(&mut frames, &mut root, Some(token))?;
        if token == ")" {
            position += 1;
            let frame = frames
                .pop()
                .ok_or_else(|| CliError::usage("unexpected Pick predicate closing parenthesis"))?;
            let predicate = finish_pick_frame(frame)?;
            accept_pick_predicate(&mut frames, &mut root, predicate)?;
            continue;
        }
        if token == "(" {
            if matches!(
                frames.last().map(|frame| &frame.kind),
                Some(PickFrameKind::AllOf | PickFrameKind::AnyOf)
            ) {
                frames
                    .try_reserve(1)
                    .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
                frames.push(PickFrame {
                    kind: PickFrameKind::Group,
                    predicates: Vec::new(),
                });
                position += 1;
                continue;
            }
            return Err(CliError::usage(
                "unexpected Pick predicate opening parenthesis",
            ));
        }
        if root.is_some() && frames.is_empty() {
            return Err(CliError::usage("Pick predicate has trailing input"));
        }
        let predicate = match token.as_str() {
            "all" => {
                position += 1;
                PickPredicate::all()
            }
            "target-kind" => {
                let kind = required_token(tokens, position + 1, "Pick target kind")?;
                position += 2;
                PickPredicate::target_kind(match kind {
                    "file" => PickTargetKind::File,
                    "paragraph" => PickTargetKind::Paragraph,
                    "line" => PickTargetKind::Line,
                    _ => return Err(CliError::usage(format!("invalid Pick target kind: {kind}"))),
                })
            }
            "one-of" => {
                position += 1;
                let mut members = Vec::new();
                while let Some(reference) = tokens.get(position) {
                    if !reference.starts_with('@') {
                        break;
                    }
                    members
                        .try_reserve(1)
                        .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
                    members.push(resolve_anddress(bindings, reference)?);
                    position += 1;
                }
                if members.is_empty() {
                    return Err(CliError::usage(
                        "one-of requires at least one Anddress reference",
                    ));
                }
                PickPredicate::one_of(members)
            }
            "same-file" => {
                let reference =
                    required_token(tokens, position + 1, "same-file Anddress reference")?;
                position += 2;
                PickPredicate::same_file(resolve_anddress(bindings, reference)?)
            }
            "not" | "all-of" | "any-of" => {
                let kind = match token.as_str() {
                    "not" => PickFrameKind::Not,
                    "all-of" => PickFrameKind::AllOf,
                    "any-of" => PickFrameKind::AnyOf,
                    _ => unreachable!(),
                };
                if required_token(tokens, position + 1, "Pick predicate opening parenthesis")?
                    != "("
                {
                    return Err(CliError::usage(
                        "Pick composition requires an opening parenthesis",
                    ));
                }
                frames
                    .try_reserve(1)
                    .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
                frames.push(PickFrame {
                    kind,
                    predicates: Vec::new(),
                });
                frames
                    .try_reserve(1)
                    .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
                frames.push(PickFrame {
                    kind: PickFrameKind::Group,
                    predicates: Vec::new(),
                });
                position += 2;
                continue;
            }
            _ => return Err(CliError::usage(format!("invalid Pick predicate: {token}"))),
        };
        accept_pick_predicate(&mut frames, &mut root, predicate)?;
    }
    finish_pick_operators(&mut frames, &mut root, None)?;
    if !frames.is_empty() {
        return Err(CliError::usage("unclosed Pick predicate parenthesis"));
    }
    root.ok_or_else(|| CliError::usage("Pick predicate requires a value"))
}

fn accept_pick_predicate(
    frames: &mut [PickFrame],
    root: &mut Option<PickPredicate>,
    predicate: PickPredicate,
) -> Result<(), CliError> {
    if let Some(frame) = frames.last_mut() {
        frame
            .predicates
            .try_reserve(1)
            .map_err(|_| CliError::execution("Pick predicate allocation failed"))?;
        frame.predicates.push(predicate);
        return Ok(());
    }
    if root.replace(predicate).is_some() {
        return Err(CliError::usage("Pick predicate has trailing input"));
    }
    Ok(())
}

fn finish_pick_operators(
    frames: &mut Vec<PickFrame>,
    root: &mut Option<PickPredicate>,
    next: Option<&String>,
) -> Result<(), CliError> {
    loop {
        let Some(frame) = frames.last() else {
            return Ok(());
        };
        let is_complete = match frame.kind {
            PickFrameKind::Not => true,
            PickFrameKind::AllOf | PickFrameKind::AnyOf => next.is_none_or(|token| token != "("),
            PickFrameKind::Group => false,
        };
        if !is_complete {
            return Ok(());
        }
        let frame = frames.pop().expect("nonempty Pick frame stack");
        let predicate = finish_pick_frame(frame)?;
        accept_pick_predicate(frames, root, predicate)?;
    }
}

fn finish_pick_frame(mut frame: PickFrame) -> Result<PickPredicate, CliError> {
    if frame.predicates.is_empty() {
        return Err(CliError::usage(
            "Pick composition requires at least one predicate",
        ));
    }
    let first = frame.predicates.remove(0);
    match frame.kind {
        PickFrameKind::Group => {
            if !frame.predicates.is_empty() {
                return Err(CliError::usage(
                    "parenthesized Pick predicate requires exactly one predicate",
                ));
            }
            Ok(first)
        }
        PickFrameKind::Not => {
            if !frame.predicates.is_empty() {
                return Err(CliError::usage("not requires exactly one predicate"));
            }
            PickPredicate::negate(first).map_err(map_pick_error)
        }
        PickFrameKind::AllOf => {
            PickPredicate::all_of(first, frame.predicates).map_err(map_pick_error)
        }
        PickFrameKind::AnyOf => {
            PickPredicate::any_of(first, frame.predicates).map_err(map_pick_error)
        }
    }
}

fn execute_session_view(
    runtime: &mut WorkspaceRuntime,
    bindings: &[SessionBinding],
    refs: &mut Vec<Anddress>,
    tokens: &[String],
) -> Result<(), CliError> {
    match required_token(tokens, 1, "view input form")? {
        "anddress" => {
            session_anddress_form(tokens, "view")?;
            let anddress = resolve_anddress(bindings, &tokens[2])?;
            write_view(&run_view(runtime, &anddress, anddress.target())?)
        }
        "anchored" => {
            if tokens.len() != 3 {
                return Err(CliError::usage(
                    "view anchored accepts exactly one handle binding",
                ));
            }
            let (handle, target) = resolve_anchedress(bindings, &tokens[2])?;
            let outcome = runtime
                .view_anchored(handle, target)
                .map_err(|error| CliError::execution(error.to_string()))?;
            write_view(&outcome)
        }
        _ => execute_session_ref_view(runtime, bindings, refs, &tokens[1..]),
    }
}

fn execute_session_anchor(
    runtime: &mut WorkspaceRuntime,
    tokens: &[String],
) -> Result<(), CliError> {
    match required_token(tokens, 1, "anchor operation")? {
        "create" => Err(CliError::usage(
            "anchor create is available only as a let right-hand side",
        )),
        "invalidate-source" => {
            let path = required_token(tokens, 2, "anchor logical path")?;
            if tokens.len() != 3 {
                return Err(CliError::usage(
                    "anchor invalidate-source accepts exactly one logical path",
                ));
            }
            runtime
                .invalidate_anchored_source(path)
                .map_err(map_anchor_error)?;
            write_session_status("OK")
        }
        _ => Err(CliError::usage("unsupported anchor operation")),
    }
}

fn execute_session_apply(
    runtime: &mut WorkspaceRuntime,
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<(), CliError> {
    if tokens.len() != 2 {
        return Err(CliError::usage("apply accepts exactly one Edit binding"));
    }
    let edit = resolve_edit(bindings, &tokens[1])?;
    runtime
        .apply(edit)
        .map_err(|error: ApplyError| CliError::execution(error.to_string()))?;
    write_session_status("OK")
}

fn execute_session_check(
    runtime: &mut WorkspaceRuntime,
    bindings: &[SessionBinding],
    refs: &mut Vec<Anddress>,
    tokens: &[String],
) -> Result<(), CliError> {
    match required_token(tokens, 1, "check input form")? {
        "anddress" => {
            session_anddress_form(tokens, "check")?;
            let anddress = resolve_anddress(bindings, &tokens[2])?;
            write_check(&run_check(runtime, anddress)?)
        }
        "search" | "pick" => {
            if tokens.len() != 3 {
                return Err(CliError::usage(format!(
                    "check {} accepts exactly one binding",
                    tokens[1]
                )));
            }
            let value = resolve_binding_value(bindings, &tokens[2])?;
            let report = match (tokens[1].as_str(), value) {
                ("search", SessionValue::Search(input)) => {
                    runtime
                        .check_search(input)
                        .map_err(|error| CliError::execution(error.to_string()))?
                        .report
                }
                ("pick", SessionValue::Pick(input)) => {
                    runtime
                        .check_pick(input)
                        .map_err(|error| CliError::execution(error.to_string()))?
                        .report
                }
                ("search", _) => {
                    return Err(CliError::usage("check search requires a Search binding"));
                }
                ("pick", _) => {
                    return Err(CliError::usage("check pick requires a Pick binding"));
                }
                _ => unreachable!(),
            };
            write_batch_check(&report)
        }
        _ => execute_session_ref_check(runtime, bindings, refs, &tokens[1..]),
    }
}

fn execute_session_ref_check(
    runtime: &WorkspaceRuntime,
    bindings: &[SessionBinding],
    refs: &mut Vec<Anddress>,
    references: &[String],
) -> Result<(), CliError> {
    if references.is_empty() {
        return Err(CliError::usage("check requires at least one reference"));
    }
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(references.len())
        .map_err(|_| CliError::execution("Session Check allocation failed"))?;
    for reference in references {
        inputs.push(resolve_session_ref(bindings, refs, reference)?);
    }
    let statuses = run_check_batch(runtime, &inputs)?;
    let current_count = statuses
        .iter()
        .filter(|status| **status == CheckStatus::Current)
        .count();
    let start = reserve_session_refs(refs, current_count)?;
    for (input, status) in inputs.iter().zip(&statuses) {
        if *status == CheckStatus::Current {
            refs.push(input.clone());
        }
    }
    write_session_check(start, &inputs, &statuses)
}

fn session_anddress_form(tokens: &[String], capability: &str) -> Result<(), CliError> {
    if required_token(tokens, 1, &format!("{capability} input form"))? != "anddress" {
        return Err(CliError::usage(format!(
            "{capability} requires the anddress input form"
        )));
    }
    required_token(tokens, 2, &format!("{capability} anddress"))?;
    if tokens.len() != 3 {
        return Err(CliError::usage(format!(
            "{capability} accepts exactly one anddress operand"
        )));
    }
    Ok(())
}

fn binding<'a>(bindings: &'a [SessionBinding], name: &str) -> Option<&'a SessionValue> {
    bindings
        .iter()
        .find(|binding| binding.name == name)
        .map(|binding| &binding.value)
}

fn store_binding(
    bindings: &mut Vec<SessionBinding>,
    name: &str,
    value: SessionValue,
) -> Result<(), CliError> {
    let mut stored_name = String::new();
    stored_name
        .try_reserve_exact(name.len())
        .map_err(|_| CliError::execution("Session binding allocation failed"))?;
    stored_name.push_str(name);
    bindings
        .try_reserve(1)
        .map_err(|_| CliError::execution("Session binding allocation failed"))?;
    bindings.push(SessionBinding {
        name: stored_name,
        value,
    });
    Ok(())
}

fn resolve_binding_value(
    bindings: &[SessionBinding],
    token: &str,
) -> Result<SessionValue, CliError> {
    let name = token
        .strip_prefix('@')
        .ok_or_else(|| CliError::usage("binding references start with @"))?;
    if name.contains('[') || name.contains(']') {
        return Err(CliError::usage(
            "indexed binding references select an Anddress, not a Session value",
        ));
    }
    validate_binding_name(name)?;
    match binding(bindings, name) {
        Some(SessionValue::Search(value)) => Ok(SessionValue::Search(value.clone())),
        Some(SessionValue::Pick(value)) => Ok(SessionValue::Pick(value.clone())),
        Some(SessionValue::Anddress(value)) => Ok(SessionValue::Anddress(value.clone())),
        Some(SessionValue::Edit(value)) => Ok(SessionValue::Edit(value.clone())),
        Some(SessionValue::View(value)) => Ok(SessionValue::View(value.clone())),
        Some(SessionValue::CheckAnddress(value)) => Ok(SessionValue::CheckAnddress(value.clone())),
        Some(SessionValue::CheckSearch(value)) => Ok(SessionValue::CheckSearch(value.clone())),
        Some(SessionValue::CheckPick(value)) => Ok(SessionValue::CheckPick(value.clone())),
        Some(SessionValue::Anchedress { .. }) => Err(CliError::usage(format!(
            "Anchedress binding cannot be cloned: {name}"
        ))),
        None => Err(CliError::usage(format!("unknown binding: {name}"))),
    }
}

fn resolve_anchedress<'a>(
    bindings: &'a [SessionBinding],
    token: &str,
) -> Result<(&'a Anchedress, AnddressTarget), CliError> {
    let name = token
        .strip_prefix('@')
        .ok_or_else(|| CliError::usage("binding references start with @"))?;
    if name.contains('[') || name.contains(']') {
        return Err(CliError::usage("Anchedress bindings cannot be indexed"));
    }
    validate_binding_name(name)?;
    match binding(bindings, name) {
        Some(SessionValue::Anchedress { handle, target }) => Ok((handle, *target)),
        Some(_) => Err(CliError::usage(format!(
            "binding is not an Anchedress: {name}"
        ))),
        None => Err(CliError::usage(format!("unknown binding: {name}"))),
    }
}

fn resolve_edit<'a>(bindings: &'a [SessionBinding], token: &str) -> Result<&'a Edit, CliError> {
    let name = token
        .strip_prefix('@')
        .ok_or_else(|| CliError::usage("Edit bindings start with @"))?;
    if name.contains('[') || name.contains(']') {
        return Err(CliError::usage("Edit bindings cannot be indexed"));
    }
    validate_binding_name(name)?;
    match binding(bindings, name) {
        Some(SessionValue::Edit(edit)) => Ok(edit),
        Some(_) => Err(CliError::usage(format!("binding is not an Edit: {name}"))),
        None => Err(CliError::usage(format!("unknown binding: {name}"))),
    }
}

fn resolve_session_ref(
    bindings: &[SessionBinding],
    refs: &[Anddress],
    token: &str,
) -> Result<Anddress, CliError> {
    if let Some(slot) = numeric_session_ref(token)? {
        return refs
            .get(slot)
            .cloned()
            .ok_or_else(|| CliError::usage(format!("numeric reference is out of range: {slot}")));
    }
    resolve_anddress(bindings, token)
}

fn resolve_anddress(bindings: &[SessionBinding], token: &str) -> Result<Anddress, CliError> {
    let reference = token
        .strip_prefix('@')
        .ok_or_else(|| CliError::usage("binding references start with @"))?;
    let Some(open) = reference.find('[') else {
        validate_binding_name(reference)?;
        return match binding(bindings, reference) {
            Some(SessionValue::Anddress(anddress)) => Ok(anddress.clone()),
            Some(SessionValue::Search(_)) => Err(CliError::usage(format!(
                "Search binding requires an index: {reference}"
            ))),
            Some(SessionValue::Pick(_)) => Err(CliError::usage(format!(
                "Pick binding requires an index: {reference}"
            ))),
            Some(SessionValue::Anchedress { .. }) => Err(CliError::usage(format!(
                "Anchedress binding cannot be used as an Anddress: {reference}"
            ))),
            Some(SessionValue::Edit(_)) => Err(CliError::usage(format!(
                "Edit binding cannot be used as an Anddress: {reference}"
            ))),
            Some(
                SessionValue::View(_)
                | SessionValue::CheckAnddress(_)
                | SessionValue::CheckSearch(_)
                | SessionValue::CheckPick(_),
            ) => Err(CliError::usage(format!(
                "result binding cannot be used as an Anddress: {reference}"
            ))),
            None => Err(CliError::usage(format!("unknown binding: {reference}"))),
        };
    };
    if !reference.ends_with(']')
        || reference[..open].contains(']')
        || reference[open + 1..reference.len() - 1].contains(['[', ']'])
    {
        return Err(CliError::usage("invalid indexed binding reference"));
    }
    let name = &reference[..open];
    validate_binding_name(name)?;
    let index = parse_session_index(&reference[open + 1..reference.len() - 1])?;
    match binding(bindings, name) {
        Some(SessionValue::Search(SearchOutcome::Found { anddresses })) => anddresses
            .get(index)
            .cloned()
            .ok_or_else(|| CliError::usage(format!("binding index is out of range: {name}"))),
        Some(SessionValue::Search(SearchOutcome::Empty)) => {
            Err(CliError::usage(format!("Search binding is empty: {name}")))
        }
        Some(SessionValue::Pick(PickOutcome::Selected { anddresses })) => anddresses
            .get(index)
            .cloned()
            .ok_or_else(|| CliError::usage(format!("binding index is out of range: {name}"))),
        Some(SessionValue::Pick(PickOutcome::Empty)) => {
            Err(CliError::usage(format!("Pick binding is empty: {name}")))
        }
        Some(SessionValue::Anddress(_)) => Err(CliError::usage(format!(
            "Anddress binding cannot be indexed: {name}"
        ))),
        Some(SessionValue::Anchedress { .. }) => Err(CliError::usage(format!(
            "Anchedress binding cannot be indexed: {name}"
        ))),
        Some(SessionValue::Edit(_)) => Err(CliError::usage(format!(
            "Edit binding cannot be indexed: {name}"
        ))),
        Some(
            SessionValue::View(_)
            | SessionValue::CheckAnddress(_)
            | SessionValue::CheckSearch(_)
            | SessionValue::CheckPick(_),
        ) => Err(CliError::usage(format!(
            "result binding cannot be indexed: {name}"
        ))),
        None => Err(CliError::usage(format!("unknown binding: {name}"))),
    }
}

fn numeric_session_ref(token: &str) -> Result<Option<usize>, CliError> {
    let Some(reference) = token.strip_prefix('@') else {
        return Ok(None);
    };
    if reference.is_empty() {
        return Err(CliError::usage("numeric reference is empty"));
    }
    if reference.starts_with(['+', '-']) {
        return Err(CliError::usage(
            "numeric reference must be an unsigned decimal",
        ));
    }
    if !reference.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(None);
    }
    if reference.len() > 1 && reference.starts_with('0') {
        return Err(CliError::usage("numeric reference must be canonical"));
    }
    reference
        .parse()
        .map(Some)
        .map_err(|_| CliError::usage("numeric reference is out of range"))
}

fn parse_session_index(value: &str) -> Result<usize, CliError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(CliError::usage(
            "binding index must be a zero-based integer",
        ));
    }
    value
        .parse()
        .map_err(|_| CliError::usage("binding index is out of range"))
}

fn validate_binding_name(name: &str) -> Result<(), CliError> {
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return Err(CliError::usage("binding name is empty"));
    };
    if !(first.is_ascii_alphabetic() || first == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(CliError::usage("invalid binding name"));
    }
    Ok(())
}

fn lex_line(line: &str) -> Result<Vec<String>, CliError> {
    if line.contains('\0') {
        return Err(CliError::usage("Session input must not contain NUL"));
    }
    let mut characters = line.chars().peekable();
    let mut tokens = Vec::new();
    while characters.peek().is_some() {
        while matches!(characters.peek(), Some(' ' | '\t')) {
            characters.next();
        }
        let Some(first) = characters.peek().copied() else {
            break;
        };
        if first == '"' {
            characters.next();
            let mut token = String::new();
            loop {
                match characters.next() {
                    Some('"') => break,
                    Some('\\') => match characters.next() {
                        Some('\\') => token.push('\\'),
                        Some('"') => token.push('"'),
                        Some('n') => token.push('\n'),
                        Some('r') => token.push('\r'),
                        Some('t') => token.push('\t'),
                        _ => return Err(CliError::usage("invalid quoted escape")),
                    },
                    Some(character) => token.push(character),
                    None => return Err(CliError::usage("unmatched quote")),
                }
            }
            if !matches!(characters.peek(), None | Some(' ' | '\t')) {
                return Err(CliError::usage("quoted token must end at a separator"));
            }
            tokens.push(token);
            continue;
        }
        let mut token = String::new();
        while let Some(character) = characters.peek().copied() {
            if matches!(character, ' ' | '\t') {
                break;
            }
            if character == '"' {
                return Err(CliError::usage("quote must begin a token"));
            }
            token.push(character);
            characters.next();
        }
        tokens.push(token);
    }
    Ok(tokens)
}

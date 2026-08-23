//! Human Search, View, Check, and initial Session adapter for Backwriter CLI V1.

use std::{
    env,
    ffi::OsString,
    io::{self, BufRead, BufWriter, Write},
    path::PathBuf,
    process::ExitCode,
};

use artext::{
    backwriter::{
        anddress::{Anddress, AnddressError, AnddressTarget, LineTerminator},
        check::CheckOutcome,
        search::{
            SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchScopeEntry, SearchTarget,
        },
        view::ViewOutcome,
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};

const USAGE: &str = "Usage:\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... view anddress <encoded-v3-Anddress>\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... check anddress <encoded-v3-Anddress>\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nOne-shot human Search, View, Check, and the initial Session slice are implemented.";

enum CliError {
    Usage(String),
    Execution(String),
    Stream(String),
}

impl CliError {
    fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    fn execution(message: impl Into<String>) -> Self {
        Self::Execution(message.into())
    }

    fn stream(message: impl Into<String>) -> Self {
        Self::Stream(message.into())
    }

    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage(_) => ExitCode::from(2),
            Self::Execution(_) | Self::Stream(_) => ExitCode::FAILURE,
        }
    }

    fn report(&self) {
        let mut stderr = io::stderr().lock();
        match self {
            Self::Usage(message) => {
                let _ = writeln!(stderr, "error: {message}\n\n{USAGE}");
            }
            Self::Execution(message) | Self::Stream(message) => {
                let _ = writeln!(stderr, "error: {message}");
            }
        }
    }
}

fn main() -> ExitCode {
    match execute() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            error.report();
            error.exit_code()
        }
    }
}

fn execute() -> Result<ExitCode, CliError> {
    let mut arguments = env::args_os().skip(1);
    let first = arguments.next();
    if matches!(first.as_deref(), Some(value) if value == "--help") && arguments.next().is_none() {
        let mut stdout = BufWriter::new(io::stdout().lock());
        writeln!(stdout, "{USAGE}").map_err(|error| CliError::stream(error.to_string()))?;
        stdout
            .flush()
            .map_err(|error| CliError::stream(error.to_string()))?;
        return Ok(ExitCode::SUCCESS);
    }

    let mut workspace = None;
    let mut admissions = Vec::new();
    let mut capability = first;
    loop {
        let Some(argument) = capability else {
            return Err(CliError::usage("missing capability"));
        };
        let argument = utf8(argument, "argument")?;
        match argument.as_str() {
            "--workspace" => {
                if workspace.is_some() {
                    return Err(CliError::usage("--workspace may appear only once"));
                }
                let path = arguments
                    .next()
                    .ok_or_else(|| CliError::usage("--workspace requires a path"))?;
                let path = PathBuf::from(path);
                if !path.is_absolute() {
                    return Err(CliError::usage("--workspace must be an absolute path"));
                }
                workspace = Some(path);
            }
            "--admit" => {
                let path = required_text(&mut arguments, "--admit")?;
                admissions.push(
                    AdmissionRoot::new(path).map_err(|error| CliError::usage(error.to_string()))?,
                );
            }
            "search" => {
                return execute_search(arguments, workspace, admissions)
                    .map(|()| ExitCode::SUCCESS);
            }
            "view" => {
                return execute_view(arguments, workspace, admissions).map(|()| ExitCode::SUCCESS);
            }
            "check" => {
                return execute_check(arguments, workspace, admissions).map(|()| ExitCode::SUCCESS);
            }
            "shell" => {
                if arguments.next().is_some() {
                    return Err(CliError::usage("shell accepts no operands"));
                }
                return execute_shell(workspace, admissions);
            }
            "pick" | "anchor" | "edit" | "apply" | "data" => {
                return Err(CliError::usage(format!(
                    "{argument} is not implemented in this slice"
                )));
            }
            "--json" | "--raw" => {
                return Err(CliError::usage(format!(
                    "{argument} is not implemented in this slice"
                )));
            }
            "--help" => return Err(CliError::usage("--help must be used alone")),
            _ => {
                return Err(CliError::usage(format!(
                    "unknown capability or option: {argument}"
                )));
            }
        }
        capability = arguments.next();
    }
}

fn execute_search(
    arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<(), CliError> {
    let arguments = text_arguments(arguments, "search argument")?;
    let request = parse_search(&arguments)?;
    let runtime = open_runtime(workspace, admissions)?;
    let outcome = run_search(&runtime, request)?;
    write_human(&outcome)
}

fn parse_search(arguments: &[String]) -> Result<SearchRequest, CliError> {
    let target = match required_token(arguments, 0, "search kind")? {
        "line" => SearchTarget::Line,
        "paragraph" => SearchTarget::Paragraph,
        "file" => SearchTarget::File,
        value => return Err(CliError::usage(format!("invalid search kind: {value}"))),
    };
    let query = SearchQuery::new(required_token(arguments, 1, "search query")?)
        .map_err(|error| CliError::usage(error.to_string()))?;

    let mut entries = Vec::new();
    let mut position = 2;
    while let Some(option) = arguments.get(position) {
        position += 1;
        match option.as_str() {
            "--source" => entries.push(
                SearchScopeEntry::source(required_token(arguments, position, "--source")?)
                    .map_err(|error| CliError::usage(error.to_string()))?,
            ),
            "--subtree" => entries.push(
                SearchScopeEntry::subtree(required_token(arguments, position, "--subtree")?)
                    .map_err(|error| CliError::usage(error.to_string()))?,
            ),
            "--json" | "--raw" => {
                return Err(CliError::usage(format!(
                    "{option} is not implemented in this slice"
                )));
            }
            "--admit" => return Err(CliError::usage("--admit must precede the capability")),
            _ => return Err(CliError::usage(format!("invalid search option: {option}"))),
        }
        position += 1;
    }
    let scope = if entries.is_empty() {
        SearchScope::all_admitted()
    } else {
        SearchScope::only(entries).map_err(|error| CliError::usage(error.to_string()))?
    };
    Ok(SearchRequest::new(query, scope, target))
}

fn execute_view(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<(), CliError> {
    let form = required_text(&mut arguments, "view input form")?;
    if form != "anddress" {
        if form == "anchored" {
            return Err(CliError::usage(
                "view anchored is not implemented in this slice",
            ));
        }
        return Err(CliError::usage("view requires the anddress input form"));
    }
    let encoded = required_text(&mut arguments, "view anddress")?;
    if arguments.next().is_some() {
        return Err(CliError::usage("view accepts exactly one anddress operand"));
    }
    let anddress = decode_anddress(encoded)?;
    let runtime = open_runtime(workspace, admissions)?;
    let outcome = run_view(&runtime, &anddress)?;
    write_view(outcome)
}

fn execute_check(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<(), CliError> {
    let form = required_text(&mut arguments, "check input form")?;
    if form != "anddress" {
        if matches!(form.as_str(), "search" | "pick") {
            return Err(CliError::usage(format!(
                "check {form} is not implemented in this slice"
            )));
        }
        return Err(CliError::usage("check requires the anddress input form"));
    }
    let encoded = required_text(&mut arguments, "check anddress")?;
    if arguments.next().is_some() {
        return Err(CliError::usage(
            "check accepts exactly one anddress operand",
        ));
    }
    let anddress = decode_anddress(encoded)?;
    let runtime = open_runtime(workspace, admissions)?;
    let outcome = run_check(&runtime, anddress)?;
    write_check(outcome)
}

fn run_search(
    runtime: &WorkspaceRuntime,
    request: SearchRequest,
) -> Result<SearchOutcome, CliError> {
    runtime
        .search(&request)
        .map_err(|error| CliError::execution(error.to_string()))
}

fn run_view(runtime: &WorkspaceRuntime, anddress: &Anddress) -> Result<ViewOutcome, CliError> {
    runtime
        .view(anddress)
        .map_err(|error| CliError::execution(error.to_string()))
}

fn run_check(
    runtime: &WorkspaceRuntime,
    anddress: Anddress,
) -> Result<CheckOutcome<Option<Anddress>>, CliError> {
    runtime
        .check(anddress)
        .map_err(|error| CliError::execution(error.to_string()))
}

fn decode_anddress(encoded: String) -> Result<Anddress, CliError> {
    match Anddress::decode(encoded.as_bytes()) {
        Ok(anddress) => Ok(anddress),
        Err(AnddressError::Resource) => Err(CliError::execution(
            "Anddress decoding ran out of resources",
        )),
        Err(error) => Err(CliError::usage(error.to_string())),
    }
}

fn open_runtime(
    workspace: Option<PathBuf>,
    mut admissions: Vec<AdmissionRoot>,
) -> Result<WorkspaceRuntime, CliError> {
    if admissions.is_empty() {
        admissions.push(AdmissionRoot::new(".").expect("dot admission is valid"));
    }
    let admission =
        WorkspaceAdmission::new(admissions).map_err(|error| CliError::usage(error.to_string()))?;
    let workspace = match workspace {
        Some(path) => path,
        None => env::current_dir().map_err(|error| CliError::execution(error.to_string()))?,
    };
    WorkspaceRuntime::open(&workspace, admission)
        .map_err(|error| CliError::execution(error.to_string()))
}

fn utf8(argument: OsString, context: &str) -> Result<String, CliError> {
    argument
        .into_string()
        .map_err(|_| CliError::usage(format!("{context} must be UTF-8")))
}

fn text_arguments(
    arguments: impl Iterator<Item = OsString>,
    context: &str,
) -> Result<Vec<String>, CliError> {
    arguments.map(|argument| utf8(argument, context)).collect()
}

fn required_text(
    arguments: &mut impl Iterator<Item = OsString>,
    option: &str,
) -> Result<String, CliError> {
    let argument = arguments
        .next()
        .ok_or_else(|| CliError::usage(format!("{option} requires a value")))?;
    utf8(argument, option)
}

fn required_token<'a>(
    arguments: &'a [String],
    position: usize,
    context: &str,
) -> Result<&'a str, CliError> {
    arguments
        .get(position)
        .map(String::as_str)
        .ok_or_else(|| CliError::usage(format!("{context} requires a value")))
}

fn write_human(outcome: &SearchOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        match outcome {
            SearchOutcome::Empty => writeln!(stdout, "Found 0"),
            SearchOutcome::Found { anddresses } => {
                writeln!(stdout, "Found {}", anddresses.len())?;
                for (index, anddress) in anddresses.iter().enumerate() {
                    match &anddress.target {
                        AnddressTarget::File => {
                            writeln!(stdout, "{index}\tFile\t{}", anddress.logical_path)?;
                        }
                        AnddressTarget::Paragraph { ordinal } => {
                            writeln!(
                                stdout,
                                "{index}\tParagraph\t{}:{ordinal}",
                                anddress.logical_path
                            )?;
                        }
                        AnddressTarget::Line { ordinal, .. } => {
                            writeln!(stdout, "{index}\tLine\t{}:{ordinal}", anddress.logical_path)?;
                        }
                    }
                }
                Ok(())
            }
        }
    })();
    result.map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_view(outcome: ViewOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        match outcome {
            ViewOutcome::File { text } | ViewOutcome::Paragraph { text, .. } => {
                stdout.write_all(text.as_bytes())?;
            }
            ViewOutcome::Line {
                content,
                terminator,
                ..
            } => {
                stdout.write_all(content.as_bytes())?;
                stdout.write_all(match terminator {
                    LineTerminator::None => b"",
                    LineTerminator::Lf => b"\n",
                    LineTerminator::Cr => b"\r",
                    LineTerminator::Crlf => b"\r\n",
                })?;
            }
        }
        Ok(())
    })();
    result.map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_check(outcome: CheckOutcome<Option<Anddress>>) -> Result<(), CliError> {
    let filtered = outcome.filtered.is_some();
    let report = outcome.report;
    let status = match (
        filtered,
        report.current_count(),
        report.removed_count(),
        report.unavailable_count(),
        report.checked_count(),
    ) {
        (true, 1, 0, 0, 1) => "Current",
        (false, 0, 1, 0, 1) => "NotCurrent",
        (true, 0, 0, 1, 1) => "Unavailable",
        _ => return Err(CliError::execution("inconsistent raw Check report")),
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{status}").map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

#[derive(Clone)]
enum SessionValue {
    Search(SearchOutcome),
    Anddress(Anddress),
}

struct SessionBinding {
    name: String,
    value: SessionValue,
}

enum SessionControl {
    Continue,
    Exit,
}

fn execute_shell(
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<ExitCode, CliError> {
    let runtime = open_runtime(workspace, admissions)?;
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut bindings = Vec::new();
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
        match execute_session_command(&runtime, &mut bindings, &tokens) {
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

fn session_error_status(error: &CliError) -> u8 {
    match error {
        CliError::Usage(_) => 2,
        CliError::Execution(_) => 1,
        CliError::Stream(_) => 1,
    }
}

fn execute_session_command(
    runtime: &WorkspaceRuntime,
    bindings: &mut Vec<SessionBinding>,
    tokens: &[String],
) -> Result<SessionControl, CliError> {
    match tokens[0].as_str() {
        "search" => {
            let outcome = run_search(runtime, parse_search(&tokens[1..])?)?;
            write_human(&outcome)?;
            Ok(SessionControl::Continue)
        }
        "let" => {
            execute_let(runtime, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "view" => {
            execute_session_view(runtime, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "check" => {
            execute_session_check(runtime, bindings, tokens)?;
            Ok(SessionControl::Continue)
        }
        "exit" if tokens.len() == 1 => Ok(SessionControl::Exit),
        "exit" => Err(CliError::usage("exit accepts no operands")),
        capability => Err(CliError::usage(format!(
            "unsupported Session command: {capability}"
        ))),
    }
}

fn execute_let(
    runtime: &WorkspaceRuntime,
    bindings: &mut Vec<SessionBinding>,
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
    let value = if right_hand_side == "search" {
        let outcome = run_search(runtime, parse_search(&tokens[4..])?)?;
        write_human(&outcome)?;
        SessionValue::Search(outcome)
    } else {
        if tokens.len() != 4 {
            return Err(CliError::usage("let reference accepts exactly one operand"));
        }
        if right_hand_side.contains('[') || right_hand_side.contains(']') {
            SessionValue::Anddress(resolve_anddress(bindings, right_hand_side)?)
        } else {
            resolve_binding_value(bindings, right_hand_side)?
        }
    };
    store_binding(bindings, name, value)
}

fn execute_session_view(
    runtime: &WorkspaceRuntime,
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<(), CliError> {
    session_anddress_form(tokens, "view")?;
    let anddress = resolve_anddress(bindings, &tokens[2])?;
    write_view(run_view(runtime, &anddress)?)
}

fn execute_session_check(
    runtime: &WorkspaceRuntime,
    bindings: &[SessionBinding],
    tokens: &[String],
) -> Result<(), CliError> {
    session_anddress_form(tokens, "check")?;
    let anddress = resolve_anddress(bindings, &tokens[2])?;
    write_check(run_check(runtime, anddress)?)
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
    binding(bindings, name)
        .cloned()
        .ok_or_else(|| CliError::usage(format!("unknown binding: {name}")))
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
        Some(SessionValue::Anddress(_)) => Err(CliError::usage(format!(
            "Anddress binding cannot be indexed: {name}"
        ))),
        None => Err(CliError::usage(format!("unknown binding: {name}"))),
    }
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

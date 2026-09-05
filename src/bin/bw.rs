//! Backwriter CLI entry, one-shot dispatch and platform Update.

#[path = "bw/error.rs"]
mod error;
#[path = "bw/help.rs"]
mod help;
#[path = "bw/output.rs"]
mod output;
#[path = "bw/shell.rs"]
mod shell;

use backwriter::backwriter::anddress::{Anddress, AnddressError, AnddressTarget, LineTerminator};
use backwriter::backwriter::apply::ApplyError;
use backwriter::backwriter::check::{CheckOutcome, CheckStatus};
use backwriter::backwriter::edit::Edit;
use backwriter::backwriter::search::{
    SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchScopeEntry, SearchTarget,
};
use backwriter::backwriter::view::ViewOutcome;
use backwriter::runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime};
use error::{
    CliError, check_usage, edit_usage, map_edit_content_error, map_edit_error_for_edit,
    promote_check_usage, promote_search_usage, promote_top_usage, promote_view_usage, search_usage,
    view_usage,
};
use help::{SHELL_HELP, TOP_LEVEL_HELP, UPDATE_HELP, VERSION_HELP, write_command_help, write_help};
use output::{
    write_check_json, write_check_status, write_edit, write_search, write_search_json, write_view,
    write_view_json,
};
use shell::execute_shell;
#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
use std::process::ExitStatus;
use std::{
    env,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(test)]
#[path = "../../tests/support.rs"]
mod test_support;

#[cfg(unix)]
const INSTALL_SH_URL: &str = "https://backwriter.pentagration.com/install.sh";
#[cfg(windows)]
const INSTALL_PS1_URL: &str = "https://backwriter.pentagration.com/install.ps1";

#[derive(Clone, Copy, Eq, PartialEq)]
enum OutputMode {
    Human,
    Json,
    Raw,
}

struct UpdateTemporary {
    root: PathBuf,
    armed: bool,
}

impl UpdateTemporary {
    fn create() -> Result<Self, CliError> {
        let parent = env::temp_dir();
        let metadata = fs::symlink_metadata(&parent).map_err(|error| {
            CliError::execution(format!("update temporary root is unavailable: {error}"))
        })?;
        if !parent.is_absolute() || !safe_update_directory(&metadata) {
            return Err(CliError::execution(
                "update temporary root must be an absolute ordinary directory",
            ));
        }

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| CliError::execution("system time is unavailable"))?
            .as_nanos();
        let process = u128::from(std::process::id());
        for attempt in 0_u128..128 {
            let nonce = nanos.wrapping_add(process << 64).wrapping_add(attempt);
            let candidate = parent.join(format!("backwriter-update-{nonce:032x}"));
            match create_update_directory(&candidate) {
                Ok(()) => {
                    let metadata = match fs::symlink_metadata(&candidate) {
                        Ok(metadata) => metadata,
                        Err(error) => {
                            let _ = fs::remove_dir(&candidate);
                            return Err(CliError::execution(format!(
                                "could not inspect update temporary directory: {error}"
                            )));
                        }
                    };
                    if !safe_update_directory(&metadata) {
                        let _ = fs::remove_dir(&candidate);
                        return Err(CliError::execution(
                            "update temporary path is not an ordinary directory",
                        ));
                    }
                    return Ok(Self {
                        root: candidate,
                        armed: true,
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(CliError::execution(format!(
                        "could not create update temporary directory: {error}"
                    )));
                }
            }
        }
        Err(CliError::execution(
            "could not create a unique update temporary directory",
        ))
    }

    fn installer(&self, name: &str) -> Result<PathBuf, CliError> {
        let path = self.root.join(name);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                CliError::execution(format!("could not create update installer file: {error}"))
            })?;
        Ok(path)
    }

    #[cfg(unix)]
    fn cleanup(mut self) -> Result<(), CliError> {
        fs::remove_dir_all(&self.root).map_err(|error| {
            CliError::execution(format!(
                "could not remove update temporary directory: {error}"
            ))
        })?;
        self.armed = false;
        Ok(())
    }

    #[cfg(windows)]
    fn handoff(mut self) {
        self.armed = false;
    }
}

impl Drop for UpdateTemporary {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn safe_update_directory(metadata: &fs::Metadata) -> bool {
    let safe = metadata.is_dir() && !metadata.file_type().is_symlink();
    #[cfg(windows)]
    let safe = safe && metadata.file_attributes() & 0x400 == 0;
    safe
}

#[cfg(unix)]
fn create_update_directory(path: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn create_update_directory(path: &Path) -> io::Result<()> {
    fs::DirBuilder::new().create(path)
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
        write_help(TOP_LEVEL_HELP)?;
        return Ok(ExitCode::SUCCESS);
    }

    let mut workspace = None;
    let mut admissions = Vec::new();
    let mut output = OutputMode::Human;
    let mut capability = first;
    loop {
        let Some(argument) = capability else {
            return Err(CliError::top_usage("command.missing", "missing capability"));
        };
        let argument = utf8(argument, "argument")
            .map_err(|error| promote_top_usage(error, "global.argument_utf8"))?;
        match argument.as_str() {
            "--workspace" => {
                if workspace.is_some() {
                    return Err(CliError::top_usage(
                        "global.workspace_duplicate",
                        "--workspace may appear only once",
                    ));
                }
                let path = arguments.next().ok_or_else(|| {
                    CliError::top_usage("global.workspace_missing", "--workspace requires a path")
                })?;
                let path = PathBuf::from(path);
                if !path.is_absolute() {
                    return Err(CliError::top_usage(
                        "global.workspace_invalid",
                        "--workspace must be an absolute path",
                    ));
                }
                workspace = Some(path);
            }
            "--admit" => {
                let path = required_text(&mut arguments, "--admit")
                    .map_err(|error| promote_top_usage(error, "global.admit_missing"))?;
                admissions.push(AdmissionRoot::new(path).map_err(|error| {
                    CliError::top_usage("global.admit_invalid", error.to_string())
                })?);
            }
            "--json" => {
                if output != OutputMode::Human {
                    return Err(CliError::top_usage(
                        "global.output_duplicate",
                        "only one output option may appear",
                    ));
                }
                output = OutputMode::Json;
            }
            "--raw" => {
                if output != OutputMode::Human {
                    return Err(CliError::top_usage(
                        "global.output_duplicate",
                        "only one output option may appear",
                    ));
                }
                output = OutputMode::Raw;
            }
            "version" => {
                let trailing: Vec<OsString> = arguments.collect();
                if workspace.is_none()
                    && admissions.is_empty()
                    && output == OutputMode::Human
                    && trailing.len() == 1
                    && trailing[0] == "--help"
                {
                    write_command_help("version")?;
                    return Ok(ExitCode::SUCCESS);
                }
                if workspace.is_some()
                    || !admissions.is_empty()
                    || output != OutputMode::Human
                    || !trailing.is_empty()
                {
                    return Err(CliError::command_usage(
                        "version.extra_operand",
                        "version accepts no options or operands",
                        VERSION_HELP,
                        "bw help version",
                    ));
                }
                write_version()?;
                return Ok(ExitCode::SUCCESS);
            }
            "update" => {
                let trailing: Vec<OsString> = arguments.collect();
                if workspace.is_none()
                    && admissions.is_empty()
                    && output == OutputMode::Human
                    && trailing.len() == 1
                    && trailing[0] == "--help"
                {
                    write_command_help("update")?;
                    return Ok(ExitCode::SUCCESS);
                }
                if workspace.is_some()
                    || !admissions.is_empty()
                    || output != OutputMode::Human
                    || !trailing.is_empty()
                {
                    return Err(CliError::command_usage(
                        "update.extra_operand",
                        "update accepts no options or operands",
                        UPDATE_HELP,
                        "bw help update",
                    ));
                }
                return execute_update();
            }
            "search" => {
                return execute_search(arguments, workspace, admissions, output)
                    .map(|()| ExitCode::SUCCESS);
            }
            "view" => {
                return execute_view(arguments, workspace, admissions, output)
                    .map(|()| ExitCode::SUCCESS);
            }
            "check" => {
                return execute_check(arguments, workspace, admissions, output)
                    .map(|()| ExitCode::SUCCESS);
            }
            "edit" => {
                return execute_edit(arguments, workspace, admissions, output)
                    .map(|()| ExitCode::SUCCESS);
            }
            "shell" => {
                if output != OutputMode::Human {
                    return Err(CliError::command_usage(
                        "shell.output_unsupported",
                        "output options are unsupported for shell",
                        SHELL_HELP,
                        "bw help shell",
                    ));
                }
                let trailing: Vec<OsString> = arguments.collect();
                if trailing.len() == 1 && trailing[0] == "--help" {
                    write_command_help("shell")?;
                    return Ok(ExitCode::SUCCESS);
                }
                if !trailing.is_empty() {
                    return Err(CliError::command_usage(
                        "shell.extra_operand",
                        "shell accepts no operands",
                        SHELL_HELP,
                        "bw help shell",
                    ));
                }
                return execute_shell(workspace, admissions);
            }
            "help" => {
                if workspace.is_some() || !admissions.is_empty() || output != OutputMode::Human {
                    return Err(CliError::top_usage(
                        "help.global_option",
                        "help accepts no global options",
                    ));
                }
                let Some(command) = arguments.next() else {
                    write_help(TOP_LEVEL_HELP)?;
                    return Ok(ExitCode::SUCCESS);
                };
                if arguments.next().is_some() {
                    return Err(CliError::top_usage(
                        "help.extra_operand",
                        "help accepts at most one command",
                    ));
                }
                write_command_help(
                    &utf8(command, "help command")
                        .map_err(|error| promote_top_usage(error, "help.command_utf8"))?,
                )?;
                return Ok(ExitCode::SUCCESS);
            }
            "pick" | "anchor" | "apply" | "data" => {
                if output != OutputMode::Human {
                    return Err(CliError::top_usage(
                        "capability.output_unsupported",
                        "output options are unsupported for this capability",
                    ));
                }
                return Err(CliError::top_usage(
                    "capability.one_shot_unavailable",
                    format!("{argument} has no one-shot command; use bw shell"),
                ));
            }
            "--help" => {
                return Err(CliError::top_usage(
                    "help.position",
                    "--help must be used alone",
                ));
            }
            _ => {
                return Err(CliError::top_usage(
                    "command.unknown",
                    format!("unknown capability or option: {argument}"),
                ));
            }
        }
        capability = arguments.next();
    }
}

fn write_version() -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "Backwriter {}", env!("CARGO_PKG_VERSION"))
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn download_update_installer(curl: &str, url: &str, destination: &Path) -> Result<(), CliError> {
    let status = Command::new(curl)
        .args([
            "--fail",
            "--show-error",
            "--silent",
            "--location",
            "--proto",
            "=https",
            "--proto-redir",
            "=https",
            "--tlsv1.2",
            "--output",
        ])
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| {
            CliError::execution(format!(
                "could not start update download with {curl}: {error}"
            ))
        })?;
    if !status.success() {
        return Err(CliError::execution("could not download update installer"));
    }
    let metadata = fs::symlink_metadata(destination).map_err(|error| {
        CliError::execution(format!(
            "could not inspect downloaded update installer: {error}"
        ))
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(CliError::execution(
            "downloaded update installer is not an ordinary file",
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn propagated_exit_code(status: ExitStatus) -> ExitCode {
    match status.code() {
        Some(code) if (0..=i32::from(u8::MAX)).contains(&code) => ExitCode::from(code as u8),
        _ => ExitCode::FAILURE,
    }
}

#[cfg(unix)]
fn execute_update() -> Result<ExitCode, CliError> {
    let temporary = UpdateTemporary::create()?;
    let installer = temporary.installer("install.sh")?;
    download_update_installer("curl", INSTALL_SH_URL, &installer)?;
    let status = Command::new("sh")
        .arg(&installer)
        .status()
        .map_err(|error| {
            CliError::execution(format!(
                "could not start downloaded update installer: {error}"
            ))
        })?;
    temporary.cleanup()?;
    Ok(propagated_exit_code(status))
}

#[cfg(windows)]
fn execute_update() -> Result<ExitCode, CliError> {
    let temporary = UpdateTemporary::create()?;
    let installer = temporary.installer("install.ps1")?;
    download_update_installer("curl.exe", INSTALL_PS1_URL, &installer)?;
    Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
        ])
        .arg(&installer)
        .arg("-WaitForProcessId")
        .arg(std::process::id().to_string())
        .arg("-BootstrapRoot")
        .arg(&temporary.root)
        .spawn()
        .map_err(|error| {
            CliError::execution(format!("could not hand off to PowerShell updater: {error}"))
        })?;
    temporary.handoff();
    Ok(ExitCode::SUCCESS)
}

#[cfg(not(any(unix, windows)))]
fn execute_update() -> Result<ExitCode, CliError> {
    Err(CliError::execution(
        "update is unsupported on this operating system",
    ))
}

fn execute_search(
    arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
) -> Result<(), CliError> {
    if output == OutputMode::Raw {
        return Err(search_usage(
            "search.output_unsupported",
            "--raw is supported only for View",
        ));
    }
    let arguments = text_arguments(arguments, "search argument")?;
    if arguments.len() == 1 && arguments[0] == "--help" {
        return write_command_help("search");
    }
    let request = parse_search(&arguments).map_err(promote_search_usage)?;
    let runtime = open_runtime(workspace, admissions, Some("search"))?;
    let outcome = run_search(&runtime, request)?;
    match output {
        OutputMode::Human => write_search(&outcome),
        OutputMode::Json => write_search_json(&outcome),
        OutputMode::Raw => unreachable!(),
    }
}

fn parse_search(arguments: &[String]) -> Result<SearchRequest, CliError> {
    let kind = required_token(arguments, 0, "search kind")?;
    if kind == "/file" {
        if arguments.len() != 2 {
            return Err(CliError::usage(
                "search /file accepts exactly one logical path",
            ));
        }
        return SearchRequest::exact_file(&arguments[1])
            .map_err(|error| CliError::usage(error.to_string()));
    }
    let target = match kind {
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
                return Err(CliError::usage(
                    "output options must precede the capability",
                ));
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

fn execute_edit(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
) -> Result<(), CliError> {
    if output == OutputMode::Raw {
        return Err(edit_usage(
            "edit.output_unsupported",
            "--raw is supported only for View",
        ));
    }
    let form = arguments
        .next()
        .ok_or_else(|| edit_usage("edit.form_missing", "edit input form requires a value"))
        .and_then(|value| utf8_for_edit(value, "edit input form"))?;
    if form == "--help" && arguments.next().is_none() {
        return write_command_help("edit");
    }
    if form != "anddress" {
        return Err(edit_usage(
            "edit.form_invalid",
            "edit requires the anddress input form",
        ));
    }
    let encoded = arguments
        .next()
        .ok_or_else(|| edit_usage("edit.address_missing", "edit anddress requires a value"))
        .and_then(|value| utf8_for_edit(value, "edit anddress"))?;
    let content_selector = arguments
        .next()
        .ok_or_else(|| edit_usage("edit.content_missing", "edit content requires a value"))?;
    if arguments.next().is_some() {
        return Err(edit_usage(
            "edit.extra_operand",
            "edit anddress accepts exactly one anddress and Content selector",
        ));
    }

    let anddress = decode_anddress_for_edit(encoded)?;
    let content = if content_selector == "--stdin" {
        read_edit_stdin()?
    } else {
        utf8_for_edit(content_selector, "edit content")?
    };
    let content = prepare_replace_content(&anddress, content).map_err(map_edit_content_error)?;

    let edit = Edit::Replace {
        target: anddress,
        content,
    };
    edit.validate().map_err(map_edit_error_for_edit)?;
    let mut runtime = open_runtime(workspace, admissions, Some("edit"))?;
    let receipt = runtime
        .apply_replace(&edit)
        .map_err(|error: ApplyError| CliError::execution(error.to_string()))?;
    write_edit(receipt, output)
}

fn utf8_for_edit(value: OsString, context: &str) -> Result<String, CliError> {
    value
        .into_string()
        .map_err(|_| edit_usage("edit.utf8_invalid", format!("{context} must be UTF-8")))
}

fn read_edit_stdin() -> Result<String, CliError> {
    let mut bytes = Vec::new();
    io::stdin()
        .lock()
        .read_to_end(&mut bytes)
        .map_err(|error| {
            CliError::execution(format!("could not read edit standard input: {error}"))
        })?;
    String::from_utf8(bytes)
        .map_err(|_| CliError::execution("edit standard input must be valid UTF-8"))
}

enum ReplaceContentError {
    Nul,
    LineTerminator,
    Resource,
}

fn prepare_replace_content(
    anddress: &Anddress,
    mut content: String,
) -> Result<String, ReplaceContentError> {
    if content.contains('\0') {
        return Err(ReplaceContentError::Nul);
    }
    if let Some(terminator) = anddress.terminator() {
        if content.contains(['\r', '\n']) {
            return Err(ReplaceContentError::LineTerminator);
        }
        let terminator = match terminator {
            LineTerminator::None => "",
            LineTerminator::Lf => "\n",
            LineTerminator::Cr => "\r",
            LineTerminator::Crlf => "\r\n",
        };
        content
            .try_reserve_exact(terminator.len())
            .map_err(|_| ReplaceContentError::Resource)?;
        content.push_str(terminator);
    }
    Ok(content)
}

fn decode_anddress_for_edit(encoded: String) -> Result<Anddress, CliError> {
    match Anddress::decode(encoded.as_bytes()) {
        Ok(anddress) => Ok(anddress),
        Err(AnddressError::Resource) => Err(CliError::execution(
            "Anddress decoding ran out of resources",
        )),
        Err(AnddressError::UnsupportedVersion) => Err(edit_usage(
            "edit.address_unsupported",
            AnddressError::UnsupportedVersion.to_string(),
        )),
        Err(error) => Err(edit_usage("edit.address_invalid", error.to_string())),
    }
}

fn execute_view(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
) -> Result<(), CliError> {
    let form = required_text(&mut arguments, "view input form").map_err(promote_view_usage)?;
    if form == "--help" && arguments.next().is_none() {
        return write_command_help("view");
    }
    if form != "anddress" {
        if form == "anchored" {
            return Err(view_usage(
                "view.form_unavailable",
                "view anchored has no one-shot form; use bw shell",
            ));
        }
        return Err(view_usage(
            "view.form_invalid",
            "view requires the anddress input form",
        ));
    }
    let arguments = text_arguments(arguments, "view operand").map_err(promote_view_usage)?;
    let as_index = arguments.iter().position(|argument| argument == "--as");
    let (encoded, projection) = match as_index {
        Some(index) => {
            if arguments[index + 1..].len() != 1 {
                return Err(view_usage(
                    "view.target_invalid",
                    "view --as requires exactly one target and must be last",
                ));
            }
            (
                &arguments[..index],
                Some(parse_view_target(&arguments[index + 1]).map_err(promote_view_usage)?),
            )
        }
        None => (arguments.as_slice(), None),
    };
    if encoded.is_empty() {
        return Err(view_usage(
            "view.operand_missing",
            "view requires at least one anddress operand",
        ));
    }
    if encoded.iter().any(|argument| argument == "--as") {
        return Err(view_usage(
            "view.target_duplicate",
            "view accepts --as only once",
        ));
    }
    if encoded.len() != 1 && output != OutputMode::Json {
        return Err(view_usage(
            "view.output_unsupported",
            "batch View requires --json",
        ));
    }
    if encoded.len() != 1 && projection.is_none() {
        return Err(view_usage(
            "view.projection_missing",
            "batch View requires --as",
        ));
    }
    let mut anddresses = Vec::new();
    anddresses
        .try_reserve_exact(encoded.len())
        .map_err(|_| CliError::execution("View input allocation failed"))?;
    for value in encoded {
        anddresses.push(decode_anddress(value.clone()).map_err(promote_view_usage)?);
    }
    let runtime = open_runtime(workspace, admissions, Some("view"))?;
    let projection = projection.unwrap_or_else(|| anddresses[0].target());
    if anddresses.len() == 1 {
        let outcome = run_view(&runtime, &anddresses[0], projection)?;
        match output {
            OutputMode::Human | OutputMode::Raw => write_view(&outcome),
            OutputMode::Json => write_view_json(std::slice::from_ref(&outcome)),
        }
    } else {
        let outcomes = runtime
            .view_batch(&anddresses, Some(projection))
            .map_err(|error| CliError::execution(error.to_string()))?;
        write_view_json(&outcomes)
    }
}

fn parse_view_target(value: &str) -> Result<AnddressTarget, CliError> {
    match value {
        "line" => Ok(AnddressTarget::Line),
        "paragraph" => Ok(AnddressTarget::Paragraph),
        "file" => Ok(AnddressTarget::File),
        _ => Err(CliError::usage(
            "view --as requires line, paragraph, or file",
        )),
    }
}

fn execute_check(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
) -> Result<(), CliError> {
    if output == OutputMode::Raw {
        return Err(check_usage(
            "check.output_unsupported",
            "--raw is supported only for View",
        ));
    }
    let form = required_text(&mut arguments, "check input form").map_err(promote_check_usage)?;
    if form == "--help" && arguments.next().is_none() {
        return write_command_help("check");
    }
    if form != "anddress" {
        if matches!(form.as_str(), "search" | "pick") {
            return Err(check_usage(
                "check.form_unavailable",
                format!("check {form} has no one-shot form; use bw shell"),
            ));
        }
        return Err(check_usage(
            "check.form_invalid",
            "check requires the anddress input form",
        ));
    }
    let mut anddresses = Vec::new();
    let encoded = required_text(&mut arguments, "check anddress").map_err(promote_check_usage)?;
    anddresses
        .try_reserve(1)
        .map_err(|_| CliError::execution("Check input allocation failed"))?;
    anddresses.push(decode_anddress(encoded).map_err(promote_check_usage)?);
    for encoded in arguments {
        anddresses
            .try_reserve(1)
            .map_err(|_| CliError::execution("Check input allocation failed"))?;
        anddresses.push(
            decode_anddress(utf8(encoded, "check anddress").map_err(promote_check_usage)?)
                .map_err(promote_check_usage)?,
        );
    }
    if anddresses.len() != 1 && output != OutputMode::Json {
        return Err(check_usage(
            "check.output_unsupported",
            "checking multiple Anddresses requires --json",
        ));
    }
    let runtime = open_runtime(workspace, admissions, Some("check"))?;
    let statuses = run_check_batch(&runtime, &anddresses)?;
    match output {
        OutputMode::Human => write_check_status(statuses[0]),
        OutputMode::Json => write_check_json(&anddresses, &statuses),
        OutputMode::Raw => unreachable!(),
    }
}

fn run_search(
    runtime: &WorkspaceRuntime,
    request: SearchRequest,
) -> Result<SearchOutcome, CliError> {
    runtime
        .search(&request)
        .map_err(|error| CliError::execution(error.to_string()))
}

fn run_view(
    runtime: &WorkspaceRuntime,
    anddress: &Anddress,
    projection: AnddressTarget,
) -> Result<ViewOutcome, CliError> {
    runtime
        .view(anddress, projection)
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

fn run_check_batch(
    runtime: &WorkspaceRuntime,
    anddresses: &[Anddress],
) -> Result<Vec<CheckStatus>, CliError> {
    runtime
        .check_batch(anddresses)
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
    command: Option<&'static str>,
) -> Result<WorkspaceRuntime, CliError> {
    if admissions.is_empty() {
        admissions.push(AdmissionRoot::new(".").expect("dot admission is valid"));
    }
    let admission = WorkspaceAdmission::new(admissions).map_err(|error| {
        let message = error.to_string();
        match command {
            Some("search") => search_usage("search.request_invalid", message),
            Some("view") => view_usage("view.request_invalid", message),
            Some("check") => check_usage("check.request_invalid", message),
            Some("edit") => edit_usage("edit.request_invalid", message),
            None => CliError::usage(message),
            Some(_) => unreachable!("one-shot runtime command"),
        }
    })?;
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

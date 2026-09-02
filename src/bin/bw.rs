//! Human and JSON Search/View/Check, one-shot Edit, and Session adapter for Backwriter CLI V1.

use std::{
    env,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, BufRead, BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
use std::process::ExitStatus;

use backwriter::{
    backwriter::{
        anchor::{Anchedress, AnchorError, AnchorOutcome},
        anddress::{Anddress, AnddressError, AnddressTarget, LineTerminator},
        apply::ApplyError,
        check::{CheckOutcome, CheckReport},
        data::{DataError, DataKind, DataName, DataStore, StoreError},
        edit::{Edit, EditError, Position},
        pick::{PickError, PickOutcome, PickPredicate, PickTargetKind, pick},
        search::{
            SearchOccurrence, SearchOutcome, SearchPosition, SearchQuery, SearchRequest,
            SearchScope, SearchScopeEntry, SearchTarget,
        },
        view::ViewOutcome,
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};

const USAGE: &str = "Usage:\n  bw version\n  bw update\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] search /file <logical-path>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json|--raw] view anddress <encoded-v4-Anddress>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... [--json] check anddress <encoded-v4-Anddress>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... edit anddress <encoded-v4-Anddress> <content>\n  bw [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... shell\n\nOne-shot Version, Update, human and JSON Search, View, and Check, raw View, and Anddress-first Edit plus Session Pick, batch Check, Anchor, Edit, Apply, result binding, and Data are implemented.";

#[cfg(unix)]
const INSTALL_SH_URL: &str = "https://backwriter.pentagration.com/install.sh";
#[cfg(windows)]
const INSTALL_PS1_URL: &str = "https://backwriter.pentagration.com/install.ps1";

enum CliError {
    Usage(String),
    Execution(String),
    Stream(String),
}

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
    let mut output = OutputMode::Human;
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
            "--json" => {
                if output != OutputMode::Human {
                    return Err(CliError::usage("only one output option may appear"));
                }
                output = OutputMode::Json;
            }
            "--raw" => {
                if output != OutputMode::Human {
                    return Err(CliError::usage("only one output option may appear"));
                }
                output = OutputMode::Raw;
            }
            "version" => {
                if workspace.is_some()
                    || !admissions.is_empty()
                    || output != OutputMode::Human
                    || arguments.next().is_some()
                {
                    return Err(CliError::usage("version accepts no options or operands"));
                }
                write_version()?;
                return Ok(ExitCode::SUCCESS);
            }
            "update" => {
                if workspace.is_some()
                    || !admissions.is_empty()
                    || output != OutputMode::Human
                    || arguments.next().is_some()
                {
                    return Err(CliError::usage("update accepts no options or operands"));
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
                    return Err(CliError::usage("output options are unsupported for shell"));
                }
                if arguments.next().is_some() {
                    return Err(CliError::usage("shell accepts no operands"));
                }
                return execute_shell(workspace, admissions);
            }
            "pick" | "anchor" | "apply" | "data" => {
                if output != OutputMode::Human {
                    return Err(CliError::usage(
                        "output options are unsupported for this capability",
                    ));
                }
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
        return Err(CliError::usage("--raw is supported only for View"));
    }
    let arguments = text_arguments(arguments, "search argument")?;
    let request = parse_search(&arguments)?;
    let runtime = open_runtime(workspace, admissions)?;
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
    if output != OutputMode::Human {
        return Err(CliError::usage("output options are unsupported for Edit"));
    }
    let form = required_text(&mut arguments, "edit input form")?;
    if form != "anddress" {
        return Err(CliError::usage("edit requires the anddress input form"));
    }
    let encoded = required_text(&mut arguments, "edit anddress")?;
    let mut content = required_text(&mut arguments, "edit content")?;
    if arguments.next().is_some() {
        return Err(CliError::usage(
            "edit anddress accepts exactly one anddress and content operand",
        ));
    }

    let anddress = decode_anddress(encoded)?;
    let mut runtime = open_runtime(workspace, admissions)?;
    let outcome = run_view(&runtime, &anddress)?;
    if let ViewOutcome::Line { terminator, .. } = outcome {
        if content
            .as_bytes()
            .iter()
            .any(|byte| matches!(byte, b'\0' | b'\r' | b'\n'))
        {
            return Err(map_edit_error(EditError::InvalidInput));
        }
        let terminator = match terminator {
            LineTerminator::None => "",
            LineTerminator::Lf => "\n",
            LineTerminator::Cr => "\r",
            LineTerminator::Crlf => "\r\n",
        };
        content
            .try_reserve_exact(terminator.len())
            .map_err(|_| map_edit_error(EditError::Resource))?;
        content.push_str(terminator);
    }

    let edit = Edit::Replace {
        target: anddress,
        content,
    };
    edit.validate().map_err(map_edit_error)?;
    runtime
        .apply(&edit)
        .map_err(|error: ApplyError| CliError::execution(error.to_string()))?;
    write_session_status("OK")
}

fn execute_view(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
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
    match output {
        OutputMode::Human | OutputMode::Raw => write_view(&outcome),
        OutputMode::Json => write_view_json(outcome),
    }
}

fn execute_check(
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
    output: OutputMode,
) -> Result<(), CliError> {
    if output == OutputMode::Raw {
        return Err(CliError::usage("--raw is supported only for View"));
    }
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
    match output {
        OutputMode::Human => write_check(&outcome),
        OutputMode::Json => write_check_json(&outcome),
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

fn run_view(runtime: &WorkspaceRuntime, anddress: &Anddress) -> Result<ViewOutcome, CliError> {
    runtime
        .view(anddress, anddress.target())
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

fn write_search(outcome: &SearchOutcome) -> Result<(), CliError> {
    let occurrences = match outcome {
        SearchOutcome::Empty => &[] as &[SearchOccurrence],
        SearchOutcome::Found { occurrences } => occurrences,
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        writeln!(stdout, "Found {}", occurrences.len())?;
        for (index, occurrence) in occurrences.iter().enumerate() {
            let anddress = occurrence.anddress();
            match occurrence.position() {
                None => writeln!(stdout, "{index}\tFile\t{}", anddress.logical_path())?,
                Some(SearchPosition::Line { line }) => {
                    writeln!(stdout, "{index}\tLine\t{}:{line}", anddress.logical_path())?
                }
                Some(SearchPosition::Paragraph {
                    start_line,
                    end_line,
                }) => writeln!(
                    stdout,
                    "{index}\tParagraph\t{}:{start_line}-{end_line}",
                    anddress.logical_path()
                )?,
            }
        }
        Ok(())
    })();
    result.map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_search_json(outcome: &SearchOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    stdout
        .write_all(b"{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"")
        .map_err(|error| CliError::stream(error.to_string()))?;
    match outcome {
        SearchOutcome::Empty => stdout
            .write_all(b"empty\",\"occurrences\":[]}")
            .map_err(|error| CliError::stream(error.to_string()))?,
        SearchOutcome::Found { occurrences } => {
            stdout
                .write_all(b"found\",\"occurrences\":[")
                .map_err(|error| CliError::stream(error.to_string()))?;
            for (index, occurrence) in occurrences.iter().enumerate() {
                if index != 0 {
                    stdout
                        .write_all(b",")
                        .map_err(|error| CliError::stream(error.to_string()))?;
                }
                let anddress = occurrence.anddress();
                stdout
                    .write_all(b"{\"logicalPath\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                serde_json::to_writer(&mut stdout, anddress.logical_path())
                    .map_err(|error| CliError::stream(error.to_string()))?;
                match occurrence.position() {
                    None => stdout
                        .write_all(b",\"kind\":\"file\"")
                        .map_err(|error| CliError::stream(error.to_string()))?,
                    Some(SearchPosition::Line { line }) => write!(
                        stdout,
                        ",\"kind\":\"line\",\"line\":\"{line}\""
                    )
                    .map_err(|error| CliError::stream(error.to_string()))?,
                    Some(SearchPosition::Paragraph {
                        start_line,
                        end_line,
                    }) => write!(
                        stdout,
                        ",\"kind\":\"paragraph\",\"lineStart\":\"{start_line}\",\"lineEnd\":\"{end_line}\""
                    )
                    .map_err(|error| CliError::stream(error.to_string()))?,
                }
                stdout
                    .write_all(b",\"anddress\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                let encoded = anddress
                    .encode()
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(&encoded)
                    .map_err(|error| CliError::stream(error.to_string()))?;
                stdout
                    .write_all(b"}")
                    .map_err(|error| CliError::stream(error.to_string()))?;
            }
            stdout
                .write_all(b"]}")
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
    }
    stdout
        .write_all(b"\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_pick(outcome: &PickOutcome) -> Result<(), CliError> {
    let anddresses = match outcome {
        PickOutcome::Empty => &[] as &[Anddress],
        PickOutcome::Selected { anddresses } => anddresses,
    };
    write_address_rows("Selected", anddresses)
}

fn write_address_rows(header: &str, anddresses: &[Anddress]) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        writeln!(stdout, "{header} {}", anddresses.len())?;
        for (index, anddress) in anddresses.iter().enumerate() {
            match anddress.target() {
                AnddressTarget::File => {
                    writeln!(stdout, "{index}\tFile\t{}", anddress.logical_path())?;
                }
                AnddressTarget::Paragraph => {
                    writeln!(
                        stdout,
                        "{index}\tParagraph\t{}:{}-{}",
                        anddress.logical_path(),
                        anddress.byte_start(),
                        anddress.byte_end()
                    )?;
                }
                AnddressTarget::Line => {
                    writeln!(
                        stdout,
                        "{index}\tLine\t{}:{}-{}",
                        anddress.logical_path(),
                        anddress.byte_start(),
                        anddress.byte_end()
                    )?;
                }
            }
        }
        Ok(())
    })();
    result.map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_view(outcome: &ViewOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        match outcome {
            ViewOutcome::File { text, .. } | ViewOutcome::Paragraph { text, .. } => {
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
            ViewOutcome::RelationAbsent => {
                return Err(io::Error::other("requested View relation is absent"));
            }
        }
        Ok(())
    })();
    result.map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_view_json(outcome: ViewOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> Result<(), CliError> {
        match outcome {
            ViewOutcome::File { text, .. } => {
                stdout
                    .write_all(b"{\"schema\":\"bw.cli.view.v1\",\"kind\":\"file\",\"text\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                serde_json::to_writer(&mut stdout, &text)
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(b"}")
                    .map_err(|error| CliError::stream(error.to_string()))?;
            }
            ViewOutcome::Paragraph { text, file, .. } => {
                stdout
                    .write_all(b"{\"schema\":\"bw.cli.view.v1\",\"kind\":\"paragraph\",\"text\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                serde_json::to_writer(&mut stdout, &text)
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(b",\"file\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                let file = file
                    .encode()
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(&file)
                    .map_err(|error| CliError::stream(error.to_string()))?;
                stdout
                    .write_all(b"}")
                    .map_err(|error| CliError::stream(error.to_string()))?;
            }
            ViewOutcome::Line {
                content,
                terminator,
                file,
                paragraph,
                ..
            } => {
                stdout
                    .write_all(b"{\"schema\":\"bw.cli.view.v1\",\"kind\":\"line\",\"content\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                serde_json::to_writer(&mut stdout, &content)
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(match terminator {
                        LineTerminator::None => b",\"terminator\":\"none\",\"file\":",
                        LineTerminator::Lf => b",\"terminator\":\"lf\",\"file\":",
                        LineTerminator::Cr => b",\"terminator\":\"cr\",\"file\":",
                        LineTerminator::Crlf => b",\"terminator\":\"crlf\",\"file\":",
                    })
                    .map_err(|error| CliError::stream(error.to_string()))?;
                let file = file
                    .encode()
                    .map_err(|error| CliError::execution(error.to_string()))?;
                stdout
                    .write_all(&file)
                    .map_err(|error| CliError::stream(error.to_string()))?;
                stdout
                    .write_all(b",\"paragraph\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                if let Some(paragraph) = paragraph {
                    let paragraph = paragraph
                        .encode()
                        .map_err(|error| CliError::execution(error.to_string()))?;
                    stdout
                        .write_all(&paragraph)
                        .map_err(|error| CliError::stream(error.to_string()))?;
                } else {
                    stdout
                        .write_all(b"null")
                        .map_err(|error| CliError::stream(error.to_string()))?;
                }
                stdout
                    .write_all(b"}")
                    .map_err(|error| CliError::stream(error.to_string()))?;
            }
            ViewOutcome::RelationAbsent => {
                return Err(CliError::execution("requested View relation is absent"));
            }
        }
        Ok(())
    })();
    result?;
    stdout
        .write_all(b"\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn raw_check_status(outcome: &CheckOutcome<Option<Anddress>>) -> Result<&'static str, CliError> {
    let status = match (
        outcome.filtered.is_some(),
        outcome.report.current_count(),
        outcome.report.removed_count(),
        outcome.report.unavailable_count(),
        outcome.report.checked_count(),
    ) {
        (true, 1, 0, 0, 1) => "Current",
        (false, 0, 1, 0, 1) => "NotCurrent",
        (true, 0, 0, 1, 1) => "Unavailable",
        _ => return Err(CliError::execution("inconsistent raw Check report")),
    };
    Ok(status)
}

fn write_check(outcome: &CheckOutcome<Option<Anddress>>) -> Result<(), CliError> {
    let status = raw_check_status(outcome)?;
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{status}").map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_check_json(outcome: &CheckOutcome<Option<Anddress>>) -> Result<(), CliError> {
    let status = match raw_check_status(outcome)? {
        "Current" => "current",
        "NotCurrent" => "not-current",
        "Unavailable" => "unavailable",
        _ => unreachable!(),
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    stdout
        .write_all(b"{\"schema\":\"bw.cli.check.v1\",\"status\":\"")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .write_all(status.as_bytes())
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .write_all(b"\",\"filtered\":")
        .map_err(|error| CliError::stream(error.to_string()))?;
    if let Some(filtered) = &outcome.filtered {
        let encoded = filtered
            .encode()
            .map_err(|error| CliError::execution(error.to_string()))?;
        stdout
            .write_all(&encoded)
            .map_err(|error| CliError::stream(error.to_string()))?;
    } else {
        stdout
            .write_all(b"null")
            .map_err(|error| CliError::stream(error.to_string()))?;
    }
    stdout
        .write_all(b"}\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

enum SessionValue {
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

fn execute_shell(
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<ExitCode, CliError> {
    let mut runtime = open_runtime(workspace, admissions)?;
    let mut data = DataStore::new();
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
        match execute_session_command(&mut runtime, &mut data, &mut bindings, &tokens) {
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
    runtime: &mut WorkspaceRuntime,
    data: &mut DataStore,
    bindings: &mut Vec<SessionBinding>,
    tokens: &[String],
) -> Result<SessionControl, CliError> {
    match tokens[0].as_str() {
        "search" => {
            let outcome = run_search(runtime, parse_search(&tokens[1..])?)?;
            write_search(&outcome)?;
            Ok(SessionControl::Continue)
        }
        "pick" => {
            let outcome = run_pick(bindings, &tokens[1..])?;
            write_pick(&outcome)?;
            Ok(SessionControl::Continue)
        }
        "let" => {
            execute_let(runtime, data, bindings, tokens)?;
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
        "exit" if tokens.len() == 1 => Ok(SessionControl::Exit),
        "exit" => Err(CliError::usage("exit accepts no operands")),
        capability => Err(CliError::usage(format!(
            "unsupported Session command: {capability}"
        ))),
    }
}

fn execute_let(
    runtime: &mut WorkspaceRuntime,
    data: &mut DataStore,
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
                run_view(runtime, &anddress)?
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
        if right_hand_side.contains('[') || right_hand_side.contains(']') {
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

fn map_data_error(error: DataError) -> CliError {
    match error {
        DataError::Resource => CliError::execution(error.to_string()),
        _ => CliError::usage(error.to_string()),
    }
}

fn map_store_error<T>(error: StoreError<T>) -> CliError {
    match error {
        StoreError::AlreadyExists { .. } => CliError::usage("Data entry already exists"),
        StoreError::Resource { .. } => CliError::execution("Data resource allocation failed"),
    }
}

fn write_data_list(data: &DataStore) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    for (kind, name) in data.list() {
        write!(stdout, "{}\t\"", data_kind_name(kind))
            .map_err(|error| CliError::stream(error.to_string()))?;
        write_data_name(&mut stdout, name.as_str())
            .map_err(|error| CliError::stream(error.to_string()))?;
        writeln!(stdout, "\"").map_err(|error| CliError::stream(error.to_string()))?;
    }
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_data_name(stdout: &mut impl Write, name: &str) -> io::Result<()> {
    for character in name.chars() {
        match character {
            '"' => stdout.write_all(b"\\\"")?,
            '\\' => stdout.write_all(b"\\\\")?,
            '\n' => stdout.write_all(b"\\n")?,
            '\r' => stdout.write_all(b"\\r")?,
            '\t' => stdout.write_all(b"\\t")?,
            character if character.is_control() => {
                write!(stdout, "\\u{{{:04X}}}", character as u32)?;
            }
            character => write!(stdout, "{character}")?,
        }
    }
    Ok(())
}

fn data_kind_name(kind: DataKind) -> &'static str {
    match kind {
        DataKind::Anddress => "anddress",
        DataKind::Search => "search",
        DataKind::Pick => "pick",
        DataKind::View => "view",
        DataKind::CheckAnddress => "check-anddress",
        DataKind::CheckSearch => "check-search",
        DataKind::CheckPick => "check-pick",
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

fn write_data_value(value: &SessionValue) -> Result<(), CliError> {
    match value {
        SessionValue::Anddress(anddress) => write_data_anddress(anddress),
        SessionValue::Search(outcome) => write_search(outcome),
        SessionValue::Pick(outcome) => write_pick(outcome),
        SessionValue::View(outcome) => write_view(outcome),
        SessionValue::CheckAnddress(outcome) => write_check(outcome),
        SessionValue::CheckSearch(outcome) => write_batch_check(&outcome.report),
        SessionValue::CheckPick(outcome) => write_batch_check(&outcome.report),
        _ => Err(CliError::usage("not a Data value")),
    }
}

fn write_data_anddress(anddress: &Anddress) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    match anddress.target() {
        AnddressTarget::File => writeln!(stdout, "Anddress\tFile\t{}", anddress.logical_path()),
        AnddressTarget::Paragraph => writeln!(
            stdout,
            "Anddress\tParagraph\t{}:{}-{}",
            anddress.logical_path(),
            anddress.byte_start(),
            anddress.byte_end()
        ),
        AnddressTarget::Line => {
            writeln!(
                stdout,
                "Anddress\tLine\t{}:{}-{}",
                anddress.logical_path(),
                anddress.byte_start(),
                anddress.byte_end()
            )
        }
    }
    .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn map_anchor_error(error: AnchorError) -> CliError {
    match error {
        AnchorError::UnsupportedVersion | AnchorError::InvalidInput => {
            CliError::usage(error.to_string())
        }
        AnchorError::Unavailable => CliError::execution(error.to_string()),
    }
}

fn map_edit_error(error: EditError) -> CliError {
    match error {
        EditError::UnsupportedVersion | EditError::InvalidInput => {
            CliError::usage(error.to_string())
        }
        EditError::Resource => CliError::execution(error.to_string()),
    }
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

fn map_pick_error(error: PickError) -> CliError {
    CliError::execution(error.to_string())
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
        Some(SessionValue::Search(SearchOutcome::Found { occurrences })) => {
            let mut candidates = Vec::new();
            candidates
                .try_reserve_exact(occurrences.len())
                .map_err(|_| CliError::execution("Pick candidate allocation failed"))?;
            candidates.extend(
                occurrences
                    .iter()
                    .map(|occurrence| occurrence.anddress().clone()),
            );
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
    tokens: &[String],
) -> Result<(), CliError> {
    match required_token(tokens, 1, "view input form")? {
        "anddress" => {
            session_anddress_form(tokens, "view")?;
            let anddress = resolve_anddress(bindings, &tokens[2])?;
            write_view(&run_view(runtime, &anddress)?)
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
        _ => Err(CliError::usage(
            "view requires the anddress or anchored input form",
        )),
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

fn write_session_status(status: &str) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{status}").map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn execute_session_check(
    runtime: &mut WorkspaceRuntime,
    bindings: &[SessionBinding],
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
        _ => Err(CliError::usage(
            "check requires the anddress, search, or pick input form",
        )),
    }
}

fn write_batch_check(report: &CheckReport) -> Result<(), CliError> {
    let checked = report.checked_count();
    let current = report.current_count();
    let removed = report.removed_count();
    let unavailable = report.unavailable_count();
    if current
        .checked_add(removed)
        .and_then(|total| total.checked_add(unavailable))
        != Some(checked)
    {
        return Err(CliError::execution("inconsistent batch Check report"));
    }
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "Checked {checked}").map_err(|error| CliError::stream(error.to_string()))?;
    writeln!(stdout, "Current {current}").map_err(|error| CliError::stream(error.to_string()))?;
    writeln!(stdout, "NotCurrent {removed}")
        .map_err(|error| CliError::stream(error.to_string()))?;
    writeln!(stdout, "Unavailable {unavailable}")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
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
        Some(SessionValue::Search(SearchOutcome::Found { occurrences })) => occurrences
            .get(index)
            .map(|occurrence| occurrence.anddress().clone())
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

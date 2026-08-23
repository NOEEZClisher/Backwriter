//! One-shot human Search and View adapter for Backwriter CLI V1.

use std::{
    env,
    ffi::OsString,
    io::{self, BufWriter, Write},
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

const USAGE: &str = "Usage:\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... view anddress <encoded-v3-Anddress>\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... check anddress <encoded-v3-Anddress>\n\nOnly one-shot human Search, View, and Check are implemented in this slice.";

enum CliError {
    Usage(String),
    Execution(String),
}

impl CliError {
    fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    fn execution(message: impl Into<String>) -> Self {
        Self::Execution(message.into())
    }

    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage(_) => ExitCode::from(2),
            Self::Execution(_) => ExitCode::FAILURE,
        }
    }

    fn report(&self) {
        let mut stderr = io::stderr().lock();
        match self {
            Self::Usage(message) => {
                let _ = writeln!(stderr, "error: {message}\n\n{USAGE}");
            }
            Self::Execution(message) => {
                let _ = writeln!(stderr, "error: {message}");
            }
        }
    }
}

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            error.exit_code()
        }
    }
}

fn execute() -> Result<(), CliError> {
    let mut arguments = env::args_os().skip(1);
    let first = arguments.next();
    if matches!(first.as_deref(), Some(value) if value == "--help") && arguments.next().is_none() {
        let mut stdout = BufWriter::new(io::stdout().lock());
        writeln!(stdout, "{USAGE}").map_err(|error| CliError::execution(error.to_string()))?;
        return stdout
            .flush()
            .map_err(|error| CliError::execution(error.to_string()));
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
            "search" => return execute_search(arguments, workspace, admissions),
            "view" => return execute_view(arguments, workspace, admissions),
            "check" => return execute_check(arguments, workspace, admissions),
            "shell" | "pick" | "anchor" | "edit" | "apply" | "data" => {
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
    mut arguments: impl Iterator<Item = OsString>,
    workspace: Option<PathBuf>,
    admissions: Vec<AdmissionRoot>,
) -> Result<(), CliError> {
    let target = match required_text(&mut arguments, "search kind")?.as_str() {
        "line" => SearchTarget::Line,
        "paragraph" => SearchTarget::Paragraph,
        "file" => SearchTarget::File,
        value => return Err(CliError::usage(format!("invalid search kind: {value}"))),
    };
    let query = SearchQuery::new(required_text(&mut arguments, "search query")?)
        .map_err(|error| CliError::usage(error.to_string()))?;

    let mut entries = Vec::new();
    while let Some(option) = arguments.next() {
        let option = utf8(option, "search option")?;
        match option.as_str() {
            "--source" => entries.push(
                SearchScopeEntry::source(required_text(&mut arguments, "--source")?)
                    .map_err(|error| CliError::usage(error.to_string()))?,
            ),
            "--subtree" => entries.push(
                SearchScopeEntry::subtree(required_text(&mut arguments, "--subtree")?)
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
    }
    let scope = if entries.is_empty() {
        SearchScope::all_admitted()
    } else {
        SearchScope::only(entries).map_err(|error| CliError::usage(error.to_string()))?
    };
    let runtime = open_runtime(workspace, admissions)?;
    let outcome = runtime
        .search(&SearchRequest::new(query, scope, target))
        .map_err(|error| CliError::execution(error.to_string()))?;
    write_human(outcome)
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
    let outcome = runtime
        .view(&anddress)
        .map_err(|error| CliError::execution(error.to_string()))?;
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
    let outcome = runtime
        .check(anddress)
        .map_err(|error| CliError::execution(error.to_string()))?;
    write_check(outcome)
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

fn required_text(
    arguments: &mut impl Iterator<Item = OsString>,
    option: &str,
) -> Result<String, CliError> {
    let argument = arguments
        .next()
        .ok_or_else(|| CliError::usage(format!("{option} requires a value")))?;
    utf8(argument, option)
}

fn write_human(outcome: SearchOutcome) -> Result<(), CliError> {
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
    result.map_err(|error| CliError::execution(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::execution(error.to_string()))
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
    result.map_err(|error| CliError::execution(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::execution(error.to_string()))
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
    writeln!(stdout, "{status}").map_err(|error| CliError::execution(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::execution(error.to_string()))
}

//! One-shot human Search adapter for Backwriter CLI V1.

use std::{
    env,
    ffi::OsString,
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::ExitCode,
};

use artext::{
    backwriter::{
        anddress::AnddressTarget,
        search::{
            SearchOutcome, SearchQuery, SearchRequest, SearchScope, SearchScopeEntry, SearchTarget,
        },
    },
    runtime::{AdmissionRoot, WorkspaceAdmission, WorkspaceRuntime},
};

const USAGE: &str = "Usage:\n  backwriter [--workspace ABSOLUTE_PATH] [--admit LOGICAL_PATH]... search <line|paragraph|file> <query> [--source LOGICAL_PATH | --subtree LOGICAL_PATH]...\n\nOnly one-shot human Search is implemented in this slice.";

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
            "search" => break,
            "shell" | "view" | "pick" | "check" | "anchor" | "edit" | "apply" | "data" => {
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
    if admissions.is_empty() {
        admissions.push(AdmissionRoot::new(".").expect("dot admission is valid"));
    }
    let admission =
        WorkspaceAdmission::new(admissions).map_err(|error| CliError::usage(error.to_string()))?;
    let workspace = match workspace {
        Some(path) => path,
        None => env::current_dir().map_err(|error| CliError::execution(error.to_string()))?,
    };
    let runtime = WorkspaceRuntime::open(&workspace, admission)
        .map_err(|error| CliError::execution(error.to_string()))?;
    let outcome = runtime
        .search(&SearchRequest::new(query, scope, target))
        .map_err(|error| CliError::execution(error.to_string()))?;
    write_human(outcome)
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

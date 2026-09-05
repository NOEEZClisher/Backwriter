use super::ReplaceContentError;
use super::help::{CHECK_HELP, EDIT_HELP, SEARCH_HELP, TOP_LEVEL_HELP, VIEW_HELP, canonical_usage};
use backwriter::backwriter::anchor::AnchorError;
use backwriter::backwriter::data::{DataError, StoreError};
use backwriter::backwriter::edit::EditError;
use backwriter::backwriter::pick::PickError;
use std::{
    io::{self, Write},
    process::ExitCode,
};

pub(super) enum CliError {
    Usage(String),
    ActionableUsage {
        code: &'static str,
        message: String,
        help: &'static str,
        hint: &'static str,
    },
    Execution(String),
    Stream(String),
}

impl CliError {
    pub(super) fn usage(message: impl Into<String>) -> Self {
        Self::Usage(message.into())
    }

    pub(super) fn execution(message: impl Into<String>) -> Self {
        Self::Execution(message.into())
    }

    pub(super) fn top_usage(code: &'static str, message: impl Into<String>) -> Self {
        Self::ActionableUsage {
            code,
            message: message.into(),
            help: TOP_LEVEL_HELP,
            hint: "bw --help",
        }
    }

    pub(super) fn command_usage(
        code: &'static str,
        message: impl Into<String>,
        help: &'static str,
        hint: &'static str,
    ) -> Self {
        Self::ActionableUsage {
            code,
            message: message.into(),
            help,
            hint,
        }
    }

    pub(super) fn stream(message: impl Into<String>) -> Self {
        Self::Stream(message.into())
    }

    pub(super) fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage(_) | Self::ActionableUsage { .. } => ExitCode::from(2),
            Self::Execution(_) | Self::Stream(_) => ExitCode::FAILURE,
        }
    }

    pub(super) fn report(&self) {
        let mut stderr = io::stderr().lock();
        match self {
            Self::Usage(message) => {
                let _ = writeln!(stderr, "error: {message}\n\n{TOP_LEVEL_HELP}");
            }
            Self::ActionableUsage {
                code,
                message,
                help,
                hint,
            } => {
                let _ = writeln!(
                    stderr,
                    "error[{code}]:\n{message}\n\nusage:\n{}\n\nhint:\nrun `{hint}`",
                    canonical_usage(help)
                );
            }
            Self::Execution(message) | Self::Stream(message) => {
                let _ = writeln!(stderr, "error: {message}");
            }
        }
    }
}

pub(super) fn search_usage(code: &'static str, message: impl Into<String>) -> CliError {
    CliError::command_usage(code, message, SEARCH_HELP, "bw help search")
}

pub(super) fn view_usage(code: &'static str, message: impl Into<String>) -> CliError {
    CliError::command_usage(code, message, VIEW_HELP, "bw help view")
}

pub(super) fn edit_usage(code: &'static str, message: impl Into<String>) -> CliError {
    CliError::command_usage(code, message, EDIT_HELP, "bw help edit")
}

pub(super) fn check_usage(code: &'static str, message: impl Into<String>) -> CliError {
    CliError::command_usage(code, message, CHECK_HELP, "bw help check")
}

pub(super) fn promote_top_usage(error: CliError, code: &'static str) -> CliError {
    match error {
        CliError::Usage(message) => CliError::top_usage(code, message),
        error => error,
    }
}

pub(super) fn promote_search_usage(error: CliError) -> CliError {
    match error {
        CliError::Usage(message)
            if message.contains("search kind") || message.contains("invalid search kind") =>
        {
            search_usage("search.kind_invalid", message)
        }
        CliError::Usage(message) if message.contains("requires a value") => {
            search_usage("search.operand_missing", message)
        }
        CliError::Usage(message)
            if message.contains("output options") || message.contains("--admit") =>
        {
            search_usage("search.option_position", message)
        }
        CliError::Usage(message) => search_usage("search.request_invalid", message),
        error => error,
    }
}

pub(super) fn map_edit_content_error(error: ReplaceContentError) -> CliError {
    match error {
        ReplaceContentError::Nul => edit_usage(
            "edit.content_contains_nul",
            "Edit Content must not contain NUL.",
        ),
        ReplaceContentError::LineTerminator => edit_usage(
            "edit.line_body_contains_terminator",
            "Line Edit accepts body Content only. Backwriter preserves the existing Line terminator automatically. Exact extent replacement is available through advanced raw Session Edit/Apply.",
        ),
        ReplaceContentError::Resource => CliError::execution(EditError::Resource.to_string()),
    }
}

pub(super) fn map_session_replace_content_error(error: ReplaceContentError) -> CliError {
    match error {
        ReplaceContentError::Nul => CliError::usage("Edit Content must not contain NUL."),
        ReplaceContentError::LineTerminator => CliError::usage(
            "Line Edit accepts body Content only. Backwriter preserves the existing Line terminator automatically. Exact extent replacement is available through advanced raw Session Edit/Apply.",
        ),
        ReplaceContentError::Resource => CliError::execution(EditError::Resource.to_string()),
    }
}

pub(super) fn map_edit_error_for_edit(error: EditError) -> CliError {
    match error {
        EditError::UnsupportedVersion => edit_usage("edit.address_unsupported", error.to_string()),
        EditError::InvalidInput => edit_usage("edit.content_invalid", error.to_string()),
        EditError::Resource => CliError::execution(error.to_string()),
    }
}

pub(super) fn promote_view_usage(error: CliError) -> CliError {
    match error {
        CliError::Usage(message) if message.contains("requires a value") => {
            view_usage("view.operand_missing", message)
        }
        CliError::Usage(message) => view_usage("view.address_invalid", message),
        error => error,
    }
}

pub(super) fn promote_check_usage(error: CliError) -> CliError {
    match error {
        CliError::Usage(message) if message.contains("requires a value") => {
            check_usage("check.operand_missing", message)
        }
        CliError::Usage(message) => check_usage("check.address_invalid", message),
        error => error,
    }
}

pub(super) fn session_error_status(error: &CliError) -> u8 {
    match error {
        CliError::Usage(_) | CliError::ActionableUsage { .. } => 2,
        CliError::Execution(_) => 1,
        CliError::Stream(_) => 1,
    }
}

pub(super) fn map_data_error(error: DataError) -> CliError {
    match error {
        DataError::Resource => CliError::execution(error.to_string()),
        _ => CliError::usage(error.to_string()),
    }
}

pub(super) fn map_store_error<T>(error: StoreError<T>) -> CliError {
    match error {
        StoreError::AlreadyExists { .. } => CliError::usage("Data entry already exists"),
        StoreError::Resource { .. } => CliError::execution("Data resource allocation failed"),
    }
}

pub(super) fn map_anchor_error(error: AnchorError) -> CliError {
    match error {
        AnchorError::UnsupportedVersion | AnchorError::InvalidInput => {
            CliError::usage(error.to_string())
        }
        AnchorError::Unavailable => CliError::execution(error.to_string()),
    }
}

pub(super) fn map_edit_error(error: EditError) -> CliError {
    match error {
        EditError::UnsupportedVersion | EditError::InvalidInput => {
            CliError::usage(error.to_string())
        }
        EditError::Resource => CliError::execution(error.to_string()),
    }
}

pub(super) fn map_pick_error(error: PickError) -> CliError {
    CliError::execution(error.to_string())
}

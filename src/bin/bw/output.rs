use super::OutputMode;
use super::error::CliError;
use super::shell::{SessionValue, reserve_session_refs};
use backwriter::backwriter::anddress::{Anddress, AnddressTarget};
use backwriter::backwriter::apply::EditReceipt;
use backwriter::backwriter::check::{CheckOutcome, CheckReport, CheckStatus};
use backwriter::backwriter::data::{DataKind, DataStore};
use backwriter::backwriter::pick::PickOutcome;
use backwriter::backwriter::search::SearchOutcome;
use backwriter::backwriter::view::ViewOutcome;
use std::io::{self, BufWriter, Write};

pub(super) fn write_edit(receipt: EditReceipt, output: OutputMode) -> Result<(), CliError> {
    let (human_outcome, json_outcome, anddress) = match receipt {
        EditReceipt::Unchanged { anddress } => ("Unchanged", "unchanged", Some(anddress)),
        EditReceipt::Changed { anddress } => ("Changed", "changed", anddress),
    };
    let encoded = match anddress.as_ref() {
        Some(anddress) => Some(
            anddress
                .encode()
                .map_err(|error| CliError::execution(error.to_string()))?,
        ),
        None => None,
    };

    let mut stdout = BufWriter::new(io::stdout().lock());
    match output {
        OutputMode::Human => {
            stdout
                .write_all(human_outcome.as_bytes())
                .map_err(|error| CliError::stream(error.to_string()))?;
            stdout
                .write_all(b"\t")
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
        OutputMode::Json => {
            stdout
                .write_all(b"{\"schema\":\"bw.cli.edit.v1\",\"outcome\":\"")
                .map_err(|error| CliError::stream(error.to_string()))?;
            stdout
                .write_all(json_outcome.as_bytes())
                .map_err(|error| CliError::stream(error.to_string()))?;
            stdout
                .write_all(b"\",\"anddress\":")
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
        OutputMode::Raw => unreachable!(),
    }
    match encoded {
        Some(encoded) => stdout
            .write_all(&encoded)
            .map_err(|error| CliError::stream(error.to_string()))?,
        None if output == OutputMode::Human => stdout
            .write_all(b"None")
            .map_err(|error| CliError::stream(error.to_string()))?,
        None => stdout
            .write_all(b"null")
            .map_err(|error| CliError::stream(error.to_string()))?,
    }
    if output == OutputMode::Json {
        stdout
            .write_all(b"}")
            .map_err(|error| CliError::stream(error.to_string()))?;
    }
    stdout
        .write_all(b"\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_search(outcome: &SearchOutcome) -> Result<(), CliError> {
    let anddresses = match outcome {
        SearchOutcome::Empty => &[] as &[Anddress],
        SearchOutcome::Found { anddresses } => anddresses,
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        writeln!(stdout, "Found {}", anddresses.len())?;
        for (index, anddress) in anddresses.iter().enumerate() {
            match anddress.target() {
                AnddressTarget::File => {
                    writeln!(stdout, "{index}\tFile\t{}", anddress.logical_path())?
                }
                AnddressTarget::Line => {
                    let line = anddress.line_number().expect("Line has a number");
                    writeln!(stdout, "{index}\tLine\t{}:{line}", anddress.logical_path())?
                }
                AnddressTarget::Paragraph => {
                    let lines = anddress.line_range();
                    let start_line = lines.start + 1;
                    let end_line = lines.end;
                    writeln!(
                        stdout,
                        "{index}\tParagraph\t{}:{start_line}-{end_line}",
                        anddress.logical_path()
                    )?
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

pub(super) fn write_search_json(outcome: &SearchOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    stdout
        .write_all(b"{\"schema\":\"bw.cli.search.v2\",\"outcome\":\"")
        .map_err(|error| CliError::stream(error.to_string()))?;
    match outcome {
        SearchOutcome::Empty => stdout
            .write_all(b"empty\",\"occurrences\":[]}")
            .map_err(|error| CliError::stream(error.to_string()))?,
        SearchOutcome::Found { anddresses } => {
            stdout
                .write_all(b"found\",\"occurrences\":[")
                .map_err(|error| CliError::stream(error.to_string()))?;
            let mut encoded = Vec::new();
            for (index, anddress) in anddresses.iter().enumerate() {
                if index != 0 {
                    stdout
                        .write_all(b",")
                        .map_err(|error| CliError::stream(error.to_string()))?;
                }
                stdout
                    .write_all(b"{\"logicalPath\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                serde_json::to_writer(&mut stdout, anddress.logical_path())
                    .map_err(|error| CliError::stream(error.to_string()))?;
                match anddress.target() {
                    AnddressTarget::File => stdout
                        .write_all(b",\"kind\":\"file\"")
                        .map_err(|error| CliError::stream(error.to_string()))?,
                    AnddressTarget::Line => {
                        let line = anddress.line_number().expect("Line has a number");
                        write!(stdout, ",\"kind\":\"line\",\"line\":\"{line}\"")
                            .map_err(|error| CliError::stream(error.to_string()))?
                    }
                    AnddressTarget::Paragraph => {
                        let lines = anddress.line_range();
                        let start_line = lines.start + 1;
                        let end_line = lines.end;
                        write!(
                            stdout,
                            ",\"kind\":\"paragraph\",\"lineStart\":\"{start_line}\",\"lineEnd\":\"{end_line}\""
                        )
                        .map_err(|error| CliError::stream(error.to_string()))?
                    }
                }
                stdout
                    .write_all(b",\"anddress\":")
                    .map_err(|error| CliError::stream(error.to_string()))?;
                anddress
                    .encode_into(&mut encoded)
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

pub(super) fn write_pick(outcome: &PickOutcome) -> Result<(), CliError> {
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

pub(super) fn write_view(outcome: &ViewOutcome) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let result = (|| -> io::Result<()> {
        match outcome {
            ViewOutcome::Projected { content, .. } => {
                stdout.write_all(content.as_bytes())?;
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

pub(super) fn write_view_json(outcomes: &[ViewOutcome]) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut encoded = Vec::new();
    stdout
        .write_all(b"{\"schema\":\"bw.cli.view.v2\",\"outcomes\":[")
        .map_err(|error| CliError::stream(error.to_string()))?;
    for (index, outcome) in outcomes.iter().enumerate() {
        if index != 0 {
            stdout
                .write_all(b",")
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
        write_view_json_item(&mut stdout, outcome, &mut encoded)?;
    }
    stdout
        .write_all(b"]}\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_view_json_item(
    stdout: &mut impl Write,
    outcome: &ViewOutcome,
    encoded: &mut Vec<u8>,
) -> Result<(), CliError> {
    match outcome {
        ViewOutcome::Projected { anddress, content } => {
            stdout
                .write_all(b"{\"outcome\":\"projected\",\"anddress\":")
                .map_err(|error| CliError::stream(error.to_string()))?;
            anddress
                .encode_into(encoded)
                .map_err(|error| CliError::execution(error.to_string()))?;
            stdout
                .write_all(encoded)
                .map_err(|error| CliError::stream(error.to_string()))?;
            stdout
                .write_all(b",\"content\":")
                .map_err(|error| CliError::stream(error.to_string()))?;
            serde_json::to_writer(&mut *stdout, content)
                .map_err(|error| CliError::execution(error.to_string()))?;
            stdout
                .write_all(b"}")
                .map_err(|error| CliError::stream(error.to_string()))
        }
        ViewOutcome::RelationAbsent => stdout
            .write_all(b"{\"outcome\":\"relation-absent\"}")
            .map_err(|error| CliError::stream(error.to_string())),
    }
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

pub(super) fn write_check(outcome: &CheckOutcome<Option<Anddress>>) -> Result<(), CliError> {
    write_check_status(check_status_from_outcome(outcome)?)
}

fn check_status_from_outcome(
    outcome: &CheckOutcome<Option<Anddress>>,
) -> Result<CheckStatus, CliError> {
    match raw_check_status(outcome)? {
        "Current" => Ok(CheckStatus::Current),
        "NotCurrent" => Ok(CheckStatus::NotCurrent),
        "Unavailable" => Ok(CheckStatus::Unavailable),
        _ => unreachable!(),
    }
}

pub(super) fn write_check_status(status: CheckStatus) -> Result<(), CliError> {
    let status = match status {
        CheckStatus::Current => "Current",
        CheckStatus::NotCurrent => "NotCurrent",
        CheckStatus::Unavailable => "Unavailable",
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{status}").map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_check_json(
    inputs: &[Anddress],
    statuses: &[CheckStatus],
) -> Result<(), CliError> {
    if inputs.len() != statuses.len() {
        return Err(CliError::execution("inconsistent ordered Check results"));
    }
    let mut stdout = BufWriter::new(io::stdout().lock());
    stdout
        .write_all(b"{\"schema\":\"bw.cli.check.v2\",\"outcomes\":[")
        .map_err(|error| CliError::stream(error.to_string()))?;
    let mut scratch = Vec::new();
    for (index, (input, status)) in inputs.iter().zip(statuses).enumerate() {
        if index != 0 {
            stdout
                .write_all(b",")
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
        let label = match status {
            CheckStatus::Current => "current",
            CheckStatus::NotCurrent => "not-current",
            CheckStatus::Unavailable => "unavailable",
        };
        stdout
            .write_all(b"{\"status\":\"")
            .map_err(|error| CliError::stream(error.to_string()))?;
        stdout
            .write_all(label.as_bytes())
            .map_err(|error| CliError::stream(error.to_string()))?;
        stdout
            .write_all(b"\",\"anddress\":")
            .map_err(|error| CliError::stream(error.to_string()))?;
        if *status == CheckStatus::NotCurrent {
            stdout
                .write_all(b"null")
                .map_err(|error| CliError::stream(error.to_string()))?;
        } else {
            input
                .encode_into(&mut scratch)
                .map_err(|error| CliError::execution(error.to_string()))?;
            stdout
                .write_all(&scratch)
                .map_err(|error| CliError::stream(error.to_string()))?;
        }
        stdout
            .write_all(b"}")
            .map_err(|error| CliError::stream(error.to_string()))?;
    }
    stdout
        .write_all(b"]}\n")
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_session_refs(start: usize, refs: &[Anddress]) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    for (offset, anddress) in refs.iter().enumerate() {
        write_session_ref_line(&mut stdout, start + offset, None, anddress)
            .map_err(|error| CliError::stream(error.to_string()))?;
    }
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_session_replace(
    slot: usize,
    status: &str,
    anddress: &Anddress,
) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    write_session_ref_line(&mut stdout, slot, Some(status), anddress)
        .map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

fn write_session_ref_line(
    stdout: &mut impl Write,
    slot: usize,
    status: Option<&str>,
    anddress: &Anddress,
) -> io::Result<()> {
    write!(stdout, "@{slot}\t")?;
    if let Some(status) = status {
        write!(stdout, "{status}\t")?;
    }
    match anddress.target() {
        AnddressTarget::File => writeln!(stdout, "File\t{}", anddress.logical_path()),
        AnddressTarget::Line => {
            let line = anddress.line_number().expect("Line has a number");
            writeln!(stdout, "Line\t{}:{line}", anddress.logical_path())
        }
        AnddressTarget::Paragraph => {
            let lines = anddress.line_range();
            writeln!(
                stdout,
                "Paragraph\t{}:{}-{}",
                anddress.logical_path(),
                lines.start + 1,
                lines.end
            )
        }
    }
}

pub(super) fn write_session_view(
    mut stdout: BufWriter<impl Write>,
    refs: &mut Vec<Anddress>,
    references: &[String],
    outcomes: Vec<ViewOutcome>,
) -> Result<(), CliError> {
    let projected = outcomes
        .iter()
        .filter(|outcome| matches!(outcome, ViewOutcome::Projected { .. }))
        .count();
    reserve_session_refs(refs, projected)?;
    let result = (|| -> io::Result<()> {
        for (reference, outcome) in references.iter().zip(outcomes) {
            match outcome {
                ViewOutcome::Projected { anddress, content } => {
                    let slot = refs.len();
                    refs.push(anddress);
                    writeln!(stdout, "View\t{reference}\tbytes={}", content.len())?;
                    write_session_ref_line(&mut stdout, slot, None, &refs[slot])?;
                    stdout.write_all(content.as_bytes())?;
                    stdout.write_all(b"\nEndView\n")?;
                }
                ViewOutcome::RelationAbsent => {
                    writeln!(stdout, "View\t{reference}\tRelationAbsent")?;
                }
            }
        }
        stdout.flush()
    })();
    // Discard unflushed bytes on failure; Drop must not retry a failed stream.
    let _ = stdout.into_parts();
    result.map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_data_list(data: &DataStore) -> Result<(), CliError> {
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

pub(super) fn write_data_value(value: &SessionValue) -> Result<(), CliError> {
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

pub(super) fn write_session_status(status: &str) -> Result<(), CliError> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    writeln!(stdout, "{status}").map_err(|error| CliError::stream(error.to_string()))?;
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_session_check(
    first_slot: usize,
    inputs: &[Anddress],
    statuses: &[CheckStatus],
) -> Result<(), CliError> {
    if inputs.len() != statuses.len() {
        return Err(CliError::execution("inconsistent ordered Check results"));
    }
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut slot = first_slot;
    for (input, status) in inputs.iter().zip(statuses) {
        match status {
            CheckStatus::Current => {
                write_session_ref_line(&mut stdout, slot, Some("Current"), input)
                    .map_err(|error| CliError::stream(error.to_string()))?;
                slot = slot
                    .checked_add(1)
                    .ok_or_else(|| CliError::execution("Session reference slot overflow"))?;
            }
            CheckStatus::NotCurrent => writeln!(stdout, "NotCurrent")
                .map_err(|error| CliError::stream(error.to_string()))?,
            CheckStatus::Unavailable => writeln!(stdout, "Unavailable")
                .map_err(|error| CliError::stream(error.to_string()))?,
        }
    }
    stdout
        .flush()
        .map_err(|error| CliError::stream(error.to_string()))
}

pub(super) fn write_batch_check(report: &CheckReport) -> Result<(), CliError> {
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

#[cfg(test)]
mod view_output_tests {
    use super::super::error::session_error_status;
    use super::*;
    use crate::test_support;

    #[derive(Default)]
    struct Sink {
        bytes: Vec<u8>,
        fail_at: Option<usize>,
        fail_flush: bool,
        writes: usize,
        flushes: usize,
        failures: usize,
    }

    impl Write for Sink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.writes += 1;
            let remaining = self.fail_at.unwrap_or(usize::MAX) - self.bytes.len();
            if remaining == 0 {
                self.failures += 1;
                return Err(io::Error::other("injected write failure"));
            }
            let count = remaining.min(bytes.len());
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.fail_flush {
                return Err(io::Error::other("injected flush failure"));
            }
            Ok(())
        }
    }

    #[test]
    fn view_writer_preserves_exact_framing_and_consumes_owned_results() {
        for (content, target, path, expected) in [
            (
                "",
                AnddressTarget::File,
                "note.txt",
                "View\t@0\tbytes=0\n@3\tFile\tnote.txt\n\nEndView\n",
            ),
            (
                "x",
                AnddressTarget::Line,
                "note.txt",
                "View\t@0\tbytes=1\n@3\tLine\tnote.txt:1\nx\nEndView\n",
            ),
            (
                "x\n",
                AnddressTarget::Line,
                "note.txt",
                "View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\n\nEndView\n",
            ),
            (
                "x\r",
                AnddressTarget::Line,
                "note.txt",
                "View\t@0\tbytes=2\n@3\tLine\tnote.txt:1\nx\r\nEndView\n",
            ),
            (
                "x\r\n",
                AnddressTarget::Line,
                "note.txt",
                "View\t@0\tbytes=3\n@3\tLine\tnote.txt:1\nx\r\n\nEndView\n",
            ),
            (
                "β\r\n",
                AnddressTarget::Line,
                "note.txt",
                "View\t@0\tbytes=4\n@3\tLine\tnote.txt:1\nβ\r\n\nEndView\n",
            ),
            (
                "EndView\n",
                AnddressTarget::File,
                "note.txt",
                "View\t@0\tbytes=8\n@3\tFile\tnote.txt\nEndView\n\nEndView\n",
            ),
            (
                "x",
                AnddressTarget::File,
                "dir/a b.txt",
                "View\t@0\tbytes=1\n@3\tFile\tdir/a b.txt\nx\nEndView\n",
            ),
        ] {
            let anddress = test_support::address(
                &"0".repeat(64),
                path,
                content.as_bytes(),
                target,
                0,
                content.len(),
            );
            let mut refs = vec![anddress.clone(); 3];
            let mut sink = Sink::default();
            assert!(
                write_session_view(
                    BufWriter::new(&mut sink),
                    &mut refs,
                    &["@0".into()],
                    vec![ViewOutcome::Projected {
                        anddress: anddress.clone(),
                        content: content.into()
                    }]
                )
                .is_ok()
            );
            assert_eq!(sink.bytes, expected.as_bytes());
            assert_eq!(sink.flushes, 1);
            assert_eq!(refs, vec![anddress; 4]);
        }
    }

    #[test]
    fn view_writer_failure_keeps_only_begun_slots_and_never_retries_on_drop() {
        let anddress = test_support::file(&"0".repeat(64), "note.txt", b"x");
        let expected = b"View\t@0\tbytes=1\n@1\tFile\tnote.txt\nx\nEndView\nView\t@1\tRelationAbsent\nView\t@0\tbytes=1\n@2\tFile\tnote.txt\nx\nEndView\n";
        let third_start = expected
            .windows(b"View\t@0".len())
            .rposition(|bytes| bytes == b"View\t@0")
            .unwrap();
        // Every byte boundary includes header, metadata, Content and end framing.
        for capacity in [0, 8192] {
            for fail_at in 0..expected.len() {
                let mut refs = vec![anddress.clone()];
                let mut sink = Sink {
                    fail_at: Some(fail_at),
                    ..Sink::default()
                };
                let outcomes = vec![
                    ViewOutcome::Projected {
                        anddress: anddress.clone(),
                        content: "x".into(),
                    },
                    ViewOutcome::RelationAbsent,
                    ViewOutcome::Projected {
                        anddress: anddress.clone(),
                        content: "x".into(),
                    },
                ];
                let error = write_session_view(
                    BufWriter::with_capacity(capacity, &mut sink),
                    &mut refs,
                    &["@0".into(), "@1".into(), "@0".into()],
                    outcomes,
                )
                .unwrap_err();
                assert!(matches!(error, CliError::Stream(_)));
                assert_eq!(session_error_status(&error), 1);
                assert_eq!(sink.bytes, expected[..fail_at]);
                assert_eq!(sink.failures, 1, "no Drop retry");
                assert_eq!(sink.flushes, 0);
                let begun = if capacity != 0 || fail_at >= third_start {
                    2
                } else {
                    1
                };
                assert_eq!(refs.len(), 1 + begun);
            }
        }
        let mut refs = vec![anddress.clone()];
        let mut sink = Sink {
            fail_at: Some(0),
            ..Sink::default()
        };
        assert!(matches!(
            write_session_view(
                BufWriter::with_capacity(0, &mut sink),
                &mut refs,
                &["@0".into(), "@0".into()],
                vec![
                    ViewOutcome::RelationAbsent,
                    ViewOutcome::Projected {
                        anddress: anddress.clone(),
                        content: "x".into()
                    }
                ]
            ),
            Err(CliError::Stream(_))
        ));
        assert_eq!(refs.len(), 1);
        assert_eq!(sink.writes, 1);
        assert_eq!(sink.flushes, 0);

        let mut sink = Sink {
            fail_flush: true,
            ..Sink::default()
        };
        assert!(matches!(
            write_session_view(
                BufWriter::new(&mut sink),
                &mut refs,
                &["@0".into()],
                vec![ViewOutcome::Projected {
                    anddress,
                    content: "x".into()
                }]
            ),
            Err(CliError::Stream(_))
        ));
        assert_eq!(refs.len(), 2);
        assert_eq!(
            sink.bytes,
            &expected[..expected
                .windows(b"View\t@1".len())
                .position(|bytes| bytes == b"View\t@1")
                .unwrap()]
        );
        assert_eq!(sink.flushes, 1);
    }

    #[test]
    fn view_reference_capacity_overflow_is_recoverable_before_output() {
        let anddress = test_support::file(&"0".repeat(64), "note.txt", b"");
        let mut refs = vec![anddress.clone()];
        let error = reserve_session_refs(&mut refs, usize::MAX).unwrap_err();
        assert!(matches!(error, CliError::Execution(_)));
        assert_eq!(session_error_status(&error), 1);
        assert_eq!(refs, vec![anddress]);
        let source = include_str!("output.rs");
        let writer = source
            .split_once("fn write_session_view(")
            .unwrap()
            .1
            .split_once("fn write_data_list(")
            .unwrap()
            .0;
        assert!(
            writer
                .find("reserve_session_refs(refs, projected)?")
                .unwrap()
                < writer.find("for (reference, outcome)").unwrap()
        );
        assert!(!writer.contains(".clone()"));
        assert!(!writer.contains("resolve_session_ref"));
    }
}

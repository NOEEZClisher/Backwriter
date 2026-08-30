//! Runtime-local Anchor liveness and anchored execution seams.

use crate::backwriter::{
    anchor::{Anchedress, AnchorError, AnchorOutcome},
    anddress::{Anddress, AnddressError},
    view::{ViewError, ViewOutcome},
};
use crate::source::validate_logical_path;

use super::{
    AnchorBinding, WorkspaceRuntime, is_backwriter_spill,
    view::{AnchoredObservation, ObservationError, observe_anchored},
};

pub(super) fn anchor(
    runtime: &mut WorkspaceRuntime,
    input: &Anddress,
) -> Result<AnchorOutcome, AnchorError> {
    validate(input)?;
    runtime.prune_dead_anchors();
    let focus = input.clone();
    let mut inputs = path_inputs(runtime, input.logical_path(), focus)?;
    let focus_index = inputs.len() - 1;
    let observed = match observe_current(runtime, input, &inputs, None) {
        Ok(observed) => observed,
        Err(ObservationError::InvalidSource) => {
            runtime.invalidate_source_state(input.logical_path());
            return Err(AnchorError::Unavailable);
        }
        Err(ObservationError::Read | ObservationError::Resource) => {
            runtime.invalidate_current_proof(input.logical_path());
            return Err(AnchorError::Unavailable);
        }
    };
    if !observed.current[focus_index] {
        return Err(AnchorError::Unavailable);
    }
    if observed
        .current
        .iter()
        .enumerate()
        .any(|(index, current)| index != focus_index && !current)
    {
        runtime.invalidate_source_state(input.logical_path());
        return Err(AnchorError::Unavailable);
    }
    if runtime
        .anchors
        .iter()
        .any(|binding| binding.anddress == inputs[focus_index])
    {
        return Ok(AnchorOutcome::AlreadyLive);
    }
    runtime
        .anchors
        .try_reserve(1)
        .map_err(|_| AnchorError::Unavailable)?;
    let handle = Anchedress::new();
    let anddress = inputs.pop().expect("focus is appended to path inputs");
    runtime.anchors.push(AnchorBinding {
        token: handle.weak(),
        anddress,
    });
    Ok(AnchorOutcome::Anchored(handle))
}

pub(super) fn view_anchored(
    runtime: &mut WorkspaceRuntime,
    handle: &Anchedress,
) -> Result<ViewOutcome, ViewError> {
    runtime.prune_dead_anchors();
    let token = handle.weak();
    let Some(index) = runtime
        .anchors
        .iter()
        .position(|binding| binding.token.ptr_eq(&token))
    else {
        return Err(ViewError::Unavailable);
    };
    let input = runtime.anchors[index].anddress.clone();
    let observed = match observe_current(runtime, &input, std::slice::from_ref(&input), Some(0)) {
        Ok(observed) => observed,
        Err(ObservationError::InvalidSource) => {
            runtime.invalidate_source_state(input.logical_path());
            return Err(ViewError::Unavailable);
        }
        Err(ObservationError::Read | ObservationError::Resource) => {
            runtime.invalidate_current_proof(input.logical_path());
            return Err(ViewError::Unavailable);
        }
    };
    if !observed.current[0] {
        runtime.invalidate_source_state(input.logical_path());
        return Err(ViewError::Unavailable);
    }
    observed.outcome.ok_or(ViewError::Unavailable)
}

pub(super) fn invalidate_source(
    runtime: &mut WorkspaceRuntime,
    path: &str,
) -> Result<(), AnchorError> {
    if validate_logical_path(path).is_err() {
        return Err(AnchorError::InvalidInput);
    }
    if is_backwriter_spill(path) || runtime.selected_root(path).is_err() {
        return Err(AnchorError::Unavailable);
    }
    runtime.prune_dead_anchors();
    runtime.invalidate_source_state(path);
    Ok(())
}

fn validate(input: &Anddress) -> Result<(), AnchorError> {
    input.validate().map_err(|error| match error {
        AnddressError::UnsupportedVersion => AnchorError::UnsupportedVersion,
        AnddressError::Invalid | AnddressError::Encoding => AnchorError::InvalidInput,
        AnddressError::Resource => AnchorError::Unavailable,
    })
}

fn observe_current(
    runtime: &WorkspaceRuntime,
    input: &Anddress,
    inputs: &[Anddress],
    capture_focus: Option<usize>,
) -> Result<AnchoredObservation, ObservationError> {
    if input.workspace_coordinate() != runtime.workspace_coordinate
        || is_backwriter_spill(input.logical_path())
    {
        return Err(ObservationError::Read);
    }
    let mut file = runtime
        .open_admitted_source(input.logical_path())
        .map_err(|_| ObservationError::Read)?;
    observe_anchored(&mut file, inputs, capture_focus)
}

fn path_inputs(
    runtime: &WorkspaceRuntime,
    path: &str,
    focus: Anddress,
) -> Result<Vec<Anddress>, AnchorError> {
    let count = runtime
        .anchors
        .iter()
        .filter(|binding| binding.anddress.logical_path() == path)
        .count()
        + 1;
    let mut inputs = Vec::new();
    inputs
        .try_reserve_exact(count)
        .map_err(|_| AnchorError::Unavailable)?;
    for binding in &runtime.anchors {
        if binding.anddress.logical_path() == path {
            inputs.push(binding.anddress.clone());
        }
    }
    inputs.push(focus);
    Ok(inputs)
}

// src/layout/table/config/engine.rs

use crate::layout::table::types::{PinPosition, TableState};

pub fn reorder_columns(state: &mut TableState, from: usize, mut to: usize) {
    if from != to {
        if from < to {
            to = to.saturating_sub(1);
        }
        let item = state.display_order.remove(from);
        let target_idx = to.min(state.display_order.len());
        state.display_order.insert(target_idx, item);
    }
}

pub fn toggle_column_visibility(state: &mut TableState, col_id: &str, is_visible: bool) {
    if is_visible {
        state.visible_columns.insert(col_id.to_string());
    } else {
        state.visible_columns.remove(col_id);
    }
}

pub fn toggle_pin_left(state: &mut TableState, col_id: &str) {
    if let Some(col) = state.columns.get_mut(col_id) {
        col.pinned = if col.pinned == PinPosition::Left {
            PinPosition::None
        } else {
            PinPosition::Left
        };
    }
}
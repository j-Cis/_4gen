// src/layout/table/tools/engine.rs

use crate::layout::table::types::{TableAction, TableState};

#[derive(Clone)]
pub struct ToolbarButton<'a> {
    pub action: TableAction,
    pub label_or_icon: &'a str,
    pub tooltip: &'a str,
}

#[derive(Clone)]
pub struct ModifierButton<'a> {
    pub action: TableAction,
    pub label: &'a str,
    pub tooltip: &'a str,
}

#[derive(Clone)]
pub enum ToolbarItem<'a> {
    Button(ToolbarButton<'a>),
    Modifier(ModifierButton<'a>),
    Separator,
    Spacer(f32),
}

pub fn toggle_shift(state: &mut TableState) {
    state.simulated_shift = !state.simulated_shift;
}

pub fn toggle_ctrl(state: &mut TableState) {
    state.simulated_ctrl = !state.simulated_ctrl;
}

pub fn toggle_alt(state: &mut TableState) {
    state.simulated_alt = !state.simulated_alt;
}
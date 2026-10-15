// src/layout/table/tools/engine.rs

use crate::layout::table::types::TableAction;

#[derive(Clone)]
pub struct ToolbarButton<'a> {
    pub action: TableAction,
    pub label_or_icon: &'a str,
    pub tooltip: &'a str,
}

#[derive(Clone)]
pub enum ToolbarItem<'a> {
    Button(ToolbarButton<'a>),
    Separator,
    Spacer(f32),
}
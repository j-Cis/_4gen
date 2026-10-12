// src/layout/table.rs

use eframe::egui;
use crate::theme::tab::TableTheme;
use crate::layout::table_types::{TableAction, TableConfig, TableState};
use crate::layout::table_toolbar::TableToolbar;
use crate::layout::table_grid_engine::TableGridEngine;

pub struct AdvancedTable<'a> {
    pub theme: &'a TableTheme,
    pub config: &'a TableConfig,
    pub state: &'a mut TableState,
}

impl<'a> AdvancedTable<'a> {
    pub fn show<F>(
        &mut self,
        ui: &mut egui::Ui,
        row_indices: &[usize], // <-- TUTAJ TEŻ ODZYSKANE
        cell_renderer: F,
    ) -> Option<TableAction>
    where
        F: FnMut(usize, &str, &mut egui::Ui),
    {
        let action_triggered = TableToolbar::show(ui, self.theme);

        ui.add_space(4.0);

        TableGridEngine::render(
            ui,
            self.state,
            self.config,
            self.theme,
            row_indices,
            cell_renderer,
        );

        action_triggered
    }
}
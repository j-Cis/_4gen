// src/layout/table.rs

pub mod types;
pub mod grid_engine;

pub mod grid_render {
    #[path = "../grid_render_header.rs"]
    pub mod header;
    #[path = "../grid_render_pinned.rs"]
    pub mod pinned;
    #[path = "../grid_render_scroll.rs"]
    pub mod scroll;
}

pub mod config {
    pub mod engine;
    pub mod render;
}

pub mod tools {
    pub mod engine;
    pub mod render;
}

use eframe::egui;
use crate::theme::tab::TableTheme;

pub use types::{ColDef, PinPosition, SortDir, TableAction, TableConfig, TableState};
pub use config::render::TableConfigWindow;

pub struct AdvancedTable<'a> {
    pub theme: &'a TableTheme,
    pub config: &'a TableConfig,
    pub state: &'a mut TableState,
}

impl<'a> AdvancedTable<'a> {
    pub fn show<F>(
        &mut self,
        ui: &mut egui::Ui,
        row_indices: &[usize],
        cell_renderer: F,
    ) -> Option<TableAction>
    where
        F: FnMut(usize, &str, &mut egui::Ui),
    {
        let action_triggered = tools::render::TableToolbar::show(ui, self.theme, self.state);

        ui.add_space(4.0);

        grid_engine::TableGridEngine::render(
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
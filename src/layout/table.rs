// src/layout/table.rs

pub mod types;
pub mod grid_engine;

#[path = "table"]
pub mod grid_render {
    #[path = "grid_render_header.rs"]
    pub mod header;

    #[path = "grid_render_pinned.rs"]
    pub mod pinned;

    #[path = "grid_render_scroll.rs"]
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
use crate::layout::input::chips::{ChipContainerMode, ChipOverflow, ChipsContainer};

pub use types::{ColDef, PinPosition, RowPinPosition, SortDir, TableAction, TableConfig, TableState};
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
        // 1. Pasek narzędzi (Toolbar)
        let action_triggered = tools::render::TableToolbar::show(ui, self.theme, self.state);

        ui.add_space(3.0);

        // 2. Komponent Chips (pomiędzy toolbarem a tabelą)
        ChipsContainer::new(
            ui.id().with("table_chips_filter"),
            &mut self.state.filter_chips,
            &mut self.state.chip_input,
            &self.state.chip_pool,
            &mut self.state.selected_chip,
        )
        .mode(ChipContainerMode::SingleLine)
        .overflow(ChipOverflow::Scroll)
        .show(ui);

        ui.add_space(3.0);

        // 3. Nagłówki i ciało tabeli
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
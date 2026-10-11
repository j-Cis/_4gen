use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::collections::{HashMap, HashSet};

// --- 1. AKCJE Z PASKA NARZĘDZI ---
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TableAction {
    Prev,
    Next,
    Add,
    Edit,
    Delete,
    Merge,
    Duplicate,
    Tag,
    ConfigColumns,
}

// --- 2. GLOBALNY WYGLĄD ---
#[derive(Clone, Copy)]
pub struct TableTheme {
    pub header_bg: egui::Color32,
    pub row_even_bg: egui::Color32,
    pub row_odd_bg: egui::Color32,
    pub row_hover_bg: egui::Color32,
    pub row_selected_bg: egui::Color32,
}

// --- 3. ZASADY KONKRETNEJ TABELI ---
#[derive(Clone)]
pub struct TableConfig {
    pub allow_selection: bool,
    pub multi_select: bool,
    pub max_selection: Option<usize>,
    pub row_height: f32,
}

// --- 4. DEFINICJA KOLUMNY ---
#[derive(Clone)]
pub struct ColDef {
    pub id: String,
    pub title: String,
    pub width: Column,
    pub sortable: bool,
    pub regex_filter: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SortDir {
    Asc,
    Desc,
}

// --- 5. DYNAMICZNY STAN ---
pub struct TableState {
    pub columns: HashMap<String, ColDef>,
    pub display_order: Vec<String>,
    pub visible_columns: HashSet<String>,
    pub sort_rules: Vec<(String, SortDir)>,
    pub selected_rows: HashSet<usize>,
}

impl TableState {
    pub fn new(cols: Vec<ColDef>) -> Self {
        let mut columns = HashMap::new();
        let mut display_order = Vec::new();
        let mut visible_columns = HashSet::new();

        for col in cols {
            display_order.push(col.id.clone());
            visible_columns.insert(col.id.clone());
            columns.insert(col.id.clone(), col);
        }

        Self {
            columns,
            display_order,
            visible_columns,
            sort_rules: Vec::new(),
            selected_rows: HashSet::new(),
        }
    }

    pub fn handle_sort_click(&mut self, col_id: &str, shift_pressed: bool) {
        if !self.columns.get(col_id).unwrap().sortable {
            return;
        }
        let existing_idx = self.sort_rules.iter().position(|(id, _)| id == col_id);

        if shift_pressed {
            if let Some(idx) = existing_idx {
                self.sort_rules[idx].1 = match self.sort_rules[idx].1 {
                    SortDir::Asc => SortDir::Desc,
                    SortDir::Desc => SortDir::Asc,
                };
            } else {
                self.sort_rules.push((col_id.to_string(), SortDir::Asc));
            }
        } else {
            let new_dir = if let Some(idx) = existing_idx {
                if self.sort_rules.len() == 1 {
                    match self.sort_rules[idx].1 {
                        SortDir::Asc => SortDir::Desc,
                        SortDir::Desc => SortDir::Asc,
                    }
                } else {
                    SortDir::Asc
                }
            } else {
                SortDir::Asc
            };
            self.sort_rules.clear();
            self.sort_rules.push((col_id.to_string(), new_dir));
        }
    }

    // Dodano parametr ctrl_pressed
    pub fn toggle_selection(&mut self, row_idx: usize, config: &TableConfig, ctrl_pressed: bool) {
        if !config.allow_selection {
            return;
        }

        if ctrl_pressed && config.multi_select {
            // Dodawanie/Usuwanie pojedynczego z Ctrl
            if self.selected_rows.contains(&row_idx) {
                self.selected_rows.remove(&row_idx);
            } else {
                if let Some(limit) = config.max_selection {
                    if self.selected_rows.len() >= limit {
                        return;
                    }
                }
                self.selected_rows.insert(row_idx);
            }
        } else {
            // Bez Ctrl: Czyści wszystko i zaznacza tylko kliknięty (standard w OS)
            self.selected_rows.clear();
            self.selected_rows.insert(row_idx);
        }
    }
}

// --- 6. RENDERER TABELI ---
pub struct AdvancedTable<'a> {
    pub theme: &'a TableTheme,
    pub config: &'a TableConfig,
    pub state: &'a mut TableState,
}

impl<'a> AdvancedTable<'a> {
    pub fn show<F>(
        &mut self,
        ui: &mut egui::Ui,
        row_count: usize,
        mut cell_renderer: F,
    ) -> Option<TableAction>
    where
        F: FnMut(usize, &str, &mut egui::Ui),
    {
        let mut action_triggered = None;

        // 1. Pasek narzędzi (Toolbar) nad tabelą - Z TWOIMI ORYGINALNYMI IKONAMI
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            if ui
                .button("◀️")
                .on_hover_text("Poprzednie zaznaczenie")
                .clicked()
            {
                action_triggered = Some(TableAction::Prev);
            }
            if ui
                .button("▶️")
                .on_hover_text("Następne zaznaczenie")
                .clicked()
            {
                action_triggered = Some(TableAction::Next);
            }

            ui.add_space(8.0);
            if ui.button("➕").on_hover_text("Nowa pozycja").clicked() {
                action_triggered = Some(TableAction::Add);
            }
            if ui.button("📝").on_hover_text("Edytuj pozycję").clicked() {
                action_triggered = Some(TableAction::Edit);
            }
            if ui.button("➖").on_hover_text("Usuń pozycję").clicked() {
                action_triggered = Some(TableAction::Delete);
            }

            ui.add_space(8.0);
            if ui.button("🔀").on_hover_text("Scal wybrane").clicked() {
                action_triggered = Some(TableAction::Merge);
            }
            if ui.button("🔂").on_hover_text("Duplikuj pozycję").clicked() {
                action_triggered = Some(TableAction::Duplicate);
            }

            ui.add_space(8.0);
            if ui.button("🏷️").on_hover_text("Etykietuj pozycję").clicked() {
                action_triggered = Some(TableAction::Tag);
            }

            ui.add_space(8.0);
            if ui
                .button("📊")
                .on_hover_text("Konfiguruj kolumny")
                .clicked()
            {
                action_triggered = Some(TableAction::ConfigColumns);
            }
        });

        ui.add_space(4.0);

        let available_height = ui.available_height();

        // 2. Renderowanie samej tabeli Z POZIOMYM SCROLLEM i NATYWNYM TŁEM
        egui::ScrollArea::horizontal()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let active_columns: Vec<String> = self
                    .state
                    .display_order
                    .iter()
                    .filter(|id| self.state.visible_columns.contains(*id))
                    .cloned()
                    .collect();

                let shift_pressed = ui.input(|i| i.modifiers.shift);
                let ctrl_pressed = ui.input(|i| i.modifiers.command);

                // --- KLUCZOWA POPRAWKA: WSTRZYKNIĘCIE TWOICH KOLORÓW DO EGUI ---
                let mut visuals = ui.visuals().clone();

                // 1. Tło bazowe (wiersze parzyste)
                visuals.panel_fill = self.theme.row_even_bg;
                visuals.extreme_bg_color = self.theme.row_even_bg;

                // 2. Wiersze nieparzyste (.striped(true) używa koloru 'faint_bg_color')
                visuals.faint_bg_color = self.theme.row_odd_bg;

                // 3. Zaznaczenie (row.set_selected(true) używa 'selection.bg_fill')
                visuals.selection.bg_fill = self.theme.row_selected_bg;
                visuals.selection.stroke.color = egui::Color32::WHITE; // by tekst po zaznaczeniu był czytelny

                // 4. Podświetlenie myszką (hover)
                visuals.widgets.hovered.weak_bg_fill = self.theme.row_hover_bg;
                visuals.widgets.hovered.bg_fill = self.theme.row_hover_bg;

                // Subtelne, oryginalne linie oddzielające komórki
                visuals.widgets.noninteractive.bg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_black_alpha(40));

                // Aplikujemy Twój motyw do Egui na czas rysowania tabeli
                ui.ctx().set_visuals(visuals);
                // ----------------------------------------------------------------

                let header_bg = self.theme.header_bg;

                let mut builder = TableBuilder::new(ui)
                    .striped(true) // Teraz używa Twojego row_odd_bg!
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .sense(egui::Sense::click())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height);

                for col_id in &active_columns {
                    let col_def = self.state.columns.get(col_id).unwrap();
                    builder = builder.column(col_def.width.clone());
                }

                builder
                    .header(28.0, |mut header| {
                        for col_id in &active_columns {
                            let col_def = self.state.columns.get(col_id).unwrap().clone();

                            header.col(|cell_ui| {
                                cell_ui
                                    .painter()
                                    .rect_filled(cell_ui.max_rect(), 0.0, header_bg);
                                cell_ui.horizontal_centered(|cell_ui| {
                                    let mut title_text = col_def.title.clone();
                                    if let Some((idx, (_, dir))) = self
                                        .state
                                        .sort_rules
                                        .iter()
                                        .enumerate()
                                        .find(|(_, (id, _))| id == &col_def.id)
                                    {
                                        let arrow = match dir {
                                            SortDir::Asc => " 🔼",
                                            SortDir::Desc => " 🔽",
                                        };
                                        let priority = if self.state.sort_rules.len() > 1 {
                                            format!(" ({})", idx + 1)
                                        } else {
                                            "".to_string()
                                        };
                                        title_text = format!("{}{}{}", title_text, arrow, priority);
                                    }
                                    let text = egui::RichText::new(title_text)
                                        .strong()
                                        .color(egui::Color32::WHITE);
                                    let resp = cell_ui.selectable_label(false, text);
                                    if col_def.sortable && resp.clicked() {
                                        self.state.handle_sort_click(&col_def.id, shift_pressed);
                                    }
                                });
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(self.config.row_height, row_count, |mut row| {
                            let row_idx = row.index();
                            let is_selected = self.state.selected_rows.contains(&row_idx);

                            // Natywna selekcja - zapobiega wylewaniu się kolorów i mruganiu
                            row.set_selected(is_selected);

                            for col_id in &active_columns {
                                row.col(|cell_ui| {
                                    cell_renderer(row_idx, col_id, cell_ui);
                                });
                            }

                            if row.response().clicked() {
                                self.state
                                    .toggle_selection(row_idx, self.config, ctrl_pressed);
                            }
                        });
                    });
            });

        action_triggered
    }
}

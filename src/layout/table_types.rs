// src/layout/table_types.rs

use std::collections::{HashMap, HashSet};

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

#[derive(Clone, PartialEq, Debug)]
pub enum PinPosition {
    None,
    Left,
    // Right, // TODO: Tymczasowo wyłączone do czasu wdrożenia pełnego wsparcia
}

#[derive(Clone, PartialEq, Debug)]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone)]
pub struct ColDef {
    pub id: String,
    pub title: String,
    pub width_px: f32, // Sztywna szerokość w px do resizowania i zamrażania
    pub sortable: bool,
    pub regex_filter: String,
    pub pinned: PinPosition,
}

#[derive(Clone)]
pub struct TableConfig {
    pub allow_selection: bool,
    pub multi_select: bool,
    pub max_selection: Option<usize>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            allow_selection: true,
            multi_select: true,
            max_selection: None,
        }
    }
}

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

    pub fn get_computed_columns_order(&self) -> Vec<String> {
        let mut left_pinned = Vec::new();
        let mut unpinned = Vec::new();
        let mut right_pinned = Vec::new();

        for id in &self.display_order {
            if self.visible_columns.contains(id) {
                if let Some(col) = self.columns.get(id) {
                    match col.pinned {
                        PinPosition::Left => left_pinned.push(id.clone()),
                        PinPosition::None => unpinned.push(id.clone()),
                        // PinPosition::Right => right_pinned.push(id.clone()),
                    }
                }
            }
        }

        let mut final_order = left_pinned;
        final_order.extend(unpinned);
        final_order.extend(right_pinned);
        final_order
    }

    pub fn handle_sort_click(&mut self, col_id: &str, shift_pressed: bool) {
        if let Some(col) = self.columns.get(col_id) {
            if !col.sortable {
                return;
            }
        } else {
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

    pub fn toggle_selection(&mut self, row_idx: usize, config: &TableConfig, ctrl_pressed: bool) {
        if !config.allow_selection {
            return;
        }

        if ctrl_pressed && config.multi_select {
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
            self.selected_rows.clear();
            self.selected_rows.insert(row_idx);
        }
    }
}
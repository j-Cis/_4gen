// src/layout/table/types.rs

use std::collections::{HashMap, HashSet};
use crate::layout::input::chips::ChipItem;
use crate::layout::system_status::SystemModifiers;

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
	Right, // TODO: Tymczasowo wyłączone do czasu wdrożenia pełnego wsparcia
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RowPinPosition {
	None,
	Top,
	Bottom,
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
	pub pinned_rows: HashMap<usize, RowPinPosition>,

	// Stan komponentu Chips (filtry)
	pub filter_chips: Vec<ChipItem>,
	pub chip_input: String,
	pub chip_pool: Vec<String>,
	pub selected_chip: Option<String>,
}

impl TableState {
	pub fn new(cols: Vec<ColDef>) -> Self {
		let mut columns = HashMap::new();
		let mut display_order = Vec::new();
		let mut visible_columns = HashSet::new();
		let mut chip_pool = Vec::new();

		for col in cols {
			display_order.push(col.id.clone());
			visible_columns.insert(col.id.clone());
			chip_pool.push(col.title.clone());
			columns.insert(col.id.clone(), col);
		}

		// Przykładowe chipy startowe do weryfikacji wyglądu
		let filter_chips = vec![
			ChipItem::Parametric {
				id: "chip_sample_1".into(),
				key: "Płeć".into(),
				slot: crate::layout::input::chips::ChipSlot::select("Kobieta", vec!["Kobieta".into(), "Mężczyzna".into()]),
				is_editing: false,
			},
			ChipItem::Parametric {
				id: "chip_sample_2".into(),
				key: "Nazwisko".into(),
				slot: crate::layout::input::chips::ChipSlot::text("Kowalski"),
				is_editing: false,
			},
			ChipItem::Parametric {
				id: "chip_sample_3".into(),
				key: "Wiek".into(),
				slot: crate::layout::input::chips::ChipSlot::range(20.0, 60.0, 0.0, 100.0),
				is_editing: false,
			},
			ChipItem::Simple {
				id: "chip_sample_4".into(),
				label: "Warszawa".into(),
			},
		];

		Self {
			columns,
			display_order,
			visible_columns,
			sort_rules: Vec::new(),
			selected_rows: HashSet::new(),
			pinned_rows: HashMap::new(),
			filter_chips,
			chip_input: String::new(),
			chip_pool,
			selected_chip: None,
		}
	}

	pub fn is_shift_active(&self, input: &eframe::egui::InputState, sys: &SystemModifiers) -> bool {
		sys.shift || input.modifiers.shift
	}

	pub fn is_ctrl_active(&self, input: &eframe::egui::InputState, sys: &SystemModifiers) -> bool {
		sys.ctrl || input.modifiers.command || input.modifiers.ctrl
	}

	pub fn is_alt_active(&self, input: &eframe::egui::InputState, sys: &SystemModifiers) -> bool {
		sys.alt || input.modifiers.alt
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
						PinPosition::Right => right_pinned.push(id.clone()),
					}
				}
			}
		}

		let mut final_order = left_pinned;
		final_order.extend(unpinned);
		final_order.extend(right_pinned);
		final_order
	}

	pub fn toggle_pin_left(&mut self, col_id: &str) {
		if let Some(col) = self.columns.get_mut(col_id) {
			if col.pinned == PinPosition::Left {
				col.pinned = PinPosition::None;
			} else if col.pinned == PinPosition::None {
				col.pinned = PinPosition::Left;
			}
			// Jeśli jest Right - nic nie robimy (najpierw trzeba odpiąć)
		}
	}

	pub fn toggle_pin_right(&mut self, col_id: &str) {
		if let Some(col) = self.columns.get_mut(col_id) {
			if col.pinned == PinPosition::Right {
				col.pinned = PinPosition::None;
			} else if col.pinned == PinPosition::None {
				col.pinned = PinPosition::Right;
			}
			// Jeśli jest Left - nic nie robimy (najpierw trzeba odpiąć)
		}
	}

	pub fn row_pin(&self, row_idx: usize) -> RowPinPosition {
		self.pinned_rows.get(&row_idx).copied().unwrap_or(RowPinPosition::None)
	}

	pub fn toggle_pin_row_top(&mut self, row_idx: usize) {
		let current = self.row_pin(row_idx);
		match current {
			RowPinPosition::Top => {
				self.pinned_rows.remove(&row_idx);
			}
			RowPinPosition::None => {
				self.pinned_rows.insert(row_idx, RowPinPosition::Top);
			}
			RowPinPosition::Bottom => {
				// Zablokowane: aby przypiąć u góry, wiersz musi najpierw zostać odpięty z dołu
			}
		}
	}

	pub fn toggle_pin_row_bottom(&mut self, row_idx: usize) {
		let current = self.row_pin(row_idx);
		match current {
			RowPinPosition::Bottom => {
				self.pinned_rows.remove(&row_idx);
			}
			RowPinPosition::None => {
				self.pinned_rows.insert(row_idx, RowPinPosition::Bottom);
			}
			RowPinPosition::Top => {
				// Zablokowane: aby przypiąć u dołu, wiersz musi najpierw zostać odpięty z góry
			}
		}
	}

	pub fn handle_sort_click(&mut self, col_id: &str, ctrl_pressed: bool) {
		if let Some(col) = self.columns.get(col_id) {
			if !col.sortable {
				return;
			}
		} else {
			return;
		}

		let existing_idx = self.sort_rules.iter().position(|(id, _)| id == col_id);

		if ctrl_pressed {
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

// Dodaj pod istniejącymi definicjami w src/layout/table/types.rs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConfigColItem {
	Visible,
	Title,
	Pinned,
	Position,
}
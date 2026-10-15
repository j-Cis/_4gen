// src/widget/records_individuals.rs

use crate::layout::table::{
	AdvancedTable, TableAction, TableConfigWindow,
	types::{ColDef, PinPosition, SortDir, TableConfig, TableState},
};
use crate::theme::tab::TableTheme;
use crate::layout::system_status::SystemStatus;
use eframe::egui;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct IndividualRecord {
	pub id: usize,
	pub first_names: String,
	pub last_names: String,
	pub gender: String,
	pub birth_date: Option<String>,
	pub birth_place: Option<String>,
	pub tags: Vec<String>,
}

pub struct RecordsIndividualsWidget {
	pub theme: TableTheme,
	pub config: TableConfig,
	pub state: TableState,
	pub config_window: TableConfigWindow,
	pub records: Vec<IndividualRecord>,
}

impl Default for RecordsIndividualsWidget {
	fn default() -> Self {
		let state = TableState::new(vec![
			ColDef {
				id: "id".into(),
				title: "ID".into(),
				width_px: 50.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::Left,
			},
			ColDef {
				id: "first_names".into(),
				title: "Imiona".into(),
				width_px: 140.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
			ColDef {
				id: "last_names".into(),
				title: "Nazwiska".into(),
				width_px: 200.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
			ColDef {
				id: "gender".into(),
				title: "Płeć".into(),
				width_px: 100.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
			ColDef {
				id: "birth_date".into(),
				title: "Data Urodzenia".into(),
				width_px: 120.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
			ColDef {
				id: "birth_place".into(),
				title: "Miejsce Urodzenia".into(),
				width_px: 160.0,
				sortable: true,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
			ColDef {
				id: "tags".into(),
				title: "Etykiety".into(),
				width_px: 220.0,
				sortable: false,
				regex_filter: "".into(),
				pinned: PinPosition::None,
			},
		]);

		let mut records = Vec::new();
		let sample_data = [
			("Jan Adam", "Kowalski", "Mężczyzna", "1885-04-12", "Warszawa", "Zweryfikowane, Linia męska"),
			("Maria Anna", "Wiśniewska (z d. Kowalska)", "Kobieta", "1910-08-22", "Kraków", "Do sprawdzenia"),
			("Piotr", "Nowak", "Mężczyzna", "1902-11-30", "Poznań", "Linia męska"),
			("Zofia Helena", "Zielińska", "Kobieta", "1925-01-15", "Gdańsk", "Akt urodzenia"),
			("Stanisław", "Kamiński", "Mężczyzna", "1879-06-05", "Lwów", "Archiwum AGAD"),
			("Helena", "Dąbrowska", "Kobieta", "1893-09-18", "Wilno", "Ślepy zaułek"),
			("Andrzej", "Kozłowski", "Mężczyzna", "1931-03-24", "Wrocław", "Do weryfikacji"),
			("Katarzyna", "Jankowska", "Kobieta", "1918-12-01", "Lublin", "Linia żeńska"),
			("Michał", "Wojciechowski", "Mężczyzna", "1865-07-14", "Stanisławów", "Materiały zebrane"),
			("Ewa", "Kwiatkowska", "Kobieta", "1940-05-09", "Katowice", "Zweryfikowane"),
		];

		for i in 1..=25 {
			let sample = &sample_data[(i - 1) % sample_data.len()];
			records.push(IndividualRecord {
				id: i,
				first_names: sample.0.into(),
				last_names: sample.1.into(),
				gender: sample.2.into(),
				birth_date: Some(sample.3.into()),
				birth_place: Some(sample.4.into()),
				tags: vec![sample.5.into()],
			});
		}

		Self {
			theme: TableTheme::default(),
			config: TableConfig::default(),
			state,
			config_window: TableConfigWindow::default(),
			records,
		}
	}
}

impl RecordsIndividualsWidget {
	pub fn show(&mut self, ui: &mut egui::Ui, sys: &mut SystemStatus) {
		let mut row_indices: Vec<usize> = (0..self.records.len()).collect();

		let sort_rules = self.state.sort_rules.clone();
		if !sort_rules.is_empty() {
			row_indices.sort_by(|&a_idx, &b_idx| {
				let rec_a = &self.records[a_idx];
				let rec_b = &self.records[b_idx];

				for (col_id, dir) in &sort_rules {
					let cmp = match col_id.as_str() {
						"id" => rec_a.id.cmp(&rec_b.id),
						"first_names" => rec_a.first_names.cmp(&rec_b.first_names),
						"last_names" => rec_a.last_names.cmp(&rec_b.last_names),
						"gender" => rec_a.gender.cmp(&rec_b.gender),
						"birth_date" => rec_a.birth_date.cmp(&rec_b.birth_date),
						"birth_place" => rec_a.birth_place.cmp(&rec_b.birth_place),
						_ => Ordering::Equal,
					};

					if cmp != Ordering::Equal {
						return if *dir == SortDir::Asc { cmp } else { cmp.reverse() };
					}
				}
				Ordering::Equal
			});
		}

		let mut table = AdvancedTable {
			theme: &self.theme,
			config: &self.config,
			state: &mut self.state,
		};

		let action = table.show(ui, &row_indices, &sys.modifiers, |true_idx, col_id, cell_ui| {
			let item = &self.records[true_idx];
			match col_id {
				"id" => { cell_ui.label(item.id.to_string()); }
				"first_names" => { cell_ui.label(&item.first_names); }
				"last_names" => { cell_ui.label(&item.last_names); }
				"gender" => { cell_ui.label(&item.gender); }
				"birth_date" => { cell_ui.label(item.birth_date.as_deref().unwrap_or("-")); }
				"birth_place" => { cell_ui.label(item.birth_place.as_deref().unwrap_or("-")); }
				"tags" => { cell_ui.label(item.tags.join(", ")); }
				_ => {}
			}
		});

		if let Some(TableAction::ConfigColumns) = action {
			self.config_window.window.open();
		}

		self.config_window.show(ui, &mut self.state);
	}
}
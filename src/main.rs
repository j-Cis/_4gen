use eframe::egui;
use egui_extras::Column;
use x::{
    data::{
        sample_events::{get_sample_events, Event},
        sample_individuals::{get_sample_individuals, Individual},
    },
    element::{
        frame::{CustomFrame, FrameType},
        side_menu::{MenuItem, SideMenu},
        table::{AdvancedTable, ColDef, TableAction, TableConfig, TableState, TableTheme},
    },
    template::association_form::{
        AssociationFormState, AssociationFormTemplate,
    },
    i,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Aplikacja _4gen",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // --- ZMIANA GLOBALNEGO TŁA APLIKACJI ---
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = egui::Color32::from_hex("#202020").unwrap();
            cc.egui_ctx.set_visuals(visuals);
            // ---------------------------------------

            Ok(Box::<App>::default())
        }),
    )
}

struct App<'a> {
    menu_left: SideMenu,
    menu_items_left: Vec<MenuItem<'a>>,

    menu_right: SideMenu,
    menu_items_right: Vec<MenuItem<'a>>,

    global_table_theme: TableTheme,
    table_config: TableConfig,

    // --- STAN DLA ZAKŁADKI: OSOBY ---
    individuals_state: TableState,
    individuals_data: Vec<Individual>,

    // --- STAN DLA ZAKŁADKI: WYDARZENIA ---
    events_state: TableState,
    events_data: Vec<Event>,

    config_frame: CustomFrame,

    // --- DEDYKOWANE OKNO DLA KSIĘGI METRYKALNEJ ---
    vital_register_frame: CustomFrame,
    vital_register_form_state: AssociationFormState,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            menu_left: SideMenu::new(
                egui::Color32::from_hex("#072b47").unwrap(),
                48.0,
                200.0,
                true,
                Some(2), // Domyślnie zakładka "Osoby" (indeks 2)
            ),
            menu_items_left: vec![
                MenuItem {
                    icon: i!("twemoji/1f4d6").into(),
                    text: "Źródła".into(),
                }, // 0
                MenuItem {
                    icon: i!("twemoji/1f4da").into(),
                    text: "Kolekcje".into(),
                }, // 1
                MenuItem {
                    icon: i!("twemoji/1faaa").into(),
                    text: "Osoby".into(),
                }, // 2
                MenuItem {
                    icon: i!("twemoji/1f4c6").into(),
                    text: "Wydarzenia".into(),
                }, // 3
                MenuItem {
                    icon: i!("twemoji/1f5fa").into(),
                    text: "Lokalizacje".into(),
                }, // 4
                MenuItem {
                    icon: i!("twemoji/1f4d2").into(),
                    text: "Notatki".into(),
                }, // 5
                MenuItem {
                    icon: i!("twemoji/1f9e9").into(),
                    text: "Załączniki".into(),
                }, // 6
            ],

            menu_right: SideMenu::new(
                egui::Color32::from_hex("#072b47").unwrap(),
                48.0,
                200.0,
                true,
                None,
            ),
            menu_items_right: vec![
                MenuItem {
                    icon: "➕".into(),
                    text: "Nowy element".into(),
                },
                MenuItem {
                    icon: "🔍".into(),
                    text: "Wyszukaj".into(),
                },
                MenuItem {
                    icon: "🔔".into(),
                    text: "Powiadomienia".into(),
                },
                MenuItem {
                    icon: i!("twemoji/1fa9b").into(),
                    text: "Konfiguracja".into(),
                },
            ],

            global_table_theme: TableTheme {
                header_bg: egui::Color32::from_hex("#000000").unwrap(),
                row_even_bg: egui::Color32::from_hex("#202020").unwrap(),
                row_odd_bg: egui::Color32::from_hex("#1a1a1a").unwrap(),
                row_hover_bg: egui::Color32::from_hex("#03121e").unwrap(),
                row_selected_bg: egui::Color32::from_hex("#062741").unwrap(),
            },

            table_config: TableConfig {
                allow_selection: true,
                multi_select: true,
                max_selection: None, // Bez limitu
                row_height: 35.0,
            },

            // --- DEFINICJA KOLUMN DLA OSÓB ---
            individuals_state: TableState::new(vec![
                ColDef {
                    id: "id".into(),
                    title: "ID".into(),
                    width: Column::exact(40.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "first_names".into(),
                    title: "Imiona".into(),
                    width: Column::initial(120.0).at_least(80.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "last_names".into(),
                    title: "Nazwiska".into(),
                    width: Column::initial(150.0).at_least(100.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "gender".into(),
                    title: "Płeć".into(),
                    width: Column::exact(90.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "birth_date".into(),
                    title: "Data Urodzenia".into(),
                    width: Column::initial(110.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "birth_place".into(),
                    title: "Miejsce Urodzenia".into(),
                    width: Column::initial(140.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "tags".into(),
                    title: "Etykiety".into(),
                    width: Column::remainder(),
                    sortable: false,
                    regex_filter: "".into(),
                },
            ]),
            individuals_data: get_sample_individuals(),

            // --- DEFINICJA KOLUMN DLA WYDARZEŃ ---
            events_state: TableState::new(vec![
                ColDef {
                    id: "id".into(),
                    title: "ID".into(),
                    width: Column::exact(40.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "event_type".into(),
                    title: "Rodzaj".into(),
                    width: Column::initial(120.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "date".into(),
                    title: "Data".into(),
                    width: Column::initial(110.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "place".into(),
                    title: "Miejsce".into(),
                    width: Column::initial(150.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "description".into(),
                    title: "Opis".into(),
                    width: Column::remainder(),
                    sortable: false,
                    regex_filter: "".into(),
                },
                ColDef {
                    id: "participants".into(),
                    title: "Uczestnicy".into(),
                    width: Column::initial(100.0),
                    sortable: true,
                    regex_filter: "".into(),
                },
            ]),
            events_data: get_sample_events(),

            config_frame: CustomFrame::new(
                "col_config",
                "Konfiguracja Kolumn",
                "⚙",
                FrameType::Floating,
            ),

            // INICJALIZACJA DEDYKOWANEGO OKNA DLA KSIĘGI METRYKALNEJ
            vital_register_frame: CustomFrame::new(
                "vital_register_form_frame",
                "Opis księgi metrykalnej",
                "📖",
                FrameType::Floating,
            ),
            vital_register_form_state: AssociationFormState::default(),
        }
    }
}

impl<'a> eframe::App for App<'a> {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_hex("#202020")
            .unwrap()
            .to_normalized_gamma_f32()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ZACZYNAMY OD LEWEJ KRAWĘDZI
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                // 1. LEWA LISTWA (A+B)
                self.menu_left.show(ui, &self.menu_items_left);

                // ZMIENIAMY KIERUNEK: TERAZ BUDUJEMY OD PRAWEJ KRAWĘDZI
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    // 2. PRAWA LISTWA (A+B)
                    self.menu_right.show(ui, &self.menu_items_right);

                    // OTWARCIE OKNA PŁYWAJĄCEGO PO KLIKNIĘCIU "Nowy element" (indeks 0)
                    if self.menu_right.active_index == Some(0) {
                        self.vital_register_frame.open();
                        self.menu_right.active_index = None;
                    }

                    // 3. KANWA "C" (ŚRODEK) - Wyłącznie dla zakładek lewego menu!
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        ui.set_clip_rect(ui.available_rect_before_wrap());

                        ui.add_space(10.0);

                        let active_tab_name = if let Some(idx) = self.menu_left.active_index {
                            self.menu_items_left[idx].text.clone()
                        } else {
                            "".to_string()
                        };

                        ui.heading(format!("Widok: {}", active_tab_name));
                        ui.add_space(10.0);

                        let mut action = None;

                        if active_tab_name == "Osoby" {
                            let mut table = AdvancedTable {
                                theme: &self.global_table_theme,
                                config: &self.table_config,
                                state: &mut self.individuals_state,
                            };

                            action = table.show(
                                ui,
                                self.individuals_data.len(),
                                |index, col_id, cell_ui| {
                                    let item = &self.individuals_data[index];
                                    match col_id {
                                        "id" => {
                                            cell_ui.label(item.id.to_string());
                                        }
                                        "first_names" => {
                                            cell_ui.label(&item.first_names);
                                        }
                                        "last_names" => {
                                            cell_ui.label(&item.last_names);
                                        }
                                        "gender" => {
                                            cell_ui.label(&item.gender);
                                        }
                                        "birth_date" => {
                                            cell_ui
                                                .label(item.birth_date.as_deref().unwrap_or("-"));
                                        }
                                        "birth_place" => {
                                            cell_ui
                                                .label(item.birth_place.as_deref().unwrap_or("-"));
                                        }
                                        "tags" => {
                                            cell_ui.label(item.tags.join(", "));
                                        }
                                        _ => {}
                                    }
                                },
                            );
                        } else if active_tab_name == "Wydarzenia" {
                            let mut table = AdvancedTable {
                                theme: &self.global_table_theme,
                                config: &self.table_config,
                                state: &mut self.events_state,
                            };

                            action =
                                table.show(ui, self.events_data.len(), |index, col_id, cell_ui| {
                                    let item = &self.events_data[index];
                                    match col_id {
                                        "id" => {
                                            cell_ui.label(item.id.to_string());
                                        }
                                        "event_type" => {
                                            cell_ui.label(&item.event_type);
                                        }
                                        "date" => {
                                            cell_ui.label(item.date.as_deref().unwrap_or("-"));
                                        }
                                        "place" => {
                                            cell_ui.label(item.place.as_deref().unwrap_or("-"));
                                        }
                                        "description" => {
                                            cell_ui.label(&item.description);
                                        }
                                        "participants" => {
                                            cell_ui.label(format!(
                                                "Osoby: {}",
                                                item.participants_count
                                            ));
                                        }
                                        _ => {}
                                    }
                                });
                        } else {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    egui::RichText::new("Wybierz zakładkę z lewego menu.")
                                        .weak()
                                        .size(18.0),
                                );
                            });
                        }

                        if let Some(akcja) = action {
                            match akcja {
                                TableAction::ConfigColumns => self.config_frame.open(),
                                other => println!("Kliknięto akcję: {:?}", other),
                            }
                        }

                        // WYŚWIETLENIE OKNA PŁYWAJĄCEGO DLA KONFIGURACJI KOLUMN
                        self.config_frame.show(ui.ctx(), |frame_ui, content_open| {
                            frame_ui.label(format!("Konfiguracja dla widoku: {}", active_tab_name));
                            if frame_ui.button("Zamknij").clicked() {
                                *content_open = false;
                            }
                        });

                        // WYŚWIETLENIE OKNA PŁYWAJĄCEGO DLA KARTY KSIĘGI METRYKALNEJ
                        self.vital_register_frame.show(
                            ui.ctx(),
                            |frame_ui, content_open| {
                                let mut form = AssociationFormTemplate::new(
                                    &mut self.vital_register_form_state,
                                );
                                if let Some(saved) = form.show(frame_ui) {
                                    if saved {
                                        println!("Zapisano nową księgę metrykalną do XML!");
                                    }
                                    *content_open = false;
                                }
                            },
                        );
                    });
                });
            },
        );
    }
}
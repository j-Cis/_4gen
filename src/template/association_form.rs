use eframe::egui;

/// Model pojedynczej miejscowości z oznaczeniem gwiazdkowym (np. "Holihrady", "***")
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceChip {
    pub name: String,
    pub stars: String,
}

/// Model objaśnienia gwiazdek / przypisu terytorialnego
#[derive(Clone, Debug, PartialEq)]
pub struct StarLegendItem {
    pub symbol: String,
    pub text: String,
}

/// Model pojedynczego wpisu archiwalnego (Sygnatura / Archiwum)
#[derive(Clone, Debug, PartialEq)]
pub struct ArchiveEntry {
    pub archive: String,
    pub signature: String,
    pub old_signature: String,
    pub microfilm: String,
    pub fonds: String,
    pub url_archive: String,
    pub url_scans: String,
}

impl Default for ArchiveEntry {
    fn default() -> Self {
        Self {
            archive: "AGAD".into(),
            signature: "1817".into(),
            old_signature: "915 (521-523)".into(),
            microfilm: "92577".into(),
            fonds: "Księgi metrykalne parafii wyznania rzymskokatolickiego z archidiecezji lwowskiej".into(),
            url_archive: "https://www.szukajwarchiwach.gov.pl/".into(),
            url_scans: "http://agadd.home.net.pl/".into(),
        }
    }
}

/// Główny stan formularza dodawania / edycji zasobu archiwalnego
pub struct AssociationFormState {
    // Sekcja 01
    pub parish: String,
    pub deanery: String,
    pub rite: String,
    pub language: String,
    pub book_desc: String,
    pub record_births: bool,
    pub record_marriages: bool,
    pub record_deaths: bool,
    pub contents_desc: String,

    // Sekcja 02
    pub towns: Vec<PlaceChip>,
    pub town_input: String,

    pub villages: Vec<PlaceChip>,
    pub village_input: String,

    pub star_legends: Vec<StarLegendItem>,

    // Sekcja 03
    pub archive_entries: Vec<ArchiveEntry>,
    pub date_from: String,
    pub date_to: String,
    pub notes: String,
}

impl Default for AssociationFormState {
    fn default() -> Self {
        Self {
            parish: "Zaleszczyki".into(),
            deanery: "Jazłowiec".into(),
            rite: "rzymskokatolicki".into(),
            language: "łac., pol.".into(),
            book_desc: "składki, zszywki, s. 32".into(),
            record_births: true,
            record_marriages: true,
            record_deaths: true,
            contents_desc: "Kopie z ksiąg metrykalnych — ur. [x], śl. [x], zg. [x]".into(),

            towns: vec![PlaceChip { name: "Zaleszczyki".into(), stars: "".into() }],
            town_input: String::new(),

            villages: vec![
                PlaceChip { name: "Bedrykowce".into(), stars: "".into() },
                PlaceChip { name: "Błyszczanka".into(), stars: "**".into() },
                PlaceChip { name: "Duninów".into(), stars: "*".into() },
                PlaceChip { name: "Holihrady".into(), stars: "***".into() },
            ],
            village_input: String::new(),

            star_legends: vec![
                StarLegendItem {
                    symbol: "*".into(),
                    text: "Szczytowce - od 1904 r. samodzielna parafia wyodrębniona z par. Zaleszczyki".into(),
                },
                StarLegendItem {
                    symbol: "**".into(),
                    text: "Uhryńkowce - od 1911 r. samodzielna parafia wyodrębniona z par. Zaleszczyki".into(),
                },
            ],

            archive_entries: vec![ArchiveEntry::default()],
            date_from: "1895".into(),
            date_to: "1895".into(),
            notes: String::new(),
        }
    }
}

/// Szablon formularza UI
pub struct AssociationFormTemplate<'a> {
    pub state: &'a mut AssociationFormState,
}

impl<'a> AssociationFormTemplate<'a> {
    pub fn new(state: &'a mut AssociationFormState) -> Self {
        Self { state }
    }

    /// Renderowanie pełnego formularza wewnątrz przydzielonego obszaru Ui
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<bool> {
        let mut save_triggered = None;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(8.0);

                // Nagłówek formularza
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("ARCHIWUM · EWIDENCJA METRYKALNA").size(11.0).strong().color(egui::Color32::from_hex("#8bc6df").unwrap()));
                        ui.heading("Opis ksiąg metrykalnych");
                        ui.label(egui::RichText::new("Formularz opisu jednostki archiwalnej / zasobu").weak().size(13.0));
                    });
                });

                ui.add_space(14.0);

                // --- SEKCJA 01: Przynależność i opis ---
                self.render_card(ui, "01", "Przynależność parafialna i opis księgi", "Parafia, dekanat, ryt, cechy fizyczne oraz zawartość", |ui, state| {
                    ui.columns(3, |cols| {
                        cols[0].label("Parafia *");
                        cols[0].text_edit_singleline(&mut state.parish);

                        cols[1].label("Dekanat");
                        cols[1].text_edit_singleline(&mut state.deanery);

                        cols[2].label("Ryt / Obrządek");
                        cols[2].text_edit_singleline(&mut state.rite);
                    });

                    ui.add_space(10.0);

                    ui.columns(2, |cols| {
                        cols[0].label("Język");
                        cols[0].text_edit_singleline(&mut state.language);

                        cols[1].label("Opis księgi / stan zachowania");
                        cols[1].text_edit_singleline(&mut state.book_desc);
                    });

                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Zawartość ksiąg").strong().size(12.0));
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut state.record_births, "Urodzenia (ur.)");
                        ui.add_space(15.0);
                        ui.checkbox(&mut state.record_marriages, "Małżeństwa (śl.)");
                        ui.add_space(15.0);
                        ui.checkbox(&mut state.record_deaths, "Zgony (zg.)");
                    });

                    ui.add_space(10.0);
                    ui.label("Opis zawartości");
                    ui.text_edit_singleline(&mut state.contents_desc);
                });

                ui.add_space(14.0);

                // --- SEKCJA 02: Miejscowości ---
                self.render_card(ui, "02", "Miejscowości objęte księgami", "Miasta parafialne, wsie filialne oraz objaśnienia terytorialne", |ui, state| {

                    // Chips - Miasta
                    ui.label(egui::RichText::new(format!("Miasta ({}):", state.towns.len())).strong());
                    Self::render_chips_input(ui, &mut state.towns, &mut state.town_input, "Wpisz miasto i wciśnij Enter…");

                    ui.add_space(12.0);

                    // Chips - Wsie
                    ui.label(egui::RichText::new(format!("Wsie / Miejscowości filialne ({}):", state.villages.len())).strong());
                    Self::render_chips_input(ui, &mut state.villages, &mut state.village_input, "Wpisz miejscowość (np. Holihrady ***) i wciśnij Enter…");
                    ui.label(egui::RichText::new("Wpisz nazwę i naciśnij Enter lub przecinek. Gwiazdki (* / **) możesz dopisać bezpośrednio.").weak().size(11.0));

                    ui.add_space(14.0);
                    ui.separator();
                    ui.add_space(6.0);

                    // Objaśnienia gwiazdek
                    ui.label(egui::RichText::new("Objaśnienia gwiazdek (* / ** / ***)").strong());

                    let mut to_remove_legend = None;
                    for (idx, legend) in state.star_legends.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.add(egui::TextEdit::singleline(&mut legend.symbol).desired_width(50.0));
                            ui.add(egui::TextEdit::multiline(&mut legend.text).desired_rows(1));
                            if ui.button("🗑").on_hover_text("Usuń objaśnienie").clicked() {
                                to_remove_legend = Some(idx);
                            }
                        });
                    }
                    if let Some(idx) = to_remove_legend {
                        state.star_legends.remove(idx);
                    }

                    if ui.button("➕ Dodaj kolejne objaśnienie gwiazdek").clicked() {
                        state.star_legends.push(StarLegendItem { symbol: "*".into(), text: String::new() });
                    }
                });

                ui.add_space(14.0);

                // --- SEKCJA 03: Oznaczenia Archiwalne ---
                self.render_card(ui, "03", "Oznaczenia archiwalne", "Wiele sygnatur i zespołów z różnych archiwów", |ui, state| {
                    let mut to_remove_archive = None;

                    for (idx, archive) in state.archive_entries.iter_mut().enumerate() {
                        egui::Frame::group(ui.style())
                            .fill(egui::Color32::from_hex("#1a222d").unwrap())
                            .inner_margin(10)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("Oznaczenie archiwalne #{}", idx + 1)).strong().color(egui::Color32::from_hex("#8bc6df").unwrap()));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("Usuń").clicked() {
                                            to_remove_archive = Some(idx);
                                        }
                                    });
                                });

                                ui.add_space(6.0);

                                ui.columns(4, |cols| {
                                    cols[0].label("Archiwum");
                                    cols[0].text_edit_singleline(&mut archive.archive);

                                    cols[1].label("Sygnatura");
                                    cols[1].text_edit_singleline(&mut archive.signature);

                                    cols[2].label("Dawna sygnatura");
                                    cols[2].text_edit_singleline(&mut archive.old_signature);

                                    cols[3].label("Mikrofilm");
                                    cols[3].text_edit_singleline(&mut archive.microfilm);
                                });

                                ui.add_space(6.0);

                                ui.label("Zespół archiwalny");
                                ui.text_edit_singleline(&mut archive.fonds);

                                ui.columns(2, |cols| {
                                    cols[0].label("Link SzukajwArchiwach");
                                    cols[0].text_edit_singleline(&mut archive.url_archive);

                                    cols[1].label("Link do skanów / indeksu");
                                    cols[1].text_edit_singleline(&mut archive.url_scans);
                                });
                            });
                        ui.add_space(8.0);
                    }

                    if let Some(idx) = to_remove_archive {
                        state.archive_entries.remove(idx);
                    }

                    if ui.button("➕ Dodaj oznaczenie archiwalne").clicked() {
                        state.archive_entries.push(ArchiveEntry {
                            archive: String::new(),
                            signature: String::new(),
                            old_signature: String::new(),
                            microfilm: String::new(),
                            fonds: String::new(),
                            url_archive: String::new(),
                            url_scans: String::new(),
                        });
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Daty skrajne od:");
                        ui.add(egui::TextEdit::singleline(&mut state.date_from).desired_width(70.0));
                        ui.add_space(10.0);
                        ui.label("do:");
                        ui.add(egui::TextEdit::singleline(&mut state.date_to).desired_width(70.0));
                    });

                    ui.add_space(8.0);
                    ui.label("Uwagi");
                    ui.text_edit_multiline(&mut state.notes);
                });

                ui.add_space(16.0);

                // --- PRZYCISKI AKCJI NA DOLE ---
                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new("Zapisz zasób w XML").fill(egui::Color32::from_hex("#245b75").unwrap())).clicked() {
                        save_triggered = Some(true);
                    }
                    if ui.button("Anuluj").clicked() {
                        save_triggered = Some(false);
                    }
                });

                ui.add_space(20.0);
            });

        save_triggered
    }

    /// Pomocniczy podkomponent dla kart (Card Section)
    fn render_card<F>(&mut self, ui: &mut egui::Ui, num: &str, title: &str, subtitle: &str, content: F)
    where
        F: FnOnce(&mut egui::Ui, &mut AssociationFormState),
    {
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_hex("#1b2432").unwrap())
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_hex("#364357").unwrap()))
            .corner_radius(12.0)
            .inner_margin(16)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Badge z numerem sekcji
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 6.0, egui::Color32::from_hex("#245b75").unwrap());
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        num,
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );

                    ui.add_space(8.0);

                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(title).strong().size(15.0));
                        ui.label(egui::RichText::new(subtitle).weak().size(12.0));
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(10.0);

                content(ui, self.state);
            });
    }

    /// Pomocniczy komponent do wprowadzania "Chips" (Miejscowości z gwiazdkami)
    fn render_chips_input(ui: &mut egui::Ui, items: &mut Vec<PlaceChip>, input_buffer: &mut String, placeholder: &str) {
        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::from_hex("#131c29").unwrap())
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_hex("#364357").unwrap()))
            .corner_radius(8.0)
            .inner_margin(8)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);

                    let mut to_remove = None;

                    for (idx, chip) in items.iter().enumerate() {
                        egui::Frame::group(ui.style())
                            .fill(egui::Color32::from_hex("#202c3d").unwrap())
                            .corner_radius(12.0)
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let text = if chip.stars.is_empty() {
                                        chip.name.clone()
                                    } else {
                                        format!("{} {}", chip.name, chip.stars)
                                    };
                                    ui.label(egui::RichText::new(text).size(13.0));
                                    if ui.small_button("×").clicked() {
                                        to_remove = Some(idx);
                                    }
                                });
                            });
                    }

                    if let Some(idx) = to_remove {
                        items.remove(idx);
                    }

                    // Pole wpisywania nowej miejscowości
                    let response = ui.add(
                        egui::TextEdit::singleline(input_buffer)
                            .hint_text(placeholder)
                            .frame(egui::Frame::NONE)
                            .desired_width(220.0),
                    );

                    // Reagowanie na Enter lub przecinek
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let trimmed = input_buffer.trim().to_string();
                        if !trimmed.is_empty() {
                            // Rozbijamy opcjonalne gwiazdki na końcu
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() > 1 && parts.last().unwrap().starts_with('*') {
                                let stars = parts.last().unwrap().to_string();
                                let name = parts[..parts.len()-1].join(" ");
                                items.push(PlaceChip { name, stars });
                            } else {
                                items.push(PlaceChip { name: trimmed, stars: String::new() });
                            }
                            input_buffer.clear();
                            response.request_focus();
                        }
                    }
                });
            });
    }
}
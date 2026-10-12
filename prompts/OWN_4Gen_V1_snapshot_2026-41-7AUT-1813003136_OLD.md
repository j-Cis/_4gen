# "WERSJA Z PRZED RESTRUKTURYZACJI" CODE SNAPSHOT v:2026417AUT1813003136

## Metadata

```text
├─ Katalog roboczy (CWD): A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT_j-Cis/🆕4gen
├─ Lokalizacje (scan_at): ["./"]
└─ Wzorce (match_pattern): ["./{examples|tests|src}/**", "./{Cargo.toml}"]
📦 Zeskanowano fizycznie: 6171 plików, 1136 katalogów
```

## Structure

```plaintext
  ▣─┬ 🆕4gen                             [63.0 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                   _j-Cis/🆕4gen/                       
 1  ├──• Cargo.toml                     [   483 B] [2026-41-7 08:40:34] ./Cargo.toml                        
    └──┬ src                            [62.6 KiB] [2026-41-7 18:06:14] ./src/                              
 2     ├──• lib.rs                      [   372 B] [2026-41-7 17:28:48] ./src/lib.rs                        
 3     ├──• main.rs                     [16.8 KiB] [2026-41-7 17:41:22] ./src/main.rs                       
       ├──┬ data                        [ 7.7 KiB] [2026-41-7 11:51:02] ./src/data/                         
 4     │  ├──• sample_albums.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_albums.rs         
 5     │  ├──• sample_annotations.rs    [     2 B] [2026-41-7 13:08:02] ./src/data/sample_annotations.rs    
 6     │  ├──• sample_assets.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_assets.rs         
 7     │  ├──• sample_credentials.rs    [     2 B] [2026-41-7 13:08:02] ./src/data/sample_credentials.rs    
 8     │  ├──• sample_events.rs         [ 3.0 KiB] [2026-41-7 13:08:02] ./src/data/sample_events.rs         
 9     │  ├──• sample_individuals.rs    [ 4.6 KiB] [2026-41-7 13:08:02] ./src/data/sample_individuals.rs    
10     │  └──• sample_places.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_places.rs         
       ├──┬ element                     [19.9 KiB] [2026-41-7 10:55:08] ./src/element/                      
11     │  ├──• frame.rs                 [ 2.0 KiB] [2026-41-7 13:08:02] ./src/element/frame.rs              
12     │  ├──• side_menu.rs             [ 5.8 KiB] [2026-41-7 13:08:02] ./src/element/side_menu.rs          
13     │  └──• table.rs                 [12.1 KiB] [2026-41-7 13:29:29] ./src/element/table.rs              
       ├─── io                          [     0 B] [2026-41-7 18:03:37] ./src/io/                           
       ├─── model                       [     0 B] [2026-41-7 18:06:13] ./src/model/                        
       └──┬ template                    [17.9 KiB] [2026-41-7 17:25:24] ./src/template/                     
14        └──• association_form.rs      [17.9 KiB] [2026-41-7 17:30:21] ./src/template/association_form.rs  
```

## Source Code Content

### [1] `./Cargo.toml`

```toml
[package]
name = "_4gen"
version = "0.1.0-dev.1"
authors = ["Jan Roman Cisowski „j-Cis” <code@cisowscy.com>"]
edition = "2024"
rust-version = "1.99.0"
repository = "https://github.com/j-Cis/_4gen"
resolver = "3"

[lib]
name = "x"

[dependencies]
eframe = "0.36.2"
egui = "0.36.2"
egui_extras = { version = "0.36.2", features = ["all_loaders"] }
image = "0.25.10"
quick-xml = { version = "0.42.0", features = ["serialize"] }
serde = { version = "1.0.229", features = ["derive"] }

```

### [2] `./src/data/sample_albums.rs`

```rust


```

### [3] `./src/data/sample_annotations.rs`

```rust


```

### [4] `./src/data/sample_assets.rs`

```rust


```

### [5] `./src/data/sample_credentials.rs`

```rust


```

### [6] `./src/data/sample_events.rs`

```rust
#[derive(Clone, Debug)]
pub struct Event {
    pub id: usize,
    pub event_type: String,   // rodzaj (np. Ślub, Narodziny, Chrzest, Pogrzeb)
    pub date: Option<String>, // data
    pub place: Option<String>, // miejsce (miejscowość)
    pub address: Option<String>, // adres
    pub description: String,  // opis
    pub participants: Vec<String>, // uczestnicy (np. lista imion/ról)
    pub credentials_count: usize, // ilość poświadczeń (źródeł)
    pub notes_count: usize,   // ilość notatek
    pub participants_count: usize, // ilość uczestników
    pub last_modified: String, // ostatnio zmieniono
    pub is_confidential: bool, // poufny
    pub tags: Vec<String>,    // etykiety
}

pub fn get_sample_events() -> Vec<Event> {
    vec![
        Event {
            id: 1,
            event_type: "Ślub".into(),
            date: Some("1910-10-15".into()),
            place: Some("Kraków".into()),
            address: Some("Kościół Mariacki, Rynek Główny".into()),
            description:
                "Ślub Jana Kowalskiego i Marii Nowak. Wesele odbyło się w pobliskiej karczmie."
                    .into(),
            participants: vec![
                "Jan Kowalski (Pan Młody)".into(),
                "Maria Nowak (Panna Młoda)".into(),
                "Piotr Kowalski (Świadek)".into(),
            ],
            credentials_count: 2,
            notes_count: 1,
            participants_count: 3,
            last_modified: "2024-03-20 10:15".into(),
            is_confidential: false,
            tags: vec!["Sakrament".into(), "Kluczowe".into()],
        },
        Event {
            id: 2,
            event_type: "Narodziny".into(),
            date: Some("1885-04-12".into()),
            place: Some("Warszawa".into()),
            address: None,
            description: "Narodziny pierworodnego syna.".into(),
            participants: vec![
                "Jan Adam Kowalski (Dziecko)".into(),
                "Anna Nowak (Matka)".into(),
            ],
            credentials_count: 1,
            notes_count: 0,
            participants_count: 2,
            last_modified: "2024-02-15 08:30".into(),
            is_confidential: false,
            tags: vec!["Akt urodzenia".into()],
        },
        Event {
            id: 3,
            event_type: "Pogrzeb".into(),
            date: Some("1942-11-06".into()),
            place: Some("Warszawa".into()),
            address: Some("Cmentarz Powązkowski".into()),
            description: "Ceremonia pogrzebowa w gronie najbliższej rodziny.".into(),
            participants: vec![
                "Jan Adam Kowalski (Zmarły)".into(),
                "Maria Wiśniewska (Córka)".into(),
            ],
            credentials_count: 3,
            notes_count: 2,
            participants_count: 12, // Zadeklarowana większa liczba uczestników, mimo wymienienia tylko kluczowych wyżej
            last_modified: "2024-03-22 18:45".into(),
            is_confidential: true,
            tags: vec!["II Wojna Światowa".into()],
        },
    ]
}

```

### [7] `./src/data/sample_individuals.rs`

```rust
#[derive(Clone, Debug)]
pub struct Individual {
    pub id: usize,
    pub first_names: String,               // imiona
    pub last_names: String,                // nazwiska
    pub gender: String,                    // płeć (Mężczyzna/Kobieta/Inna)
    pub father_first_name: Option<String>, // imie ojca
    pub father_last_name: Option<String>,  // nazwisko ojca
    pub mother_first_name: Option<String>, // imie matki
    pub mother_last_name: Option<String>,  // nazwisko matki
    pub personal_note: String,             // przypis do osoby
    pub occupation_status: String,         // zawód/status
    pub birth_date: Option<String>,        // data urodzenia
    pub birth_place: Option<String>,       // miejscowość urodzenia
    pub birth_address: Option<String>,     // adres urodzenia
    pub death_date: Option<String>,        // data śmierci
    pub death_place: Option<String>,       // miejscowość śmierci
    pub death_address: Option<String>,     // adres śmierci
    pub parents_count: usize,              // liczba rodziców
    pub spouses_count: usize,              // liczba małżonków
    pub children_count: usize,             // liczba dzieci
    pub credentials_count: usize,          // ilość poświadczeń (źródeł)
    pub notes_count: usize,                // ilość notatek
    pub last_modified: String,             // ostatnio zmieniono
    pub is_confidential: bool,             // poufny
    pub tags: Vec<String>,                 // etykiety
}

pub fn get_sample_individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: 1,
            first_names: "Jan Adam".into(),
            last_names: "Kowalski".into(),
            gender: "Mężczyzna".into(),
            father_first_name: Some("Piotr".into()),
            father_last_name: Some("Kowalski".into()),
            mother_first_name: Some("Anna".into()),
            mother_last_name: Some("Nowak".into()),
            personal_note: "Zasłużony rzemieślnik".into(),
            occupation_status: "Kowal".into(),
            birth_date: Some("1885-04-12".into()),
            birth_place: Some("Warszawa".into()),
            birth_address: Some("ul. Stara 4".into()),
            death_date: Some("1942-11-03".into()),
            death_place: Some("Warszawa".into()),
            death_address: None,
            parents_count: 2,
            spouses_count: 1,
            children_count: 3,
            credentials_count: 4,
            notes_count: 2,
            last_modified: "2024-03-12 14:30".into(),
            is_confidential: false,
            tags: vec!["Zweryfikowane".into(), "Linia męska".into()],
        },
        Individual {
            id: 2,
            first_names: "Maria Anna".into(),
            last_names: "Wiśniewska (z d. Kowalska)".into(),
            gender: "Kobieta".into(),
            father_first_name: Some("Jan".into()),
            father_last_name: Some("Kowalski".into()),
            mother_first_name: Some("Zofia".into()),
            mother_last_name: Some("Kamińska".into()),
            personal_note: "Brak aktu urodzenia".into(),
            occupation_status: "Nauczycielka".into(),
            birth_date: Some("1910-08-22".into()),
            birth_place: Some("Kraków".into()),
            birth_address: None,
            death_date: None, // Osoba może żyć lub data jest nieznana
            death_place: None,
            death_address: None,
            parents_count: 2,
            spouses_count: 2,
            children_count: 1,
            credentials_count: 1,
            notes_count: 0,
            last_modified: "2024-03-15 09:15".into(),
            is_confidential: true,
            tags: vec!["Do sprawdzenia".into()],
        },
        Individual {
            id: 3,
            first_names: "Brak danych".into(),
            last_names: "Nieznany".into(),
            gender: "Nieznana".into(),
            father_first_name: None,
            father_last_name: None,
            mother_first_name: None,
            mother_last_name: None,
            personal_note: "Znaleziony jedynie wpis w księdze parafialnej".into(),
            occupation_status: "Rolnik".into(),
            birth_date: None,
            birth_place: Some("Gniezno".into()),
            birth_address: None,
            death_date: Some("1805-01-10".into()),
            death_place: Some("Gniezno".into()),
            death_address: None,
            parents_count: 0,
            spouses_count: 1,
            children_count: 0,
            credentials_count: 1,
            notes_count: 5,
            last_modified: "2024-02-28 18:00".into(),
            is_confidential: false,
            tags: vec!["Ślepy zaułek".into()],
        },
    ]
}

```

### [8] `./src/data/sample_places.rs`

```rust


```

### [9] `./src/element/frame.rs`

```rust
use eframe::egui;

#[derive(PartialEq)]
pub enum FrameType {
    Modal,
    Floating,
}

pub struct CustomFrame {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub frame_type: FrameType,
    pub is_open: bool,
}

impl CustomFrame {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        icon: impl Into<String>,
        frame_type: FrameType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: icon.into(),
            frame_type,
            is_open: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn show<F>(&mut self, ctx: &egui::Context, add_contents: F)
    where
        F: FnOnce(&mut egui::Ui, &mut bool),
    {
        if !self.is_open {
            return;
        }

        let full_title = format!("{} {}", self.icon, self.title);

        // 1. Zmienna dla krzyżyka w nagłówku Egui (zewnętrzna)
        let mut window_open = self.is_open;
        // 2. Zmienna dla Twojego kodu wewnątrz ramki (wewnętrzna)
        let mut content_open = true;

        let mut window = egui::Window::new(full_title)
            .id(egui::Id::new(&self.id))
            .open(&mut window_open); // Egui "pożycza" window_open

        match self.frame_type {
            FrameType::Modal => {
                window = window
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .resizable(false)
                    .movable(false);
            }
            FrameType::Floating => {
                window = window.collapsible(true).resizable(true).movable(true);
            }
        }

        window.show(ctx, |ui| {
            // Wstrzykujemy bezpieczną zmienną do wnętrza ramki
            add_contents(ui, &mut content_open);
        });

        // 3. Ramka pozostaje otwarta tylko jeśli ŻADEN z mechanizmów jej nie zamknął
        self.is_open = window_open && content_open;
    }
}

```

### [10] `./src/element/side_menu.rs`

```rust
use eframe::egui;

// --- UKRYTY MECHANIZM ŁADOWANIA IKON ---

/// Makro ułatwiające ładowanie ikon SVG w czasie kompilacji
#[macro_export]
macro_rules! i {
    ($path:literal) => {
        eframe::egui::Image::new(eframe::egui::include_image!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/icons/",
            $path,
            ".svg"
        )))
    };
}

#[derive(Clone)]
pub enum MenuIcon<'a> {
    Text(String),
    Image(egui::Image<'a>),
}

// Pozwala użyć "🪪".into()
impl<'a> From<&'a str> for MenuIcon<'a> {
    fn from(s: &'a str) -> Self {
        MenuIcon::Text(s.to_owned())
    }
}

// Pozwala użyć i!("twemoji/1faaa").into()
impl<'a> From<egui::Image<'a>> for MenuIcon<'a> {
    fn from(img: egui::Image<'a>) -> Self {
        MenuIcon::Image(img)
    }
}

#[derive(Clone)]
pub struct MenuItem<'a> {
    pub icon: MenuIcon<'a>,
    pub text: String,
}

// --- WŁAŚCIWY KOMPONENT MENU ---

pub struct SideMenu {
    pub bg_color: egui::Color32,
    pub icon_size: f32,
    pub expanded_width: f32,
    pub is_collapsed: bool,
    pub active_index: Option<usize>,
}

impl SideMenu {
    pub fn new(
        bg_color: egui::Color32,
        icon_size: f32,
        expanded_width: f32,
        is_collapsed: bool,
        active_index: Option<usize>,
    ) -> Self {
        Self {
            bg_color,
            icon_size,
            expanded_width,
            is_collapsed,
            active_index,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, items: &[MenuItem<'_>]) -> Option<usize> {
        let mut clicked_index = None;

        let current_width = if self.is_collapsed {
            self.icon_size
        } else {
            self.expanded_width
        };

        let (sidebar_rect, _) = ui.allocate_exact_size(
            egui::vec2(current_width, ui.available_height()),
            egui::Sense::hover(),
        );

        ui.painter().rect_filled(sidebar_rect, 0.0, self.bg_color);

        let mut current_y = sidebar_rect.min.y;

        let toggle_rect = egui::Rect::from_min_max(
            egui::pos2(sidebar_rect.min.x, current_y),
            egui::pos2(sidebar_rect.max.x, current_y + self.icon_size),
        );
        current_y += self.icon_size;

        let toggle_resp = ui.interact(toggle_rect, ui.id().with("toggle"), egui::Sense::click());
        if toggle_resp.clicked() {
            self.is_collapsed = !self.is_collapsed;
        }
        if toggle_resp.hovered() {
            ui.painter()
                .rect_filled(toggle_rect, 0.0, egui::Color32::from_white_alpha(20));
        }

        let icon_x_center = sidebar_rect.min.x + (self.icon_size / 2.0);
        ui.painter().text(
            egui::pos2(icon_x_center, toggle_rect.center().y),
            egui::Align2::CENTER_CENTER,
            "☰",
            egui::FontId::proportional(22.0),
            egui::Color32::WHITE,
        );

        current_y += 10.0;

        for (i, item) in items.iter().enumerate() {
            let is_active = self.active_index == Some(i);

            let item_rect = egui::Rect::from_min_max(
                egui::pos2(sidebar_rect.min.x, current_y),
                egui::pos2(sidebar_rect.max.x, current_y + self.icon_size),
            );
            current_y += self.icon_size;

            let item_resp = ui.interact(item_rect, ui.id().with(i), egui::Sense::click());
            if item_resp.clicked() {
                self.active_index = Some(i);
                clicked_index = Some(i);
            }

            if is_active {
                ui.painter()
                    .rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(30));

                let accent_rect = egui::Rect::from_min_max(
                    egui::pos2(item_rect.min.x, item_rect.min.y + 10.0),
                    egui::pos2(item_rect.min.x + 3.0, item_rect.max.y - 10.0),
                );
                ui.painter()
                    .rect_filled(accent_rect, 2.0, egui::Color32::from_rgb(0, 120, 215));
            } else if item_resp.hovered() {
                ui.painter()
                    .rect_filled(item_rect, 0.0, egui::Color32::from_white_alpha(15));
            }

            let color = if is_active || item_resp.hovered() {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(200)
            };

            // Rysowanie zależy od tego, jaki typ ikony przekazano
            match &item.icon {
                MenuIcon::Text(text) => {
                    ui.painter().text(
                        egui::pos2(icon_x_center, item_rect.center().y),
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::proportional(20.0),
                        color,
                    );
                }
                MenuIcon::Image(img) => {
                    let img_size = self.icon_size * 0.5;
                    let img_rect = egui::Rect::from_center_size(
                        egui::pos2(icon_x_center, item_rect.center().y),
                        egui::vec2(img_size, img_size),
                    );

                    let tint = if is_active || item_resp.hovered() {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::from_white_alpha(150)
                    };

                    img.clone().tint(tint).paint_at(ui, img_rect);
                }
            }

            if !self.is_collapsed {
                let text_x_start = sidebar_rect.min.x + self.icon_size + 4.0;
                ui.painter().text(
                    egui::pos2(text_x_start, item_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &item.text,
                    egui::FontId::proportional(16.0),
                    color,
                );
            }
        }

        clicked_index
    }
}

```

### [11] `./src/element/table.rs`

```rust
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

```

### [12] `./src/lib.rs`

```rust
/// .src/lib.rs

pub mod element {
    pub mod frame;
    pub mod side_menu;
    pub mod table;
}

pub mod data {
    pub mod sample_albums;
    pub mod sample_annotations;
    pub mod sample_assets;
    pub mod sample_credentials;
    pub mod sample_events;
    pub mod sample_individuals;
    pub mod sample_places;
}

pub mod template {
    pub mod association_form;
}

```

### [13] `./src/main.rs`

```rust
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
```

### [14] `./src/template/association_form.rs`

```rust
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
```

## End Structure Summary

```plaintext
  ▣─┬ 🆕4gen                             [63.0 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                   _j-Cis/🆕4gen/                       
 1  ├──• Cargo.toml                     [   483 B] [2026-41-7 08:40:34] ./Cargo.toml                        
    └──┬ src                            [62.6 KiB] [2026-41-7 18:06:14] ./src/                              
 2     ├──• lib.rs                      [   372 B] [2026-41-7 17:28:48] ./src/lib.rs                        
 3     ├──• main.rs                     [16.8 KiB] [2026-41-7 17:41:22] ./src/main.rs                       
       ├──┬ data                        [ 7.7 KiB] [2026-41-7 11:51:02] ./src/data/                         
 4     │  ├──• sample_albums.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_albums.rs         
 5     │  ├──• sample_annotations.rs    [     2 B] [2026-41-7 13:08:02] ./src/data/sample_annotations.rs    
 6     │  ├──• sample_assets.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_assets.rs         
 7     │  ├──• sample_credentials.rs    [     2 B] [2026-41-7 13:08:02] ./src/data/sample_credentials.rs    
 8     │  ├──• sample_events.rs         [ 3.0 KiB] [2026-41-7 13:08:02] ./src/data/sample_events.rs         
 9     │  ├──• sample_individuals.rs    [ 4.6 KiB] [2026-41-7 13:08:02] ./src/data/sample_individuals.rs    
10     │  └──• sample_places.rs         [     2 B] [2026-41-7 13:08:02] ./src/data/sample_places.rs         
       ├──┬ element                     [19.9 KiB] [2026-41-7 10:55:08] ./src/element/                      
11     │  ├──• frame.rs                 [ 2.0 KiB] [2026-41-7 13:08:02] ./src/element/frame.rs              
12     │  ├──• side_menu.rs             [ 5.8 KiB] [2026-41-7 13:08:02] ./src/element/side_menu.rs          
13     │  └──• table.rs                 [12.1 KiB] [2026-41-7 13:29:29] ./src/element/table.rs              
       ├─── io                          [     0 B] [2026-41-7 18:03:37] ./src/io/                           
       ├─── model                       [     0 B] [2026-41-7 18:06:13] ./src/model/                        
       └──┬ template                    [17.9 KiB] [2026-41-7 17:25:24] ./src/template/                     
14        └──• association_form.rs      [17.9 KiB] [2026-41-7 17:30:21] ./src/template/association_form.rs  
```

---
*Generated automatically by querypath-snapshot*

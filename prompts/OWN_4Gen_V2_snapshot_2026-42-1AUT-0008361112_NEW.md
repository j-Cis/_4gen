# "NOWE PODEJŚCIE" CODE SNAPSHOT v:2026-42-1AUT-0008361113

## Metadata

```text
├─ Katalog roboczy (CWD): A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT_j-Cis/_4gen
├─ Lokalizacje (scan_at): ["./"]
└─ Wzorce (match_pattern): ["./{examples|tests|src|scripts}/**", "./{Cargo|Trunk|Makefile}.toml"]
📦 Zeskanowano fizycznie: 3834 plików, 762 katalogów
```

## Structure

```plaintext
  ▣─┬ _4gen                             [26.0 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                   _j-Cis/_4gen/                       
 1  ├──• Cargo.toml                     [   981 B] [2026-41-7 23:36:40] ./Cargo.toml                        
 2  ├──• Makefile.toml                  [ 1.9 KiB] [2026-41-7 23:11:52] ./Makefile.toml                     
 3  ├──• Trunk.toml                     [   127 B] [2026-41-7 22:56:12] ./Trunk.toml                        
    ├──┬ scripts                        [   984 B] [2026-41-7 22:56:56] ./scripts/                          
 4  │  ├──• build-wasm.ps1              [   386 B] [2026-41-7 22:57:10] ./scripts/build-wasm.ps1            
 5  │  └──• run-wasm.ps1                [   598 B] [2026-41-7 22:56:45] ./scripts/run-wasm.ps1              
    └──┬ src                            [22.0 KiB] [2026-41-7 21:50:36] ./src/                              
 6     ├──• lib.rs                      [   190 B] [2026-42-1 00:03:47] ./src/lib.rs                        
 7     ├──• main.rs                     [   921 B] [2026-42-1 00:03:18] ./src/main.rs                       
       ├──┬ layout                      [11.0 KiB] [2026-41-7 21:17:40] ./src/layout/                       
 8     │  ├──• bar.rs                   [ 2.9 KiB] [2026-41-7 22:46:17] ./src/layout/bar.rs                 
 9     │  ├──• board.rs                 [ 1.9 KiB] [2026-41-7 21:10:59] ./src/layout/board.rs               
10     │  ├──• table.rs                 [ 2.3 KiB] [2026-41-7 21:08:34] ./src/layout/table.rs               
11     │  ├──• themes.rs                [     0 B] [2026-41-7 21:17:40] ./src/layout/themes.rs              
12     │  └──• window.rs                [ 3.8 KiB] [2026-41-7 22:49:13] ./src/layout/window.rs              
       ├──┬ pipe                        [ 7.9 KiB] [2026-41-7 21:50:46] ./src/pipe/                         
13     │  └──• egui_color.rs            [ 7.9 KiB] [2026-41-7 22:25:04] ./src/pipe/egui_color.rs            
       └──┬ theme                       [ 2.1 KiB] [2026-41-7 21:18:39] ./src/theme/                        
14        ├──• hue.rs                   [   704 B] [2026-41-7 22:27:12] ./src/theme/hue.rs                  
15        └──• ico.rs                   [ 1.4 KiB] [2026-41-7 22:39:18] ./src/theme/ico.rs                  
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
crate-type = ["cdylib", "rlib"]

[dependencies]
eframe = { version = "0.36.2", features = ["glow"] }
egui = "0.36.2"
egui_extras = { version = "0.36.2", features = ["svg", "image"] }
image = { version = "0.25.10", default-features = false, features = ["jpeg", "png", "webp"] }
palette = "0.7.7"
quick-xml = { version = "0.42.0", features = ["serialize"] }
serde = { version = "1.0.229", features = ["derive"] }

[profile.release]
opt-level = 2

[profile.release.package."*"]
opt-level = 2

[lints.rust]
unused_imports = "allow"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    "Element",
    "HtmlCanvasElement",
] }
```

### [2] `./Makefile.toml`

```toml
[config]
default_to_workspace = false
skip_core_tasks = true

[tasks.check]
description = "Sprawdza projekt"
command = "cargo"
args = ["check", "--all-targets"]

[tasks.test]
description = "Uruchamia testy"
command = "cargo"
args = ["test"]

[tasks.fmt]
description = "Sprawdza formatowanie kodu"
command = "cargo"
args = ["fmt", "--all", "--", "--check"]

[tasks.clippy]
description = "Uruchamia Clippy"
command = "cargo"
args = ["clippy", "--all-targets", "--", "-D", "warnings"]

[tasks.run]
description = "Uruchamia aplikację natywną"
command = "cargo"
args = ["run"]

[tasks.wasm-target]
description = "Dodaje target WASM"
command = "rustup"
args = ["target", "add", "wasm32-unknown-unknown"]

[tasks.wasm-serve]
description = "Buduje i uruchamia aplikację WASM w przeglądarce"
dependencies = ["wasm-target"]
command = "trunk"
args = ["serve", "--address", "127.0.0.1", "--port", "8080", "--open"]

[tasks.wasm-build]
description = "Buduje wersję WASM w trybie release"
dependencies = ["wasm-target"]
command = "trunk"
args = ["build", "--release"]

[tasks.wasm-check]
description = "Sprawdza kompilację dla WASM"
dependencies = ["wasm-target"]
command = "cargo"
args = ["check", "--target", "wasm32-unknown-unknown"]

[tasks.clean]
description = "Czyści artefakty kompilacji"
command = "cargo"
args = ["clean"]

[tasks.verify]
description = "Pełna kontrola projektu"
dependencies = ["fmt", "check", "test", "clippy"]

[tasks.snapshot]
description = "Generuje migawkę dokumentacyjną aktualnego stanu projektu"
command = "cargo"
args = [
"plot",
"-o", "./prompts",
"--version-pattern", "WYYY-WW-DSSS-hhmmssttqq",
"-c", "H W D P",
"--dir-weight", "matched",
"-Y", "WYYY-WW-D hh:mm:ss",
"--ignore-leading-dot",
"-d", "./",
"-p", "./{examples|tests|src|scripts}/**",
"-p", "./{Cargo|Trunk|Makefile}.toml",
"--file-name", "snapshot_{VERSION}.md",
]

```

### [3] `./Trunk.toml`

```toml
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[serve]
addresses = ["127.0.0.1"]
port = 8080
open = true
```

### [4] `./scripts/build-wasm.ps1`

```plaintext
$ErrorActionPreference = "Stop"

$ProjectRoot = if ($args.Count -gt 0) {
    (Resolve-Path $args[0]).Path
} else {
    (Get-Location).Path
}

Set-Location $ProjectRoot

if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    throw "Nie znaleziono Trunk. Zainstaluj: cargo install --locked trunk"
}

rustup target add wasm32-unknown-unknown
trunk build --release
```

### [5] `./scripts/run-wasm.ps1`

```plaintext
$ErrorActionPreference = "Stop"

# Run from the repository root, or pass the repository path as the first argument.
$ProjectRoot = if ($args.Count -gt 0) {
    (Resolve-Path $args[0]).Path
} else {
    (Get-Location).Path
}

Set-Location $ProjectRoot

if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    throw @"
Nie znaleziono polecenia 'trunk'.
Zainstaluj Trunk poleceniem:
    cargo install --locked trunk
Następnie ponownie uruchom:
    .\scripts\run-wasm.ps1
"@
}

rustup target add wasm32-unknown-unknown
trunk serve --address 127.0.0.1 --port 8080 --open
```

### [6] `./src/layout/bar.rs`

```rust

use crate::theme::ico;
use egui::{Align, Layout, Response, Sense, Ui, Vec2};

/// Element wstążki: przycisk albo separator.
#[derive(Clone)]
pub enum BarItem {
    Action {
        id: &'static str,
        icon: &'static str,
        tooltip: &'static str,
    },
    Separator,
}

/// Wstążka pionowa lub pozioma z ikonami SVG.
pub struct Bar {
    pub vertical: bool,
    pub items: Vec<BarItem>,
}

impl Bar {
    pub fn vertical(items: Vec<BarItem>) -> Self {
        Self {
            vertical: true,
            items,
        }
    }

    pub fn horizontal(items: Vec<BarItem>) -> Self {
        Self {
            vertical: false,
            items,
        }
    }

    pub fn show(&self, ui: &mut Ui) -> Vec<&'static str> {
        let mut activated = Vec::new();

        if self.vertical {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                for item in &self.items {
                    self.show_item(ui, item, &mut activated);
                }
            });
        } else {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                for item in &self.items {
                    self.show_item(ui, item, &mut activated);
                }
            });
        }

        activated
    }

    fn show_item(
        &self,
        ui: &mut Ui,
        item: &BarItem,
        activated: &mut Vec<&'static str>,
    ) {
        match item {
            BarItem::Separator => {
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
            }

            BarItem::Action { id, icon, tooltip } => {
                let uri = format!("file://assets/ico/{icon}");

                let (rect, response): (_, Response) =
                    ui.allocate_exact_size(
                        Vec2::splat(ico::ICON_BUTTON_SIZE),
                        Sense::click(),
                    );

                let visuals = ui.style().interact(&response);

                ui.painter().rect_filled(
                    rect,
                    ico::ICON_BUTTON_CORNER_RADIUS,
                    visuals.bg_fill,
                );

                ui.painter().rect_stroke(
                    rect,
                    ico::ICON_BUTTON_CORNER_RADIUS,
                    visuals.bg_stroke,
                    egui::StrokeKind::Inside,
                );

                let icon_rect = egui::Rect::from_center_size(
                    rect.center(),
                    ico::ICON_SIZE_VEC,
                );

                egui::Image::from_uri(uri)
                    .fit_to_exact_size(ico::ICON_SIZE_VEC)
                    .paint_at(ui, icon_rect);

                let response = response.on_hover_text(*tooltip);

                if response.clicked() {
                    activated.push(*id);
                }
            }
        }
    }
}
```

### [7] `./src/layout/board.rs`

```rust
use egui::{Align2, Area, Context, Id, Order, Pos2, Sense, Vec2};

/// Przykładowe okno wewnętrzne na planszy.
/// To okno egui, a nie osobne okno systemowe.
pub struct WorkspaceWindow {
    pub id: &'static str,
    pub title: String,
    pub position: Pos2,
    pub size: Vec2,
    pub open: bool,
}

pub struct Workspace {
    pub windows: Vec<WorkspaceWindow>,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            windows: vec![WorkspaceWindow {
                id: "welcome",
                title: "Witaj w _4gen".to_owned(),
                position: Pos2::new(110.0, 90.0),
                size: Vec2::new(360.0, 190.0),
                open: true,
            }],
        }
    }
}

impl Workspace {
    pub fn show(&mut self, ui: &mut egui::Ui) {
    for window in &mut self.windows {
        if !window.open {
            continue;
        }

            let area = Area::new(Id::new(window.id))
                .order(Order::Middle)
                .fixed_pos(window.position)
                .movable(true)
                .constrain(true)
                .default_size(window.size)
                .show(ui, |ui| {
                    egui::Frame::window(ui.style()).show(ui, |ui| {
                        ui.set_min_size(window.size);
                        ui.horizontal(|ui| {
                            ui.strong(&window.title);
                            if ui.button("×").on_hover_text("Zamknij okno").clicked() {
                                window.open = false;
                            }
                        });
                        ui.separator();
                        ui.label("To przykładowe, ruchome okno planszy.");
                        ui.label("Pozycję i zawartość dopracujemy w kolejnych krokach.");
                    });
                });

            window.position = area.response.rect.min;
        }
    }
}
```

### [8] `./src/layout/table.rs`

```rust
use crate::layout::bar::{Bar, BarItem};
use egui::{CentralPanel, Context, Grid, ScrollArea};

pub struct TablePanel {
    pub rows: Vec<(String, String, String)>,
}

impl Default for TablePanel {
    fn default() -> Self {
        Self {
            rows: vec![
                ("001".into(), "Przykładowa osoba".into(), "1881".into()),
                ("002".into(), "Przykładowe miejsce".into(), "1904".into()),
                ("003".into(), "Przykładowy dokument".into(), "1910".into()),
            ],
        }
    }
}

impl TablePanel {
    pub fn show(&mut self, ui: &mut egui::Ui) {
    egui::Window::new("Tabela — przykład")
        .default_size([560.0, 300.0])
        .show(ui, |ui| {
            let bar = Bar::horizontal(vec![
                    BarItem::Action { id: "add", icon: "+", tooltip: "Dodaj wiersz" },
                    BarItem::Action { id: "remove", icon: "−", tooltip: "Usuń ostatni wiersz" },
                    BarItem::Separator,
                    BarItem::Action { id: "refresh", icon: "↻", tooltip: "Odśwież tabelę" },
                ]);

                for action in bar.show(ui) {
                    match action {
                        "add" => self.rows.push((
                            format!("{:03}", self.rows.len() + 1),
                            "Nowy wiersz".into(),
                            String::new(),
                        )),
                        "remove" => { self.rows.pop(); }
                        _ => {}
                    }
                }

                ui.separator();
                ScrollArea::both().show(ui, |ui| {
                    Grid::new("example_table")
                        .striped(true)
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.strong("ID");
                            ui.strong("Nazwa");
                            ui.strong("Rok");
                            ui.end_row();

                            for (id, name, year) in &self.rows {
                                ui.label(id);
                                ui.label(name);
                                ui.label(year);
                                ui.end_row();
                            }
                        });
                });
            });
    }
}
```

### [9] `./src/layout/themes.rs`

```rust

```

### [10] `./src/layout/window.rs`

```rust

use crate::layout::{
    bar::{Bar, BarItem},
    board::Workspace,
    table::TablePanel,
};
use crate::theme::ico;
use eframe::egui::{self, Align, Context, Layout, Panel};

pub struct FrameApp {
    left_bar: Bar,
    right_bar: Bar,
    board: Workspace,
    table: TablePanel,
    show_table: bool,
    status: String,
}


impl FrameApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&_cc.egui_ctx);
        Self {
            left_bar: Bar::vertical(vec![
                BarItem::Action {
                    id: "files",
                    icon: ico::ICON_FILE,
                    tooltip: "Źródła i pliki",
                },
                BarItem::Action {
                    id: "search",
                    icon: ico::ICON_MAGNIFIER,
                    tooltip: "Szukaj",
                },
                BarItem::Separator,
                BarItem::Action {
                    id: "table",
                    icon: ico::ICON_LEDGER,
                    tooltip: "Pokaż/ukryj tabelę",
                },
            ]),
            right_bar: Bar::vertical(vec![
                BarItem::Action {
                    id: "layers",
                    icon: ico::ICON_PUZZLE,
                    tooltip: "Warstwy",
                },
                BarItem::Action {
                    id: "properties",
                    icon: ico::ICON_NOTEBOOK,
                    tooltip: "Właściwości",
                },
                BarItem::Separator,
                BarItem::Action {
                    id: "settings",
                    icon: ico::ICON_GEAR,
                    tooltip: "Ustawienia",
                },
            ]),
            board: Workspace::default(),
            table: TablePanel::default(),
            show_table: false,
            status: "Gotowy".into(),
        }
    }
}

impl eframe::App for FrameApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Panel::bottom("status_bar")
            .resizable(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status);

                    ui.with_layout(
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            ui.label("_4gen · wersja rozwojowa");
                        },
                    );
                });
            });

        Panel::left("left_ribbon")
            .default_size(ico::ICON_BUTTON_SIZE + 8.0)
            .resizable(false)
            .show(ui, |ui| {
                ui.add_space(6.0);

                for id in self.left_bar.show(ui) {
                    match id {
                        "files" => self.status = "Źródła i pliki".into(),
                        "search" => self.status = "Wyszukiwanie".into(),
                        "table" => self.show_table = !self.show_table,
                        _ => {}
                    }
                }
            });

        Panel::right("right_ribbon")
            .default_size(ico::ICON_BUTTON_SIZE + 8.0)
            .resizable(false)
            .show(ui, |ui| {
                ui.add_space(6.0);

                for id in self.right_bar.show(ui) {
                    self.status = match id {
                        "layers" => "Warstwy".into(),
                        "properties" => "Właściwości".into(),
                        "settings" => "Ustawienia".into(),
                        _ => self.status.clone(),
                    };
                }
            });

        egui::CentralPanel::no_frame().show(ui, |_ui| {});

        self.board.show(ui);

        if self.show_table {
            self.table.show(ui);
        }
    }
}
```

### [11] `./src/lib.rs`

```rust
pub mod layout {
  pub mod board;
  pub mod table;
  pub mod window;
  pub mod bar;
}

pub mod theme {
  pub mod hue;
  pub mod ico;
}

pub mod pipe {
    pub mod egui_color;
}
```

### [12] `./src/main.rs`

```rust
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    x::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;

    // Aplikacje Wasm działają asynchronicznie, podpinamy logikę wewnątrz main
    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .expect("Brak window")
            .document()
            .expect("Brak document")
            .get_element_by_id("the_canvas_id")
            .expect("Nie znaleziono elementu canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Element nie jest canvas");

        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(x::layout::window::FrameApp::new(cc)))),
            )
            .await
            .expect("Błąd startu eframe");
    });
}
```

### [13] `./src/pipe/egui_color.rs`

```rust
// src/pipe/egui_color.rs

use egui::Color32;

/// Rozszerzenie Color32 o przestrzenie barw OKLab i OKLCH.
pub trait Color32OklabExt {
    fn from_oklab(l: f32, a: f32, b: f32) -> Self;
    fn from_oklch(l: f32, c: f32, h: f32) -> Self;
}

impl Color32OklabExt for Color32 {
    fn from_oklab(l: f32, a: f32, b: f32) -> Self {
        oklab(l, a, b)
    }

    fn from_oklch(l: f32, c: f32, h: f32) -> Self {
        oklch(l, c, h)
    }
}

/// Helper: Konwersja Oklab do liniowego sRGB (zwraca r, g, b bez obcinania)
const fn oklab_to_linear_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let ll = l_ * l_ * l_;
    let mm = m_ * m_ * m_;
    let ss = s_ * s_ * s_;

    let r =  4.0767416621 * ll - 3.3077115913 * mm + 0.2309699292 * ss;
    let g = -1.2684380046 * ll + 2.6097574011 * mm - 0.3413193965 * ss;
    let b = -0.0041960863 * ll - 0.7034186147 * mm + 1.7076147010 * ss;

    (r, g, b)
}

/// Helper: Sprawdza czy składowe RGB mieszczą się w gamucie sRGB (0.0 - 1.0)
const fn is_in_srgb_gamut(r: f32, g: f32, b: f32) -> bool {
    r >= 0.0 && r <= 1.0 && g >= 0.0 && g <= 1.0 && b >= 0.0 && b <= 1.0
}

/// Konwersja OKLab do sRGB — dostępna w czasie kompilacji.
pub const fn oklab(l: f32, a: f32, b: f32) -> Color32 {
    if l <= 0.0 {
        return Color32::BLACK;
    }
    let (r, g, b) = oklab_to_linear_srgb(l, a, b);
    Color32::from_rgb(channel(r), channel(g), channel(b))
}

/// Konwersja OKLCH (L, C, h w stopniach) do sRGB — const.
pub const fn oklch(l: f32, c: f32, h: f32) -> Color32 {
    if l <= 0.0 {
        return Color32::BLACK;
    }
    let l_clamped = if l < 0.0 { 0.0 } else if l > 1.0 { 1.0 } else { l };
    if c <= 0.0 {
        return oklab(l, 0.0, 0.0);
    }
    let (sin_h, cos_h) = sin_cos_degrees(h);

    let (mut r, mut g, mut b) = oklab_to_linear_srgb(l_clamped, c * cos_h, c * sin_h);

    if !is_in_srgb_gamut(r, g, b) {
        let mut min_c = 0.0;
        let mut max_c = c;
        let mut iters = 0;

        // BAZA: Rozpoczynamy od gwarantowanego koloru w gamucie (szarość dla danego L)
        let (mut best_r, mut best_g, mut best_b) = oklab_to_linear_srgb(l_clamped, 0.0, 0.0);

        while iters < 16 {
            let mid_c = min_c + (max_c - min_c) / 2.0;
            let (tr, tg, tb) = oklab_to_linear_srgb(l_clamped, mid_c * cos_h, mid_c * sin_h);

            if is_in_srgb_gamut(tr, tg, tb) {
                best_r = tr; best_g = tg; best_b = tb;
                min_c = mid_c;
            } else {
                max_c = mid_c;
            }
            iters += 1;
        }
        
        r = best_r; g = best_g; b = best_b;
    }

    Color32::from_rgb(channel(r), channel(g), channel(b))
}

const fn channel(linear: f32) -> u8 {
    let x = if linear < 0.0 { 0.0 } else if linear > 1.0 { 1.0 } else { linear };

    let srgb = if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * pow_five_twelths(x) - 0.055
    };

    (srgb * 255.0 + 0.5) as u8
}

/// Oblicza x^(5/12) metodą Newtona: 
/// Rozwiązujemy równanie y^12 = x, a następnie zwracamy y^5.
const fn pow_five_twelths(x: f32) -> f32 {
    let mut y = 1.0_f32;
    let mut i = 0;

    // Znajdujemy y = x^(1/12)
    while i < 16 {
        let y2 = y * y;
        let y4 = y2 * y2;
        let y8 = y4 * y4;
        let y11 = y8 * y2 * y;
        let y12 = y11 * y;

        y = y - (y12 - x) / (12.0 * y11);
        i += 1;
    }

    // Podnosimy wynik do potęgi 5, otrzymując x^(5/12)
    let y2 = y * y;
    let y4 = y2 * y2;
    y4 * y 
}

/// Sinus i cosinus kąta w stopniach.
const fn sin_cos_degrees(degrees: f32) -> (f32, f32) {
    let mut angle = degrees;

    angle = angle % 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }
    if angle > 180.0 {
        angle -= 360.0;
    }

    let mut x = angle * 0.017453292519943295;
    let mut cos_sign = 1.0_f32;

    if x > 1.5707963267948966 {
        x = 3.141592653589793 - x;
        cos_sign = -1.0;
    } else if x < -1.5707963267948966 {
        x = -3.141592653589793 - x;
        cos_sign = -1.0;
    }

    let x2 = x * x;

    let sin = x * (1.0 + x2 * (-1.0 / 6.0 + x2 * (1.0 / 120.0 + x2 * (-1.0 / 5040.0 + x2 * (1.0 / 362880.0 + x2 * (-1.0 / 39916800.0 + x2 * (1.0 / 6227020800.0)))))));
    let cos = 1.0 + x2 * (-1.0 / 2.0 + x2 * (1.0 / 24.0 + x2 * (-1.0 / 720.0 + x2 * (1.0 / 40320.0 + x2 * (-1.0 / 3628800.0 + x2 * (1.0 / 479001600.0 + x2 * (-1.0 / 87178291200.0)))))));

    (sin, cos * cos_sign)
}




#[cfg(test)]
mod tests {
    use super::*;
    use palette::{Oklab, Oklch, Srgb, FromColor};

    // Helper: tolerancja dla różnic precyzji w obliczeniach const vs std
    const TOLERANCE: u8 = 1; 

    fn assert_color_eq(actual: Color32, expected: Srgb<u8>) {
        assert!(
            actual.r().abs_diff(expected.red) <= TOLERANCE &&
            actual.g().abs_diff(expected.green) <= TOLERANCE &&
            actual.b().abs_diff(expected.blue) <= TOLERANCE,
            "Kolory się różnią! Wynik const: {:?}, Oczekiwano (palette): {:?}",
            actual, expected
        );
    }

    #[test]
    fn test_oklab_matches_palette() {
        let cases = [
            (0.5, 0.1, -0.1),
            (0.8, -0.05, 0.05),
            (0.2, 0.0, 0.0), // Szarość
        ];

        for (l, a, b) in cases {
            let actual = oklab(l, a, b);
            let palette_lab = Oklab::new(l, a, b);
            let palette_srgb: Srgb<u8> = Srgb::from_color(palette_lab).into_format();

            assert_color_eq(actual, palette_srgb);
        }
    }

    #[test]
    fn test_oklch_matches_palette() {
        let cases = [
            (0.5, 0.1, 45.0),
            (0.8, 0.05, 180.0),
            (0.3, 0.15, 300.0),
        ];

        for (l, c, h) in cases {
            let actual = oklch(l, c, h);
            let palette_lch = Oklch::new(l, c, h);
            let palette_srgb: Srgb<u8> = Srgb::from_color(palette_lch).into_format();

            assert_color_eq(actual, palette_srgb);
        }
    }

    #[test]
    fn test_edge_cases() {
        // L = 0 (Czerń)
        assert_eq!(oklch(0.0, 0.5, 100.0), Color32::from_rgb(0, 0, 0));
        assert_eq!(oklab(0.0, 0.5, 0.5), Color32::from_rgb(0, 0, 0));

        // L = 1 (Biel) - zakłada zerową chromę po clippingu
        assert_eq!(oklch(1.0, 0.0, 0.0), Color32::from_rgb(255, 255, 255));
        
        // Ujemna chroma (powinna być traktowana jak 0)
        let actual_neg_c = oklch(0.5, -0.1, 45.0);
        let actual_zero_c = oklch(0.5, 0.0, 45.0);
        assert_eq!(actual_neg_c, actual_zero_c);

        // Kąty ujemne i > 360
        let base = oklch(0.5, 0.1, 45.0);
        assert_eq!(oklch(0.5, 0.1, 405.0), base, "Kąt > 360 zawiódł");
        assert_eq!(oklch(0.5, 0.1, -315.0), base, "Kąt ujemny zawiódł");
    }

    #[test]
    fn test_gamut_mapping_out_of_bounds() {
        // Bardzo nasycony niebieski z dużym L - niemożliwy w sRGB
        let l = 0.9;
        let c = 0.4;
        let h = 265.0; // Odcień błękitu/fioletu
        
        let result = oklch(l, c, h);
        
        // Zamiast zwracać "przepalony" kolor (np. 255, 255, 255 lub coś wpadającego w magenta),
        // Gamut mapping powinien obniżyć chromę, by zwrócić jasny błękit mieszczący się w gamucie.
        // Oczekujemy wartości, które nie zostały chamsko ucięte do 0/255.
        assert!(result.r() > 0 && result.r() < 255, "R powinno zostać sprowadzone do wnętrza gamutu");
        assert!(result.g() > 0 && result.g() < 255, "G powinno zostać sprowadzone do wnętrza gamutu");
        assert!(result.b() == 255, "B zazwyczaj ląduje na krawędzi gamutu");
    }
}
```

### [14] `./src/theme/hue.rs`

```rust
// src\theme\hue.rs

use egui::Color32;
use crate::pipe::egui_color::oklch;

pub const MAIN_TOOLBAR_BG: Color32 = oklch(0.281, 0.065, 247.0);
pub const MAIN_TOOLBAR_TOOLTIP: Color32 = oklch(0.175, 0.034, 247.0);
pub const MAIN_TOOLBAR_ACTIVE: Color32 = oklch(0.406, 0.102, 247.0);
pub const MAIN_TOOLBAR_HOVER: Color32 = oklch(0.405, 0.163, 247.0);

pub const APP_BG: Color32 = Color32::from_rgb(25, 26, 27);

pub const SUB_TOOLBAR_BG: Color32 = Color32::from_rgb(51, 51, 51);
pub const SUB_TOOLBAR_TOOLTIP: Color32 = Color32::from_rgb(36, 36, 36);
pub const SUB_TOOLBAR_ACTIVE: Color32 = Color32::from_rgb(76, 76, 76);
pub const SUB_TOOLBAR_HOVER: Color32 = Color32::from_rgb(76, 76, 76);
```

### [15] `./src/theme/ico.rs`

```rust
use egui::Vec2;

// Rozmiary ikon i przycisków.
pub const ICON_SIZE: f32 = 38.0;
pub const ICON_BUTTON_SIZE: f32 = 74.0;
pub const ICON_BUTTON_CORNER_RADIUS: f32 = 5.0;

// Ikony dostępne w assets/ico/.
pub const ICON_DIAMOND: &str = "twemoji_1f4a0.svg";
pub const ICON_CALENDAR: &str = "twemoji_1f4c6.svg";
pub const ICON_LEDGER: &str = "twemoji_1f4d2.svg";
pub const ICON_NOTEBOOK: &str = "twemoji_1f4d4.svg";
pub const ICON_BOOK: &str = "twemoji_1f4d6.svg";
pub const ICON_BOOKS: &str = "twemoji_1f4da.svg";
pub const ICON_SCROLL: &str = "twemoji_1f4dc.svg";
pub const ICON_MEMO: &str = "twemoji_1f4dd.svg";
pub const ICON_MAGNIFIER: &str = "twemoji_1f50e.svg";
pub const ICON_MAP: &str = "twemoji_1f5fa.svg";
pub const ICON_TOOLS: &str = "twemoji_1f6e0.svg";
pub const ICON_PUZZLE: &str = "twemoji_1f9e9.svg";
pub const ICON_COMPASS: &str = "twemoji_1f9ed.svg";
pub const ICON_TOOLBOX: &str = "twemoji_1f9f0.svg";
pub const ICON_FILE: &str = "twemoji_1f9fe.svg";
pub const ICON_LADDER: &str = "twemoji_1fa9b.svg";
pub const ICON_BRICK: &str = "twemoji_1faa2.svg";
pub const ICON_HATCH: &str = "twemoji_1faaa.svg";
pub const ICON_FEMALE: &str = "twemoji_2640.svg";
pub const ICON_MALE: &str = "twemoji_2642.svg";
pub const ICON_GEAR: &str = "twemoji_2699.svg";

pub const ICON_SIZE_VEC: Vec2 = Vec2::new(ICON_SIZE, ICON_SIZE);
pub const ICON_BUTTON_SIZE_VEC: Vec2 =
    Vec2::new(ICON_BUTTON_SIZE, ICON_BUTTON_SIZE);
```

## End Structure Summary

```plaintext
  ▣─┬ _4gen                             [26.0 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                   _j-Cis/_4gen/                       
 1  ├──• Cargo.toml                     [   981 B] [2026-41-7 23:36:40] ./Cargo.toml                        
 2  ├──• Makefile.toml                  [ 1.9 KiB] [2026-41-7 23:11:52] ./Makefile.toml                     
 3  ├──• Trunk.toml                     [   127 B] [2026-41-7 22:56:12] ./Trunk.toml                        
    ├──┬ scripts                        [   984 B] [2026-41-7 22:56:56] ./scripts/                          
 4  │  ├──• build-wasm.ps1              [   386 B] [2026-41-7 22:57:10] ./scripts/build-wasm.ps1            
 5  │  └──• run-wasm.ps1                [   598 B] [2026-41-7 22:56:45] ./scripts/run-wasm.ps1              
    └──┬ src                            [22.0 KiB] [2026-41-7 21:50:36] ./src/                              
 6     ├──• lib.rs                      [   190 B] [2026-42-1 00:03:47] ./src/lib.rs                        
 7     ├──• main.rs                     [   921 B] [2026-42-1 00:03:18] ./src/main.rs                       
       ├──┬ layout                      [11.0 KiB] [2026-41-7 21:17:40] ./src/layout/                       
 8     │  ├──• bar.rs                   [ 2.9 KiB] [2026-41-7 22:46:17] ./src/layout/bar.rs                 
 9     │  ├──• board.rs                 [ 1.9 KiB] [2026-41-7 21:10:59] ./src/layout/board.rs               
10     │  ├──• table.rs                 [ 2.3 KiB] [2026-41-7 21:08:34] ./src/layout/table.rs               
11     │  ├──• themes.rs                [     0 B] [2026-41-7 21:17:40] ./src/layout/themes.rs              
12     │  └──• window.rs                [ 3.8 KiB] [2026-41-7 22:49:13] ./src/layout/window.rs              
       ├──┬ pipe                        [ 7.9 KiB] [2026-41-7 21:50:46] ./src/pipe/                         
13     │  └──• egui_color.rs            [ 7.9 KiB] [2026-41-7 22:25:04] ./src/pipe/egui_color.rs            
       └──┬ theme                       [ 2.1 KiB] [2026-41-7 21:18:39] ./src/theme/                        
14        ├──• hue.rs                   [   704 B] [2026-41-7 22:27:12] ./src/theme/hue.rs                  
15        └──• ico.rs                   [ 1.4 KiB] [2026-41-7 22:39:18] ./src/theme/ico.rs                  
```

---
*Generated automatically by querypath-snapshot*

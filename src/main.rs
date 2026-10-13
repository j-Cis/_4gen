// src/main.rs

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

        let mut web_options = eframe::WebOptions::default();
        web_options.renderer = eframe::Renderer::Glow;

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(x::app::App4Gen::new(cc)))),
            )
            .await
            .expect("Błąd startu eframe");
    });
}
pub mod layout {
    pub mod frame;
    pub mod frame_bar;
    pub mod frame_setup;

    pub mod workspace;
    pub mod canvas;

    pub mod window;
    pub mod bar;

    pub mod table;
    pub mod table_types;
    pub mod table_toolbar;
    pub mod table_grid_engine;
    pub mod table_render_pinned;
    pub mod table_render_scroll;
    pub mod table_config_window;
}

pub mod theme {
    pub mod hue;
    pub mod ico;
    pub mod tab;
}

pub mod pipe {
    pub mod egui_color;
    pub mod icon_button;
}

pub mod widget {
    pub mod records_assets;
    pub mod records_individuals;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> eframe::Result {
    eframe::run_native(
        "_4gen",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(layout::frame::FrameApp::new(cc)))),
    )
}

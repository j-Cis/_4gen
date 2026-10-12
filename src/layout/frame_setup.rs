// src/layout/frame_setup.rs

use crate::layout::frame_bar::{FrameBar, FrameBarItem};
use crate::theme::ico;

pub fn build_left_bar<'a>() -> FrameBar<'a> {
    FrameBar::left(vec![
        FrameBarItem::Action {
            id: "files",
            icon: ico::file(),
            tooltip: "Źródła i pliki",
        },
        FrameBarItem::Action {
            id: "search",
            icon: ico::magnifier(),
            tooltip: "Szukaj",
        },
    ])
}

pub fn build_right_bar<'a>() -> FrameBar<'a> {
    FrameBar::right(vec![
        FrameBarItem::Action {
            id: "layers",
            icon: ico::puzzle(),
            tooltip: "Warstwy",
        },
        FrameBarItem::Action {
            id: "properties",
            icon: ico::notebook(),
            tooltip: "Właściwości",
        },
        FrameBarItem::Separator,
        FrameBarItem::Action {
            id: "settings",
            icon: ico::gear(),
            tooltip: "Ustawienia",
        },
    ])
}
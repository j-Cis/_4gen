use eframe::egui::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowClosePosition {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowMinimizePosition {
    Opposite,
    BesideLeft,
    BesideRight,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WindowHeaderConfig {
    pub close_pos: WindowClosePosition,
    pub minimize_pos: WindowMinimizePosition,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct HeaderAction {
    pub drag_delta: Vec2,
    pub close_clicked: bool,
    pub toggle_minimize: bool,
}
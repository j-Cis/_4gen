use eframe::egui::{self, Align2, Color32, Frame, Id, Pos2, Sense, Vec2};
use crate::layout::frame::{draw_custom_header, ContainerWindow, ContainerWindowMode};
use crate::layout::setup::build_window_header;
use crate::theme::hue;

pub fn render_container_window<F>(window: &mut ContainerWindow, ui: &mut egui::Ui, add_contents: F)
where
    F: FnOnce(&mut egui::Ui),
{
    let bounds = ui.max_rect();
    let screen_rect = ui
        .ctx()
        .input(|i| i.raw.screen_rect.unwrap_or(egui::Rect::EVERYTHING));

    let header_config = build_window_header();
    let title = window.formatted_title();
    let is_movable = window.is_movable;
    let is_minimized = window.is_minimized;

    let mut close_requested = false;
    let mut toggle_minimize = false;

    match window.mode {
        ContainerWindowMode::Floating => {
            let pos_id = Id::unique(&window.id).with("float_pos");
            let mut current_pos = ui.data(|d| d.get_temp::<Pos2>(pos_id)).unwrap_or_else(|| {
                window.default_pos.unwrap_or(bounds.center() - Vec2::new(180.0, 120.0))
            });

            let window_id = Id::unique(&window.id);
            let mut drag_delta = Vec2::ZERO;

            let min_size = if is_minimized {
                Vec2::new(window.min_size.x, 0.0)
            } else {
                window.min_size
            };

            let mut win = egui::Window::new(&title)
                .id(window_id)
                .constrain_to(bounds)
                .title_bar(false)
                .resizable(window.is_resizable && !is_minimized)
                .movable(false)
                .frame(Frame::window(ui.style()).fill(hue::WINDOW_CONTENT_BG).inner_margin(0))
                .min_size(min_size);

            if let Some(pos) = window.default_pos {
                win = win.default_pos(pos);
            }
            if let Some(sz) = window.default_size {
                win = win.default_size(sz);
            }

            win.current_pos(current_pos).show(ui.ctx(), |window_ui| {
                let action = draw_custom_header(
                    window_ui,
                    window_id,
                    &title,
                    is_movable,
                    &header_config,
                );
                drag_delta = action.drag_delta;
                if action.close_clicked {
                    close_requested = true;
                }
                if action.toggle_minimize {
                    toggle_minimize = true;
                }

                if !is_minimized {
                    window_ui.add_space(4.0);
                    Frame::new().inner_margin(8).show(window_ui, |content_ui| {
                        add_contents(content_ui);
                    });
                }
            });

            if close_requested {
                window.is_open = false;
            }
            if toggle_minimize {
                window.is_minimized = !window.is_minimized;
            }
            current_pos += drag_delta;
            ui.data_mut(|d| d.insert_temp(pos_id, current_pos));
        }

        ContainerWindowMode::FloatingModal => {
            egui::Area::new(Id::unique(&window.id).with("floating_modal_shield"))
                .order(egui::Order::Foreground)
                .fixed_pos(screen_rect.min)
                .show(ui.ctx(), |shield_ui| {
                    shield_ui.allocate_exact_size(screen_rect.size(), Sense::click_and_drag());
                    shield_ui.painter().rect_filled(screen_rect, 0.0, Color32::from_black_alpha(60));
                });

            let pos_id = Id::unique(&window.id).with("float_modal_pos");
            let mut current_pos = ui.data(|d| d.get_temp::<Pos2>(pos_id)).unwrap_or_else(|| {
                window.default_pos.unwrap_or(bounds.center() - Vec2::new(180.0, 120.0))
            });

            let window_id = Id::unique(&window.id).with("floating_modal_win");
            let mut drag_delta = Vec2::ZERO;

            let min_size = if is_minimized {
                Vec2::new(window.min_size.x, 0.0)
            } else {
                window.min_size
            };

            let mut win = egui::Window::new(&title)
                .id(window_id)
                .order(egui::Order::Tooltip)
                .constrain_to(bounds)
                .title_bar(false)
                .resizable(window.is_resizable && !is_minimized)
                .movable(false)
                .frame(Frame::window(ui.style()).fill(hue::WINDOW_CONTENT_BG).inner_margin(0))
                .min_size(min_size);

            win = win.current_pos(current_pos);
            if window.default_pos.is_none() && ui.data(|d| d.get_temp::<Pos2>(pos_id)).is_none() {
                win = win.pivot(Align2::CENTER_CENTER).default_pos(bounds.center());
            }

            win.show(ui.ctx(), |window_ui| {
                let action = draw_custom_header(
                    window_ui,
                    window_id,
                    &title,
                    is_movable,
                    &header_config,
                );
                drag_delta = action.drag_delta;
                if action.close_clicked {
                    close_requested = true;
                }
                if action.toggle_minimize {
                    toggle_minimize = true;
                }

                if !is_minimized {
                    window_ui.add_space(4.0);
                    Frame::new().inner_margin(8).show(window_ui, |content_ui| {
                        add_contents(content_ui);
                    });
                }
            });

            if close_requested {
                window.is_open = false;
            }
            if toggle_minimize {
                window.is_minimized = !window.is_minimized;
            }
            current_pos += drag_delta;
            ui.data_mut(|d| d.insert_temp(pos_id, current_pos));
        }

        ContainerWindowMode::ScreenModal => {
            egui::Area::new(Id::unique(&window.id).with("screen_backdrop"))
                .order(egui::Order::Foreground)
                .fixed_pos(screen_rect.min)
                .show(ui.ctx(), |backdrop_ui| {
                    backdrop_ui.allocate_exact_size(screen_rect.size(), Sense::click_and_drag());
                    backdrop_ui.painter().rect_filled(screen_rect, 0.0, Color32::from_black_alpha(180));
                });

            let window_id = Id::unique(&window.id).with("screen_modal_body");

            egui::Area::new(window_id)
                .order(egui::Order::Tooltip)
                .pivot(Align2::CENTER_CENTER)
                .fixed_pos(screen_rect.center())
                .show(ui.ctx(), |modal_ui| {
                    Frame::window(modal_ui.style())
                        .fill(hue::WINDOW_CONTENT_BG)
                        .inner_margin(0)
                        .show(modal_ui, |window_ui| {
                            let action = draw_custom_header(
                                window_ui,
                                window_id,
                                &title,
                                false,
                                &header_config,
                            );
                            if action.close_clicked {
                                close_requested = true;
                            }
                            if action.toggle_minimize {
                                toggle_minimize = true;
                            }

                            if !is_minimized {
                                window_ui.add_space(4.0);
                                Frame::new().inner_margin(8).show(window_ui, |content_ui| {
                                    add_contents(content_ui);
                                });
                            }
                        });
                });

            if close_requested {
                window.is_open = false;
            }
            if toggle_minimize {
                window.is_minimized = !window.is_minimized;
            }
        }
    }
}
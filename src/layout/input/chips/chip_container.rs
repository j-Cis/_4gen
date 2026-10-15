// src/layout/input/chips/chip_container.rs

use eframe::egui::{self, Color32, CursorIcon, Frame, Id, Key, Margin, Order, Rect, Stroke, Vec2};
use crate::layout::input::chips::{
    render_autocomplete_input, render_chip, ChipAction, ChipContainerMode, ChipItem, ChipLimits,
    ChipOverflow,
};

pub struct ChipsContainer<'a> {
    pub id: Id,
    pub items: &'a mut Vec<ChipItem>,
    pub input_buffer: &'a mut String,
    pub autocomplete_pool: &'a [String],
    pub mode: ChipContainerMode,
    pub overflow: ChipOverflow,
    pub limits: ChipLimits,
    pub selected_chip: &'a mut Option<String>,
}

impl<'a> ChipsContainer<'a> {
    pub fn new(
        id: impl Into<Id>,
        items: &'a mut Vec<ChipItem>,
        input_buffer: &'a mut String,
        autocomplete_pool: &'a [String],
        selected_chip: &'a mut Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            items,
            input_buffer,
            autocomplete_pool,
            mode: ChipContainerMode::SingleLine,
            overflow: ChipOverflow::Wrap,
            limits: ChipLimits::default(),
            selected_chip,
        }
    }

    pub fn mode(mut self, mode: ChipContainerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn overflow(mut self, overflow: ChipOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn limits(mut self, limits: ChipLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn show(mut self, ui: &mut egui::Ui) -> Vec<ChipAction> {
        let mut actions = Vec::new();

        let container_frame = Frame::new()
            .fill(Color32::from_rgb(28, 30, 34))
            .stroke(Stroke::new(1.0, Color32::from_gray(60)))
            .corner_radius(6.0)
            .inner_margin(Margin::same(4));

        let edit_chip_id_key = self.id.with("active_param_edit_id");
        let edit_anchor_rect_key = self.id.with("active_param_anchor_rect");
        let edit_buffer_key = self.id.with("active_param_buffer");

        let mut editing_chip_id = ui.data(|d| d.get_temp::<String>(edit_chip_id_key));
        let mut editing_anchor_rect = ui.data(|d| d.get_temp::<Rect>(edit_anchor_rect_key));
        let mut just_opened = false;

        container_frame.show(ui, |ui| {
            ui.style_mut().interaction.selectable_labels = false;
            ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);

            match (self.mode, self.overflow) {
                (ChipContainerMode::MultiLine, ChipOverflow::Wrap) => {
                    ui.horizontal_wrapped(|ui| {
                        self.render_chips_flow(ui, 0, self.items.len(), &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                        self.render_input_slot(ui, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                    });
                }

                (ChipContainerMode::SingleLine, ChipOverflow::Scroll) => {
                    egui::ScrollArea::horizontal()
                        .id_salt(self.id.with("chips_hscroll"))
                        .auto_shrink([false, true])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                self.render_chips_flow(ui, 0, self.items.len(), &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                                self.render_input_slot(ui, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                            });
                        });
                }

                (ChipContainerMode::MultiLine, ChipOverflow::Scroll) => {
                    egui::ScrollArea::vertical()
                        .id_salt(self.id.with("chips_vscroll"))
                        .auto_shrink([false, true])
                        .max_height(80.0)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                self.render_chips_flow(ui, 0, self.items.len(), &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                                self.render_input_slot(ui, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                            });
                        });
                }

                (_, ChipOverflow::Popup { max_visible }) => {
                    let total = self.items.len();
                    let visible_count = max_visible.min(total);
                    let hidden_count = total.saturating_sub(visible_count);

                    ui.horizontal(|ui| {
                        self.render_chips_flow(ui, 0, visible_count, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);

                        if hidden_count > 0 {
                            let popup_id = self.id.with("hidden_chips_popup");
                            let is_open_id = self.id.with("hidden_chips_is_open");
                            let mut is_open = ui.data(|d| d.get_temp::<bool>(is_open_id)).unwrap_or(false);

                            let badge_text = format!("+{} ▾", hidden_count);
                            let btn_resp = ui.selectable_label(is_open, egui::RichText::new(badge_text).strong());
                            if btn_resp.hovered() {
                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                            }

                            if btn_resp.clicked() {
                                is_open = !is_open;
                                ui.data_mut(|d| d.insert_temp(is_open_id, is_open));
                            }

                            if is_open {
                                let popup_pos = btn_resp.rect.left_bottom() + Vec2::new(0.0, 4.0);
                                let popup_resp = egui::Area::new(popup_id)
                                    .fixed_pos(popup_pos)
                                    .order(Order::Tooltip)
                                    .show(ui.ctx(), |pop_ui| {
                                        Frame::popup(pop_ui.style())
                                            .fill(Color32::from_rgb(32, 34, 38))
                                            .inner_margin(Margin::same(6))
                                            .show(pop_ui, |ui| {
                                                ui.vertical(|ui| {
                                                    self.render_chips_flow(ui, visible_count, total, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                                                });
                                            });
                                    });

                                if ui.input(|i| i.pointer.any_pressed()) {
                                    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                        if !popup_resp.response.rect.contains(pos) && !btn_resp.rect.contains(pos) {
                                            ui.data_mut(|d| d.insert_temp(is_open_id, false));
                                        }
                                    }
                                }
                            }
                        }

                        self.render_input_slot(ui, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                    });
                }

                _ => {
                    ui.horizontal(|ui| {
                        self.render_chips_flow(ui, 0, self.items.len(), &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                        self.render_input_slot(ui, &mut actions, &mut editing_chip_id, &mut editing_anchor_rect, &mut just_opened);
                    });
                }
            }
        });

        // 3. Obsługa okienka edycji parametru zakotwiczonego dokładnie pod chipem
        if let Some(target_id) = &editing_chip_id {
            let mut close_editor = false;
            let mut new_val_to_apply = None;

            if let Some(chip) = self.items.iter().find(|c| c.id() == target_id) {
                let mut param_buf = ui.data(|d| d.get_temp::<String>(edit_buffer_key)).unwrap_or_else(|| {
                    chip.param_value().unwrap_or("").to_string()
                });

                let anchor_rect = editing_anchor_rect.unwrap_or_else(|| ui.cursor());
                let popup_pos = anchor_rect.left_bottom() + Vec2::new(0.0, 4.0);
                let edit_area_id = self.id.with("param_edit_popup");

                let area_resp = egui::Area::new(edit_area_id)
                    .order(Order::Tooltip)
                    .fixed_pos(popup_pos)
                    .show(ui.ctx(), |pop_ui| {
                        Frame::popup(pop_ui.style())
                            .fill(Color32::from_rgb(36, 38, 44))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(0, 150, 255)))
                            .inner_margin(Margin::same(8))
                            .show(pop_ui, |p_ui| {
                                p_ui.horizontal(|h_ui| {
                                    h_ui.label(
                                        egui::RichText::new(format!("{}:", chip.key_or_label()))
                                            .strong()
                                            .color(Color32::from_rgb(170, 190, 220)),
                                    );
                                    let edit_resp = h_ui.add(
                                        egui::TextEdit::singleline(&mut param_buf)
                                            .hint_text("Wpisz parametr...")
                                            .desired_width(130.0),
                                    );
                                    if just_opened {
                                        edit_resp.request_focus();
                                    }

                                    let enter_pressed = h_ui.input(|i| i.key_pressed(Key::Enter));
                                    if h_ui.button("OK").clicked() || enter_pressed {
                                        new_val_to_apply = Some(param_buf.trim().to_string());
                                        close_editor = true;
                                    }
                                });
                            });
                    });

                // Zamknięcie na Escape
                if ui.input(|i| i.key_pressed(Key::Escape)) {
                    close_editor = true;
                }

                // Zamknięcie po kliknięciu poza dymkiem (ignorujemy klatkę otwarcia!)
                if !just_opened && ui.input(|i| i.pointer.any_pressed()) {
                    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                        if !area_resp.response.rect.contains(pos) && !anchor_rect.contains(pos) {
                            close_editor = true;
                        }
                    }
                }

                ui.data_mut(|d| d.insert_temp(edit_buffer_key, param_buf));
            } else {
                close_editor = true;
            }

            if let Some(val) = new_val_to_apply {
                if let Some(chip) = self.items.iter_mut().find(|c| c.id() == target_id) {
                    chip.set_param_value(val);
                }
            }

            if close_editor {
                editing_chip_id = None;
                editing_anchor_rect = None;
                ui.data_mut(|d| {
                    d.remove::<String>(edit_buffer_key);
                    d.remove::<Rect>(edit_anchor_rect_key);
                });
            }
        }

        ui.data_mut(|d| {
            d.insert_temp(edit_chip_id_key, editing_chip_id);
            if let Some(rect) = editing_anchor_rect {
                d.insert_temp(edit_anchor_rect_key, rect);
            }
        });

        for action in &actions {
            if let ChipAction::Removed(id) = action {
                self.items.retain(|c| c.id() != id);
            }
        }

        actions
    }

    fn render_chips_flow(
        &mut self,
        ui: &mut egui::Ui,
        start_idx: usize,
        end_idx: usize,
        actions: &mut Vec<ChipAction>,
        editing_chip_id: &mut Option<String>,
        editing_anchor_rect: &mut Option<Rect>,
        just_opened: &mut bool,
    ) {
        for idx in start_idx..end_idx {
            if idx >= self.items.len() {
                break;
            }
            let item = &self.items[idx];
            let is_selected = self.selected_chip.as_deref() == Some(item.id());
            let out = render_chip(ui, item, is_selected);

            if out.remove_clicked {
                actions.push(ChipAction::Removed(item.id().to_string()));
            } else if out.param_clicked {
                *editing_chip_id = Some(item.id().to_string());
                *editing_anchor_rect = Some(out.rect);
                *just_opened = true;
                let initial_val = item.param_value().unwrap_or("").to_string();
                ui.data_mut(|d| d.insert_temp(self.id.with("active_param_buffer"), initial_val));
                actions.push(ChipAction::ParamEditRequested(item.id().to_string()));
            } else if out.clicked {
                *self.selected_chip = Some(item.id().to_string());
                actions.push(ChipAction::Clicked(item.id().to_string()));
            }
        }
    }

    fn render_input_slot(
        &mut self,
        ui: &mut egui::Ui,
        actions: &mut Vec<ChipAction>,
        editing_chip_id: &mut Option<String>,
        editing_anchor_rect: &mut Option<Rect>,
        just_opened: &mut bool,
    ) {
        let res = render_autocomplete_input(
            ui,
            self.id.with("autocomplete"),
            self.input_buffer,
            self.autocomplete_pool,
            self.limits.allow_custom,
        );

        if let Some(text) = res.committed_text {
            let new_chip = if let Some((key, val)) = text.split_once(':') {
                let k = key.trim();
                let v = val.trim();
                let id = format!("chip_{}", ui.input(|i| i.time));

                if v.is_empty() {
                    *editing_chip_id = Some(id.clone());
                    *editing_anchor_rect = Some(ui.cursor());
                    *just_opened = true;
                    ChipItem::Parametric {
                        id,
                        key: k.to_string(),
                        display_value: String::new(),
                    }
                } else {
                    ChipItem::Parametric {
                        id,
                        key: k.to_string(),
                        display_value: v.to_string(),
                    }
                }
            } else {
                ChipItem::Simple {
                    id: format!("chip_{}", ui.input(|i| i.time)),
                    label: text.clone(),
                }
            };

            if self.limits.can_add(new_chip.key_or_label(), self.items) {
                actions.push(ChipAction::Added(new_chip.clone()));
                self.items.push(new_chip);
            }
        }
    }
}
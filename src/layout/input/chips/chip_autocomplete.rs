// src/layout/input/chips/chip_autocomplete.rs

use eframe::egui::{self, Color32, CursorIcon, Frame, Id, Key, Margin, Order, Vec2};

pub struct AutocompleteResult {
    pub committed_text: Option<String>,
}

pub fn render_autocomplete_input(
    ui: &mut egui::Ui,
    id: Id,
    buffer: &mut String,
    pool: &[String],
    allow_custom: bool,
) -> AutocompleteResult {
    let mut committed = None;

    let matches: Vec<&String> = if buffer.trim().is_empty() {
        Vec::new()
    } else {
        let needle = buffer.to_lowercase();
        pool.iter()
            .filter(|cand| cand.to_lowercase().contains(&needle))
            .take(6)
            .collect()
    };

    let input_id = id.with("text_edit");
    let input_resp = ui.add(
        egui::TextEdit::singleline(buffer)
            .id(input_id)
            .desired_width(100.0)
            .frame(Frame::NONE)
            .hint_text("Wpisz..."),
    );

    let popup_id = id.with("autocomplete_popup");
    let is_open_id = id.with("autocomplete_is_open");

    // Dymek pozostaje otwarty, gdy pole ma fokus, LUB gdy był otwarty w poprzedniej klatce (umożliwia kliknięcie)
    let was_open = ui.data(|d| d.get_temp::<bool>(is_open_id)).unwrap_or(false);
    let mut should_open = (input_resp.has_focus() && !matches.is_empty()) || (was_open && !matches.is_empty());

    // 1. Obsługa zatwierdzenia klawiszem Enter
    if (input_resp.has_focus() || input_resp.lost_focus()) && ui.input(|i| i.key_pressed(Key::Enter)) {
        let trimmed = buffer.trim();
        if !trimmed.is_empty() {
            if let Some(first_match) = matches.first() {
                committed = Some((*first_match).clone());
                buffer.clear();
                should_open = false;
            } else if allow_custom {
                committed = Some(trimmed.to_string());
                buffer.clear();
                should_open = false;
            }
        }
    }

    // 2. Renderowanie dymka podpowiedzi
    if should_open {
        let popup_pos = input_resp.rect.left_bottom() + Vec2::new(0.0, 4.0);
        let mut clicked_item = None;

        let area_resp = egui::Area::new(popup_id)
            .fixed_pos(popup_pos)
            .order(Order::Tooltip)
            .show(ui.ctx(), |pop_ui| {
                Frame::popup(pop_ui.style())
                    .fill(Color32::from_rgb(32, 34, 38))
                    .inner_margin(Margin::same(4))
                    .show(pop_ui, |ui| {
                        ui.set_min_width(120.0);
                        for candidate in &matches {
                            let item_resp = ui.selectable_label(false, *candidate);
                            if item_resp.hovered() {
                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                            }
                            if item_resp.clicked() {
                                clicked_item = Some((*candidate).clone());
                            }
                        }
                    });
            });

        // 3. Obsługa kliknięcia w pozycję z dymka
        if let Some(chosen) = clicked_item {
            committed = Some(chosen);
            buffer.clear();
            should_open = false;
            ui.memory_mut(|m| m.request_focus(input_id));
        } else if ui.input(|i| i.pointer.any_pressed()) {
            // Kliknięcie w tło zamyka dymek
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if !area_resp.response.rect.contains(pos) && !input_resp.rect.contains(pos) {
                    should_open = false;
                }
            }
        }
    }

    ui.data_mut(|d| d.insert_temp(is_open_id, should_open));

    AutocompleteResult {
        committed_text: committed,
    }
}
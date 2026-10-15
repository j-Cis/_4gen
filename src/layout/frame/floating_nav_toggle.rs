use eframe::egui::{self, Color32, CursorIcon, Id, Pos2, Rect, Sense, Stroke, Vec2};

pub fn show_floating_nav_toggle(ctx: &egui::Context, is_visible: &mut bool) {
    let pos_id = Id::unique("floating_nav_toggle_pos");
    let screen_rect = ctx.input(|i| i.raw.screen_rect.unwrap_or(Rect::EVERYTHING));

    // Domyślna pozycja: lewy dolny róg, tuż nad dolnym paskiem stanu
    let default_pos = Pos2::new(8.0, (screen_rect.max.y - 70.0).max(screen_rect.min.y));
    let mut current_pos = ctx.data(|d| d.get_temp::<Pos2>(pos_id)).unwrap_or(default_pos);

    let button_size = Vec2::splat(38.0);
    let radius = button_size.x / 2.0;

    egui::Area::new(Id::unique("floating_nav_toggle_area"))
        .order(egui::Order::Foreground)
        .fixed_pos(current_pos)
        .show(ctx, |ui| {
            let (rect, resp) = ui.allocate_exact_size(button_size, Sense::click_and_drag());

            // 1. Obsługa przeciągania
            if resp.dragged() {
                current_pos += resp.drag_delta();
                current_pos.x = current_pos.x.clamp(screen_rect.min.x, screen_rect.max.x - button_size.x);
                current_pos.y = current_pos.y.clamp(screen_rect.min.y, screen_rect.max.y - button_size.y);
                ctx.data_mut(|d| d.insert_temp(pos_id, current_pos));
            }

            // 2. Obsługa kliknięcia
            if resp.clicked() {
                *is_visible = !*is_visible;
            }

            // 3. Ikona kursora
            if resp.dragged() {
                ctx.set_cursor_icon(CursorIcon::Grabbing);
            } else if resp.hovered() {
                ctx.set_cursor_icon(CursorIcon::PointingHand);
            }

            // 4. Stylistyka i przezroczystość
            let bg_color = if resp.is_pointer_button_down_on() {
                Color32::from_rgba_unmultiplied(22, 24, 28, 245)
            } else if resp.hovered() {
                Color32::from_rgba_unmultiplied(42, 48, 60, 230)
            } else {
                Color32::from_rgba_unmultiplied(32, 34, 38, 175)
            };

            let border_stroke = if resp.hovered() {
                Stroke::new(1.5, Color32::from_rgba_unmultiplied(255, 255, 255, 180))
            } else {
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60))
            };

            let center = rect.center();

            // Lekki cień za guzikiem
            ui.painter().circle_filled(center + Vec2::new(0.0, 1.5), radius, Color32::from_black_alpha(70));
            // Tło guzika i obwódka
            ui.painter().circle(center, radius, bg_color, border_stroke);

            // 5. Rysowanie wektorowej ikony układu pasków
            let icon_col = if resp.hovered() {
                Color32::WHITE
            } else {
                Color32::from_rgb(200, 205, 215)
            };

            let box_rect = Rect::from_center_size(center, Vec2::new(18.0, 14.0));
            ui.painter().rect_stroke(box_rect, 2.0, Stroke::new(1.2, icon_col), egui::StrokeKind::Inside);

            let left_bar_x = box_rect.left() + 4.5;
            let right_bar_x = box_rect.right() - 4.5;

            ui.painter().line_segment(
                [egui::pos2(left_bar_x, box_rect.top()), egui::pos2(left_bar_x, box_rect.bottom())],
                Stroke::new(1.0, icon_col),
            );
            ui.painter().line_segment(
                [egui::pos2(right_bar_x, box_rect.top()), egui::pos2(right_bar_x, box_rect.bottom())],
                Stroke::new(1.0, icon_col),
            );

            // Wypełnienie pasków, gdy listwy są rozwinięte
            if *is_visible {
                let fill_l = Rect::from_min_max(box_rect.left_top(), egui::pos2(left_bar_x, box_rect.bottom()));
                let fill_r = Rect::from_min_max(egui::pos2(right_bar_x, box_rect.top()), box_rect.right_bottom());
                ui.painter().rect_filled(fill_l, 1.0, icon_col);
                ui.painter().rect_filled(fill_r, 1.0, icon_col);
            }

            resp.on_hover_text(if *is_visible {
                "Ukryj pionowe listwy (Pełny ekran)"
            } else {
                "Pokaż pionowe listwy"
            });
        });
}
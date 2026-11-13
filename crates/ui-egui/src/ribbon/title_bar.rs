use super::{Align2, Icon, Rect, Sense, SlideApp, Tokens, Ui, icon_toggle, json, pos2, theme, vec2};
use crate::widgets::ndl_icon_button;

/// The title bar's height where Lámina draws the whole window frame (Linux, the BSDs, Windows):
/// room for Nodalia's 28 pt window controls with a margin above and below.
pub const TITLE_H: f32 = 44.0;

/// macOS draws the window into the title bar (`with_fullsize_content_view`) and AppKit paints the
/// traffic lights itself, 16 pt from the top of the window on macOS 27. Moving them would take
/// AppKit calls (`unsafe`, forbidden here), so on macOS the title bar is the band they are centred
/// in, with the controls beside them (#71). Up to macOS 15 the lights are smaller and centred 14 pt
/// from the top, about 2 pt above this bar's centre.
pub const MAC_TITLE_BAR: f32 = 32.0;

/// The traffic lights as AppKit lays them out, measured on macOS 27: button diameter,
/// centre-to-centre pitch and the first button's left edge.
pub(super) const MAC_LIGHT: f32 = 14.0;

pub(super) const MAC_LIGHT_PITCH: f32 = 23.0;

pub(super) const MAC_LIGHTS_LEFT: f32 = 9.0;

/// Where title-bar content starts on macOS: the end of the three lights plus the gap the lights
/// keep from the top of the window.
pub const MAC_CONTENT_LEFT: f32 = MAC_LIGHTS_LEFT + 2.0 * MAC_LIGHT_PITCH + MAC_LIGHT + (MAC_TITLE_BAR - MAC_LIGHT) / 2.0;

/// The title bar's height, the left edge of its content and the height of its tallest control.
/// With the integrated title bar (macOS) the bar is shorter, so the controls shrink to keep a
/// margin above and below them.
pub(super) fn title_geometry(integrated: bool) -> (f32, f32, f32) {
    if integrated { (MAC_TITLE_BAR, MAC_CONTENT_LEFT, 22.0) } else { (TITLE_H, theme::ISLAND_GAP + 2.0, 30.0) }
}

/// The window's title row: the app menu and quick access, the centred title, then search, the
/// comments pane, Present and (Linux, unless the desktop's are preferred) Nodalia's window
/// controls.
pub fn title_bar(app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (height, left, control_h) = title_geometry(app.integrated_titlebar);
    egui::Panel::top("title_bar").show_separator_line(false).exact_size(height).frame(egui::Frame::NONE.fill(t.chrome)).show(ui, |ui| {
        let rect = ui.max_rect();
        // Dragging the empty title bar moves the window.
        let drag = ui.interact(rect, ui.scope_id().with("titledrag"), Sense::click_and_drag());
        // The drag surface is not a button: a full-width Button would be announced as one.
        drag.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Window, true, "Window title bar"));
        if drag.drag_started() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if drag.double_clicked() {
            let max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
        }
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_max(pos2(rect.min.x + left, rect.min.y), pos2(rect.min.x + 460.0, rect.max.y)))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child.spacing_mut().item_spacing.x = 2.0;
        app_menu(app, &mut child, control_h);
        child.add_space(6.0);
        let has_doc = app.session.active().is_some();
        let dirty = app.session.active().is_some_and(|d| d.is_dirty());
        crate::qat::show(app, &mut child);
        let title = app.session.active().map(|d| format!("{}{}", d.title(), if dirty { " — Edited" } else { "" })).unwrap_or_else(|| "Lámina".into());
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, title, theme::bold(13.0), t.text);
        // Right side.
        let mut right_edge = rect.max.x - theme::ISLAND_GAP - 2.0;
        if app.draws_window_controls() {
            let controls = Rect::from_min_max(pos2(right_edge - nodalia_theme::window_controls::WIDTH, rect.min.y), pos2(right_edge, rect.max.y));
            window_controls(ui, controls);
            right_edge = controls.min.x - 14.0;
        }
        let mut right = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_max(pos2(right_edge - 360.0, rect.min.y), pos2(right_edge, rect.max.y)))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        right.spacing_mut().item_spacing.x = 6.0;
        let present = right.add(
            egui::Button::new(egui::RichText::new("Present").color(t.accent_text).font(theme::bold(13.0)))
                .fill(t.accent)
                .corner_radius(control_h / 2.0)
                .min_size(vec2(92.0, control_h)),
        );
        if present.clicked() && has_doc {
            let from = app.session.active().map(|d| d.selection.slide).unwrap_or(0);
            app.start_show(from, false);
        }
        if icon_toggle(&mut right, Icon::CommentsBubble, "Comments", has_doc, app.ui.pane.as_deref() == Some("comments")).clicked() {
            let _ = app.run("view.pane", json!({"pane": "comments", "toggle": true}));
        }
        if search_field(&mut right, control_h).clicked() {
            app.palette = Some((String::new(), 0));
        }
    });
}

/// The app menu (the "hamburger"): Preferences, Keyboard Shortcuts and About, as in every Nodalia
/// app. Everything else lives in the ribbon.
fn app_menu(app: &mut SlideApp, ui: &mut Ui, size: f32) {
    let open = egui::Popup::is_id_open(ui.ctx(), ui.make_persistent_id("app_menu_popup"));
    let r = ndl_icon_button(ui, "menu", "Main menu", size, open);
    let mut run = None;
    egui::Popup::menu(&r).id(ui.make_persistent_id("app_menu_popup")).width(220.0).show(|ui| {
        for (label, cmd) in APP_MENU {
            if *label == "-" {
                ui.separator();
            } else if ui.button(*label).clicked() {
                run = Some(*cmd);
                ui.close();
            }
        }
    });
    if let Some(cmd) = run {
        let _ = app.run(cmd, json!({}));
    }
}

/// The app menu's entries, in order ("-" is a separator).
pub const APP_MENU: &[(&str, &str)] =
    &[("Preferences", "app.preferences"), ("Keyboard Shortcuts", "app.shortcuts"), ("-", ""), ("About Lámina", "app.about")];

/// The command search, drawn as a rounded field: a click opens the command palette.
fn search_field(ui: &mut Ui, height: f32) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(240.0, height), Sense::click());
    let fill = if resp.hovered() { t.hover } else { t.text_field };
    ui.painter().rect_filled(rect, height / 2.0, fill);
    if let Some(icon) = nodalia_theme::icons::get("search") {
        let glyph = Rect::from_center_size(pos2(rect.min.x + height / 2.0 + 2.0, rect.center().y), vec2(16.0, 16.0));
        nodalia_theme::icons::paint(ui.painter(), glyph, icon, nodalia_theme::icons::Style::bold(t.text_dim));
    }
    let hint = if cfg!(target_os = "macos") { "Search commands  ⇧⌘P" } else { "Search commands  Ctrl+Shift+P" };
    ui.painter().text(pos2(rect.min.x + height + 6.0, rect.center().y), Align2::LEFT_CENTER, hint, theme::font(12.5), t.text_faint);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Search commands"));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Nodalia's minimize, maximize (or restore) and close buttons in `rect`.
fn window_controls(ui: &Ui, rect: Rect) {
    let dark = Tokens::get(ui.ctx()).dark;
    let mode = if dark { nodalia_theme::Mode::Dark } else { nodalia_theme::Mode::Light };
    let (focused, maximized) = ui.input(|i| (i.viewport().focused.unwrap_or(true), i.viewport().maximized.unwrap_or(false)));
    let labels = nodalia_theme::window_controls::Labels { minimize: "Minimize", maximize: "Maximize", restore: "Restore", close: "Close" };
    let action = nodalia_theme::window_controls::show(ui, rect, &theme::palette(dark), mode, focused, maximized, labels);
    use nodalia_theme::window_controls::Action;
    let cmd = match action {
        Some(Action::Minimize) => egui::ViewportCommand::Minimized(true),
        Some(Action::Maximize) => egui::ViewportCommand::Maximized(true),
        Some(Action::Restore) => egui::ViewportCommand::Maximized(false),
        Some(Action::Close) => egui::ViewportCommand::Close,
        None => return,
    };
    ui.ctx().send_viewport_cmd(cmd);
}

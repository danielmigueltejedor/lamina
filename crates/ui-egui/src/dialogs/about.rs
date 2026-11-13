use super::{Color32, Rect, Sense, SlideApp, Tokens, Ui, pos2, theme, vec2};

/// Room the About credits need for their table; the About tab keeps its compact size.
pub(super) const CREDITS_SIZE: egui::Vec2 = vec2(820.0, 380.0);

/// The Contributors / Models body in a block of its own fixed size, below the tab row. Sizing the
/// dialog's `ui` instead (`set_min_size` / `set_max_height`) squeezed the tab row and the first
/// credits row together, so the text was garbled and the tabs stopped answering (#843).
pub(super) fn credits_block(ui: &mut Ui, models: bool) -> Rect {
    ui.allocate_ui(CREDITS_SIZE, |ui| {
        ui.set_min_size(CREDITS_SIZE);
        if models { crate::credits::models_ui(ui) } else { crate::credits::contributors_ui(ui) }
    })
    .response
    .rect
}

pub fn about(_app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.vertical_centered(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(96.0, 96.0), Sense::hover());
        paint_logo(ui.painter(), r);
        ui.label(egui::RichText::new("Lámina").font(theme::bold(22.0)));
        ui.label(egui::RichText::new(version_line()).color(t.text_dim));
        ui.add_space(6.0);
        ui.label("Presentations and slide shows, rebuilt from scratch in pure Rust.");
        ui.label("An open-source, clean-room project. Not affiliated with Microsoft.");
        ui.add_space(6.0);
        ui.hyperlink_to("Report a problem or suggest an idea", lamina_engine::links::DISCORD);
        ui.hyperlink_to("getnodalia.com/lamina", lamina_engine::links::APP_PAGE);
        ui.hyperlink_to("Source on GitHub", lamina_engine::links::GITHUB);
        ui.add_space(6.0);
        ui.label(egui::RichText::new("MIT OR Apache-2.0 · Nodalia").size(11.0).color(t.text_faint));
    });
}

/// "Version X.Y.Z" plus, for release builds, the short commit and build date. The version comes
/// from `[workspace.package] version` (the single source of truth, `cargo xtask version`); the
/// release workflow sets `LAMINA_BUILD_SHA` / `LAMINA_BUILD_DATE` at build time.
pub fn version_line() -> String {
    let mut s = format!("Version {}", env!("CARGO_PKG_VERSION"));
    let sha = option_env!("LAMINA_BUILD_SHA").map(|s| s.get(..9).unwrap_or(s)).filter(|s| !s.is_empty());
    let date = option_env!("LAMINA_BUILD_DATE").filter(|s| !s.is_empty());
    match (sha, date) {
        (Some(sha), Some(date)) => s.push_str(&format!(" ({sha}, {date})")),
        (Some(x), None) | (None, Some(x)) => s.push_str(&format!(" ({x})")),
        (None, None) => {}
    }
    s
}

/// Lámina's app icon, the same picture the desktop shows for it.
const ICON_PNG: &[u8] = include_bytes!("../../../../assets/app-icon/hicolor/256x256/apps/dev.danielmiguelt.Lamina.png");

/// Paints Lámina's app icon into `r`. The texture is decoded once per context; if the picture
/// can't be decoded the space stays empty rather than failing the dialog.
pub fn paint_logo(p: &egui::Painter, r: Rect) {
    let ctx = p.ctx();
    let id = egui::Id::unique("lamina_app_icon");
    let texture = ctx.data(|d| d.get_temp::<Option<egui::TextureHandle>>(id)).unwrap_or_else(|| {
        let t = lamina_render::decode_image(ICON_PNG).map(|img| {
            let size = [img.width() as usize, img.height() as usize];
            ctx.load_texture("lamina_app_icon", egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()), egui::TextureOptions::LINEAR)
        });
        ctx.data_mut(|d| d.insert_temp(id, t.clone()));
        t
    });
    if let Some(t) = texture {
        p.image(t.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
    }
}

#[cfg(test)]
mod about_credits_tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::{CREDITS_SIZE, credits_block};
    use crate::dialogs::Dialog;
    use crate::{Services, SlideApp};

    fn tab_of(h: &Harness<'_, SlideApp>) -> String {
        let d = h.state().dialog.as_ref().expect("the About dialog is open");
        d.fields.get("tab").cloned().unwrap_or_else(|| "about".into())
    }

    /// The reported symptom (#843): in the real dialog the tabs stopped answering once Contributors
    /// or Models was open. Click through Contributors, Models and back to About.
    #[test]
    fn about_dialog_tabs_keep_responding() {
        let mut app = SlideApp::new(lamina_engine::Session::with_new(), Services::default());
        app.dialog = Some(Dialog::new("about"));
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_step_dt(1.0 / 60.0).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        assert_eq!(tab_of(&h), "about");
        for label in ["Contributors", "Models", "About", "Models", "Contributors"] {
            h.get_by_label(label).click();
            h.run_steps(3);
            assert_eq!(tab_of(&h), label.to_lowercase(), "clicking {label} switches the tab");
        }
    }

    #[test]
    fn credits_get_their_own_block_below_the_tab_row() {
        for models in [false, true] {
            let ctx = egui::Context::default();
            let mut tabs = egui::Rect::NOTHING;
            let mut block = egui::Rect::NOTHING;
            let mut whole = egui::Rect::NOTHING;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                tabs = ui.label("About").rect;
                block = credits_block(ui, models);
                whole = ui.min_rect();
            });
            out.textures_delta.clear();
            assert_eq!(block.size(), CREDITS_SIZE, "models={models}");
            assert!(block.min.y >= tabs.max.y, "the credits start below the tab row (models={models})");
            assert!(whole.max.y >= block.max.y, "the dialog grows to hold the block (models={models})");
            assert!(whole.height() > CREDITS_SIZE.y, "the tab row is not squeezed into the credits height (models={models})");
        }
    }
}

use super::{Color32, Rect, Sense, SlideApp, Tokens, Ui, pos2, theme, vec2};

/// About Lámina, laid out as in every Nodalia office app: the icon, the name and version, "Part of
/// Nodalia", the links and the licence. Upstream authors are credited in NOTICE, which the last
/// line points to.
pub fn about(_app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(360.0);
    ui.vertical_centered(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(96.0, 96.0), Sense::hover());
        paint_logo(ui.painter(), r);
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Lámina").font(theme::bold(22.0)));
        ui.label(egui::RichText::new(version_line()).color(t.text_dim));
        ui.label(egui::RichText::new("Part of Nodalia").color(t.text_dim));
        ui.add_space(10.0);
        ui.label("Presentations and slide shows, in pure Rust.");
        ui.add_space(10.0);
        ui.hyperlink_to("getnodalia.com/lamina", lamina_engine::links::APP_PAGE);
        ui.add_space(2.0);
        ui.hyperlink_to("Source code", lamina_engine::links::GITHUB);
        ui.add_space(2.0);
        ui.hyperlink_to("Report a problem or suggest an idea", lamina_engine::links::ISSUES);
        ui.add_space(10.0);
        ui.hyperlink_to("Licence: MIT OR Apache-2.0", LICENCE);
        ui.label(egui::RichText::new("Includes work by other open-source authors (see NOTICE).").size(11.0).color(t.text_faint));
    });
}

/// The licence the About window links to.
const LICENCE: &str = "https://github.com/getnodalia/lamina/blob/main/NOTICE";

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
mod about_tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use crate::dialogs::Dialog;
    use crate::{Services, SlideApp};

    /// About is one page in the office apps' layout: no credits tabs, no Cancel/Close row (the
    /// round close button at the top is the way out), and upstream authors only through NOTICE.
    #[test]
    fn about_is_one_page_that_closes_from_its_own_button() {
        let mut app = SlideApp::new(lamina_engine::Session::with_new(), Services::default());
        app.dialog = Some(Dialog::new("about"));
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(3);
        assert!(h.query_by_label("Part of Nodalia").is_some());
        assert!(h.query_by_label_contains("see NOTICE").is_some());
        for gone in ["Contributors", "Models", "Cancel"] {
            assert!(h.query_by_label(gone).is_none(), "{gone} is not in About");
        }
        h.get_by_label("Close window").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none(), "the close button closes About");
    }
}

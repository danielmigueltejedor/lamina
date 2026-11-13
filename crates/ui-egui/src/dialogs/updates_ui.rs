//! One background check at start-up (and on demand from Help) for a newer release, and the
//! windows that offer to update and restart. The flow and the windows are `nodalia-update`'s,
//! shared with the other Nodalia apps; this file is what is this app's own.

use nodalia_update::egui::{Action, Updater};

use crate::SlideApp;

pub(crate) struct Runtime {
    updater: Updater,
    /// The desktop host turns the check at start on; tests and the web build leave it off.
    enabled: bool,
    check_requested: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self { updater: Updater::new(crate::UPDATE_APP).with_accent([0xf0, 0x7a, 0x1e]), enabled: false, check_requested: false }
    }
}

pub(crate) fn enable(app: &mut SlideApp) {
    app.updates_rt.enabled = true;
}

/// Help > Check for Updates…: runs a check and reports the outcome either way.
pub(crate) fn request_check(app: &mut SlideApp) {
    app.updates_rt.check_requested = true;
}

pub(crate) fn tick(app: &mut SlideApp, ctx: &egui::Context) {
    let rt = &mut app.updates_rt;
    if std::mem::take(&mut rt.check_requested) {
        rt.updater.check_now(ctx);
    } else if rt.enabled {
        rt.updater.check_at_start(ctx);
    }
    let action = rt.updater.poll();
    act(ctx, action);
}

/// Carries out what the update windows asked for.
fn act(ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. Close
            // normally so the preferences are saved; the install only ran once nothing was
            // unsaved. Exit anyway if the app has not ended soon.
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut SlideApp, ctx: &egui::Context) {
    // An installation waits until nothing is unsaved. The interface is in English.
    let clean = !app.session.documents().iter().any(|d| d.is_dirty());
    let action = app.updates_rt.updater.show(ctx, nodalia_update::Language::English, clean);
    act(ctx, action);
}

#[cfg(test)]
mod tests {
    use super::*;
    use nodalia_update::egui::Status;

    #[test]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/lamina");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn an_offered_release_opens_the_prompt_and_escape_closes_it() {
        let mut app = SlideApp::new(lamina_engine::Session::new(), crate::Services::default());
        app.updates_rt.updater = Updater::new(crate::UPDATE_APP).with_stamp(None);
        let ctx = egui::Context::default();
        assert!(!app.updates_rt.updater.is_open());
        app.updates_rt.updater.offer(nodalia_update::Release {
            tag: "v99.0.0".into(),
            version: "99.0.0".into(),
            notes: String::new(),
            page: "https://github.com/getnodalia/lamina/releases/tag/v99.0.0".into(),
            asset: None,
        });
        ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx())).textures_delta.clear();
        assert!(app.updates_rt.updater.is_open());
        let escape = egui::Event::Key { key: egui::Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE };
        ctx.run_ui(egui::RawInput { events: vec![escape], ..Default::default() }, |ui| show(&mut app, ui.ctx())).textures_delta.clear();
        assert_eq!(app.updates_rt.updater.status(), &Status::Idle);
    }
}

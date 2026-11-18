//! Shift+letter steps a key's tool group in the toolbar's slot order.

use egui::{Key, Modifiers};
use serde_json::json;

use crate::PhotozApp;
use crate::state::Tool;

const HEALING: [Tool; 6] = [Tool::SpotHealing, Tool::Remove, Tool::Healing, Tool::Patch, Tool::ContentAwareMove, Tool::RedEye];

fn app() -> PhotozApp {
    let mut app = PhotozApp::new(photoz_engine::Session::new(), crate::Services::default());
    app.run("file.new", json!({"width": 20, "height": 10})).unwrap();
    app.run("layer.new.layer", json!({})).unwrap();
    app
}

fn press(app: &mut PhotozApp, key: Key, modifiers: Modifiers) {
    crate::canvas::testing::key(app, key, modifiers);
}

fn shift_sequence(key: Key, steps: usize) -> Vec<Tool> {
    let mut app = app();
    app.ui.tool = Tool::Hand;
    (0..steps)
        .map(|_| {
            press(&mut app, key, Modifiers::SHIFT);
            app.ui.tool
        })
        .collect()
}

#[test]
fn shift_j_follows_the_reference_sequence() {
    let want: Vec<Tool> = HEALING.iter().chain(&HEALING[..1]).copied().collect();
    assert_eq!(shift_sequence(Key::J, 7), want);
}

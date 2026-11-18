//! #7520: the options bar's Select and Mask… button follows the command `select.selectAndMask`
//! (it maps to `select.refineEdge`): greyed out while nothing is selected, live after Select All.
use crate::PhotozApp;
use crate::state::Tool;
use egui::vec2;
use egui_kittest::{
    Harness,
    kittest::{NodeT, Queryable},
};
use serde_json::json;

fn harness() -> Harness<'static, PhotozApp> {
    // The whole app: the button's label and enabled state come from the command state its frame publishes.
    let mut h = Harness::builder().with_size(vec2(1440.0, 900.0)).with_max_steps(64).build_eframe(|cc| {
        PhotozApp::setup_context(&cc.egui_ctx, Default::default());
        let mut app = PhotozApp::with_context(cc.egui_ctx.clone(), photoz_engine::Session::new(), crate::Services::default());
        app.run("file.new", json!({"width": 64, "height": 64})).unwrap();
        app.sync_views();
        app.ui.tool = Tool::RectMarquee;
        app
    });
    h.run_steps(8);
    h
}

fn button<'a>(h: &'a Harness<'static, PhotozApp>) -> egui_kittest::Node<'a> {
    h.get_by_role_and_label(egui::accesskit::Role::Button, "Select and Mask…")
}

#[test]
fn select_and_mask_follows_the_selection() {
    let mut h = harness();
    assert!(button(&h).accesskit_node().is_disabled(), "the button is greyed out with no selection");
    h.state_mut().run("select.all", json!({})).unwrap();
    h.run_steps(4);
    assert!(!button(&h).accesskit_node().is_disabled(), "the button is live with a selection");
}

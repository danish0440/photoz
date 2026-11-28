//! The Layers panel's Background row in an Indexed Color document (#16763): it reads "Index" in italics, while the
//! stored layer name stays "Background"; other modes keep the "Background" row.
use craft::layers::ui::tree::{LayerInfo, LayerTree};
use serde_json::json;

use super::PhotoLayers;
use crate::PhotozApp;

/// A new RGB document with its Background.
fn rgb_app() -> Box<PhotozApp> {
    let mut app = PhotozApp::new(photoz_engine::Session::new(), crate::Services::default());
    app.run("file.new", json!({"width": 32, "height": 24})).unwrap();
    app
}

fn indexed_app() -> Box<PhotozApp> {
    let mut app = rgb_app();
    app.run("image.mode.indexedColor", json!({"palette": "exact", "forced": "none", "dither": "none"})).unwrap();
    app
}

/// The Background's stored name and the panel row the Layers panel builds for it.
fn background_row(app: &PhotozApp) -> (String, LayerInfo) {
    let st = app.session.active().expect("a document");
    let layer = st.doc.walk().into_iter().map(|(_, _, l)| l).find(|l| st.doc.is_background(l.id)).expect("the document has a Background");
    let info = PhotoLayers::new(app, st, &Default::default(), false, None).info(&layer.id);
    (layer.name.clone(), info)
}

#[test]
fn an_indexed_documents_background_row_reads_index_in_italics() {
    let app = indexed_app();
    let (stored, row) = background_row(&app);
    assert_eq!(row.name, "Index");
    assert!(row.italic, "the Index row is italic");
    assert!(row.background, "the row is still the Background");
    assert_eq!(stored, "Background", "the stored layer name is unchanged");
}

#[test]
fn an_rgb_documents_background_row_keeps_its_name() {
    let app = rgb_app();
    let (stored, row) = background_row(&app);
    assert_eq!(row.name, stored);
    assert!(!row.italic, "the Background row stays upright");
}

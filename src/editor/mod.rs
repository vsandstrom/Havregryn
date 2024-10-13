mod components;

use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;

use crate::HavregrynParams;
use components::{
  header::header,
  body::body,
};

#[derive(Lens, Clone)]
pub(crate) struct Data {
    pub params: Arc<HavregrynParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(||(500, 440))
}

pub fn create(params: Data, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
  create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
    cx.add_stylesheet(include_style!("src/styles.css")).expect("failed to read stylesheet");
    assets::register_noto_sans_thin(cx);
    assets::register_noto_sans_light(cx);
    params.clone().build(cx);
    build_gui(cx);
  })
}

fn build_gui(cx: &mut Context) {
  VStack::new(cx, |cx| {
    header(cx);
    body(cx);
  });
}


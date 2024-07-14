use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug_vizia::vizia::image::Pixels;
use nih_plug_vizia::widgets::{ParamButton, ParamSlider};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;

use crate::HavregrynParams;

#[derive(Lens)]
struct Data {
    params: Arc<HavregrynParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(||(200, 150))
}

pub fn create(params: Arc<HavregrynParams>, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
  create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
    assets::register_noto_sans_thin(cx);
    assets::register_noto_sans_light(cx);

    Data {
      params: params.clone()
    }.build(cx);

    VStack::new(cx, |cx| {
      // Label::new(cx, "position")
      //   .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
      //   .font_weight(FontWeightKeyword::Thin)
      //   .font_size(15.0)
      //   .height(Pixels(15.0))
      //   .child_top(Stretch(1.0))
      //   .child_bottom(Pixels(0.0));
      Label::new(cx, "position");
      ParamSlider::new(cx, Data::params, |params| &params.position);

      Label::new(cx, "duration");
      ParamSlider::new(cx, Data::params, |params| &params.duration);

      Label::new(cx, "rate");
      ParamSlider::new(cx, Data::params, |params| &params.rate);

      Label::new(cx, "jitter");
      ParamSlider::new(cx, Data::params, |params| &params.jitter);

      Label::new(cx, "random");
      ParamButton::new(cx, Data::params, |params| &params.random);

      Label::new(cx, "resample");
      ParamButton::new(cx, Data::params, |params| &params.resample)
        .role(Role::SpinButton);
    })
      .row_between(Pixels(3.0))
      .child_top(Stretch(1.0))
      .child_bottom(Stretch(1.0))
    ;
  })

}


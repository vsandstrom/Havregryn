use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug::params::persist::PersistentField;
use nih_plug_vizia::widgets::{ParamButton, ParamButtonExt, ParamSlider};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;

use crate::HavregrynParams;

#[derive(Lens)]
struct Data {
    params: Arc<HavregrynParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(||(250, 550))
}

pub fn create(params: Arc<HavregrynParams>, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
  create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
    assets::register_noto_sans_thin(cx);
    assets::register_noto_sans_light(cx);

    Data {
      params: params.clone()
    }.build(cx);

    VStack::new(cx, |cx| {
      Label::new(cx, "havregryn")
        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
        .font_weight(FontWeightKeyword::Thin)
        .font_size(15.0)
        .height(Pixels(15.0))
        .child_top(Stretch(1.0))
        .child_bottom(Pixels(0.0));

      Label::new(cx, "position")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.position)
      .width(Percentage(80.0));

      Label::new(cx, "duration")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.duration)
      .width(Percentage(80.0));

      Label::new(cx, "rate")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.rate)
      .width(Percentage(80.0));
      
      Label::new(cx, "rate freq")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.rate_mod_freq)
      .width(Percentage(80.0));
      
      Label::new(cx, "rate mod")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.rate_mod_amount)
      .width(Percentage(80.0));

      Label::new(cx, "jitter")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.jitter)
      .width(Percentage(80.0));

      Label::new(cx, "trigger")
      .text_align(TextAlign::Left);
      ParamSlider::new(cx, Data::params, |params| &params.trigger)
      .width(Percentage(80.0));

      HStack::new(cx, |cx| {
        ParamButton::new(cx, Data::params, |params| &params.random)
        .width(Percentage(40.0));
        ParamButton::new(cx, Data::params, |params| &params.resample).with_label("sample")
        .width(Percentage(40.0));
      })
      .width(Percentage(80.0))
      .col_between(Percentage(10.0))
      .top(Stretch(1.0))
      .bottom(Stretch(1.5));
    })
    .child_left(Stretch(1.0))
    .child_right(Stretch(1.0))
    .child_top(Stretch(1.0))
    .child_bottom(Stretch(1.0))
    .row_between(Stretch(0.4))
    ;
  })

}


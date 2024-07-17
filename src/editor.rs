use std::borrow::BorrowMut;
use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug::params::Param;
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
  ViziaState::new(||(500, 360))
}

pub fn create(params: Arc<HavregrynParams>, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
  create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
    assets::register_noto_sans_thin(cx);
    assets::register_noto_sans_light(cx);

    Data {
      params: params.clone()
    }.build(cx);

    build_gui(cx);
  })

}

fn build_gui(cx: &mut Context) {

  const LH: Units = Pixels(20.0);
  const LW: Units = Percentage(84.0);

  const SH: Units = Pixels(28.0);
  const SW: Units = Percentage(100.0);

  const BH: Units = Pixels(34.0);
  const BW: Units = Percentage(40.0);


  VStack::new(cx, |cx| {
    Label::new(cx, "havregryn")
      .height(Percentage(20.0))
      .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
      .font_weight(FontWeightKeyword::Thin)
      .font_size(35.0)
      .left(Stretch(1.0))
      .right(Stretch(1.0))
      .top(Stretch(1.0))
      .bottom(Stretch(1.0))
      .child_top(Stretch(1.0))
      .child_bottom(Pixels(0.0));

    HStack::new(cx, |cx| {
      // Left column
      VStack::new(cx, |cx| {
        create_slider(cx, "position", Data::params, LH, LW, SH, SW, |params| &params.position);
        create_slider(cx, "jitter",   Data::params, LH, LW, SH, SW, |params| &params.jitter);
        create_slider(cx, "duration", Data::params, LH, LW, SH, SW, |params| &params.duration);
        create_slider(cx, "trigger",  Data::params, LH, LW, SH, SW, |params| &params.trigger);

      })
        .height(Percentage(90.0))
        .left(Pixels(42.0))
        .right(Pixels(22.0))
        .top(Pixels(25.0))
        .bottom(Pixels(18.0))
        .row_between(Pixels(6.0));
      
      // Right column
      VStack::new(cx, |cx| {
        create_slider(cx, "rate",       Data::params, LH, LW, SH, SW, |params| &params.rate);
        create_slider(cx, "mod freq",   Data::params, LH, LW, SH, SW, |params| &params.rate_mod_freq);
        create_slider(cx, "mod amount", Data::params, LH, LW, SH, SW, |params| &params.rate_mod_amount);
        // create_slider(cx, "mod shape", Data::params, LH, SH, SW, |params| &params.rate_mod_shape);
                                                    
        VStack::new(cx, |cx| {
          HStack::new(cx, |cx| {
            create_button(cx, "random", Data::params, BH, BW, |params| &params.random);
            create_button(cx, "sample", Data::params, BH, BW, |params| &params.resample);
          })
            .width(SW)
            .height(Pixels(42.0))
            .col_between(Percentage(10.0))
            .top(Stretch(1.8))
            .bottom(Stretch(1.0))
            .child_right(Stretch(1.0))
            .child_left(Stretch(1.0));
        })
          .width(Percentage(100.0));

      })
        .height(Percentage(90.0))
        .left(Pixels(22.0))
        .right(Pixels(42.0))
        .top(Pixels(25.0))
        .bottom(Pixels(18.0))
        .row_between(Pixels(6.0));

    })
      .width(Percentage(100.0))
      .height(Percentage(80.0))
      .col_between(Percentage(0.0))
      .top(Stretch(1.0))
      .bottom(Pixels(16.0));
  });
}

fn create_slider<L, Params, P, FMap>(
  cx: &mut Context,
  name: &str,
  params: L,
  label_height: Units,
  label_width: Units,
  height: Units,
  width: Units,
  f: FMap
  ) 
  where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: Param + 'static,
    FMap: Fn(&Params) ->  &P + Copy + 'static
{
  VStack::new(cx, |cx| {
    Label::new(cx, name)
      .width(label_width)
      .height(label_height)
      .text_align(TextAlign::Left);
    ParamSlider::new(cx, params, f)
      .height(height)
      .width(width);
  })
  .width(Percentage(100.0));
}

fn create_button<L, Params, P, FMap>(cx: &mut Context, name: &str, params: L, height: Units, width: Units, f: FMap) 
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: Param + 'static,
    FMap: Fn(&Params) ->  &P + Copy + 'static
{
  if name == "sample" {
    ParamButton::new(cx, params, f).with_label(name).width(width).height(height)
      .on_press(|e| {
        if !e.is_checked() {
          e.set_background_color(Color::rgb(0xff, 0x25, 0x5c)) // Lingonberry
        } else {
          // e.set_background_color(Color::rgb(0x80, 0x0, 0x20)) // Burgundy
          e.set_background_color(Color::default()) // Lingonberry
        }
      });
  } else {
    ParamButton::new(cx, params, f).with_label(name).width(width).height(height);
  }
}


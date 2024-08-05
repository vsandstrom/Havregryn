use nih_plug::params::Param;
use nih_plug_vizia::widgets::{ParamButton, ParamButtonExt};
use nih_plug_vizia::vizia::prelude::*;

use crate::editor::Data;
use super::slider::create_slider;
use super::button::create_button;

const LH: Units = Pixels(20.0);
const LW: Units = Percentage(84.0);

const SH: Units = Pixels(28.0);
const SW: Units = Percentage(100.0);

const BH: Units = Pixels(34.0);
const BW: Units = Percentage(40.0);

pub fn left_col(cx: &mut Context) {
  VStack::new(cx, |cx| {
    create_slider(cx, "position", Data::params, LH, LW, SH, SW, |params| &params.position);
    create_slider(cx, "jitter",   Data::params, LH, LW, SH, SW, |params| &params.jitter);
    create_slider(cx, "duration", Data::params, LH, LW, SH, SW, |params| &params.duration);
    create_slider(cx, "trigger",  Data::params, LH, LW, SH, SW, |params| &params.trigger);
    VStack::new(cx, |cx| {
    });
  })
    .height(Percentage(90.0))
    .left(Pixels(42.0))
    .right(Pixels(22.0))
    .top(Pixels(25.0))
    .bottom(Pixels(18.0))
    .row_between(Pixels(6.0));
}

pub fn right_col(cx: &mut Context) {
  VStack::new(cx, |cx| {
    create_slider(cx, "rate",       Data::params, LH, LW, SH, SW, |params| &params.rate);
    create_slider(cx, "mod freq",   Data::params, LH, LW, SH, SW, |params| &params.rate_mod_freq);
    create_slider(cx, "mod amount", Data::params, LH, LW, SH, SW, |params| &params.rate_mod_amount);
    create_slider(cx, "mod shape",  Data::params, LH, LW, SH, SW, |params| &params.rate_mod_shape);
  VStack::new(cx, |cx| {
      HStack::new(cx, |cx| {
        create_button(
          cx, 
          "random",
          Data::params,
          BH,
          BW,
          |params| &params.random
        );
        create_button(
          cx,
          "sample",
          Data::params,
          BH,
          BW,
          |params| &params.resample
        );
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

}




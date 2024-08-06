use nih_plug_vizia::vizia::prelude::*;
use super::columns::{left_col, right_col};

pub fn body(cx: &mut Context) {
  HStack::new(cx, |cx| {
    left_col(cx);
    right_col(cx)
  })
    .width(Percentage(100.0))
    .height(Percentage(80.0))
    .col_between(Percentage(0.0))
    .top(Stretch(1.0))
    .bottom(Pixels(16.0));
}

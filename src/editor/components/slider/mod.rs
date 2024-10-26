use nih_plug_vizia::widgets::ParamSlider;
use nih_plug_vizia::vizia::prelude::*;
use crate::Param;

#[allow(clippy::too_many_arguments)]
pub fn create_slider<L, Params, P, FMap>(
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
      .id("slider")
      .height(height)
      .width(width);
  })
  .width(Percentage(100.0));
}

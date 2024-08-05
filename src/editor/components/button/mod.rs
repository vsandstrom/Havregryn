use nih_plug_vizia::widgets::{ParamButton, ParamButtonExt};
use nih_plug_vizia::vizia::prelude::*;
use crate::Param;

pub fn create_button<L, Params, P, FMap>(cx: &mut Context, name: &str, params: L, height: Units, width: Units, f: FMap) 
where
    L: Lens<Target = Params> + Clone,
    Params: 'static + std::clone::Clone,
    P: Param + 'static,
    FMap: Fn(&Params) ->  &P + Copy + 'static
{
  // if name == "sample" {
    ParamButton::new(cx, params, f)
      .with_label(name)
      .width(width)
      .height(height)
      .child_left(Stretch(1.0))
      .child_right(Stretch(1.0))
      .class(name);
}

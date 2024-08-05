use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::assets;

pub fn header(cx: &mut Context) {
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
}

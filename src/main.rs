use nih_plug::prelude::*;
use havregryn::Havregryn;

fn main() {
  nih_export_standalone::<Havregryn<32, {8*48000}>>;
}

use rust_dsp::interpolation::{Interpolation, Linear};
pub struct MultiTable {
  position: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl MultiTable {
  pub fn new() -> Self {
    Self {
      position: 0.0,
      samplerate: 0.0,
      sr_recip: 0.0,
    }
  }


  #[inline]
  pub fn play<const N: usize>(&mut self, table: &[f32; N], frequency: f32, phase: f32) -> f32 {
    if frequency > self.samplerate * 0.5 { return 0.0 }
    let len = N as f32;

    self.position += (len * self.sr_recip * frequency) + ( phase * len );
    while self.position > len {
      self.position -= len;
    }
    Linear::interpolate(self.position, table, N)
  }

  #[inline]
  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.sr_recip = 1.0 / samplerate;
    self.samplerate = samplerate;
  }
}

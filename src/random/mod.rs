use rand::{self, Rng};
use rust_dsp::trig::Trigger;

pub struct Random {
  next: f32,
  duration: u64,
  counter: u64,
  samplerate: f32,
  value: f32,
  inc: f32,
  sr_recip: f32,
}

impl Trigger for Random {
  fn new(samplerate: f32) -> Self {
    Self {
      sr_recip: 1.0/ samplerate,
      samplerate,
      next: 0.0,
      value: 0.0,
      inc: 0.0,
      counter: 0,
      duration: 0,
    }
  }

  fn play(&mut self, duration: f32) -> f32 {
    if self.counter >= self.duration {
      self.duration = (self.samplerate * duration) as u64;
      self.counter = 0;
      self.next = (rand::thread_rng().gen::<f32>() * 2.0) - 1.0;
      self.inc = ( self.next - self.value ) / self.duration as f32;
    }
    self.value += self.inc;
    self.counter += 1;
    self.value
  }

  fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn poll() {
    let mut rnd = Random::new(48000.0);
    let v = rnd.play(0.2);
    assert!( v <= 1.0 || v >= -1.0 || v != 0.0);

    
  }
}

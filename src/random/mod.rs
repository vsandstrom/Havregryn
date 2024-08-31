use rand::{self, Rng};
use rust_dsp::trig::Trigger;

pub struct Random {
  next: f32,
  duration: f32,
  duration_in_samples: u64,
  counter: u64,
  samplerate: f32,
  current: f32,
  inc: f32,
  sr_recip: f32,
}

impl Trigger for Random {
  fn new(samplerate: f32) -> Self {
    Self {
      sr_recip: 1.0/ samplerate,
      samplerate,
      next: 0.0,
      current: 0.0,
      inc: 0.0,
      counter: 0,
      duration: 0.0,
      duration_in_samples: 0,
    }
  }

  fn play(&mut self, duration: f32) -> f32 {
    self.counter += 1;
    if self.counter >= (self.duration * self.samplerate) as u64 {
      self.duration = duration;
      self.duration_in_samples = (self.samplerate * duration) as u64;
      self.counter = 0;
      self.next = (rand::thread_rng().gen::<f32>() * 2.0) - 1.0;
      self.inc = ( self.next - self.current ) / self.duration_in_samples as f32;
    }
    self.current += self.inc;
    self.current
  }

  fn set_samplerate(&mut self, samplerate: f32) {
      self.sr_recip = 1.0 / samplerate;
      self.samplerate = samplerate;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn poll() {
    let rnd = Random::new(48000.0);

    
  }
}

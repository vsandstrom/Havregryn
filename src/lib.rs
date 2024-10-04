mod editor;
mod multitable;

use std::sync::Arc;
use rand::Rng;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use rust_dsp::{
  grains::{
    GrainTrait,
    stereo::Granulator
  },
  envelope::EnvType,
  waveshape::traits::Waveshape,
  interpolation::Linear,
  trig::{Dust, Impulse, Trigger},
  noise::Noise,
  dsp::math::mtor
};

/* 
 *
 *      TÄNK ÖVER HASHMAPPEN!!!!!!!!!!
 *
 *
 *
 *
 * */

use crate::multitable::MultiTable;
// use crate::random::Random;

const SIZE: usize = 1<<13;
const MIDI: usize = 1<<7;

struct Havregryn<const NUMGRAINS: usize, const BUFSIZE: usize> {
  params:          Arc<HavregrynParams>,
  granulator:      Granulator<NUMGRAINS, BUFSIZE>,
  rate_modulator:  MultiTable,
  rate_random_mod: Noise,
  sin:             [f32; SIZE],
  tri:             [f32; SIZE],
  saw:             [f32; SIZE],
  sqr:             [f32; SIZE],
  imp:             Impulse,
  dust:            Dust,
  start_bool:      bool,
  sr_recip:        f32,
  pitches:         Vec<usize>,
  midi_rates:      [f32; MIDI],
  midi_idx:        usize,
}

#[derive(Enum, PartialEq)]
enum ModShape {
  Sine,
  Tri,
  Saw,
  Square,
  // RANDOM
}

#[derive(Params)]
struct HavregrynParams {
  /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
  /// these IDs remain constant, you can rename and reorder these fields as you wish. The
  /// parameters are exposed to the host in the same order they were defined. In this case, this
  /// gain parameter is stored as linear gain while the values are displayed in decibels.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "position"]
  pub position: FloatParam,
  #[id = "duration"]
  pub duration: FloatParam,
  #[id = "jitter"]
  pub jitter: FloatParam,
  #[id = "trigger"]
  pub trigger: FloatParam,
  
  #[id = "spread"]
  pub spread: FloatParam,

  #[id = "rate"]
  pub rate: FloatParam,
  #[id = "rate-mod-amount"]
  pub rate_mod_amount: FloatParam,
  #[id = "rate-mod-shape"]
  pub rate_mod_shape: EnumParam<ModShape>,
  #[id = "rate-mod-freq"]
  pub rate_mod_freq: FloatParam,

  #[id = "random"]
  pub random: BoolParam,
  #[id = "resample"]
  pub resample: BoolParam,

  // #[allow(unused)]
  // pub color: AtomicBool,
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> Default for Havregryn<NUMGRAINS, BUFSIZE> { 
  fn default() -> Self { 
    let env_shape: EnvType = EnvType::Vector([0.0;512].hanning().to_vec());
    Self {
      params: Arc::new(HavregrynParams::default()),
      sin: [0.0; SIZE].sine(),
      tri: [0.0; SIZE].triangle(),
      saw: [0.0; SIZE].sawtooth(),
      sqr: [0.0; SIZE].square(),
      // rate_modulator: WaveTable::<WT_BUFSIZE>::new(sin.borrow_mut(), 0.0),
      rate_modulator:  MultiTable::new(),
      rate_random_mod: Noise::new(0.0),
      granulator:      Granulator::new(&env_shape, 0.0),
      imp:             Impulse::new(0.0),
      dust:            Dust::new(0.0),
      // sample_color_active: Color::rgba(0xff, 0x25, 0x5c, 0x00),
      // sample_color_deactive: Color::rgba(0xfa, 0xfa, 0xfa, 0x00),
      sr_recip:        0.0,
      start_bool:      true,
      pitches:         vec!(),
      midi_rates:      [0.0; MIDI],
      midi_idx:        0
    }
  }
}

impl Default for HavregrynParams {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      position: FloatParam::new(
        "position", 
        0.0, 
        FloatRange::Linear { min: 0.0, max: 1.0 }
      )
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) })),
      
      jitter: FloatParam::new(
        "jitter amount",
        0.0,
        FloatRange::Skewed { min: 0.001, max: 1.0, factor: 1.46 }
      )
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) })),

      duration: FloatParam::new(
        "grain length", 
        0.2, 
        FloatRange::Skewed { min: 0.05, max: 2.5, factor: 0.8 }
      )
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) }))
        .with_unit(" sec"),

      trigger: FloatParam::new(
        "trigger interval", 
        0.3, 
        FloatRange::Skewed { min: 0.03, max: 2.5, factor: 0.8 }
      )
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) }))
        .with_unit(" sec"),
      
      spread: FloatParam::new(
        "stereo spread", 
        0.0, 
        FloatRange::Linear { min: 0.0, max: 1.0 }
      )
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) })),

      rate: FloatParam::new(
        "speed",
        1.0,
        FloatRange::Linear { min: -1.0, max: 1.0 }
      ) 
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) })),

      rate_mod_freq: FloatParam::new(
        "mod freq",
        12.0,
        FloatRange::Skewed { min: 0.2, max: 60.0, factor: 0.3 }
      )
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_value_to_string(Arc::new(|f| { format!("{:.2}", f) }))
        .with_unit(" Hz"),

      rate_mod_amount: FloatParam::new(
        "mod amount",
        0.0,
        FloatRange::Skewed { min: 0.002, max: 1.0, factor: 0.46 }
      )
        .with_smoother(SmoothingStyle::Logarithmic(50.0))
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) })),

      rate_mod_shape: EnumParam::new("mod shape", ModShape::Sine),

      resample: BoolParam::new(
        "sample", 
        false
      ),

      random: BoolParam::new(
        "random", 
        false
      ),

      // color: AtomicBool::new(false)
    }
  }
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> Plugin for Havregryn<NUMGRAINS, BUFSIZE> {
  const NAME: &'static str = "Havregryn";
  const VENDOR: &'static str = "Viktor Sandström";
  const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
  const EMAIL: &'static str = "sandstrom.viktor@gmail.com";

  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  // The first audio IO layout is used as the default. The other layouts may be selected either
  // explicitly or automatically by the host or the user depending on the plugin API/backend.
  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(2),
    main_output_channels: NonZeroU32::new(2),

    aux_input_ports: &[],
    aux_output_ports: &[],

    // Individual ports and the layout as a whole can be named here. By default these names
    // are generated as needed. This layout will be called 'Stereo', while a layout with
    // only one input and output channel would be called 'Mono'.
    names: PortNames::const_default(),
  }];


  const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
  const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
  // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
  // from plain byte buffers.
  type SysExMessage = ();
  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any backgrouggnd
  // tasks.
  type BackgroundTask = ();

  fn params(&self) -> Arc<dyn Params + 'static> {
      self.params.clone()
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    // Resize buffers and perform other potentially expensive initialization operations here.
    // The `reset()` function is always called right after this function. You can remove this
    // function if you do not need it.
    let sr = buffer_config.sample_rate;
    self.imp.set_samplerate(sr);
    self.dust.set_samplerate(sr);
    self.granulator.set_samplerate(sr);

    // self.granulators[0].set_buffersize((4.0 * sr) as usize);
    // self.granulators[1].set_buffersize((4.0 * sr) as usize);
    self.rate_modulator.set_samplerate(sr);
    self.rate_random_mod.set_samplerate(sr);
    self.sr_recip = 1.0 / sr;

    // initialize midi lookup table
    for i in 0..MIDI {
      self.midi_rates[i] = mtor(i as u8)
    }
    // for (i, note) in self.midi_rates.iter_mut().enumerate() {
    //   *note = f32::powf(2.0, (i as f32 - 36.0) / 12.0);
    // }
    true
  }

  fn reset(&mut self) {
    // Reset buffers and envelopes here. This can be called from the audio thread and may not
    // allocate. You can remove this function if you do not need it.
  }
  
  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(
      self.params.clone(),
      self.params.editor_state.clone()
    )
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {

    // Once per buffer
    for mut frame in buffer.iter_samples() {
      // Mono sum of input
      // since frame is already a product of an iterator, 
      // this should be fine.
      let mono: f32 = unsafe {(
        *frame.get_unchecked_mut(0) 
        + *frame.get_unchecked_mut(1)
        ) * 0.5
      };


      'midi_loop: while let Some(event) = context.next_event() {
        // if event.timing() != sample_id as u32 {
        //   break;
        // }
        match event {
          NoteEvent::NoteOn {note, ..} => { self.pitches.push(note as usize); },
          NoteEvent::NoteOff {note, ..} => {
            if let Some(p) = self.pitches.iter().position(|p| *p == note as usize) {
              self.pitches.remove(p);
            }
          },
          _ => { break 'midi_loop; }
        }
      }
      
      if self.params.resample.value() {
        self.start_bool = true;
        self.granulator.reset_record();
      }
    
      if self.start_bool {
        // Once per frame
        let trig = self.params.trigger.value();
        let position  = self.params.position.smoothed.next();
        let duration  = self.params.duration.smoothed.next();
        let rmod = self.params.rate_mod_amount.smoothed.next();
        let rfrq = self.params.rate_mod_freq.smoothed.next();
        let mut rate = self.params.rate.smoothed.next();
        let mut jitter  = self.params.jitter.smoothed.next();
        let mut pan = self.params.spread.smoothed.next();

        let trigger = match self.params.random.value() {
          true => {
            // keep the triggers going even when unused
            let _ = self.imp.play(trig);
            self.dust.play(trig)
          },
          false => {
            let _ = self.dust.play(trig);
            self.imp.play(trig)
          }
        };
        
        
        // granulator record buffer returns None when the buffer is full.
        if self.granulator.record(mono).is_none() {
          let modulator = match self.params.rate_mod_shape.value() {
            ModShape::Sine =>   { self.rate_modulator.play(&self.sin, rfrq, 0.0) },
            ModShape::Tri =>    { self.rate_modulator.play(&self.tri, rfrq, 0.0) },
            ModShape::Saw =>    { self.rate_modulator.play(&self.saw, rfrq, 0.0) },
            ModShape::Square => { self.rate_modulator.play(&self.sqr, rfrq, 0.0) },
            // ModShape::RANDOM => { self.rate_random_mod.play(rfrq * self.sr_recip) },
          };

          if trigger >= 1.0 {
            pan *= rand::thread_rng().gen_range(-1.0..=1.0);
            jitter *= rand::thread_rng().gen::<f32>();
            for p in self.pitches.iter() {
              let r = rate * self.midi_rates[*p] + (rmod * modulator);
              self.granulator.trigger_new(
                position,
                duration,
                pan,
                r,
                jitter
              );

            }
            // trigger grains for all active midi notes at once. 
            // if !self.pitches.is_empty() {
            //   self.midi_idx %= self.pitches.len();
            //   rate *= self.midi_rates[self.pitches[self.midi_idx]] + (rmod * modulator);
            //   self.midi_idx += 1;
            // } 
            // self.granulator.trigger_new(
            //   position,
            //   duration,
            //   pan,
            //   rate,
            //   jitter
            // );
          }

          let out_frame = self.granulator.play::<Linear, Linear>();

          frame
            .into_iter()
            .zip(out_frame.iter())
            .for_each(
              |(sample, grain)| { 
              *sample = *grain 
            }
          );
        }
      }
    }
    ProcessStatus::Normal
  }
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> ClapPlugin for Havregryn<NUMGRAINS, BUFSIZE> {
  const CLAP_ID: &'static str = "com.your-domain.havregryn";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("Granular sampler");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;

  // Don't forget to change these features
  const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> Vst3Plugin for Havregryn<NUMGRAINS, BUFSIZE> {
  const VST3_CLASS_ID: [u8; 16] = *b"Havregryn       ";

  // And also don't forget to change these categories
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
      &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

// nih_export_clap!(Havregryn<16, {8*48000}>);
nih_export_vst3!(Havregryn<16, {8*48000}>);

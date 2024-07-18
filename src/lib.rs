use std::{borrow::BorrowMut, sync::{
  atomic::AtomicBool, 
  Arc}};
use rand::Rng;

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use rust_dsp::{
  grains::Granulator,
  envelope::EnvType,
  waveshape::traits::Waveshape,
  wavetable::owned::WaveTable,
  interpolation::Linear,
  trig::{Dust, Impulse, Trigger},
};

mod editor;
mod multitable;

use crate::multitable::MultiTable;

struct Havregryn<const NUMGRAINS: usize, const BUFSIZE: usize> {
  params: Arc<HavregrynParams>,
  granulators: [Granulator<NUMGRAINS, BUFSIZE>; 2],
  rate_modulator: MultiTable,
  // rate_modulator: WaveTable<{1<<13}>,
  rate_wt_bufsize: usize,
  sin: [f32; 1<<13],
  tri: [f32; 1<<13],
  saw: [f32; 1<<13],
  sqr: [f32; 1<<13],
  imp: Impulse,
  dust: Dust
}

#[derive(Enum, PartialEq)]
enum ModShape {
  SINE,
  TRI,
  SAW,
  SQUARE,
  RANDOM
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

  #[allow(unused)]
  pub resample_bool: Arc<AtomicBool>
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> Default for Havregryn<NUMGRAINS, BUFSIZE> {
  fn default() -> Self {
    let env_shape: EnvType = EnvType::Vector([0.0;512].hanning().to_vec());
    const WT_BUFSIZE: usize = 1<<13;
    let mut sin = [0.0; WT_BUFSIZE];
    let sin = sin.sine();

    Self {
      params: Arc::new(HavregrynParams::default()),
      rate_wt_bufsize: WT_BUFSIZE,
      sin: *sin,
      tri: *[0.0; WT_BUFSIZE].triangle(),
      saw: *[0.0; WT_BUFSIZE].sawtooth(),
      sqr: *[0.0; WT_BUFSIZE].square(),
      // rate_modulator: WaveTable::<WT_BUFSIZE>::new(sin.borrow_mut(), 0.0),
      rate_modulator: MultiTable::new(),
      granulators: [Granulator::new(&env_shape, 0.0), Granulator::new(&env_shape, 0.0)],
      imp: Impulse::new(0.0),
      dust: Dust::new(0.0),
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
        1.0, 
        FloatRange::Skewed { min: 0.03, max: 2.5, factor: 0.8 }
      )
        .with_value_to_string(Arc::new(|i| { format!("{:.2}", i) }))
        .with_unit(" sec"),

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

      rate_mod_shape: EnumParam::new("mod shape", ModShape::SINE),

      resample: BoolParam::new(
        "sample", 
        false
      ),

      random: BoolParam::new(
        "random", 
        false
      ),

      resample_bool: Arc::new(AtomicBool::new(false))
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


  const MIDI_INPUT: MidiConfig = MidiConfig::None;
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
    self.granulators[0].set_samplerate(sr);
    self.granulators[1].set_samplerate(sr);

    // self.granulators[0].set_buffersize((4.0 * sr) as usize);
    // self.granulators[1].set_buffersize((4.0 * sr) as usize);
    self.rate_modulator.set_samplerate(sr);
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
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    // Once per buffer


    for channel_samples in buffer.iter_samples() {
      // Once per frame
      let random = self.params.random.value();
      let trig = self.params.trigger.value();


      match self.params.resample.value() {
        true => {
            self.granulators[0].reset_record();
            self.granulators[1].reset_record();
        },
        false => {
          ()
        }
      } 
    

      let trigger = match random {
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

      for (ch, sample) in channel_samples.into_iter().enumerate() {
        // Once per channel/sample
        if let Some(sig) = self.granulators[ch].record(*sample) {
          // passthru
          *sample = sig;
        } else {
          let pos  = self.params.position.smoothed.next();
          let dur  = self.params.duration.smoothed.next();
          let rate = self.params.rate.smoothed.next();
          let jit  = self.params.jitter.smoothed.next() * rand::thread_rng().gen::<f32>();
          let rmod = self.params.rate_mod_amount.smoothed.next();
          let rfrq = self.params.rate_mod_freq.smoothed.next();

          let modulator = match self.params.rate_mod_shape.value() {
            ModShape::SINE =>   { self.rate_modulator.play(&self.sin, rfrq, 0.0) },
            ModShape::TRI =>    { self.rate_modulator.play(&self.tri, rfrq, 0.0) },
            ModShape::SAW =>    { self.rate_modulator.play(&self.saw, rfrq, 0.0) },
            ModShape::SQUARE => { self.rate_modulator.play(&self.sqr, rfrq, 0.0) },
            ModShape::RANDOM => { rand::thread_rng().gen::<f32>() },
          };

          *sample = self.granulators[ch].play::<Linear, Linear>(
            pos,
            dur,
            // rate + (self.rate_modulator.play::<Linear>(rfrq, 0.0) * rmod),
            rate + (modulator * rmod),
            jit,
            trigger
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

nih_export_clap!(Havregryn<16, {4*48000}>);
nih_export_vst3!(Havregryn<16, {4*48000}>);

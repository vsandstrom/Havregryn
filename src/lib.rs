use interpolation::interpolation::Linear;
use nih_plug::{params::persist, prelude::*};
use nih_plug_iced::IcedState;
use nih_plug_webview::*;
use trig::{Dust, Impulse, Trigger};
use std::sync::Arc;
use grains::Granulator;
use envelope::EnvType;
use waveshape::traits::Waveshape;
use serde::Deserialize;
use rand::Rng;

mod editor;


#[derive(Deserialize)]
#[serde(tag = "type")]
enum Action {
  Init,
  SetSize   { width: u32, height: u32 },
  SetPos    { value: f32 },
  SetDur    { value: f32 },
  SetRate   { value: f32 },
  SetJit    { value: f32 },
  SetTrig   { value: f32 },
  SetRandom { value: bool },
  SetSample { value: bool },
}

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct Havregryn<const NUMGRAINS: usize, const BUFSIZE: usize> {
  params: Arc<HavregrynParams>,
  granulators: [Granulator<NUMGRAINS, BUFSIZE>; 2],
  imp: Impulse,
  dust: Dust
}

#[derive(Params)]
struct HavregrynParams {
  /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
  /// these IDs remain constant, you can rename and reorder these fields as you wish. The
  /// parameters are exposed to the host in the same order they were defined. In this case, this
  /// gain parameter is stored as linear gain while the values are displayed in decibels.
  #[persist = "editor-state"]
  pub editor_state: Arc<IcedState>,
  #[id = "wet"]
  pub wet: FloatParam,
  #[id = "position"]
  pub position: FloatParam,
  #[id = "duration"]
  pub duration: FloatParam,
  #[id = "rate"]
  pub rate: FloatParam,
  #[id = "jitter"]
  pub jitter: FloatParam,
  #[id = "trigger"]
  pub trigger: FloatParam,
  #[id = "random"]
  pub random: BoolParam,
  #[id = "resample"]
  pub resample: BoolParam,
}

impl<const NUMGRAINS: usize, const BUFSIZE: usize> Default for Havregryn<NUMGRAINS, BUFSIZE> {
  fn default() -> Self {
    let env_shape: EnvType = EnvType::Vector([0.0;512].hanning().to_vec());
    let granulators = [Granulator::new(&env_shape, 0.0), Granulator::new(&env_shape, 0.0)];
    let imp = Impulse::new(0.0);
    let dust = Dust::new(0.0);

    Self {
      params: Arc::new(HavregrynParams::default()),
      granulators,
      imp,
      dust
    }
  }
}

impl Default for HavregrynParams {
  fn default() -> Self {
    Self {
      // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
      // to treat these kinds of parameters as if we were dealing with decibels. Storing this
      // as decibels is easier to work with, but requires a conversion for every sample.
      editor_state: IcedState::from_size(650, 450),
      wet: FloatParam::new(
        "wet",
        util::db_to_gain(0.0),
        FloatRange::Skewed {
          min: util::db_to_gain(-30.0),
          max: util::db_to_gain(30.0),
          // This makes the range appear as if it was linear when displaying the values as
          // decibels
          factor: FloatRange::gain_skew_factor(-30.0, 30.0),
        },
      )
      // Because the gain parameter is stored as linear gain instead of storing the value as
      // decibels, we need logarithmic smoothing
      .with_smoother(SmoothingStyle::Logarithmic(50.0))
      .with_unit(" dB")
      // There are many predefined formatters we can use here. If the gain was stored as
      // decibels instead of as a linear gain value, we could have also used the
      // `.with_step_size(0.1)` function to get internal rounding.
      .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
      .with_string_to_value(formatters::s2v_f32_gain_to_db()),
      position: FloatParam::new(
        "position", 
        0.0, 
        FloatRange::Linear { min: 0.0, max: 1.0 }
      ),
      duration: FloatParam::new(
        "grain length", 
        0.2, 
        FloatRange::Skewed { min: 0.05, max: 2.5, factor: 0.8 }
      ).with_unit(" sec"),
      rate: FloatParam::new(
        "speed",
        1.0,
        FloatRange::Linear { min: -1.0, max: 1.0 }
      ),
      jitter: FloatParam::new(
        "jitter amount",
        0.0,
        FloatRange::Linear { min: 0.0, max: 1.0 }
      ),
      trigger: FloatParam::new(
        "trigger interval", 
        1.0, 
        FloatRange::Skewed { min: 0.03, max: 5.0, factor: 0.7 }
      ).with_unit(" sec"),
      resample: BoolParam::new(
        "sample", 
        false
      ),
      random: BoolParam::new(
        "random", 
        false
      ),
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
    self.imp.samplerate = sr;
    self.dust.samplerate = sr;
    for i in 0..2 { self.granulators[i].samplerate = sr; }

    true
  }

  fn reset(&mut self) {
    // Reset buffers and envelopes here. This can be called from the audio thread and may not
    // allocate. You can remove this function if you do not need it.
  }
  
  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }


  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    // Once per buffer
    let wet = self.params.trigger.smoothed.next();
    let trig = self.params.trigger.smoothed.next();
    let random = self.params.random.value();

    for channel_samples in buffer.iter_samples() {
      // Once per frame
      let pos  = self.params.position.smoothed.next();
      let dur  = self.params.duration.smoothed.next();
      let rate = self.params.rate.smoothed.next();
      let jit  = self.params.jitter.smoothed.next() * rand::thread_rng().gen::<f32>();

      if self.params.resample.value() {
        for gr in self.granulators.iter_mut() {
          gr.reset_record();
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
          let gr = self.granulators[ch].play::<Linear, Linear>(
            pos,
            dur,
            rate,
            jit,
            trigger
          );
          *sample = (wet * gr) + (1.0 - wet) * (*sample);

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

nih_export_clap!(Havregryn<32, {8*48000}>);
nih_export_vst3!(Havregryn<32, {8*48000}>);

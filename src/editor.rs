use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use nih_plug_iced::button;
use pane_grid::Content;
use widgets::generic_ui::GenericUi;
use widgets::ParamMessage;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use crate::HavregrynParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
  IcedState::from_size(200, 150)
}

pub(crate) fn create(
  params: Arc<HavregrynParams>,
  editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
  create_iced_editor::<HavregrynEditor>(editor_state, params.clone())
}

struct HavregrynEditor {
  params: Arc<HavregrynParams>,
  context: Arc<dyn GuiContext>,

  // wet: Arc<AtomicF32>,
  // position: Arc<AtomicF32>,
  // duration: Arc<AtomicF32>,
  // rate: Arc<AtomicF32>,
  // jitter: Arc<AtomicF32>,
  // trigger: Arc<AtomicF32>,
  // random: Arc<AtomicBool>,
  // resample: Arc<AtomicBool>,

  wet_slider_state: nih_widgets::param_slider::State,
  pos_slider_state: nih_widgets::param_slider::State,
  dur_slider_state: nih_widgets::param_slider::State,
  rate_slider_state: nih_widgets::param_slider::State,
  jit_slider_state: nih_widgets::param_slider::State,
  trig_slider_state: nih_widgets::param_slider::State,
  rand_button_state: nih_widgets::param_slider::State,
  // rand2_button_state: nih_wi,
  resample_button_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  /// Update a parameter's value.
  ParamUpdate(nih_widgets::ParamMessage),
  // RandTrue,
  // RandFalse,
  // ResampleTrue, 
  // ResampleFalse,
}

impl IcedEditor for HavregrynEditor {
  type Executor = executor::Default;
  type Message = Message;
  type InitializationFlags = Arc<HavregrynParams>;

  fn new(
    params: Self::InitializationFlags,
    context: Arc<dyn GuiContext>,
  ) -> (Self, Command<Self::Message>) {
    let editor = HavregrynEditor {
      params,
      context,

      // wet:      Arc::new(AtomicF32::new(0.0)),
      // position: Arc::new(AtomicF32::new(0.0)),
      // duration: Arc::new(AtomicF32::new(0.2)),
      // rate:     Arc::new(AtomicF32::new(1.0)),
      // jitter:   Arc::new(AtomicF32::new(0.0)),
      // trigger:  Arc::new(AtomicF32::new(0.5)),
      // random:   Arc::new(AtomicBool::new(false)),
      // resample: Arc::new(AtomicBool::new(false)),

      wet_slider_state:      Default::default(),
      pos_slider_state:      Default::default(),
      rate_slider_state:     Default::default(),
      dur_slider_state:      Default::default(),
      jit_slider_state:      Default::default(),
      trig_slider_state:     Default::default(),
      resample_button_state: Default::default(),
      rand_button_state:     Default::default(),
      // rand2_button_state:    Default::default(),
    };

    (editor, Command::none())
  }

  fn context(&self) -> &dyn GuiContext {
    self.context.as_ref()
  }

  fn update(
    &mut self,
    _window: &mut WindowQueue,
    message: Self::Message,
  ) -> Command<Self::Message> {
    match message {
      Message::ParamUpdate(message) => self.handle_param_message(message),
      // Message::RandTrue => self.handle_param_message(message)
    }

    Command::none()
  }

  fn view(&mut self) -> Element<'_, Self::Message> {
    Column::new()
      .align_items(Alignment::Center)
      .push(Space::with_width(180.into()))
      .push(Space::with_height(60.into()))
      .push(
        Text::new("pos")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(

        nih_widgets::ParamSlider::new(&mut self.pos_slider_state, &self.params.position)
          .height(15.into())
          .map(Message::ParamUpdate),
      )

      .push(Space::with_height(10.into()))
      .push(
        Text::new("dur")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.dur_slider_state, &self.params.duration)
          .height(15.into())
          .map(Message::ParamUpdate),
      )

      .push(Space::with_height(10.into()))
      .push(
        Text::new("rate")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.rate_slider_state, &self.params.rate)
          .height(15.into())
          .map(Message::ParamUpdate),
      )

      .push(Space::with_height(10.into()))
      .push(
        Text::new("jit")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.jit_slider_state, &self.params.jitter)
          .height(15.into())
          .map(Message::ParamUpdate),
      )

      .push(Space::with_height(10.into()))
      .push(
        Text::new("trig")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.trig_slider_state, &self.params.trigger)
          .height(15.into())
          .map(Message::ParamUpdate),
      )
      .push(Space::with_height(10.into()))
      .push(
        Text::new("rand")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.rand_button_state, &self.params.random)
          .height(15.into())
          .map(Message::ParamUpdate),
      )

      .push(Space::with_height(10.into()))
      .push(
        Text::new("sample")
          .height(15.into())
          .width(80.into())
          .horizontal_alignment(alignment::Horizontal::Left)
          .vertical_alignment(alignment::Vertical::Center),
      )
      .push(
        nih_widgets::ParamSlider::new(&mut self.resample_button_state, &self.params.resample)
          .height(15.into())
          .map(Message::ParamUpdate)
      )
      .push(Space::with_height(10.into()))
    .into()
  }

  fn background_color(&self) -> nih_plug_iced::Color {
    nih_plug_iced::Color {
      r: 0.98,
      g: 0.98,
      b: 0.98,
      a: 1.0,
    }
  }
}

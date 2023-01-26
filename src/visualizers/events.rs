use druid::widget::prelude::*;
use druid::widget::Controller;
use druid::{Data, KeyEvent, MouseButton, MouseEvent};
// use std::sync::Arc;
use crate::visualizers::user_mode::datas::TraderUi;

/// the LoggedEvent struct is used to store the data of
/// a single event
///
/// **Federico Brancasi**
#[derive(Debug)]
pub(crate) struct EventLogger<F: Fn(&Event) -> bool> {
    pub(crate) filter: F,
}

/// impl block of the LoggedEvent struct
///
/// **Federico Brancasi**
impl<W: Widget<TraderUi>, F: Fn(&Event) -> bool> Controller<TraderUi, W> for EventLogger<F> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TraderUi,
        env: &Env,
    ) {
        // Every time this controller receives an event we check `f()`.
        // If `f()` returns true it means that we can add it to the log,
        // if not then we can skip it.
        if (self.filter)(event) {
            println!("Event logged: {:?}", event);
            if let Some(to_log) = LoggedEvent::try_from_event(event, data.events.len()) {
                // Arc::make_mut(&mut data.events).push(to_log);
                if to_log.typ == EventType::MouseDown {
                    if to_log.mouse_button() == "Right" {
                        data.events_number += 1;

                        if data.events_number == 2 {
                            if data.current_view == 0 {
                                data.current_view = 1;
                            } else if data.current_view == 1 {
                                data.current_view = 0;
                            }
                            data.events_number = 0;
                            println!("!!!current_view: {}", data.current_view);
                        }
                    }
                }
            }
        }
        // Always pass on the event!
        child.event(ctx, event, data, env)
    }
}


/// The types of events we display
///
/// **Federico Brancasi**
#[derive(Clone, Copy, Data, PartialEq, Debug)]
pub(crate) enum EventType {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    Wheel,
}

/// A type that represents any logged event shown in the list
///
/// **Federico Brancasi**
#[derive(Clone, Data, Debug)]
pub struct LoggedEvent {
    typ: EventType,
    number: usize,
    #[data(ignore)]
    mouse: Option<MouseEvent>,
    #[data(ignore)]
    _key: Option<KeyEvent>,
}

/// impl block of the LoggedEvent struct
///
/// **Federico Brancasi**
impl LoggedEvent {
    pub fn try_from_event(event: &Event, number: usize) -> Option<Self> {
        let to_log = match event {
            Event::MouseUp(mouse) => Some((EventType::MouseUp, Some(mouse.clone()), None)),
            Event::MouseDown(mouse) => Some((EventType::MouseDown, Some(mouse.clone()), None)),
            Event::Wheel(mouse) => Some((EventType::Wheel, Some(mouse.clone()), None)),
            Event::KeyUp(key) => Some((EventType::KeyUp, None, Some(key.clone()))),
            Event::KeyDown(key) => Some((EventType::KeyDown, None, Some(key.clone()))),
            _ => None,
        };

        to_log.map(|(typ, mouse, key)| LoggedEvent {
            typ,
            number,
            mouse: mouse,
            _key: key,
        })
    }

    fn mouse_button(&self) -> String {
        self.mouse
            .as_ref()
            .map(|m| {
                match m.button {
                    // MouseButton::Left => "Left",
                    MouseButton::Right => "Right",
                    // MouseButton::X1 => "X1",
                    // MouseButton::X2 => "X2",
                    // MouseButton::None => "",
                    // MouseButton::Middle => "Middle",
                    _ => "Unknown",
                }
                    .into()
            })
            .unwrap_or_default()
    }
}
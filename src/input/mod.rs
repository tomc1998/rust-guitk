use LibState;
use glium::glutin::{Event, TouchPhase};
use entity::EntityID;
use view::View;
use logger;

mod scroll;

const NUM_POINTS_TRACKED : usize = 128;

/// A position on screen touched by a finger
#[derive(Clone, Copy)]
pub struct TouchPoint (f64, f64);

/// Struct holds last few 'frames' of touch data for a given finger
pub struct FingerTrack {
  finger_id: u64,
  start_point: TouchPoint,
  /// Array of points, last NUM_POINTS_TRACKED sampled positions
  points: [TouchPoint; NUM_POINTS_TRACKED],
  /// Index into points which indicates the most recently updated point. This
  /// is so that every time we add a point to the list, we don't have to shift
  /// the whole vec back one.
  latest_point: usize,

  /// True if this finger has just been pressed
  just_down: bool,

  /// ID of the entity this finger is currently dragging
  curr_dragging: Option<EntityID>,

  /// Offset of the finger from the current dragging entity
  offset: Option<(f32, f32)>,
}

impl FingerTrack {
  fn new(view: &View, finger_id : u64, loc: (f64, f64)) -> FingerTrack {
    let mut entity_id = None;
    let mut offset = None;
    for layer in &view.layers {
      let result = scroll::is_on_entity_drag_trigger(layer, 
                                                    loc.0 as f32, loc.1 as f32);
      if result.is_some() {
        let result = result.unwrap();
        entity_id = Some(result.0);
        offset = Some(result.1);

        break;
      }
    }
    let mut track = FingerTrack {
      finger_id: finger_id,
      start_point: TouchPoint(loc.0, loc.1),
      points: [TouchPoint(0.0, 0.0); NUM_POINTS_TRACKED],
      latest_point: 0,
      just_down: true,
      curr_dragging: entity_id,
      offset: offset,
    };
    track.points[0] = TouchPoint(loc.0, loc.1);
    return track;
  }
}

pub struct InputState {
  fingers : Vec<FingerTrack>,
}

impl InputState {
  pub fn new() -> InputState {
    InputState {
      fingers: Vec::new(),
    }
  }
}

/// Function which polls and processes input. When an input event is received
/// from Glutin, this function processes the event then continues waiting for
/// another event.
pub fn process_input(lib_state: &mut LibState) {
  'Outer:
  for e in lib_state.display.poll_events() {
    match e {
      Event::Touch(touch) => {
        // Find the index of the finger
        let mut index : Option<usize> = None;
        for ii in 0..lib_state.input_state.fingers.len() {
          if lib_state.input_state.fingers[ii].finger_id == touch.id {
            index = Some(ii);
            break;
          }
        }
        // Parse touch event
        if index.is_none() && touch.phase == TouchPhase::Started {
          // New touch event!
          logger::log_default("New touch event");
          let curr_view = lib_state.view_stack.last();
          if curr_view.is_none() { continue 'Outer; }
          logger::log_default("Found view to pass it to");
          lib_state.input_state.fingers.push(
            FingerTrack::new(curr_view.unwrap(), touch.id, touch.location));
          continue;
        }
        else if index.is_some() {
          let index = index.unwrap();
          if touch.phase == TouchPhase::Moved {
            lib_state.input_state.fingers[index].latest_point += 1;
            if lib_state.input_state.fingers[index].latest_point 
              >= NUM_POINTS_TRACKED {
                lib_state.input_state.fingers[index].latest_point = 0;
              }
            lib_state.input_state.fingers[index]
              .points[lib_state.input_state.fingers[index].latest_point] 
              = TouchPoint(touch.location.0, touch.location.1);
          }
          else if touch.phase == TouchPhase::Ended ||
            touch.phase == TouchPhase::Cancelled {
              lib_state.input_state.fingers.remove(index);
              continue;
            }
        }
      }
      _ => continue,
    }
  }

  // Check there is a current view
  let mut has_scrolled = false;
  let curr_view = lib_state.view_stack.last_mut();
  if curr_view.is_some() { 
    // There is a current view! Process scrolling on this view!
    let curr_view = curr_view.unwrap();
    for layer in &mut curr_view.layers {
      has_scrolled = 
        if scroll::process_scroll(layer, &lib_state.input_state) {true} 
        else {has_scrolled};
    }
    if has_scrolled {
      curr_view.layout();
    }
  }
}

use LibState;
use glium::glutin::{Event, TouchPhase};
use logger;

const NUM_POINTS_TRACKED : usize = 128;

/// Struct holds last few 'frames' of touch data for a given finger
pub struct FingerTrack {
  finger_id: u64,
  points: [(f64, f64); NUM_POINTS_TRACKED],
  /// Index into points which indicates the most recently updated point. This
  /// is so that every time we add a point to the list, we don't have to shift
  /// the whole vec back one.
  latest_point: usize,
}

impl FingerTrack {
  fn new(finger_id : u64, loc: (f64, f64)) -> FingerTrack {
    let mut track = FingerTrack {
      finger_id: finger_id,
      points: [(0.0, 0.0); NUM_POINTS_TRACKED],
      latest_point: 0,
    };
    track.points[0] = loc;
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
          lib_state.input_state.fingers.push(
            FingerTrack::new(touch.id, touch.location));
          logger::log_default(&format!("New touch ID = {} at: {}, {}", 
                                       touch.id,
                                       touch.location.0, 
                                       touch.location.1));
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
              = touch.location;
          }
          else if touch.phase == TouchPhase::Ended ||
            touch.phase == TouchPhase::Cancelled {
              lib_state.input_state.fingers.remove(index);
              logger::log_default(&format!("Touch ID {} finished at: {}, {}", 
                                           touch.id,
                                           touch.location.0, 
                                           touch.location.1));
              continue;
            }
        }
      }
      _ => continue,
    }
  }
}

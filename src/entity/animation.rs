use entity::{EntityID, Component};

/// Enum for different tween functions
#[derive(Clone, Copy)]
pub enum TweenFunction {
  /// Instant translation, happens in 1 frame
  Instant,
  /// Linear movement
  Linear,
  /// Ease in (start slow, end fast)
  EaseIn,
  /// Ease out (start fast, end slow)
  EaseOut,
  /// Ease in and out (start slow, end slow)
  EaseInOut,
}

impl TweenFunction {
  /// Given the animation length and timers, returns a transformed timer value
  /// to reflect an instant translation.
  fn tween(&self, anim_len: u32, anim_timer: u32) -> u32 { 
    match *self {
      TweenFunction::Instant => anim_len,
      TweenFunction::Linear => anim_timer,
      _ => {
        unimplemented!();
      }
    }
  }
}

/// Component for a translation animation. Translate's the entity's AABB over a
/// period of time, given a tweening function (set of predefined functions).
#[derive(Clone)]
pub struct ComponentAnimTranslate {
  pub entity_id: EntityID,

  /// Starting X location. The entity will be instantly moved here once the
  /// animation component is added to the system.
  pub start_x: f32,
  /// Starting Y location. The entity will be instantly moved here once the
  /// animation component is added to the system.
  pub start_y: f32,

  /// Ending X location. The entity's AABB will end up here after anim_len
  /// millis.
  pub end_x: f32,
  /// Ending Y location. The entity's AABB will end up here after anim_len
  /// millis.
  pub end_y: f32,

  /// Animation length in milliseconds
  pub anim_len: u32,

  /// Millisecond timer. Counts up every frame, until it reaches anim_len.
  pub anim_timer: u32,

  /// Tween function
  pub tween_func: TweenFunction,
}
impl Component for ComponentAnimTranslate {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

impl ComponentAnimTranslate {
  /// Returns a tweened f32 value between 0 and 1.
  pub fn tween(&self) -> f32 {
    self.tween_func.tween(self.anim_len, self.anim_timer) as f32 / self.anim_len as f32
  }
}



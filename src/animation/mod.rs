use LibState;
use view::Layer;
use logger;

/// Increments all animationn timers for the given layer, and repeats the
/// process recursively for nested layers.
fn increment_animation_timers(frame_delta: u64, layer: &mut Layer) {
  use logger;
  for anim in &mut layer.component_anim_translate {
    anim.anim_timer += (frame_delta / 1000000) as u32;
  }
  // Recursively call function for nested layers
  for l in &mut layer.component_layer {
    increment_animation_timers(frame_delta, l);
  }
}

/// Process translation animations on a given layer, and all of its nested layers.
fn process_layer_translate_animations(frame_delta: u64, layer: &mut Layer) {
  for anim in &mut layer.component_anim_translate {
    let interp = anim.tween(); // Get interpolation value between 0 and 1
    logger::log_default(&format!("Interp: {}", interp));

    let aabb = layer.component_aabb.get_component_mut(anim.entity_id);
    if aabb.is_none() { continue; }
    let aabb = aabb.unwrap();
    logger::log_default(&format!("AABB X, Y: {}, {}", aabb.x, aabb.y));
    aabb.x = anim.start_x + (anim.end_x - anim.start_x) * interp;
    aabb.y = anim.start_y + (anim.end_y - anim.start_y) * interp;
  }

  // Recursively call function for nested layers
  for l in &mut layer.component_layer {
    process_layer_translate_animations(frame_delta, l);
  }
}

/// Process all animations
pub fn process_animations(lib_state: &mut LibState) {
  // Get top view
  let view = lib_state.view_stack.last_mut();
  if view.is_none() { return; }
  let view = view.unwrap();
  
  for layer in &mut view.layers {
    increment_animation_timers(lib_state.frame_delta, layer);
    process_layer_translate_animations(lib_state.frame_delta, layer);
  }
}

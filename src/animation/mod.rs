use LibState;
use view::Layer;

/// Increments all animationn timers for the given layer, and repeats the
/// process recursively for nested layers.
fn increment_animation_timers(frame_delta: u64, layer: &mut Layer) {
  for anim in &mut layer.component_anim_translate {
    anim.anim_timer += (frame_delta / 1000000) as u32;
    if anim.anim_timer > anim.anim_len {
      anim.anim_timer = anim.anim_len;
    }
  }
  // Recursively call function for nested layers
  for l in &mut layer.component_layer {
    increment_animation_timers(frame_delta, l);
  }
}

/// Process translation animations on a given layer, and all of its nested layers.
fn process_layer_translate_animations(frame_delta: u64, layer: &mut Layer) {
  // List of dead animation indexes
  let mut dead_anim = Vec::<usize>::new();
  for ii in 0..layer.component_anim_translate.len() {
    let anim = &mut layer.component_anim_translate[ii];
    let interp = anim.tween(); // Get interpolation value between 0 and 1

    let aabb = layer.component_aabb.get_component_mut(anim.entity_id);
    if aabb.is_none() { continue; }
    let aabb = aabb.unwrap();
    aabb.x = anim.start_x + (anim.end_x - anim.start_x) * interp;
    aabb.y = anim.start_y + (anim.end_y - anim.start_y) * interp;

    if anim.anim_timer >= anim.anim_len {
      aabb.x = anim.end_x;
      aabb.y = anim.end_y;
      dead_anim.push(ii);
    }
  }
  for dead in dead_anim {
    
    layer.component_anim_translate.remove(dead);
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

use view::Layer;
use input::InputState;
use entity::EntityID;
use entity::core::ComponentTrigger;
use logger;


/// Gets an entity's trigger box. Fails if entity does not have the right
/// components.
fn get_entity_trigger(layer: &Layer, 
                      e_id: EntityID) -> Option<(f32, f32, f32, f32)> {
  // Find trigger box position
  let t : Option<&ComponentTrigger> = layer.component_trigger.get_component(e_id);
  if t.is_none() { return None; }
  let t = t.unwrap();
  let (mut tx, mut ty, tw, th) = (t.x, t.y, t.w, t.h); // Trigger box
  if t.relative {
    let aabb = layer.component_aabb.get_component(e_id);
    if aabb.is_none() { return None; }
    let aabb = aabb.unwrap();
    tx += aabb.x;
    ty += aabb.y;
  }
  return Some((tx, ty, tw, th));
}

/// Tests if the given position is on an entity's drag trigger.
/// Returns the entity ID of the entity touched, and the offset of the touch.
pub fn is_on_entity_drag_trigger(layer: &Layer, 
                                 x: f32, y: f32) -> Option<(EntityID, (f32, f32))> {
  logger::log_default("Testing if entity is on a drag trigger...");
  for scroll in &layer.component_touch_scroll {
    let e_id = scroll.entity_id;
    logger::log_default("Found a scroll component");
    // Find trigger box position
    let trigger_box = get_entity_trigger(layer, e_id);
    if trigger_box.is_none() { continue; }
    logger::log_default("Found a drag trigger");
    let (tx, ty, tw, th) = trigger_box.unwrap();
    // Check if the x & y are inside the trigger AABB
    logger::log_default(&format!("AABB: {}, {}, {}, {} - X, Y: {}, {}",
                                 tx, ty, tw, th, x, y));
    if tx < x && tx + tw > x && ty < y && ty + th > y {
      logger::log_default("Is touching!");
      // Get AABB
      let aabb = layer.component_aabb.get_component(e_id);
      if aabb.is_none() { continue; }
      let aabb = aabb.unwrap();
      return Some((e_id, (x - aabb.x, y - aabb.y)));
    }
  }

  // Recursively look in nested layers
  for layer in &layer.component_layer {
    let res = is_on_entity_drag_trigger(layer, x, y);
    if res.is_some() { return res; }
  }

  return None;
}

/// Process scrolling. Returns true if scrolling has happened.
pub fn process_scroll(layer: &mut Layer, input_state: &InputState) -> bool {
  let mut has_scrolled = false;
  // Loop through current touches, is it dragging something?
  for touch in &input_state.fingers {
    let (curr_touch, start_touch, offset, e_id);
    {
      if touch.curr_dragging.is_none() || touch.offset.is_none() { continue; }
      // Check if touch is just pressed, if so, skip this touch
      if touch.start_point.0 == touch.points[touch.latest_point].0 && 
        touch.start_point.1 == touch.points[touch.latest_point].1 { continue; }

      e_id = touch.curr_dragging.unwrap();
      offset = touch.offset.unwrap();
      curr_touch = touch.points[touch.latest_point];
      start_touch = touch.start_point;
    }

    // Scroll the entity!
    // Get the scroll behaviour & max / min
    let scroll_behaviour;
    let bounds; // Coord bounds on scrolling
    {
      let scroll = layer.component_touch_scroll.get_component(e_id);
      if scroll.is_none() { continue; }
      let scroll = scroll.unwrap();
      scroll_behaviour = scroll.behaviour_flags;
      bounds = (scroll.min_x, scroll.max_x, scroll.min_y, scroll.max_y);
    }
    let (min_x, max_x, min_y, max_y) = bounds;

    let aabb = layer.component_aabb.get_component_mut(e_id);
    if aabb.is_none() { continue; }
    let aabb = aabb.unwrap();

    use entity::core::scroll_behaviour;
    // Check if scrolling is locked on an axis...
    if scroll_behaviour & scroll_behaviour::LOCKED_X == 0 {
      // Check if scrolling is inverted on X
      if scroll_behaviour & scroll_behaviour::INVERTED_X > 0 {
        aabb.x = start_touch.0 as f32 - offset.0 -
          (curr_touch.0 - start_touch.0) as f32;
      }
      else {
        aabb.x = start_touch.0 as f32 - offset.0 + 
          (curr_touch.0 - start_touch.0) as f32; }
      // Now lock AABB scrolling to the max / min values in scroll
      if aabb.x <= min_x { aabb.x = min_x; }
      else if aabb.x + aabb.w >= max_x { aabb.x = max_x - aabb.w; }
    }

    if scroll_behaviour & scroll_behaviour::LOCKED_Y == 0 {
      // Check if scrolling is inverted on Y
      if scroll_behaviour & scroll_behaviour::INVERTED_Y > 0 {
        aabb.y = start_touch.1 as f32 - offset.1 -
          (curr_touch.1 - start_touch.1) as f32;
      }
      else {
        aabb.y = start_touch.1 as f32 - offset.1 + 
          (curr_touch.1 - start_touch.1) as f32; }
      if aabb.y <= min_y { aabb.y = min_y; }
      else if aabb.y + aabb.h >= max_y { aabb.y = max_y - aabb.h; }
    }

    has_scrolled = true;
  }

  // Try nested layers
  for l in &mut layer.component_layer {
    has_scrolled = if process_scroll(l, input_state) {true} else {has_scrolled};
  }

  return has_scrolled;
}

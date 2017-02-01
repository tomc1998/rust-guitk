use layout::Layout;
use entity::{EntityID, Component};
use common::color::RGBf32;

/// Namespace to contain constant bitmasks for ComponentTouchScroll::behaviour_flags.
pub mod scroll_behaviour {
  /// Controls whether this entity will 'stick' to its position after the user
  /// raises their finger, or whether it will glide smoothly in the direction
  /// and speed the user left the screen with their finger.
  pub const STICKY: u32     = 0x00000001;

  /// Controls whether the entity can scroll on the X axis
  pub const LOCKED_X:   u32 = 0x00000002;
  /// Controls X scroll direction (default entity scrolls left when user drags left)
  pub const INVERTED_X: u32 = 0x00000004;

  /// Controls whether the entity can scroll on the Y axis
  pub const LOCKED_Y:   u32 = 0x00000008;
  /// Controls Y scroll direction (default entity scrolls up when user drags up)
  pub const INVERTED_Y: u32 = 0x00000010;
}

/// Position component for entity. Defines a 2D view position and size.
#[derive(Clone)]
pub struct ComponentAABB {
  pub entity_id: EntityID,
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}
impl Component for ComponentAABB {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

/// Draws a rectangle at this entity's position over everything else. 50% opacity.
/// Dependencies: 
/// ComponentAABB
#[derive(Clone)]
pub struct ComponentDebugDraw {
  pub entity_id: EntityID,
  pub color:    RGBf32,
}
impl Component for ComponentDebugDraw {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

/// Container component. An entity with this will contain other entities.
#[derive(Clone)]
pub struct ComponentContainer {
  pub entity_id: EntityID,
  /// The layout of this container. A container can only be 1 layout at one time.
  pub layout: Layout,
}
impl Component for ComponentContainer {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}

/// Defines a trigger zone AABB. x and y are given relative to the entity's
/// current AABB.
/// Dependencies: 
/// ComponentAABB
#[derive(Clone)]
pub struct ComponentTrigger {
  pub entity_id: EntityID,

  /// ID of this trigger zone. One entity could have multiple trigger zones for
  /// different behaviour (IE a button you can move around by dragging the
  /// edge, but click by pressing the centre)
  pub trigger_id: u8,

  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
  /// Whether or not the x and y values of this component should be offset by
  /// the current position or not. (For stuff like drag-out menus, where you drag
  /// on the edge of the menu to move it.
  pub relative: bool,
}
impl Component for ComponentTrigger {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}


/// Component that lets the user scroll (move) the component with their finger.
/// Dependencies: 
/// ComponentTrigger
/// ComponentAABB
#[derive(Clone)]
pub struct ComponentTouchScroll {
  pub entity_id: EntityID,
  /// The ID of the trigger component used (Entity will only scroll if the user
  /// is scrolling the trigger zone).
  pub trigger_id: u8,

  /// See scroll_behaviour.
  pub behaviour_flags: u32,

  /// Bounds on scrolling. Make sure the entity size is smaller than these
  /// bounds as a rect! Max x and max y will be checked based on aabb.x +
  /// aabb.w.
  pub max_x: f32,
  pub min_x: f32,
  pub max_y: f32,
  pub min_y: f32,
}
impl Component for ComponentTouchScroll {
  fn get_entity_id(&self) -> EntityID { self.entity_id }
}


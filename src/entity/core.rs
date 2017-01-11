use layout::Layout;
use entity::{EntityID, Component};
use common::color::RGBf32;

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


use entity::EntityID;
use common::color::RGBf32;

/// Position component for entity. Defines a 2D relative view position and size.
pub struct ComponentAABB {
  pub entity_id: EntityID,
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

/// Draws a rectangle at this entity's position over everything else. 50% opacity.
/// Dependencies: 
/// ComponentPosition
pub struct ComponentDebugDraw {
  pub entity_id: EntityID,
  pub color:    RGBf32,
}

/// Container component. An entity with this will contain other entities.
pub struct ComponentContainer {
  pub children: Vec<EntityID>,
}


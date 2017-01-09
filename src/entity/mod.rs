/// Mod containing core components for entities.
/// Components will have their 'dependencies' listed in the documentation. If
/// their dependencies aren't found as components connected to the same entity,
/// then the engine will panic at runtime.
use std::ops::AddAssign;
use std::cmp::PartialEq;
use layout::Layout;

pub mod core;

#[derive(Copy, Clone, PartialOrd)]
pub struct EntityID (pub u16);

impl AddAssign for EntityID {
  fn add_assign(&mut self, other: EntityID) {
    self.0 += other.0;
  }
}

impl PartialEq for EntityID {
  fn eq(&self, other: &EntityID) -> bool {
    self.0 == other.0
  }
}

pub trait Component {
  fn get_entity_id(&self) -> EntityID;
}

impl EntityID {
  /// Sets the layout of this entity. Any current layout (and subsequently,
  /// children) is/are erased.
  /// @param layout The layout to use.
  pub fn set_layout(layout: Layout) {
    // Find this entity's layout (if it exists)
  }
}


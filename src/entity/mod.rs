/// Mod containing core components for entities.
/// Components will have their 'dependencies' listed in the documentation. If
/// their dependencies aren't found as components connected to the same entity,
/// then the engine will panic at runtime.
use std::ops::AddAssign;
use std::cmp::PartialEq;

pub mod core;

#[derive(Copy, Clone)]
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


/// Mod containing core components for entities.
/// Components will have their 'dependencies' listed in the documentation. If
/// their dependencies aren't found as components connected to the same entity,
/// then the engine will panic at runtime.
use std::ops::AddAssign;
use std::cmp::PartialEq;
use layout::Layout;
use view::View;

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

impl EntityID {
  /// Sets the layout of this entity. Any current layout (and subsequently,
  /// children) is/are erased.
  pub fn set_layout(&self, view: &mut View, layout: Layout) {
    // Find this entity's layout (if it exists)
    let comp_container_list = &mut view.component_container;
    {
      // Check if it exists
      let comp_container = comp_container_list.get_component_mut(*self);
      if comp_container.is_some() { // Already exists, just change it
        comp_container.unwrap().layout = layout;
        return;
      }
    }
    // Doesn't exist...
    // Need to create a new container component, then add it
    comp_container_list.add_component(
      core::ComponentContainer {
        entity_id: *self,
        layout: layout,
      });
  }
}

pub trait Component {
  fn get_entity_id(&self) -> EntityID;
}


use entity::core::{ComponentContainer, ComponentAABB, ComponentDebugDraw};
use entity::{Component, EntityID};
use std::slice;

/// Trait which defines a 'ViewListener', a listener who is called when a view
/// is added or removed from the view stack.
pub trait ViewListener {
  /// Called when the view is added to the view stack
  fn on_show(&self, view : &View);
  /// Called when the view is removed to the view stack
  fn on_hide(&self, view : &View);
}

/// Struct to represent a list of components. Internally, components are stored
/// in an ordered list (by entty ID) to allow binary searching.
pub struct ComponentList<T : Component> {
  list : Vec<T>,
}

impl<T : Component> ComponentList<T> {
  pub fn new() -> ComponentList<T> {
    ComponentList {
      list: Vec::new(),
    }
  }

  /// Add a component to the list. If a component with the same entity ID
  /// already exists, replace it. O(log(n)).
  /// @param component The component to add to the list.
  pub fn add_component(&mut self, component : T) {
    if self.list.len() == 0 {
      self.list.push(component);
      return;
    }
    let entity_id = component.get_entity_id();
    let (mut l_bound, mut u_bound) = (0, self.list.len() - 1);
    let mut target;
    loop {
      if u_bound - l_bound <= 1 {
        if u_bound == self.list.len()-1 && 
          self.list[u_bound].get_entity_id() < entity_id { // End of the list?
            self.list.push(component);
          }
        else if l_bound == 0 &&
          self.list[l_bound].get_entity_id() > entity_id { // Start of list?
            self.list.insert(0, component);
          }
        else { // In between u_bound and l_bound
          self.list.insert(u_bound, component);
        }
        break;
      }
      target = (u_bound + l_bound) / 2;
      if self.list[target].get_entity_id() > entity_id {
        u_bound = target;
      }
      else if self.list[target].get_entity_id() < entity_id {
        l_bound = target;
      }
      else { // Same entity ID, replace the component at this index
        self.list[target] = component;
        return;
      }
    }
  }

  /// Binary searches for the component belonging to the entity ID given.
  /// @param entity_id The ID of the entity who owns the component to look for.
  pub fn get_component(&self, entity_id: EntityID) -> Option<&T> {
    if self.list.len() == 0 {
      return None;
    }
    let (mut l_bound, mut u_bound) = (0, self.list.len() - 1);
    let mut target;
    loop {
      // Check if we've searched everything...
      if u_bound - l_bound <= 1 
          && self.list[u_bound].get_entity_id() != entity_id 
          && self.list[l_bound].get_entity_id() != entity_id {
        return None;
      }

      target = (u_bound + l_bound) / 2;
      if self.list[target].get_entity_id() == entity_id {
        return Some(&self.list[target]);
      }
      if self.list[target].get_entity_id() > entity_id {
        u_bound = target;
      }
      else {
        l_bound = target + 1;
      }
    }
  }

  pub fn len(&self) -> usize {
    self.list.len()
  }
}

/// Implement the IntoIterator for ComponentList, lets us iterate over the list
/// of components.
impl<'a, T : Component> IntoIterator for &'a ComponentList<T> {
  type Item = &'a T;
  type IntoIter = slice::Iter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
    self.list.iter()
  }
}

/// Structure which contains the data for a view. It is essentially an ECS.
pub struct View<'a> {
  pub view_listeners : Vec<&'a ViewListener>,
  pub component_debug_draw : ComponentList<ComponentDebugDraw>,
  pub component_container : ComponentList<ComponentContainer>,
  pub component_aabb : ComponentList<ComponentAABB>,
}

impl<'a> View<'a> {
  pub fn new() -> View<'a> {
    View {
      view_listeners : Vec::new(),
      component_debug_draw : ComponentList::new(),
      component_container : ComponentList::new(),
      component_aabb : ComponentList::new(),
    }
  }
}

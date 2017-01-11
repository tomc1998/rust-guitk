use entity::{Component, EntityID};
use std::slice;
use std::ops::{Index, IndexMut};

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

  pub fn get(&self, index : usize) -> &T {
    &self.list[index]
  }

  /// Searches for the index of the component in the list belonging to the
  /// entity ID given. Used by get_component and get_component_mut.
  /// @param entity_id The ID of the entity who owns the component to look for.
  /// @return The index of the component, or None if not found.
  pub fn get_component_index(&self, entity_id: EntityID) -> Option<usize> {
    if self.list.len() == 0 {
      return None;
    }
    let (mut l_bound, mut u_bound) = (0, self.list.len() - 1);
    let mut target : usize;
    loop {
      // Check if we've searched everything...
      if u_bound - l_bound <= 1 
          && self.list[u_bound].get_entity_id() != entity_id 
          && self.list[l_bound].get_entity_id() != entity_id {
        return None;
      }

      target = (u_bound + l_bound) / 2;
      if self.list[target].get_entity_id() == entity_id {
        return Some(target);
      }
      if self.list[target].get_entity_id() > entity_id {
        u_bound = target;
      }
      else {
        l_bound = target + 1;
      }
    }
  }

  /// Binary searches for the component belonging to the entity ID given.
  /// @param entity_id The ID of the entity who owns the component to look for.
  pub fn get_component(&self, entity_id: EntityID) -> Option<&T> {
    let index = self.get_component_index(entity_id);
    if index.is_none() { return None; }
    else {
      return Some(&self.list[index.unwrap()]);
    }
  }

  /// Binary searches for the component belonging to the entity ID given.
  /// @param entity_id The ID of the entity who owns the component to look for.
  pub fn get_component_mut(&mut self, entity_id: EntityID) -> Option<&mut T> {
    let index = self.get_component_index(entity_id);
    if index.is_none() { return None; }
    else {
      return Some(&mut self.list[index.unwrap()]);
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

impl<T : Component> Index<usize> for ComponentList<T> {
  type Output = T;
  fn index(&self, index: usize) -> &T {
    &self.list[index]
  }
}
impl<T : Component> IndexMut<usize> for ComponentList<T> {
  fn index_mut(&mut self, index: usize) -> &mut T {
    &mut self.list[index]
  }
}




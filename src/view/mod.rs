pub mod component_list;

use entity::core::{ComponentContainer, ComponentAABB, ComponentDebugDraw};
use view::component_list::ComponentList;
use layout::manager;

/// Trait which defines a 'ViewListener', a listener who is called when a view
/// is added or removed from the view stack.
pub trait ViewListener {
  /// Called when the view is added to the view stack
  fn on_show(&self, view : &View);
  /// Called when the view is removed to the view stack
  fn on_hide(&self, view : &View);
}
/// Structure which contains the data for a view. It is essentially an ECS.
pub struct View<'a> {
  pub view_listeners : Vec<&'a ViewListener>,
  pub component_debug_draw : ComponentList<ComponentDebugDraw>,
  pub component_aabb : ComponentList<ComponentAABB>,
  pub component_container : ComponentList<ComponentContainer>,
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

  /// Run the layout manager on this view. Alters AABB components based on
  /// Container component hierarchies. If the hierarchy is malforms (component
  /// with multiple parents / circular hierarchy) then this thread will panic.
  pub fn layout(&mut self) {
    manager::layout(self);
  }
}

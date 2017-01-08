use entity::core::{ComponentContainer, ComponentAABB, ComponentDebugDraw};

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
  pub component_debug_draw : Vec<ComponentDebugDraw>,
  pub component_container : Vec<ComponentContainer>,
  pub component_aabb : Vec<ComponentAABB>,
}

impl<'a> View<'a> {
  pub fn new() -> View<'a> {
    View {
      view_listeners : Vec::new(),
      component_debug_draw : Vec::new(),
      component_container : Vec::new(),
      component_aabb : Vec::new(),
    }
  }
}

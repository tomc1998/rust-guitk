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

/// Structure which contains the data for a view. It is essentially an ECS, with clipping
/// information.
pub struct Layer {
  pub component_debug_draw : ComponentList<ComponentDebugDraw>,
  pub component_aabb : ComponentList<ComponentAABB>,
  pub component_container : ComponentList<ComponentContainer>,
}

impl Layer {
  pub fn new() -> Layer {
    Layer {
      component_debug_draw : ComponentList::new(),
      component_container : ComponentList::new(),
      component_aabb : ComponentList::new(),
    }
  }
}

/// Structure which contains the data for a view. Contains a list of layers, and view listeners,
/// which track events occuring to the view (hiding, showing etc).
pub struct View<'a> {
  pub view_listeners : Vec<&'a ViewListener>,
  pub layers : Vec<Layer>,
}

impl<'a> View<'a> {
  pub fn new() -> View<'a> {
    View {
      view_listeners : Vec::new(),
      layers: Vec::new(),
    }
  }

  /// Run the layout manager on this view. Alters AABB components based on
  /// Container component hierarchies. If the hierarchy is malforms (component
  /// with multiple parents / circular hierarchy) then this thread will panic.
  pub fn layout(&mut self) {
    for layer in &mut self.layers {
      manager::layout_layer(layer);
    }
  }
}

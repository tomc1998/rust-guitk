pub mod component_list;

use entity::core::*;
use entity::animation::*;
use entity::{Component, EntityID};
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
  pub component_trigger : ComponentList<ComponentTrigger>,
  pub component_touch_scroll : ComponentList<ComponentTouchScroll>,
  pub component_anim_translate : ComponentList<ComponentAnimTranslate>,
  pub component_layer : ComponentList<Layer>,
  /// Optional entity ID association. If this layer has an AABB associated with
  /// it (for GL scissor clipping), then this ID will be Some. Otherwise, None.
  pub entity_id : Option<EntityID>,
}

impl Component for Layer {
  fn get_entity_id(&self) -> EntityID {
    if self.entity_id.is_none() {
      EntityID(u16::max_value())
    }
    else {
      self.entity_id.unwrap()
    }
  }
}

impl Layer {
  pub fn new() -> Layer {
    Layer {
      component_debug_draw : ComponentList::new(),
      component_container : ComponentList::new(),
      component_aabb : ComponentList::new(),
      component_trigger : ComponentList::new(),
      component_touch_scroll : ComponentList::new(),
      component_anim_translate : ComponentList::new(),
      component_layer: ComponentList::new(),
      entity_id: None,
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

use view::Layer;
use entity::core::{ComponentContainer};
use layout::Layout;

pub fn layout(layer: &mut Layer, component: &ComponentContainer) {
  match component.layout {
    Layout::HeaderBar {entity_header, entity_body, header_height} => {
      // Find this component's AABB
      let layer_component_aabb = &mut layer.component_aabb;
      let (c_x, c_y, c_w, c_h);
      {
        let c_aabb 
          = layer_component_aabb.get_component(component.entity_id);
        if c_aabb.is_none() { return; }
        let c_aabb = c_aabb.unwrap();
        c_x = c_aabb.x;
        c_y = c_aabb.y;
        c_w = c_aabb.w;
        c_h = c_aabb.h;
      }

      {
        // Find header bar entity ID, and find the AABB component in the layer.
        let header_bar_id = entity_header;
        let header_bar_aabb = layer_component_aabb.get_component_mut(header_bar_id);
        if header_bar_aabb.is_none() { return; }
        let header_bar_aabb = header_bar_aabb.unwrap();
        // Set header AABB component values
        header_bar_aabb.x = c_x;
        header_bar_aabb.y = c_y;
        header_bar_aabb.w = c_w;
        header_bar_aabb.h = header_height;
      }

      {
        // Find the body entity ID, and the AABB component for it.
        let body_id = entity_body;
        let body_aabb = layer_component_aabb.get_component_mut(body_id);
        if body_aabb.is_none() { return; }
        let body_aabb = body_aabb.unwrap();
        // Set header AABB component values
        body_aabb.x = c_x;
        body_aabb.y = c_y + header_height;
        body_aabb.w = c_w;
        body_aabb.h = c_h - header_height;
      }
    }, 
    _ => return
  }
}


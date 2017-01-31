use view::Layer;
use entity::core::ComponentContainer;
use layout::Layout;

pub fn layout(layer: &mut Layer, component: &ComponentContainer) {
  match component.layout {
    Layout::VSplit {entity_l, entity_r, split_pos} => {
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
        // Find left side entity AABB
        let l_id = entity_l;
        let l_aabb = layer_component_aabb.get_component_mut(l_id);
        if l_aabb.is_none() { return; }
        let l_aabb = l_aabb.unwrap();
        // Set left side AABB component values
        l_aabb.x = c_x;
        l_aabb.y = c_y;
        l_aabb.w = split_pos;
        l_aabb.h = c_h;
      }

      {
        // Find left side entity AABB
        let r_id = entity_r;
        let r_aabb = layer_component_aabb.get_component_mut(r_id);
        if r_aabb.is_none() { return; }
        let r_aabb = r_aabb.unwrap();
        // Set left side AABB component values
        r_aabb.x = c_x + split_pos;
        r_aabb.y = c_y;
        r_aabb.w = c_w - split_pos;
        r_aabb.h = c_h;
      }
    },
    _ => return
  }
}

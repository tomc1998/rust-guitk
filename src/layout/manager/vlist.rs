use view::Layer;
use layout::Layout;
use entity::core::ComponentContainer;

pub fn layout(layer: &mut Layer, component: &ComponentContainer) {
  match component.layout {
    Layout::VList {ref entity_list, item_height} => {
      // Find container aabb
      let c_aabb_tuple;
      {
        let cont_aabb = layer.component_aabb.get_component(component.entity_id);
        if cont_aabb.is_none() { return; }
        let cont_aabb = cont_aabb.unwrap();
        c_aabb_tuple = (cont_aabb.x, cont_aabb.y, cont_aabb.w);
      }
      let (c_x, c_y, c_w) = c_aabb_tuple;

      let mut curr_y = c_y;

      for item in entity_list {
        // Find AABB
        let aabb = layer.component_aabb.get_component_mut(*item);
        if aabb.is_none() { continue; }
        let aabb = aabb.unwrap();

        // Set AABB
        aabb.x = c_x;
        aabb.y = curr_y;
        aabb.w = c_w;
        aabb.h = item_height;
        curr_y += item_height;
      }
    },
    _ => unimplemented!()
  }
}


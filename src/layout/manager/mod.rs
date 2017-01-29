use view::Layer;
use entity::core::ComponentContainer;
use layout::Layout;
use logger;

/// Module is used to manipulate container entities as if they were in a tree.
/// Contains functions to create the tree, and check whether the list of
/// containers in a view are in a malformed hierarchy.
mod entity_tree;

mod header_bar;
mod vsplit;

/// Layout a view layer.
pub fn layout_layer(layer : &mut Layer) {
  // Check that the hierarchy is not malformed
  let tree = entity_tree::EntityTree::new_from_layer(layer);
  if tree.is_none() {
    logger::log_default("View is malformed");
    return;
  }
  logger::log_default("View is not malformed.");
  let tree = tree.unwrap();
  logger::log_default(&format!("There are {} entities in the container tree.", 
                              tree.len()));
  let roots = tree.get_roots();
  logger::log_default(&format!("There are {} root entities in the container tree.", 
                              roots.len()));
  let mut node_queue = Vec::with_capacity(tree.len());
  let mut new_nodes = Vec::new();
  for root in roots {
    node_queue.push(root);
  }
  while !node_queue.is_empty() {
    for node in &node_queue {
      let component;
      {
        let component_opt
          = layer.component_container.get_component(tree[*node].value);
        if component_opt.is_none() { continue; }
        component = component_opt.unwrap().clone();
      }
      layout_component(layer, component);
      for child in &tree[*node].children {
        new_nodes.push(child);
      }
    }
    node_queue.clear();
    for new in &new_nodes {
      node_queue.push(**new);
    }
    new_nodes.clear();
  }
}

fn layout_component(layer: &mut Layer, component: ComponentContainer) {
  match component.layout {
    Layout::HeaderBar {entity_header:_, entity_body:_, header_height:_} => 
      header_bar::layout(layer, &component),
    Layout::VSplit {entity_l:_, entity_r:_, split_pos:_} => 
      vsplit::layout(layer, &component),
  }
}


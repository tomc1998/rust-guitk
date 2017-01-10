use entity::EntityID;
use view::View;

/// Tree structure, used for parent - child hierarchy validation.
/// All nodes in a tree will be stored in a list, and the nodes will use
/// indexes to refer to one another. This means that they can be stored
/// contiguously, removing as much heap fragmentation as possible, as the
/// indexes can be used to reference the nodes regardless of where the vector's
/// contents are in memory.
pub struct EntityTreeNode {
  /// Parent node, index into array
  parent: Option<usize>,
  /// Child nodes, indexes into array
  children: Vec<usize>,
  value: EntityID,
}

impl EntityTreeNode {
  /// Create a new EntityTreeNode with no children.
  fn new(entity_id: EntityID, 
         parent: Option<usize>) -> EntityTreeNode {
    EntityTreeNode {
      parent: parent,
      children: Vec::new(),
      value: entity_id,
    }
  }
}

/// Function takes a node and a list of remaining candidates for children,
/// and recursively adds them to the tree, setting the children's value to
/// None in the given vector once complete.
/// If no children candidates are found, this method will return false.
/// # Arguments
/// - node The node to fill with children
/// - node_children A list of the node's children's entity IDs
/// - containers A list of containers still left to fill children
/// - view The view that node is part of
/// - node_space A vector to store all the EntityTreeNodes
fn fill_children(node_index: usize,
                 node_children: Vec<EntityID>,
                 containers: &mut Vec<Option<EntityID>>,
                 view: & View,
                 node_space: &mut Vec<EntityTreeNode>) -> bool {
  // Loop through all the children's entity IDs of this node
  for child in node_children {
    // Find child container index in the view.
    let index = view.component_container
      .get_component_index(child);
    if index.is_none() { continue; } // Must be a non-container, just ignore it.
    let index = index.unwrap();
    // Check if it already has a parent (i.e it's None in the containers list)
    if containers[index].is_none() { return false; }

    // Everything is fine... Add a new node to the space
    let new_node_index = node_space.len();
    node_space.push(EntityTreeNode::new(containers[index].unwrap(), None));
    // Link the two parent and child nodes together
    node_space[node_index].children.push(new_node_index);
    node_space[new_node_index].parent = Some(node_index);
    // Make sure we set this to None, indicating the node has a parent already
    // and shouldn't be parented again in future calls
    containers[index] = None;
    // Find this child node's children, and repeat the process by using a
    // recursive call (i.e this function fills the children depth first)
    let new_node_children = view.component_container.get(index)
      .layout.get_children();
    // Fill the child node's children
    fill_children(new_node_index, new_node_children, 
                  containers, view, node_space);
  }
  // All done, didn't exit badly, return true.
  return true;
}

/// Looks at the view's component_container list, and checks whether or not the
/// hierarchy is malformed (is it circular, do some children have more than 1
/// parent?)
pub fn is_hierarchy_malformed(view: &View) -> Option<Vec<EntityTreeNode>> {
  // List of root nodes in the container hierarchy
  let mut nodes = Vec::<EntityTreeNode>::new();
  'outer: for c in &view.component_container {
    for c2 in &view.component_container {
      let children = c2.layout.get_children();
      for child in children {
        if child == c.entity_id {
          continue 'outer;
        }
      }
    }
    nodes.push(EntityTreeNode::new(c.entity_id, None));
  }
  let mut containers = Vec::<Option<EntityID>>::with_capacity(
    view.component_container.len());
  for c in &view.component_container {
    containers.push(Some(c.entity_id));
  }
  for ii in 0..nodes.len() {
    let children = view.component_container
      .get_component(nodes[ii].value).unwrap().layout.get_children();
    let result = fill_children(ii, children, &mut containers, view, &mut nodes);
    if !result {
      return None;
    }
  }

  return Some(nodes);
}


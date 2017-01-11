use entity::EntityID;
use view::View;
use std::ops::{Index, IndexMut};
use std::slice;

/// Newtype for a list of tree nodes.
pub struct EntityTree (Vec<EntityTreeNode>);

impl EntityTree {
  fn new() -> EntityTree {
    EntityTree (Vec::new())
  }

  /// Looks at the view's component_container list, and checks whether or not the
  /// hierarchy is malformed (is it circular, do some children have more than 1
  /// parent?)
  /// - If malformed, then returns None.
  /// - If not malformed, then creates a new EntityTree from the view's container
  /// list.
  pub fn new_from_view(view: &View) -> Option<EntityTree> {
    // List of root nodes in the container hierarchy
    let mut tree = EntityTree::new();
    'outer: for c in &view.component_container {
      for c2 in &view.component_container {
        let children = c2.layout.get_children();
        for child in children {
          if child == c.entity_id {
            continue 'outer;
          }
        }
      }
      tree.push(EntityTreeNode::new(c.entity_id, None));
    }
    let mut containers = Vec::<Option<EntityID>>::with_capacity(
      view.component_container.len());
    for c in &view.component_container {
      containers.push(Some(c.entity_id));
    }
    for ii in 0..tree.len() {
      let children = view.component_container
        .get_component(tree[ii].value).unwrap().layout.get_children();
      let result = tree.fill_children(ii, children, &mut containers, view);
      if !result {
        return None;
      }
    }

    return Some(tree);
  }

  /// Function to get a list of the root nodes in this tree. 
  /// # Returns
  /// A list of indexes into the tree.
  pub fn get_roots(&self) -> Vec<usize> {
    let mut roots = Vec::<usize>::new();
    'outer: for ii in 0..self.len() {
      for jj in 0..self.len() {
        if ii == jj { continue; }
        let children = &self[jj].children;
        for child in children {
          if *child == ii {
            continue 'outer;
          }
        }
      }
      roots.push(ii);
    }
    return roots;
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
  fn fill_children(&mut self,
                   node_index: usize,
                   node_children: Vec<EntityID>,
                   containers: &mut Vec<Option<EntityID>>,
                   view: & View) -> bool {
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
      let new_node_index = self.len();
      self.push(EntityTreeNode::new(containers[index].unwrap(), None));
      // Link the two parent and child nodes together
      self[node_index].children.push(new_node_index);
      self[new_node_index].parent = Some(node_index);
      // Make sure we set this to None, indicating the node has a parent already
      // and shouldn't be parented again in future calls
      containers[index] = None;
      // Find this child node's children, and repeat the process by using a
      // recursive call (i.e this function fills the children depth first)
      let new_node_children = view.component_container.get(index)
        .layout.get_children();
      // Fill the child node's children
      self.fill_children(new_node_index, new_node_children, 
                    containers, view);
    }
    // All done, didn't exit badly, return true.
    return true;
  }

  fn push(&mut self, node: EntityTreeNode) { self.0.push(node) }
  pub fn len(&self) -> usize{ self.0.len() }
}

impl Index<usize> for EntityTree {
  type Output = EntityTreeNode;
  fn index(&self, index: usize) -> &EntityTreeNode {
    &self.0[index]
  }
}
impl IndexMut<usize> for EntityTree {
  fn index_mut(&mut self, index: usize) -> &mut EntityTreeNode {
    &mut self.0[index]
  }
}

impl<'a> IntoIterator for &'a EntityTree {
  type Item = &'a EntityTreeNode;
  type IntoIter = slice::Iter<'a, EntityTreeNode>;
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

/// Tree structure, used for parent - child hierarchy validation.
/// All nodes in a tree will be stored in a list, and the nodes will use
/// indexes to refer to one another. This means that they can be stored
/// contiguously, removing as much heap fragmentation as possible, as the
/// indexes can be used to reference the nodes regardless of where the vector's
/// contents are in memory.
pub struct EntityTreeNode {
  /// Parent node, index into array
  pub parent: Option<usize>,
  /// Child nodes, indexes into array
  pub children: Vec<usize>,
  pub value: EntityID,
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



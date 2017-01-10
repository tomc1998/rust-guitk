use view::View;
use logger;

/// Module is used to manipulate container entities as if they were in a tree.
/// Contains functions to create the tree, and check whether the list of
/// containers in a view are in a malformed hierarchy.
mod entity_tree;

/// Layout a view.
pub fn layout(view : &mut View) {
  // Check that the hierarchy is not malformed
  let tree = entity_tree::is_hierarchy_malformed(view);
  if tree.is_none() {
    logger::log_default("View is malformed");
    return;
  }
  logger::log_default("View is not malformed.");
  let tree = tree.unwrap();
  logger::log_default(&format!("There are {} entities in the container tree.", 
                              tree.len()));
}

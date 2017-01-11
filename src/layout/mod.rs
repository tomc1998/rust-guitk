/// Layout manager module. Handles dynamically resizing entities based on the
/// child / parent hierarchy, and layout rules defined by the container's
/// 'Layout'.
pub mod manager;

use entity::EntityID;

/// Enum which lists different types of layouts. The layout variants contain
/// data about the child entity IDs and where the entities are positioned in
/// the layout. 
/// In order for an entity to be a child or parent, it must have
/// an AABB component. Any parent / child not conforming to this will not be
/// layed out.
#[derive(Clone)]
pub enum Layout {
  /// Header bar layout. Layout specified a header bar at the top, at a certain
  /// given height.
  HeaderBar {
    /// The EntityID of the header bar
    entity_header : EntityID,
    /// The EntityID of the body (rest of the page)
    entity_body : EntityID,
    /// Height of the header in pixels
    header_height: f32,
  },
  /// Vertical split layout, (as in vim). A line drawn down the split would be
  /// vertical.
  VSplit {
    /// Entity on the left
    entity_l : EntityID,
    /// Entity on the right
    entity_r : EntityID,
    /// Relative position of the split in pixels. This will be the size of
    /// entity_l.
    split_pos : f32,
  },
}

impl Layout {
  /// Function which will get the children from an enum variant.
  fn get_children(&self) -> Vec<EntityID> {
    match *self {
      // Header bar
      Layout::HeaderBar {entity_header, entity_body, header_height: _} => 
        vec![entity_header, entity_body],
      // Vertical split
      Layout::VSplit {entity_l, entity_r, split_pos: _} => 
        vec![entity_l, entity_r],
      //_ => {
      //  // If we get here, then we haven't implemented get_children for all the
      //  // layout types yet.
      //  unimplemented!();
      //}
    }
  }
}

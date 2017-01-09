use entity::EntityID;

/// Enum which lists different types of layouts. The layouts contain data
/// on where entity IDs will go.
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
}

impl Layout {
  /// Function which will get the children from an enum variant.
  fn get_children(&self) -> Vec<EntityID> {
    match *self {
      // Header bar
      Layout::HeaderBar {entity_header, entity_body, header_height: _} => 
        vec![entity_header, entity_body],
      //_ => {
      //  // If we get here, then we haven't implemented get_children for all the
      //  // layout types yet.
      //  unimplemented!();
      //}
    }
  }
}

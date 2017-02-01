/// Get the distance between 2 points
pub fn distance (p0: (f32, f32), p1: (f32, f32)) -> f32 {
  ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt()
}

/// Get the square distance between 2 points
pub fn sq_distance (p0: (f32, f32), p1: (f32, f32)) -> f32 {
  (p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)
}

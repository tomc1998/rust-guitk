#[derive(Clone)]
pub struct RGBf32 {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl RGBf32 {
  pub fn new(r: f32, g: f32, b: f32) -> RGBf32 {
    RGBf32 {
      r: r,
      g: g,
      b: b,
    }
  }
}

#[derive(Clone)]
pub struct RGBAf32 {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

pub struct RGBf32 {
  r: f32,
  g: f32,
  b: f32,
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

pub struct RGBAf32 {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

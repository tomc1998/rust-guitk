use glium;
use glium::Surface;
use glium::uniforms::{UniformsStorage, EmptyUniforms};

use view::Layer;
use LibState;

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

pub struct Renderer<'a> {
  program: glium::Program,
  uniforms: UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>,
  view_w: u32,
  view_h: u32,
}

impl<'a> Renderer<'a> {
  pub fn new(lib_state: &LibState, w: u32, h: u32) -> Renderer<'a> {
    // Vertex shader
    let vert_src = r#"
      #version 100
      attribute vec2 position;
      attribute vec4 color;

      varying vec4 v_color;

      uniform mat4 proj_mat;

      void main() {
          v_color = color;
          gl_Position = proj_mat * vec4(position, 0.0, 1.0);
      }
    "#;

    // Fragment shader
    let frag_src = r#"
      #version 100
      precision mediump float; // Float precision to medium

      varying vec4 v_color;

      void main() {
        gl_FragColor = v_color;
      }
    "#;

    let uniforms = UniformsStorage::new(
      "proj_mat",
      // Orthographic proj mat:
      // glOrtho(0, w, h, 0, -1, 1);
      [[2.0/w as f32, 0.0,           0.0, -0.0],
       [0.0,         -2.0/h as f32,  0.0,  0.0],
       [0.0,          0.0,          -1.0,  0.0],
       [-1.0,          1.0,           0.0,  1.0]]);

    Renderer { 
      uniforms: uniforms,
      view_w: w, view_h: h,
      program: glium::Program::from_source(&lib_state.display,
                                           vert_src, 
                                           frag_src, 
                                           None).unwrap(),
    }
  }

  pub fn render(&self, lib_state: &LibState, target: &mut glium::Frame, 
                layer : &Layer, scissor_rect: Option<glium::Rect>) {
    // Create VBO data inside vec
    let mut data = Vec::<Vertex>::with_capacity(
      layer.component_debug_draw.len()*6);
    // Loop through debug draw components, find matching AABB component, then
    // draw
    for dd in &layer.component_debug_draw {
      let aabb = layer.component_aabb.get_component(dd.entity_id);
      if aabb.is_none() { continue; }
      // Found a matching AABB component, we can draw!
      let aabb = aabb.unwrap();
      data.push(Vertex{
        position: [aabb.x, aabb.y], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
      data.push(Vertex{
        position: [aabb.x+aabb.w, aabb.y], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
      data.push(Vertex{
        position: [aabb.x+aabb.w, aabb.y+aabb.h], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
      data.push(Vertex{
        position: [aabb.x, aabb.y], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
      data.push(Vertex{
        position: [aabb.x, aabb.y+aabb.h], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
      data.push(Vertex{
        position: [aabb.x+aabb.w, aabb.y+aabb.h], 
        color: [dd.color.r, dd.color.g, dd.color.b, 0.5]});
    }

    let vbo = glium::VertexBuffer::new(&lib_state.display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Apply scissor to draw params
    let mut draw_params = glium::draw_parameters::DrawParameters::default();
    draw_params.scissor = scissor_rect;

    target.draw(&vbo, 
                indices, &self.program,
                &self.uniforms,
                &draw_params).unwrap();

    // Find nested layers and render them
    for l in &layer.component_layer {
      if l.entity_id.is_none() { continue; }
      let aabb = layer.component_aabb.get_component(l.entity_id.unwrap());
      if aabb.is_none() { continue; }
      let aabb = aabb.unwrap();
      // Render nested layer with the correct scissor params
      self.render(lib_state, target, l, Some(glium::Rect {
        left: aabb.x as u32,
        bottom: aabb.x as u32,
        width: aabb.w as u32,
        height: aabb.h as u32,
      }));
    }
  }

  pub fn get_view_size(&self) -> (u32, u32) {
    (self.view_w, self.view_h)
  }
}

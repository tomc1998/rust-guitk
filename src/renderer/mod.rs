use glium;
use glium::Surface;
use glium::uniforms::{UniformsStorage, EmptyUniforms};

use view::View;
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

  pub fn render(&self, lib_state: &LibState, view : &View) {
    // Create VBO data inside vec
    let mut data = Vec::<Vertex>::with_capacity(
      view.component_debug_draw.len()*6);
    // Loop through debug draw components, find matching AABB component, then
    // draw
    for dd in &view.component_debug_draw {
      let aabb = view.component_aabb.get_component(dd.entity_id);
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
    let mut target = lib_state.display.draw();
    target.draw(&vbo, 
                indices, &self.program,
                &self.uniforms,
                &Default::default()).unwrap();
    let _ = target.finish().unwrap();
  }

  pub fn get_view_size(&self) -> (u32, u32) {
    (self.view_w, self.view_h)
  }
}

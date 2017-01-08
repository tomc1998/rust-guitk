extern crate android_glue;
extern crate ffi_glue;
extern crate libc;
#[macro_use]
extern crate glium;

pub mod logger;
pub mod renderer;
pub mod view;
pub mod common;
pub mod entity;

pub fn add(a: i32, b: i32) -> i32 {
  a * b
}


/// A struct which contains all the state needed by the library to function -
/// i.e a reference to the glutin Facade created for event handling and
/// rendering. 
pub struct LibState<'a> {
  /// The glutin display. 
  display: glium::backend::glutin_backend::GlutinFacade,
  /// The renderer
  renderer: Option<renderer::Renderer>,
  pub view_stack: Vec<view::View<'a>>,
}

/// Initialise guitk. Creates an OpenGL context.
pub fn init<'a>() -> LibState<'a> {
  use glium::DisplayBuild;
  let mut lib_state = LibState{
    display: glium::glutin::WindowBuilder::new()
      .with_gl(glium::glutin::GlRequest::Specific(
          glium::glutin::Api::OpenGlEs, (2, 0)))
      .build_glium().unwrap(),
    renderer: None,
    view_stack: Vec::new(),
  };
  lib_state.renderer = Some(renderer::Renderer::new(&lib_state));
  lib_state
}

impl<'a> LibState<'a> {
  /// Renders the view at the top of the view stack
  pub fn render(&self) {
    let view = self.view_stack.last();
    if view.is_some() {
      self.renderer.as_ref().unwrap().render(self, view.unwrap());
    }
  }
}

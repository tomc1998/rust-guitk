extern crate android_glue;
extern crate ffi_glue;
extern crate libc;
#[macro_use]
extern crate glium;

/// Logger module. Uses android glue to write messages to logcat.
pub mod logger;

/// Renderer module, contains methods called by the guitk lib to render
/// entities to the screen.
pub mod renderer;

/// View module, defines structures to represent a 'view', a collection of
/// entities which are grouped and presented as a single screen on the device.
pub mod view;

/// Contains common structs and functions.
pub mod common;

/// Contains definitions for the EntityID type and types of components.
pub mod entity;

/// A module which defines ways in which to layout entities.
pub mod layout;

/// A struct which contains all the state needed by the library to function -
/// i.e a reference to the glutin Facade created for event handling and
/// rendering. 
pub struct LibState<'a> {
  /// The glutin display. 
  display: glium::backend::glutin_backend::GlutinFacade,
  /// The renderer
  renderer: Option<renderer::Renderer<'a>>,
  pub view_stack: Vec<view::View<'a>>,
}

/// Initialise guitk. Creates an OpenGL context.
pub fn init<'a>() -> Option<LibState<'a>> {
  use glium::DisplayBuild;
  let mut lib_state = LibState{
    display: glium::glutin::WindowBuilder::new()
      .with_gl(glium::glutin::GlRequest::Specific(
          glium::glutin::Api::OpenGlEs, (2, 0)))
      .build_glium().unwrap(),
    renderer: None,
    view_stack: Vec::new(),
  };
  // Get width / height of window
  {
    let win_ref = lib_state.display.get_window();
    if win_ref.is_none() { 
      logger::log("guitk", logger::LogPriority::ERROR, 
                  "Could not acquire window ref. Exiting.");
      return None;
    }
    let win_ref = win_ref.unwrap();
    let size_opt = win_ref.get_inner_size();
    if size_opt.is_none() {
      logger::log("guitk", logger::LogPriority::ERROR, 
                  "Win ref closed unexpectedly. Exiting.");
      return None;
    }
    let (w, h) = size_opt.unwrap();
    lib_state.renderer = Some(renderer::Renderer::new(&lib_state, w, h));
  }
  return Some(lib_state);
}

impl<'a> LibState<'a> {
  /// Renders the view at the top of the view stack
  pub fn render(&self) {
    let view = self.view_stack.last();
    if view.is_some() {
      self.renderer.as_ref().unwrap().render(self, view.unwrap());
    }
  }

  /// Returns the size of the screen currently, or (0, 0) if the renderer has
  /// not been initialised.
  pub fn get_view_size(&self) -> (u32, u32) {
    if self.renderer.is_some() {
      let (w, h) = self.renderer.as_ref().unwrap().get_view_size();
      
      return (w, h);
    }
    else { (0,0) }
  }
}

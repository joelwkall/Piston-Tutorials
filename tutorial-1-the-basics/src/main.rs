extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
use piston::event::*;

use sdl2_window::Sdl2Window;
use opengl_graphics::OpenGL;
use piston::window::WindowSettings;

fn main() {
	let (width, height) = (300, 300);
	let opengl = OpenGL::_3_2;
	let window = Sdl2Window::new(
		opengl,
		WindowSettings::new("piston-tutorial-1", (width, height))
		.exit_on_esc(true)
	);
	for e in window.events() {/*event loop code goes here*/}
}
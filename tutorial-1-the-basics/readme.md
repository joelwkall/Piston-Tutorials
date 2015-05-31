##Piston Tutorial One
Before beginning this tutorial check out the [getting started](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started) tutorial to make sure you have the necessary dependencies installed. In this tutorial, we will go over each line of the code and build a small game. 

To begin create a new directory for your project by using the `cargo new piston-tutorial_1 --bin` command in terminal. This will create a Cargo.toml file, as well as a `src/main.rs` file with hello world already implemented.

Begin by opening the `Cargo.toml` file, and adding the necessary fields as shown below, to learn more about the cargo system, read [here](http://doc.crates.io/guide.html). 

	[package]
	name = "piston-tutorial-1"
	version = "0.0.1"
	authors = ["Sam <??????@gmail.com>"]
	keywords = []

	[[bin]]
	name = "piston-tutorial-1"
	path = "src/main.rs"

	[dependencies]
	pistoncore-sdl2_window = "0.1.0"
	piston = "0.1.4"
	piston2d-graphics = "0.1.3"
	piston2d-opengl_graphics = "0.1.0"

Now that all of the dependencies are set up, we can get rolling with Rust. To begin, we will just create an empty window to give you an idea of what the bare minimum is. Replace the contents of `src/main.js` with the code below.

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

In less than 20 lines of code we have created a blank window (kind of anti climactic, I know). The first three lines include the crates we need for our dependencies, the core piston library and two other libraries used for graphics. After that, the events library is included in the namespace [<mark>is that the correct way to phrase it?</mark>]. Other objects are included from the sdl2 and opengl libraries along with the window settings struct.

Inside the main method is where the magic really happens. After specifying the width and height of the window to be created, OpenGL is used to render the contents of the sdl2 window. In the last line of the main method the event loop drives the logic of the game. It is within this loop that we will determine how the different objects interact as a function of key presses, mouse clicks, time, etc. In the next part of this tutorial we will add a background, a basic character and make them move!
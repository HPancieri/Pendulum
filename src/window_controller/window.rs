use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

use crate::window_controller::pendulum::Pendulum;


pub fn run() {
	let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

	let win = MyWindowHandler {
		pendulum: Pendulum::new(400.0, 0.0, 200.0),
		pendulum2: Pendulum::new(400.0, 0.0, 400.0),
	};

	window.run_loop(win);
}

struct MyWindowHandler {
	pendulum: Pendulum,
	pendulum2: Pendulum,
}

impl WindowHandler for MyWindowHandler {
	fn on_draw(
			&mut self,
			helper: &mut WindowHelper<()>,
			graphics: &mut Graphics2D
		) {
			// Clear the screen
			graphics.clear_screen(Color::from_rgb(0.635294118, 0.839215686, 0.976470588));

			// Update and draw the pendulum
			self.pendulum.update();
			self.pendulum.draw(graphics);

			self.pendulum2.update();
			self.pendulum2.draw(graphics);

			// Draw the frame
			helper.request_redraw();
	}
}
use crate::window_controller::vector::Vector;
use speedy2d::{Graphics2D, color::Color};

pub struct Pendulum {
	length: f32,
	// mass: f32,
	gravity: f32,

	angle: f32,
	angular_velocity: f32,
	angular_acceleration: f32,

	origin: Vector,
	position: Vector,
}

impl Pendulum {
	pub fn new(x: f32, y: f32, length: f32) -> Pendulum {
		return Pendulum {
			length,
			// mass: 1.0, // Mass of the Pendulum will be always 1.0
			gravity: 1.5, // Gravity will be always 1.5

			angle: 1.0, // Initial angle will be 1.0 radians

			// Angular velocity and acceleration will start at 0.0
			angular_velocity: 0.0,
			angular_acceleration: 0.0,

			origin: Vector::new(x, y),
			position: Vector::new(0.0, 0.0),
		};
	}

	pub fn update(&mut self) {
		// Angular acceleration calculation
		self.angular_acceleration = -1.0 * self.gravity * self.angle.sin() / self.length;

		// Updating the angular velocity
		self.angular_velocity += self.angular_acceleration;

		// Updating the angle
		self.angle += self.angular_velocity;

		// The position will be the polar coordenates converted to cartesian coordenates
		self.position.set(self.length * self.angle.sin(), self.length * self.angle.cos());

		// The final position will be the origin position plus the position vector
		self.position.add(&self.origin);
	}

	pub fn draw(&self, graphics: &mut Graphics2D) {
		// Drawing the line of the pendulum
		graphics.draw_line(
			(self.origin.x, self.origin.y), 
			(self.position.x, self.position.y), 
			3.0,
			Color::from_rgb(0.752941176, 0.196078431, 0.129411765),
		);

		// Drawing the circle of the pendulum
		graphics.draw_circle(
			(self.position.x, self.position.y), 
			30.0,
			Color::from_rgb(0.752941176, 0.196078431, 0.129411765),
		);
	}
}

pub struct Vector {
	pub x: f32,
	pub y: f32,
}

impl Vector {
	pub fn new(x: f32, y: f32) -> Vector {
		return Vector { x, y };
	}

	pub fn add(&mut self, vec2: &Vector) -> &Vector {
		self.x += vec2.x;
		self.y += vec2.y;
		return self;
	}

	pub fn set(&mut self, x: f32, y: f32) -> &Vector {
		self.x = x;
		self.y = y;
		return self;
	}
}

use ultraviolet::Vec2;

#[derive(Clone, Copy)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub map_size: f32,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32, map_size:f32) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::zero(),
            mass,
            radius,
	    map_size,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
	

	let map_size = 200000.0f32;

	if self.pos.x > self.map_size {
		self.vel.x = -self.vel.x
	}
	if self.pos.y > self.map_size {
		self.vel.y = -self.vel.y
	}
	if self.pos.x < -(self.map_size) {
		self.vel.x = -self.vel.x
	}
	if self.pos.y < -(self.map_size) {
		self.vel.y = -self.vel.y
	}
  
     }
}

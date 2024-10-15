use crate::body::Body;
use ultraviolet::Vec2;

pub fn uniform_disc(n: usize) -> Vec<Body> {
    fastrand::seed(0);
    let inner_radius = 25.0;
    let outer_radius = (n as f32).sqrt() * 5.0;

    let mut bodies: Vec<Body> = Vec::with_capacity(n);

    let m = 1e4;
    let center = Body::new(Vec2::zero(), Vec2::zero(), m as f32, 100.0,20000.0);
    let other = Body::new(Vec2::new(0.0,13000.0), Vec2::zero(), m as f32, 100.0,20000.0);

    bodies.push(center);
    bodies.push(other);

    while bodies.len() < n {
        let a = fastrand::f32() * std::f32::consts::TAU;
        let (sin, cos) = a.sin_cos();
        let t = inner_radius / outer_radius;
        let r = fastrand::f32() * (1.0 - t * t) + t * t;
        let pos = Vec2::new(cos*5.0, sin*3.0) * outer_radius * r.sqrt();
        let vel = Vec2::new(sin, -cos);
        let mass = 3.0f32;
        let radius = mass.sqrt();

        bodies.push(Body::new(pos, vel, mass, radius,20000.0));
    }

    bodies.sort_by(|a, b| a.pos.mag_sq().total_cmp(&b.pos.mag_sq()));
    let mut mass = 0.0;
    for i in 0..n {
        mass += bodies[i].mass;
        if bodies[i].pos == Vec2::zero() {
            continue;
        }

        let v = (mass / bodies[i].pos.mag()).sqrt();
        bodies[i].vel *= v;
    }

    bodies
}

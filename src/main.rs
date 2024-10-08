use std::sync::atomic::Ordering;

mod body;
mod quadtree;
mod renderer;
mod simulation;
mod utils;

use renderer::Renderer;
use simulation::Simulation;

fn main() {
    let config = quarkstrom::Config {
        window_mode: quarkstrom::WindowMode::Windowed(900, 900),
    };
    quarkstrom::run::<Renderer>(config);

    let mut simulation = Simulation::new();

    loop {
        if renderer::PAUSED.load(Ordering::Relaxed) {
            std::thread::yield_now();
        } else {
            simulation.step();
        }
        render(&mut simulation);
    }
}

fn render(simulation: &mut Simulation) {
    let mut lock = renderer::UPDATE_LOCK.lock();
    for body in renderer::SPAWN.lock().drain(..) {
        simulation.bodies.push(body);
    }
    {
        let mut lock = renderer::BODIES.lock();
        lock.clear();
        lock.extend_from_slice(&simulation.bodies);
    }
    {
        let mut lock = renderer::QUADTREE.lock();
        lock.clear();
        lock.extend_from_slice(&simulation.quadtree.nodes);
    }
    *lock |= true;
}

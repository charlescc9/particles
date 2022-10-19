// Get number of particles and size of space
// Construct space
// Randomly initialize particles' positions and velocities in space
// Run main loop and at every time steps:
//    1. Update particle positions based on velocity
//    2. Perform collision computations (later feature)
//    3. Print out space with particles

struct State {
    space: Space,
    particles: Vec<Particle>,
}

struct Space {
    representation: Vec<Vec<char>>,
}

struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    direction: Position,
    speed: f64,
}

struct Particle {
    position: (usize, usize),
    velocity: Velocity,
}

fn generate_particles(num_particles: i32) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();

    for _ in 0..num_particles {
        particles.push(Particle {
            position: (0, 1),
            velocity: Velocity {
                direction: Position { x: 2, y: 4 },
                speed: 4.5,
            },
        })
    }

    return particles;
}

fn main() {
    // Initialize space
    let size = 10;
    let mut space = Space {
        representation: vec![vec![' '; size]; size],
    };

    // Initialize particles
    let particles = vec![
        Particle {
            position: (1, 2),
            velocity: Velocity {
                direction: Position { x: 2, y: 4 },
                speed: 4.5,
            },
        },
        Particle {
            position: (0, 1),
            velocity: Velocity {
                direction: Position { x: 2, y: 4 },
                speed: 4.5,
            },
        },
    ];
    for particle in particles {
        space.representation[particle.position.0][particle.position.1] = '*';
    }

    for row in &space.representation {
        println!("{:?}", row);
    }
}

const BIG_G: f64 = 6.67430E-11; //  N⋅m^2⋅kg^−2
#[allow(dead_code)]
const N_BODIES: i32 = 5;

#[derive(Debug, PartialEq)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq)]
struct Particle {
    pos: Vector3,
    vel: Vector3,
    mass: f64,
}

impl Particle {
    fn distance(&self, other: &Particle) -> f64 {
        let sum = (self.pos.x - other.pos.x).powi(2)
            + (self.pos.y - other.pos.y).powi(2)
            + (self.pos.z - other.pos.z).powi(2);
        sum.sqrt()
    }
    fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, m: f64) -> Particle {
        Particle {
            pos: Vector3 { x: x, y: y, z: z },
            vel: Vector3 {
                x: vx,
                y: vy,
                z: vz,
            },
            mass: m,
        }
    }
}

#[allow(dead_code)]
fn create_particles(n: i32) -> Vec<Particle> {
    let mut vec: Vec<Particle> = Vec::new();
    for _ in 0..n {
        let p = Particle {
            pos: Vector3 {
                x: rand::random::<f64>() * 1e7,
                y: rand::random::<f64>() * 1e7,
                z: 0.,
            },
            vel: Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            mass: 1e15,
        };
        vec.push(p)
    }
    return vec;
}

fn main() {
    let t_0 = 0.;
    let mut t = t_0;
    let dt = 86400.; // one day in seconds
    let t_end = 86400. * 365. * 10.; // one decade in seconds

    // let mut particles = create_particles(N_BODIES);
    let mut particles = Vec::new();

    particles.push(Particle::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.989e30)); // sun

    particles.push(Particle::new(
        57.909e9, 0.0, 0.0, 0.0, 47.36e3, 0.0, 0.33011e24,
    )); // mercury

    particles.push(Particle::new(
        108.209e9, 0.0, 0.0, 0.0, 35.02e3, 0.0, 4.8675e24,
    )); // venus

    particles.push(Particle::new(
        149.596e9, 0.0, 0.0, 0.0, 29.78e3, 0.0, 5.9724e24,
    )); // earth

    particles.push(Particle::new(
        227.923e9, 0.0, 0.0, 0.0, 24.07e3, 0.0, 0.64171e24,
    )); // mars

    particles.push(Particle::new(
        778.570e9, 0.0, 0.0, 0.0, 13e3, 0.0, 1898.19e24,
    )); // jupiter

    particles.push(Particle::new(
        1433.529e9, 0.0, 0.0, 0.0, 9.68e3, 0.0, 568.34e24,
    )); // saturn

    particles.push(Particle::new(
        2872.463e9, 0.0, 0.0, 0.0, 6.80e3, 0.0, 86.813e24,
    )); // uranus

    particles.push(Particle::new(
        4495.060e9, 0.0, 0.0, 0.0, 5.43e3, 0.0, 102.413e24,
    )); // neptune

    while t < t_end {
        for i in 0..particles.len() {
            let mut dvel = Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            for j in 0..particles.len() {
                if i != j {
                    let particle = &particles[i];
                    let other_particle = &particles[j];

                    let distance = particle.distance(other_particle);

                    let static_accel = -1. * BIG_G * other_particle.mass * distance.powi(-3);

                    dvel.x += static_accel * (particle.pos.x - other_particle.pos.x);
                    dvel.y += static_accel * (particle.pos.y - other_particle.pos.y);
                    dvel.z += static_accel * (particle.pos.z - other_particle.pos.z);
                }
            }
            let p = &mut particles[i];
            p.vel.x += dvel.x * dt;
            p.vel.y += dvel.y * dt;
            p.vel.z += dvel.z * dt;
        }
        for p in particles.iter_mut() {
            p.pos.x += p.vel.x * dt;
            p.pos.y += p.vel.y * dt;
            p.pos.z += p.vel.z * dt;
        }
        t += dt;
        for i in 0..particles.len() {
            // println!("{:?}", p);
            let p = &particles[i];
            println!("{} {} {} {}", i, p.pos.x, p.pos.y, p.pos.z);
        }
    }
}

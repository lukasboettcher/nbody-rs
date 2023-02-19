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

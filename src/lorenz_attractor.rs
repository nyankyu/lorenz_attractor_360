use nannou::prelude::*;

const SIGMA: f32 = 10.0;
const BETA: f32 = 8.0 / 3.0;
const RHO: f32 = 28.0;

const ORBIT_NUM: usize = 1000;
const ORBIT_LEN: usize = 600;
const DELTA_T: f32 = 0.001;
const ORBIT_WEIGHT: f32 = 1.0 / crate::SCALE;
const CAMERA_Z: f32 = 10.0;

pub(crate) struct LorenzAttractor {
    attractor: Vec<Particle>,
}

impl LorenzAttractor {
    pub(crate) fn new() -> Self {
        LorenzAttractor {
            attractor: (0..ORBIT_NUM).map(|_| Particle::new()).collect(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.attractor
            .iter_mut()
            .for_each(|particle| particle.update());
    }

    pub(crate) fn draw(&self, draw: &Draw, theta: f32) {
        self.attractor
            .iter()
            .for_each(|particle| particle.draw(draw, theta));
    }
}

struct Particle {
    orbit: Vec<Vec3>,
    last: Vec3,
}

impl Particle {
    fn new() -> Self {
        let last = random();

        Particle {
            orbit: vec![last; ORBIT_LEN],
            last,
        }
    }

    fn update(&mut self) {
        let d = lorenz_attractor(self.last);
        let last = self.last + d;
        self.orbit.push(last);
        self.last = last;

        if self.orbit.len() > ORBIT_LEN {
            self.orbit.remove(0);
        }
    }

    fn draw(&self, draw: &Draw, theta: f32) {
        let rotation =
            Mat3::from_euler(nannou::glam::EulerRot::ZYX, theta, theta * 7.9, theta * 1.3);
        let translation = vec3(0.0, 0.0, -20.0);

        let rotated: Vec<Vec3> = self
            .orbit
            .iter()
            .map(|&p| rotation * (p + translation))
            .collect();

        draw.polyline()
            .weight(ORBIT_WEIGHT)
            .join_round()
            .points_colored(rotated.iter().enumerate().map(|(i, &p)| {
                let longi_lati = equirectangular(p);
                let color = if -3.1 < longi_lati.x && longi_lati.x < 3.1 {
                    let alpha = 500.0 * i as f32 / ORBIT_LEN as f32 / (CAMERA_Z - p.z).abs();
                    rgba8(255, 0, 0, alpha as u8)
                } else {
                    rgba8(0, 0, 0, 0)
                };
                (longi_lati, color)
            }));
    }
}

fn equirectangular(p: Vec3) -> Vec2 {
    let dist_xy = (p.x * p.x + p.y * p.y).sqrt();
    let longitude = (p.x / dist_xy).acos() * p.y.signum();
    let latitude = ((p.z - CAMERA_Z) / dist_xy).atan();

    vec2(longitude, latitude)
}

fn random() -> Vec3 {
    vec3(
        random_range(-30.0, 30.0),
        random_range(-30.0, 30.0),
        random_range(0.0, 60.0),
    )
}

fn lorenz_attractor(point: Vec3) -> Vec3 {
    let dx = SIGMA * (point.y - point.x);
    let dy = point.x * (RHO - point.z) - point.y;
    let dz = point.x * point.y - BETA * point.z;
    vec3(dx, dy, dz) * DELTA_T
}

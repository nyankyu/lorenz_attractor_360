use nannou::prelude::*;

const SIGMA: f32 = 10.0;
const BETA: f32 = 8.0 / 3.0;
const RHO: f32 = 28.0;

const ORBIT_NUM: usize = 1000;
const ORBIT_LEN: usize = 600;
const DELTA_T: f32 = 0.001;
const ORBIT_WEIGHT: f32 = 10.0 / crate::SCALE;
const ORBIT_WEIGHT2: f32 = ORBIT_WEIGHT * 0.5;
const CAMERA_Z: f32 = 10.0;
const CENTER_Z: f32 = 20.0;

pub(crate) struct LorenzAttractor {
    attractor: Vec<Particle>,
    camera: Vec3,
    center: Vec3,
}

impl LorenzAttractor {
    pub(crate) fn new() -> Self {
        LorenzAttractor {
            attractor: (0..ORBIT_NUM).map(|_| Particle::new()).collect(),
            camera: vec3(0.0, 0.0, CAMERA_Z),
            center: vec3(0.0, 0.0, CENTER_Z),
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
            .for_each(|particle| particle.draw(draw, self.camera, self.center, theta));
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

    fn draw(&self, draw: &Draw, camera: Vec3, center: Vec3, theta: f32) {
        let rotation =
            Mat3::from_euler(nannou::glam::EulerRot::ZYX, theta, theta * 7.9, theta * 1.3);

        let mut coordinate_depth = self.orbit.iter().map(|&p| {
            let rotated = rotation * (p - center);
            let coordinate = equirectangular(&rotated);
            let depth = camera.distance(rotated);
            (coordinate, depth)
        });

        let mut pre = coordinate_depth.next().unwrap().0;
        for (_i, (coordinate, depth)) in coordinate_depth.enumerate() {
            let color = rgb8(180, 0, 0);
            let weight = 2.0 * (ORBIT_WEIGHT2 / depth).atan();

            let len_x = (pre.x - coordinate.x).abs();
            if len_x > PI {
                let center_y = (pre.y + coordinate.y) / 2.0;
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(vec2(PI * pre.x.signum(), center_y))
                    .end(pre)
                    .color(color);
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(vec2(PI * coordinate.x.signum(), center_y))
                    .end(coordinate)
                    .color(color);
            } else {
                draw.line()
                    .weight(weight)
                    .join_round()
                    .start(pre)
                    .end(coordinate)
                    .color(color);
            }

            pre = coordinate;
        }
    }
}

fn equirectangular(p: &Vec3) -> Vec2 {
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

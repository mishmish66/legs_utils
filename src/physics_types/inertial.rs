use super::basic::*;
use super::mass::*;
use super::na;
use super::transform::*;

pub struct Inertial {
    transform: Transform,
    mass: PointMass,
}

impl Inertial {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            mass: PointMass::new(),
        }
    }

    pub fn momentum(&self) -> MomTransform {
        let mom_lin = self.mass.mass * self.transform.vel.lin;

        let mom_ang = self.mass.moment * self.transform.vel.ang;

        MomTransform {
            lin: mom_lin,
            ang: mom_ang,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inertia_constructs() {
        let mass = PointMass {
            location: Vec3::new(1.0, 0.0, 0.0),
            mass: 1.0,
            moment: na::one(),
        };

        let mut inertial = Inertial {
            transform: Transform::new(),
            mass: mass,
        };

        inertial.transform.vel.ang = Vec3::new(0.0, 0.0, 1.0);


    }
}

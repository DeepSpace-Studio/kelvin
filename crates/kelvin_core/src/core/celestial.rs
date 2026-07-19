use nalgebra::{Rotation3, Vector3};
type Real = f64;

pub struct CelestialBody {
    pub(crate) pos: Vector3<f64>,
    pub(crate) rotation: Rotation3<f64>,

    pub(crate) compute: bool,
    pub(crate) linvel: Vector3<Real>,
    pub(crate) angvel: Real,
    pub(crate) mass: f64,
    pub(crate) forces: Vec<Force>,

    pub user_data: u128,
}

pub struct Force {
    pub force: Vector3<f64>,
    pub time: f64,

    pub user_data: u128,
}



impl CelestialBody {
    pub fn new(pos: Vector3<f64>, rotation: Rotation3<f64>, compute: bool, linvel: Vector3<f64>, angvel: f64, mass: f64, user_data: u128, ) -> Self {
        Self { pos, rotation, compute, linvel, angvel, mass, forces: Vec::new(), user_data, }
    }

    pub fn add_force(&mut self, force: Force) {
        self.forces.push(force);
    }

    pub fn get_total_force(&self) -> Vector3<f64> {
        self.forces.iter().map(|f| f.force).sum()
    }

    pub fn get_acceleration(&self) -> Vector3<f64> {
        if self.mass > 0.0 {
            self.get_total_force() / self.mass
        } else {
            Vector3::zeros()
        }
    }

    pub fn up(&mut self, dt: f64) {
        if !self.compute {
            return;
        }

        for f in &mut self.forces {
            f.up(dt);
        }

        self.forces.retain(|f| !f.is_dirt());

        let acc = self.get_acceleration();
        self.linvel += acc * dt;
    }
}

impl Force {
    pub fn new(force: Vector3<f64>, time: f64, user_data: u128) -> Self {
        Self { force, time, user_data }
    }

    pub fn up(&mut self, dt: f64) {
        self.time -= dt;
    }

    pub fn is_dirt(&self) -> bool { self.time <= 0.0 }
}


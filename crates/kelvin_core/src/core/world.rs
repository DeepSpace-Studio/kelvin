use nalgebra::{Rotation3, Vector3};
use crate::core::celestial::CelestialBody;

pub struct CelestialWorld {
    pub(crate) forces: Vec<CelestialBody>,
}
use nalgebra::Vector3;

struct RocketState {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub mass: f64,
    pub thrust: Vector3<f64>,
}

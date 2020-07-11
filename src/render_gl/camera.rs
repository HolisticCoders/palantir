use nalgebra::{Matrix4, Rotation3, Translation3, Unit, Vector3};

pub struct Camera {
    local_matrix: Matrix4<f32>,
    target_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            local_matrix: Matrix4::<f32>::identity(),
            target_matrix: Matrix4::<f32>::identity(),
        };

        camera.zoom(-3.0);

        camera
    }
    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.local_matrix * self.target_matrix
    }
    pub fn zoom(&mut self, amount: f32) {
        let translation = Translation3::new(0.0, 0.0, amount);
        self.local_matrix *= translation.to_homogeneous();
    }
    pub fn rotate(&mut self, axis: Unit<Vector3<f32>>, angle: f32) {
        let rotation = Rotation3::from_axis_angle(&axis, angle);

        if axis == Vector3::<f32>::y_axis() {
            self.target_matrix = self.target_matrix * rotation.to_homogeneous();
        } else {
            self.target_matrix = rotation.to_homogeneous() * self.target_matrix;
        }
    }
}

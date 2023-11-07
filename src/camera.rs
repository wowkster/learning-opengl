use nalgebra_glm as glm;

pub struct Camera {
    // Camera Attributes
    pub position: glm::Vec3,
    pub world_up: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,

    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,

    // Camera Options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub scroll_sensitivity: f32,
    pub fov: f32,
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: glm::vec3(0.0, 0.0, 0.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),

            yaw: Self::DEFAULT_YAW,
            pitch: Self::DEFAULT_PITCH,

            movement_speed: 2.5,
            mouse_sensitivity: 0.075,
            scroll_sensitivity: 1.0,
            fov: 45.0,
        }
    }
}

impl Camera {
    const DEFAULT_YAW: f32 = -90.0;
    const DEFAULT_PITCH: f32 = 0.0;

    const PITCH_LIMIT: f32 = 89.0;
    const FOV_MIN: f32 = 1.0;
    const FOV_MAX: f32 = 90.0;

    pub fn new(position: glm::Vec3, world_up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Self {
            position,
            world_up,
            yaw,
            pitch,
            ..Default::default()
        };

        camera.update_camera_vectors();

        camera
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;

        let offset = match direction {
            CameraMovement::Forward => self.front * velocity,
            CameraMovement::Backward => -self.front * velocity,
            CameraMovement::Left => -self.right * velocity,
            CameraMovement::Right => self.right * velocity,
            CameraMovement::Up => self.up * velocity,
            CameraMovement::Down => -self.up * velocity,
        };

        self.position += offset;
    }

    pub fn process_mouse_movement(
        &mut self,
        mut x_offset: f32,
        mut y_offset: f32,
        constrain_pitch: bool,
    ) {
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch {
            if self.pitch > Self::PITCH_LIMIT {
                self.pitch = Self::PITCH_LIMIT;
            }
            if self.pitch < -Self::PITCH_LIMIT {
                self.pitch = -Self::PITCH_LIMIT;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.fov -= y_offset * self.scroll_sensitivity;

        if self.fov < Self::FOV_MIN {
            self.fov = Self::FOV_MIN;
        }
        if self.fov > Self::FOV_MAX {
            self.fov = Self::FOV_MAX;
        }
    }

    fn update_camera_vectors(&mut self) {
        let direction = glm::vec3(
            f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch)),
            f32::sin(f32::to_radians(self.pitch)),
            f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch)),
        );

        self.front = glm::normalize(&direction);
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}

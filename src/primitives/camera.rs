use nalgebra_glm as glm;
use glm::{Vec3,Mat4};
use crate::consts::{WINDOW_WIDTH,WINDOW_HEIGHT};

pub struct Camera {
    matrix_view: Mat4,
    matrix_projection: Mat4,

    position: Vec3,
    pos_mod: bool,
    looking_at: Vec3,
    looking_mod: bool,
}

impl Camera {
    pub fn new(position: Vec3, looking_at: Vec3, distance_of_vision: f32) -> Self {
        let matrix_view = glm::look_at(&position, &looking_at, &Vec3::new(0.0,1.0,0.0));
        let matrix_projection = glm::perspective(
            WINDOW_WIDTH as f32/WINDOW_HEIGHT as f32,
            glm::radians(&glm::Vec1::new(80.0))[(0,0)], // I HATE NALGEBRA I HATE NALGEBRA
            0.1,
            distance_of_vision
        );

        Self {
            matrix_view,
            matrix_projection,
            position,
            pos_mod: false,
            looking_at,
            looking_mod: false
        }
    }

    pub fn view(&mut self) -> &Mat4 {
        if self.pos_mod && self.looking_mod {
            self.matrix_view = glm::look_at(&self.position, &self.looking_at, &Vec3::new(0.0,1.0,0.0));
        }
        &self.matrix_view
    }

    pub fn projection(&self) -> &Mat4 {
        &self.matrix_projection
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
        self.pos_mod = true;
    }

    pub fn set_look_at(&mut self, new_look_at: Vec3) {
        self.looking_at = new_look_at;
        self.looking_mod = true;
    }
}
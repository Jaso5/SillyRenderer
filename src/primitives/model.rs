use nalgebra_glm as glm;
use glm::{Vec3, Mat4};

use obj;


use std::io::BufReader;

use crate::util::file::get_file;

use serde::Deserialize;

pub struct Model {
    matrix_model: Mat4,
    position: Vec3,
    pos_mod: bool,

    pub model: obj::Obj,
}

impl Model {
    pub fn new_textured(position: Vec3, scale: Vec3, path: &str) -> Self {
        let matrix_model = Mat4::new(
            scale.x, 0.0, 0.0, position.x,
            0.0, scale.y, 0.0, position.y,
            0.0, 0.0, scale.z, position.z,
            0.0, 0.0, 0.0, 1.0,
        );
        // Add rotation code https://learnopengl.com/Getting-started/Coordinate-Systems

        let model = obj::load_obj(
            BufReader::new(get_file(path))
        ).unwrap();

        Self {
            matrix_model,
            position,
            pos_mod: false,

            model
        }
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
        self.pos_mod = true;
    }

    pub fn matrix(&mut self) -> &Mat4 {
        if self.pos_mod {
            self.matrix_model = Mat4::new(
                1.0, 0.0, 0.0, self.position.x,
                0.0, 1.0, 0.0, self.position.y,
                0.0, 0.0, 1.0, self.position.z,
                0.0, 0.0, 0.0, 1.0,
            );
            self.pos_mod = false;
        }

        return &self.matrix_model
    }
}

#[derive(Deserialize)]
pub struct ModelInfo {
    obj_path: String,
    tex_path: String,

    tags: Vec<String>
}

impl From<&ModelInfo> for Model {
    fn from(_: &ModelInfo) -> Self {
        todo!()
    }
}
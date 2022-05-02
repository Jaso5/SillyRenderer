use crate::util::file::get_file;
use glium::{Display, Program};
use std::io::Read;

pub struct Shader {
    program: Program,
    geom: bool,
}

impl Shader {
    pub fn new(display: &Display, vert: &str, frag: &str, geom: Option<&str>) -> Self {
        let mut _vert = String::new();
        let mut _frag = String::new();

        get_file(vert).read_to_string(&mut _vert).unwrap();
        get_file(frag).read_to_string(&mut _frag).unwrap();

        match geom {
            None => Self {
                program: Program::from_source(display, _vert.as_str(), _frag.as_str(), None)
                    .unwrap(),
                geom: false,
            },
            Some(path) => {
                let mut _geom = String::new();
                get_file(path).read_to_string(&mut _geom).unwrap();

                Self {
                    program: Program::from_source(
                        display,
                        _vert.as_str(),
                        _frag.as_str(),
                        Some(_geom.as_str()),
                    )
                    .unwrap(),
                    geom: true,
                }
            }
        }
    }

    pub fn has_geometry_shader(&self) -> bool {
        self.geom
    }

    pub fn get(&self) -> &Program {
        &self.program
    }
}

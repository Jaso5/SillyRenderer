mod primitives;
use primitives::shader::Shader;

mod util;
mod consts;

use consts::*;

#[macro_use]
extern crate glium;
use glium::{glutin, Surface};

use nalgebra_glm as glm;
use glm::Vec3;
use nalgebra_glm::pi;


fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_title("Jacobs GL Game");
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();



    let mut camera = primitives::camera::Camera::new(
        Vec3::new(0.0,1.0,1.0),
        Vec3::zeros(),
        100.0
    );

    let mut model = primitives::model::Model::new_textured(Vec3::new(0.0,-0.5,0.0), Vec3::new(0.4,0.4,0.4), &"assets/models/bunny.obj");





    // let mut shape = Vec::new();
    // shape.push(Vertex::new([-0.5, -0.5, 0.0]));
    // shape.push(Vertex::new([0.0, 0.5, 0.0]));
    // shape.push(Vertex::new([0.5, -0.25, 0.0]));

    let shader = Shader::new(&display, &"shaders/basic.vert", &"shaders/basic.frag", None);

    let vb = model.model.vertex_buffer(&display).unwrap();
    let ib = model.model.index_buffer(&display).unwrap();


    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let mut i = 0.0;

    // Main Loop
    event_loop.run(move |event, _, control_flow| {

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let uniform = uniform! {
            model: glm::rotate_normalized_axis(&model.matrix(), i, &Vec3::new(0.0, 1.0, 0.0)).data.0,
            view: camera.view().data.0,
            projection: camera.projection().data.0,
            light: Vec3::new(1.0,0.0,1.0).data.0[0]
        };

        i += pi::<f32>()/64.0;

        let mut target = display.draw();
        target.clear_color_and_depth((0.2, 0.2, 0.1, 0.0), 1.0);

        target.draw(
                &vb,
                &ib,
                shader.get(),
                &uniform,
                &params
        ).unwrap();
        target.finish().unwrap();
    });
}

mod vertex;

#[macro_use]
extern crate glium;

use std::time::Instant;
use vertex::Vertex;
use glium::{glutin, Surface};
use crate::glutin::event_loop::ControlFlow;

static VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    uniform vec2 movement;

    void main() {
        vec2 pos = position;
        pos.x += movement.x;
        pos.y += movement.y;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

static FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

const TARGET_FPS: u64 = 60;

fn main() {
    println!("Hello, world!");

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [0.05,0.05] };
    let vertex2 = Vertex { position: [ 0.05,  -0.05] };
    let vertex3 = Vertex { position: [ -0.05, 0.05] };
    let vertex4 = Vertex { position: [ -0.05,  -0.05] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program = glium::Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None).unwrap();

    let mut movement: [f32; 2] = [0.0, 0.0];

    let mut movement_other: [f32; 2] = [0.0, 0.0];

    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;


    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(glutin::event::VirtualKeyCode::Escape) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    Some(glutin::event::VirtualKeyCode::W) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            w_pressed = true;
                        } else {
                            w_pressed = false;
                        }
                    },
                    Some(glutin::event::VirtualKeyCode::S) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            s_pressed = true;
                        } else {
                            s_pressed = false;
                        }
                    },
                    Some(glutin::event::VirtualKeyCode::D) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            d_pressed = true;
                        } else {
                            d_pressed = false;
                        }
                    },
                    Some(glutin::event::VirtualKeyCode::A) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            a_pressed = true;
                        } else {
                            a_pressed = false;
                        }
                    },
                    _ => return,
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

        if w_pressed {
            movement[1] += 0.01;
        }
        if s_pressed {
            movement[1] -= 0.01;
        }
        if d_pressed {
            movement[0] += 0.01;
        }
        if a_pressed {
            movement[0] -= 0.01;
        }

        movement_other[0] = -movement[0];
        movement_other[1] = -movement[1];

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { movement: movement },
                    &Default::default()).unwrap();
        target.draw(&vertex_buffer, &indices, &program, &uniform! { movement: movement_other },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = start_time +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}


use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod common;
use std::borrow::Cow;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    window.set_title("3D engine");
    env_logger::init();

    let inputs = common::Inputs {
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None
    };

    pollster::block_on(common::run(event_loop, window, inputs, 3));

    // event_loop.run(move |event, _, control_flow| {
    //     *control_flow = ControlFlow::Wait;
    //
    //     match event {
    //         Event::WindowEvent {
    //             event: WindowEvent::CloseRequested,
    //             ..
    //         } => *control_flow = ControlFlow::Exit,
    //         _ => {}
    //     }
    // }
}

use std::process::ExitCode;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{Window, WindowId};

pub struct Raindeer {
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
}

impl ApplicationHandler for Raindeer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

            }
            _ => (),
        }
    }
}

impl Raindeer {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        Self {
            window: None,
            event_loop: Some(event_loop),
        }
    }

    pub fn run(&mut self) -> Result<(), ExitCode> {
        let mut event_loop_wrapper = self.event_loop.take();

        let Some(ref mut event_loop) = event_loop_wrapper else { 
            panic!("no event loop");
        };

        let status = event_loop.pump_app_events(Some(Duration::ZERO), self);

        self.event_loop = event_loop_wrapper;

        if let PumpStatus::Exit(exitcode) = status {
            return Err(ExitCode::from(exitcode as u8));
        }

        return Ok(());
    }
}

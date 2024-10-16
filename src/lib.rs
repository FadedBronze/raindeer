mod triangulate;
pub mod color;
pub mod path_builder;
pub mod scene;
pub mod treemap;

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use bytemuck::{Pod, Zeroable};
use scene::{RDScene, VAO};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};
use winit::window::{Window, WindowId};

struct GfxState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    storage_buffer: wgpu::Buffer,
}

pub struct Raindeer {
    pub scene: RDScene,
    size: winit::dpi::PhysicalSize<u32>,

    window: Option<Arc<Window>>,
    event_loop: Option<EventLoop<()>>,
    gfx_state: Option<GfxState>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RDVertex {
    color: u32,
    position: [f32; 2],
    texture_position: [f32; 2],
    id: u32,
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct RDStorage {
    transform: [[f32; 4]; 4],
    texture: u32,
}

unsafe impl Zeroable for RDStorage {}
unsafe impl Pod for RDStorage {}

impl RDVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RDVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[u8; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                }
            ]
        }
    }
}

unsafe impl Zeroable for RDVertex {}
unsafe impl Pod for RDVertex {}

impl ApplicationHandler for Raindeer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attibutes = Window::default_attributes()
            .with_title("heya")
            .with_inner_size(
                Size::Physical(PhysicalSize::new(800, 800))
            );

        let window = Arc::new(event_loop.create_window(window_attibutes).unwrap());

        self.init_graphics(window.clone());
        self.window = Some(window.clone());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.window.clone().take().unwrap().request_redraw();

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                match self.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    }
}

impl Raindeer {
    pub fn new() -> Self {
        pollster::block_on(Raindeer::async_new())
    }

    pub async fn async_new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        Self {
            scene: RDScene::new(10),
            size: PhysicalSize::new(800, 800),
            window: None,
            gfx_state: None,
            event_loop: Some(event_loop),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let Some(ref mut gfx) = self.gfx_state else { panic!("gfx state uninitialized"); };

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            gfx.config.width = new_size.width;
            gfx.config.height = new_size.height;
            gfx.surface.configure(&gfx.device, &gfx.config);
        }
    }

    pub async fn async_init_graphics(&mut self, window: Arc<Window>) {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            ..Default::default()
        });
        
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None,
                    }
                }
            ],
        });
        
        let storage_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Storage Buffer"),
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                size: std::mem::size_of::<RDStorage>() as u64 * 16384,
            }
        );
        
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Storage Buffer"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: storage_buffer.as_entire_binding(),
                }
            ],
            layout: &bind_group_layout,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    RDVertex::desc(),
                ], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None, // 6.
        });

        let vertex_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                size: std::mem::size_of::<RDVertex>() as u64 * 16384,
            }
        );
                
        let index_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Index Buffer"),
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                size: std::mem::size_of::<u32>() as u64 * 32768,
            }
        );

        self.gfx_state = Some(GfxState {
            bind_group,
            storage_buffer,
            index_buffer,
            vertex_buffer,
            device,
            queue,
            surface,
            config,
            render_pipeline,
        });
    }

    pub fn init_graphics(&mut self, window: Arc<Window>) {
        pollster::block_on(self.async_init_graphics(window));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut gfx_wrapper = self.gfx_state.take();
        let Some(ref mut gfx) = gfx_wrapper else { panic!("gfx state uninitialized"); };

        let output = gfx.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = gfx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        let storage = self.scene.output_gfx_storage(self.size.height as f32, self.size.width as f32);
        gfx.queue.write_buffer(&gfx.storage_buffer, 0, bytemuck::cast_slice(&storage));

        if !self.scene.vertex_cache {
            let VAO { vertices, indicies } = self.scene.output_gfx_vao();

            println!("{:#?}", storage);
            println!("{:#?}", vertices);
            println!("{:?}", indicies);

            gfx.queue.write_buffer(&gfx.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            gfx.queue.write_buffer(&gfx.index_buffer, 0, bytemuck::cast_slice(&indicies));

            self.scene.vertex_cache = true;
            self.scene.index_count = indicies.len() as u32;
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&gfx.render_pipeline);
            render_pass.set_bind_group(0, &gfx.bind_group, &[]);
            render_pass.set_vertex_buffer(0, gfx.vertex_buffer.slice(..));
            render_pass.set_index_buffer(gfx.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.scene.index_count, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        gfx.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.gfx_state = gfx_wrapper;

        Ok(())
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

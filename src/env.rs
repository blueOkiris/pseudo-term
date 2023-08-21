// Author(s): Dylan Turner <dylan.turner@tutanota.com>
//! Build an environment up and then instantiate it to run the application

use std::{
    time::{
        SystemTime, Duration, Instant
    }, collections::HashMap,
    borrow::Cow
};
use wgpu::{
    Instance, RequestAdapterOptions, PowerPreference, Device, Surface, SurfaceConfiguration,
    Queue, Features, Limits, DeviceDescriptor, TextureViewDescriptor,
    CommandEncoderDescriptor, RenderPassDescriptor, RenderPassColorAttachment, Operations,
    LoadOp, Color, ShaderSource, ShaderModuleDescriptor, PipelineLayoutDescriptor,
    RenderPipelineDescriptor, RenderPipeline
};
use wgpu_text::{
    BrushBuilder,
    glyph_brush::{
        VerticalAlign, BuiltInLineBreaker, Layout, Section, Text
    }
};
use winit::{
    event_loop::{
        EventLoop, ControlFlow
    }, window::{
        Window, Theme
    }, event::{
        Event, WindowEvent, KeyboardInput, ElementState
    }, dpi::PhysicalSize,
};
use crate::obj::GameObject;

/// The terminal will appear to be 80x25, but this is the size of the window surrounding that
const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize { width: 1024, height: 576 };
const TEXT_OFFSET: (f32, f32) = (85.0, WINDOW_SIZE.height as f32 * 0.5);
const FONT_SIZE: f32 = 22.0;
const FRAME_RATE: f64 = 60.0;
const BASE_BUFF: [[char; 81]; 25] = [
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\n'
    ], [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '\0'
    ]
];

/// Core engine. Create game objs & rooms via builder then run with this immutably.
pub struct Environment {
    global_game_objs: Vec<Box<dyn GameObject>>,
    rooms: HashMap<String, Vec<Box<dyn GameObject>>>,
    start_room: String,
    win_title: String,

    ev_loop: EventLoop<()>,
    window: Window,

    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    font: &'static [u8]
}

impl Environment {
    /// Create a window and run the game from established code
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut global_objs = self.global_game_objs.clone();
        let mut rooms = self.rooms.clone();
        let mut cur_room = self.start_room.clone();
        let mut brush = BrushBuilder::using_font_bytes(self.font)
            .expect("Failed to create text brush.")
            .build(
                &self.device, WINDOW_SIZE.width, WINDOW_SIZE.height,
                self.config.format
            );
        let mut then = SystemTime::now();
        let mut now = SystemTime::now();
        let mut fps = 0;
        let target_framerate = Duration::from_secs_f64(1.0 / FRAME_RATE);
        let mut delta_time = Instant::now();
        self.ev_loop.run(move |ev, _, ctl_flow| {
            *ctl_flow = ControlFlow::Poll;
            match ev {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } =>
                    *ctl_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    // Build the text buffer
                    let mut text_buf = BASE_BUFF.clone();
                    for obj in global_objs.iter() {
                        obj.draw(&mut text_buf);
                    }
                    for obj in rooms[&cur_room].iter() {
                        obj.draw(&mut text_buf);
                    }

                    // Apply it to screen
                    let mut buff_strs = text_buf.iter()
                        .map(|line| line.iter().collect::<String>())
                        .collect::<Vec<String>>();
                    let buf_str = buff_strs.iter_mut().reduce(|final_str, line| {
                        final_str.push_str(line.as_str());
                        final_str
                    }).unwrap();
                    let section = Section::default()
                        .add_text(
                            Text::new(buf_str.as_str())
                                .with_scale(FONT_SIZE)
                                .with_color([ 0.8, 0.85, 0.9, 1.0 ])
                        ).with_bounds((WINDOW_SIZE.width as f32, WINDOW_SIZE.height as f32))
                        .with_layout(
                            Layout::default()
                                .v_align(VerticalAlign::Center)
                                .line_breaker(BuiltInLineBreaker::AnyCharLineBreaker)
                        ).with_screen_position(TEXT_OFFSET)
                        .to_owned();
                    brush.queue(&self.device, &self.queue, vec![ &section ])
                        .expect("Failed to draw text.");
                    let frame = self.surface.get_current_texture()
                        .expect("Failed to acquire next swap chain texture.");
                    let view = frame.texture.create_view(&TextureViewDescriptor::default());
                    let mut encoder = self.device.create_command_encoder(
                        &CommandEncoderDescriptor { label: None }
                    );
                    {
                        let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: Operations {
                                    load: LoadOp::Clear(Color::BLACK),
                                    store: true
                                }
                            })], depth_stencil_attachment: None
                        });
                        rpass.set_pipeline(&self.render_pipeline);
                        rpass.draw(0..6, 0..1);
                        brush.draw(&mut rpass);
                    }
                    self.queue.submit(Some(encoder.finish()));
                    frame.present();

                    fps += 1;
                    if now.duration_since(then).unwrap().as_millis() > 1000 {
                        self.window.set_title(&format!("{}, FPS: {}", self.win_title, fps));
                        fps = 0;
                        then = now;
                    }
                    now = SystemTime::now();
                }, Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                    virtual_keycode, state, ..
                }, .. }, .. } => {
                    if virtual_keycode.is_none() {
                        return;
                    }
                    let globals_clone = global_objs.clone();
                    let rooms_clone = rooms.clone();
                    let mut room = rooms[&cur_room].clone();
                    match state {
                        ElementState::Pressed => {
                            for obj in global_objs.iter_mut() {
                                obj.on_key_pressed(
                                    virtual_keycode.unwrap(),
                                    &globals_clone, &rooms_clone, &mut cur_room
                                );
                            }
                            for obj in room.iter_mut() {
                                obj.on_key_pressed(
                                    virtual_keycode.unwrap(),
                                    &globals_clone, &rooms_clone, &mut cur_room
                                );
                            }
                        }, ElementState::Released => {
                            for obj in global_objs.iter_mut() {
                                obj.on_key_released(
                                    virtual_keycode.unwrap(),
                                    &globals_clone, &rooms_clone, &mut cur_room
                                );
                            }
                            for obj in room.iter_mut() {
                                obj.on_key_released(
                                    virtual_keycode.unwrap(),
                                    &globals_clone, &rooms_clone, &mut cur_room
                                );
                            } 
                        }
                    }
                    rooms.insert(cur_room.clone(), room);
                }, Event::MainEventsCleared => {
                    let globals_clone = global_objs.clone();
                    let rooms_clone = rooms.clone();
                    let mut room = rooms[&cur_room].clone();
                    let old_cur_room = cur_room.clone();
                    for obj in global_objs.iter_mut() {
                        obj.update(
                            delta_time.elapsed().as_secs_f32(),
                            &globals_clone, &rooms_clone, &mut cur_room
                        );
                    }
                    for obj in room.iter_mut() {
                        obj.update(
                            delta_time.elapsed().as_secs_f32(),
                            &globals_clone, &rooms_clone, &mut cur_room
                        );
                    }
                    if cur_room != old_cur_room {
                        // Reset unless persistent on room change
                        let new_rooms = self.rooms.clone();
                        let new_room = new_rooms.get(&old_cur_room).unwrap();
                        for i in 0..room.len() {
                            if !room[i].persistent() {
                                room[i] = new_room[i].clone();
                            }
                        }
                    }
                    rooms.insert(cur_room.clone(), room);

                    if target_framerate <= delta_time.elapsed() {
                        self.window.request_redraw();
                        delta_time = Instant::now();
                    } else {
                        *ctl_flow = ControlFlow::WaitUntil(
                            Instant::now().checked_sub(delta_time.elapsed()).unwrap()
                                + target_framerate
                        );
                    }
                }, _ => {}
            }
        });
    }
}

/// Builder for the game environment. Create rooms w/ objs and add them here, then build and run
#[derive(Clone)]
pub struct EnvironmentBuilder {
    global_game_objs: Vec<Box<dyn GameObject>>,
    rooms: HashMap<String, Vec<Box<dyn GameObject>>>,
    start_room: String,
    win_title: String
}

impl EnvironmentBuilder {
    pub fn new(start_room: &str) -> Self {
        Self {
            global_game_objs: Vec::new(),
            rooms: HashMap::new(),
            start_room: start_room.to_string(),
            win_title: "Pseudo-Term Window".to_string()
        }
    }

    pub fn set_window_title(&self, win_title: &str) -> Self {
        Self {
            global_game_objs: self.global_game_objs.clone(),
            rooms: self.rooms.clone(),
            start_room: self.start_room.clone(),
            win_title: win_title.to_string()
        }
    }

    pub fn add_global_obj(&self, obj: Box<dyn GameObject>) -> Self {
        let mut new = self.clone();
        new.global_game_objs.push(obj);
        new
    }

    pub fn add_room(&self, name: &str, room: &Vec<Box<dyn GameObject>>) -> Self {
        let mut new = self.clone();
        new.rooms.insert(name.to_string(), room.clone());
        new
    }

    pub async fn build(&self) -> Result<Environment, Box<dyn std::error::Error>> {
        let ev_loop = EventLoop::new();

        let window = Window::new(&ev_loop)?;
        window.set_inner_size(WINDOW_SIZE);
        window.set_resizable(false);
        window.set_title(&self.win_title);
        window.set_theme(Some(Theme::Dark));

        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(&window) }?;
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await;
        if adapter.is_none() {
            Err("Failed to find an appropriate adapter.")?;
        }
        let adapter = adapter.unwrap();
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
            }, None
        ).await?;

        let bg_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("bg_shader.wgsl"))),
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &bg_shader,
                entry_point: "vs_main",
                buffers: &[],
            }, fragment: Some(wgpu::FragmentState {
                module: &bg_shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }), primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let font: &[u8] = include_bytes!("font/OverpassMono-Regular.ttf");

        Ok(Environment {
            global_game_objs: self.global_game_objs.clone(),
            rooms: self.rooms.clone(),
            start_room: self.start_room.clone(),
            win_title: self.win_title.clone(),
            ev_loop,
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            font
        })
    }
}


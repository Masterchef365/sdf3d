use anyhow::{Result, Context};
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Material, Object, Vertex, Matrix4
};
use std::sync::mpsc::{channel, Receiver};
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::time::Duration;
use std::path::PathBuf;
use structopt::StructOpt;
use shaderc::Compiler;

struct MyApp {
    fullscreen: Object,
    time: f32,
    compiler: Compiler,
    shader_path: PathBuf,
    file_watch_rx: Receiver<DebouncedEvent>,
    _file_watcher: RecommendedWatcher,
}

impl App for MyApp {
    const NAME: &'static str = "Signed Distance Functions in 3D";

    type Args = Opt;

    fn new(engine: &mut dyn Engine, args: Self::Args) -> Result<Self> {
        // Set up file watch
        let (tx, file_watch_rx) = channel();
        let mut file_watcher = watcher(tx, Duration::from_millis(250))?;
        let shader_path = args.shader_path.canonicalize()?;
        let parent_dir = args.shader_path.parent().context("Shader has no parent dir?")?;
        file_watcher.watch(parent_dir, RecursiveMode::NonRecursive)?;

        // Create fullscreen mesh
        let (vertices, indices) = fullscreen_quad();
        let mesh = engine.add_mesh(&vertices, &indices)?;

        // Load initial material
        let mut compiler = Compiler::new().context("Failed to set up GLSL compiler")?;
        let material = load_shader(&args.shader_path, engine, &mut compiler)?;

        // Fullscreen quad
        let fullscreen = Object {
            mesh,
            material,
            transform: Matrix4::identity(),
        };

        Ok(Self {
            file_watch_rx,
            _file_watcher: file_watcher,
            shader_path,
            compiler,
            fullscreen,
            time: 0.0,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        // Reload shader on file change
        match self.file_watch_rx.try_recv() {
            Ok(DebouncedEvent::Create(p)) | Ok(DebouncedEvent::Write(p)) => {
                if p == self.shader_path {
                    match load_shader(&p, engine, &mut self.compiler) {
                        Ok(material) => {
                            let old = std::mem::replace(&mut self.fullscreen.material, material);
                            engine.remove_material(old)?;
                        },
                        Err(e) => {
                            println!("ERROR: {}", e.to_string());
                        }
                    }
                }
            }
            _ => (),
        };

        engine.update_time_value(self.time)?;
        self.time += 0.01;
        
        Ok(FramePacket {
            objects: vec![self.fullscreen],
        })
    }
}

// Simple fullscreen vertex shader
const FULLSCREEN_VERT: &[u8] = include_bytes!("fullscreen.vert.spv");

fn load_shader(path: &PathBuf, engine: &mut dyn Engine, compiler: &mut Compiler) -> Result<Material> {
    let text = fs::read_to_string(path)?;
    let spirv = compiler.compile_into_spirv(&text, shaderc::ShaderKind::Fragment, path.to_str().unwrap(), "main", None)?;
    engine.add_material(
        FULLSCREEN_VERT,
        spirv.as_binary_u8(),
        DrawType::Triangles
    )
}

#[derive(Debug, StructOpt)]
#[structopt(name = "SDF 3D", about = "Signed Distance Functions in 3D")]
struct Opt {
    /// Use OpenXR backend
    #[structopt(short, long)]
    vr: bool,

    /// Set shader directory (will look for glsl files to update, and will use those as fragment
    /// shaders)
    #[structopt(short, long)]
    shader_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    launch::<MyApp>(args.vr, args)
}

fn fullscreen_quad() -> (Vec<Vertex>, Vec<u16>) {
    let vertices = vec![
        Vertex { pos: [-1.0, -1.0, 0.0], color: [1.; 3] },
        Vertex { pos: [-1.0, 1.0, 0.0], color: [1.; 3] },
        Vertex { pos: [1.0, -1.0, 0.0], color: [1.; 3] },
        Vertex { pos: [1.0, 1.0, 0.0], color: [1.; 3] },
    ];

    let indices = vec![
        2, 1, 0,
        3, 1, 2,
    ];

    (vertices, indices)
}

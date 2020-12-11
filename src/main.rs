use anyhow::Result;
use klystron::{
    runtime_3d::{launch, App},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, 
};
use nalgebra::{Matrix4, Point3};
use std::fs;

struct MyApp {
    material: Material,
    mesh: Mesh,
    time: f32,
}

impl App for MyApp {
    const NAME: &'static str = "MyApp";

    type Args = ();

    fn new(engine: &mut dyn Engine, _args: Self::Args) -> Result<Self> {
        let material = engine.add_material(
            &fs::read("./shaders/unlit.vert.spv")?, 
            &fs::read("./shaders/unlit.frag.spv")?, 
            DrawType::Triangles
        )?;

        let (vertices, indices) = fullscreen_quad();
        let mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            mesh,
            material,
            time: 0.0,
        })
    }

    fn next_frame(&mut self, engine: &mut dyn Engine) -> Result<FramePacket> {
        let transform = Matrix4::from_euler_angles(0.0, 0.0, 0.0);
        let object = Object {
            material: self.material,
            mesh: self.mesh,
            transform,
        };
        engine.update_time_value(self.time)?;
        self.time += 0.01;
        Ok(FramePacket {
            objects: vec![object],
        })
    }
}

fn main() -> Result<()> {
    let vr = std::env::args().skip(1).next().is_some();
    launch::<MyApp>(vr, ())
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

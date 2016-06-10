use std::sync::Arc;
use glium;
use current::Current;
use dyon::*;

use Window;

pub type Programs = Vec<(Arc<String>, glium::Program)>;
pub type VertexBuffers = Vec<glium::VertexBuffer<Vertex>>;
pub type IndexBuffers = Vec<glium::IndexBuffer<u32>>;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    norm: [f32; 3],
}

implement_vertex!{Vertex, pos, norm}

pub fn register_shader(module: &mut Module) {
    module.add(Arc::new("load_program__name_vshader_fshader".into()),
        load_program__name_vshader_fshader, PreludeFunction {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::Text; 3],
            ret: Type::Result(Box::new(Type::Text))
        });
    module.add(Arc::new("program".into()),
        program, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::Text],
            ret: Type::Option(Box::new(Type::F64))
        });
    module.add(Arc::new("count_vertex_buffers".into()),
        count_vertex_buffers, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::F64
        });
    module.add(Arc::new("count_index_buffers".into()),
        count_index_buffers, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::F64
        });
    module.add(Arc::new("create_vertex_buffer__size".into()),
        create_vertex_buffer__size, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::Result(Box::new(Type::F64))
        });
    module.add(Arc::new("create_index_buffer__size".into()),
        create_index_buffer__size, PreludeFunction {
            lts: vec![Lt::Default],
            tys: vec![Type::F64],
            ret: Type::Result(Box::new(Type::F64))
        });
    module.add(Arc::new("fill_vertex_buffer__buffer_pos_norm".into()),
        fill_vertex_buffer__buffer_pos_norm, PreludeFunction {
            lts: vec![Lt::Default; 3],
            tys: vec![Type::F64, Type::Array(Box::new(Type::Vec4)),
                Type::Array(Box::new(Type::Vec4))],
            ret: Type::Void
        });
    module.add(Arc::new("fill_index_buffer__buffer_data".into()),
        fill_index_buffer__buffer_data, PreludeFunction {
            lts: vec![Lt::Default; 2],
            tys: vec![Type::F64, Type::Array(Box::new(Type::F64))],
            ret: Type::Void
        });
    module.add(Arc::new("clear_depth".into()),
        clear_depth, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::Void
        });
    module.add(Arc::new("draw__program_vbuf_ibuf_pos_angle_scale_color".into()),
        draw__program_vbuf_ibuf_pos_angle_scale_color, PreludeFunction {
            lts: vec![Lt::Default; 7],
            tys: vec![Type::F64, Type::F64, Type::F64,
                      Type::Vec4, Type::F64, Type::F64,
                      Type::Vec4],
            ret: Type::Void
        });
}

dyon_fn!{fn load_program__name_vshader_fshader(
    name: Arc<String>,
    vshader: Arc<String>,
    fshader: Arc<String>
) -> Result<Arc<String>, String> {
    use std::error::Error;

    let programs = unsafe { &mut *Current::<Programs>::new() };
    let window = unsafe { &*Current::<Window>::new() };

    let program = try!(glium::Program::from_source(
        &window.context, &vshader, &fshader, None).map_err(|err|
            match err {
                glium::program::ProgramCreationError::CompilationError(err) => err,
                _ => String::from(err.description())
            }
        ));
    programs.push((name.clone(), program));

    Ok(name)
}}

dyon_fn!{fn program(name: Arc<String>) -> Option<usize> {
    let programs = unsafe { &*Current::<Programs>::new() };
    for (i, n) in programs.iter().enumerate() {
        if &n.0 == &name { return Some(i) }
    }
    None
}}

dyon_fn!{fn count_vertex_buffers() -> usize {
    let vertex_buffers = unsafe { &*Current::<VertexBuffers>::new() };
    vertex_buffers.len()
}}

dyon_fn!{fn count_index_buffers() -> usize {
    let index_buffers = unsafe { &*Current::<IndexBuffers>::new() };
    index_buffers.len()
}}

dyon_fn!{fn create_vertex_buffer__size(size: usize) -> Result<usize, String> {
    use std::error::Error;

    let vertex_buffers = unsafe { &mut *Current::<VertexBuffers>::new() };
    let window = unsafe { &*Current::<Window>::new() };
    let n = vertex_buffers.len();
    vertex_buffers.push(try!(glium::VertexBuffer::empty(&window.context, size).map_err(|err|
        String::from(err.description())
    )));
    Ok(n)
}}

dyon_fn!{fn create_index_buffer__size(size: usize) -> Result<usize, String> {
    use std::error::Error;

    let index_buffers = unsafe { &mut *Current::<IndexBuffers>::new() };
    let window = unsafe { &*Current::<Window>::new() };
    let n = index_buffers.len();
    index_buffers.push(try!(glium::IndexBuffer::empty(
        &window.context, glium::index::PrimitiveType::TrianglesList, size).map_err(|err| {
            String::from(err.description())
        })));
    Ok(n)
}}

dyon_fn!{fn fill_vertex_buffer__buffer_pos_norm
    (buffer: usize, pos: Vec<Vec4>, norm: Vec<Vec4>) {
    let vertex_buffers = unsafe { &mut *Current::<VertexBuffers>::new() };

    let n = pos.len();
    let slice = vertex_buffers[buffer].slice(0..n).unwrap();
    slice.write({
        &(0..n).map(|i| Vertex { pos: pos[i].into(), norm: norm[i].into() }).collect::<Vec<_>>()
    });
}}

dyon_fn!{fn fill_index_buffer__buffer_data(buffer: usize, data: Vec<u32>) {
    let index_buffers = unsafe { &*Current::<IndexBuffers>::new() };

    index_buffers[buffer].write(&data);
}}

dyon_fn!{fn clear_depth() {
    use glium::{Frame, Surface};

    let target = unsafe { &mut *Current::<Frame>::new() };
    target.clear_depth(1.0);
}}

dyon_fn!{fn draw__program_vbuf_ibuf_pos_angle_scale_color(
    program: usize,
    vbuf: usize,
    ibuf: usize,
    pos: Vec4,
    angle: f32,
    scale: f32,
    color: Vec4
) {
    use glium::{Depth, DepthTest, Frame, Surface};
    use glium::draw_parameters::{DepthClamp, DrawParameters};
    use piston::input::{Event, RenderEvent};
    use super::math;

    let pos: [f32; 3] = pos.into();
    let pos_transform = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [pos[0], pos[1], pos[2], 1.0]
    ];
    let e = unsafe { &*Current::<Option<Event>>::new() };
    let mat: [[f32; 4]; 4] = if let Some(args) = e.as_ref().unwrap().render_args() {
        let mat: [[f32; 3]; 2] = args.viewport().abs_transform();
        let sz = -0.1;
        [
            [mat[0][0], mat[1][0], 0.0, 0.0],
            [mat[0][1], mat[1][1], 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [mat[0][2], mat[1][2], 0.0, 1.0]
        ]
    } else {
        panic!("No render event");
    };
    let programs = unsafe { &*Current::<Programs>::new() };
    let vertex_buffers = unsafe { &*Current::<VertexBuffers>::new() };
    let index_buffers = unsafe { &*Current::<IndexBuffers>::new() };
    let target = unsafe { &mut *Current::<Frame>::new() };
    let mvp = math::mul(mat, math::mul(
        pos_transform,
        math::mul(math::rotate_angle(angle), math::scale(scale))
    ));
    /*
    let mvp: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ];
    */
    target.draw(&vertex_buffers[vbuf], &index_buffers[ibuf], &programs[program].1,
        &uniform!{mvp: mvp, color: color.0}, &DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                range: (0.0, 1.0),
                clamp: DepthClamp::NoClamp,
            },
            ..Default::default()
        }).unwrap();
}}

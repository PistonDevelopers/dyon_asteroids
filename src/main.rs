extern crate glium_graphics;
#[macro_use]
extern crate glium;
extern crate piston;
#[macro_use]
extern crate dyon;
extern crate current;
extern crate dyon_interactive;
extern crate sdl2_window;
extern crate wavefront_obj;
extern crate vecmath;

use sdl2_window::Sdl2Window;
use glium_graphics::{Glium2d, GliumWindow, OpenGL};
use piston::window::WindowSettings;
use piston::input::Event;
use dyon::{error, load, Module, Runtime};
use current::CurrentGuard;

use engine::{IndexBuffers, Materials, ObjSets, Programs, VertexBuffers};

mod engine;

type Window = GliumWindow<Sdl2Window>;

fn main() {
    let opengl = OpenGL::V3_2;
    let ref mut window: Window = WindowSettings::new("Dyon: Asteroids!", [512, 512])
        .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();

    let mut runtime = Runtime::new();
    let module = match load_module() {
        None => return,
        Some(m) => m
    };

    let mut g2d = Glium2d::new(opengl, window);
    let mut e: Option<Event> = None;
    let mut target = window.draw();
    let mut materials: Materials = vec![];
    let mut obj_sets: ObjSets = vec![];
    let mut programs: Programs = vec![];
    let mut vertex_buffers: VertexBuffers = vec![];
    let mut index_buffers: IndexBuffers = vec![];

    {
        let window_guard = CurrentGuard::new(window);
        let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut e);
        let g2d_guard = CurrentGuard::new(&mut g2d);
        let target_guard = CurrentGuard::new(&mut target);
        let materials_guard = CurrentGuard::new(&mut materials);
        let obj_sets_guard = CurrentGuard::new(&mut obj_sets);
        let programs_guard = CurrentGuard::new(&mut programs);
        let vertex_buffers_guard = CurrentGuard::new(&mut vertex_buffers);
        let index_buffers_guard = CurrentGuard::new(&mut index_buffers);
        if error(runtime.run(&module)) {
            return;
        }
        drop(index_buffers_guard);
        drop(vertex_buffers_guard);
        drop(programs_guard);
        drop(obj_sets_guard);
        drop(materials_guard);
        drop(target_guard);
        drop(g2d_guard);
        drop(event_guard);
        drop(window_guard);
    }

    target.finish().unwrap();
}


fn load_module() -> Option<Module> {
    use std::sync::Arc;
    use dyon_functions::*;
    use dyon_interactive::add_functions;
    use dyon::{Lt, Module, PreludeFunction, Type};

    let mut module = Module::new();
    add_functions::<Window>(&mut module);
    module.add(Arc::new("draw".into()), draw, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::array()],
        ret: Type::Void
    });
    module.add(Arc::new("next_event".into()),
        next_event, PreludeFunction {
            lts: vec![],
            tys: vec![],
            ret: Type::Bool
        });
    engine::register_obj(&mut module);
    engine::register_shader(&mut module);
    if error(load("src/main.dyon", &mut module)) {
        None
    } else {
        Some(module)
    }
}

mod dyon_functions {
    use dyon::Runtime;
    use dyon_interactive::{draw_2d, NO_EVENT};
    use current::Current;
    use super::Window;

    pub fn draw(rt: &mut Runtime) -> Result<(), String> {
        use piston::input::*;
        use glium_graphics::Glium2d;
        use glium::Frame;

        let e = unsafe { &*Current::<Option<Event>>::new() };
        let g2d = unsafe { &mut *Current::<Glium2d>::new() };
        let target = unsafe { &mut *Current::<Frame>::new() };
        if let &Some(ref e) = e {
            if let Some(args) = e.render_args() {
                g2d.draw(target, args.viewport(), |c, g| {
                    draw_2d(rt, c, g)
                })
            } else {
                Ok(())
            }
        } else {
            Err(NO_EVENT.into())
        }
    }

    pub fn next_event(rt: &mut Runtime) -> Result<(), String> {
        use piston::input::*;
        use glium::Frame;

        let window = unsafe { &mut *Current::<Window>::new() };
        let e = unsafe { &mut *Current::<Option<Event>>::new() };
        let target = unsafe { &mut *Current::<Frame>::new() };
        if let Some(new_e) = window.next() {
            if new_e.after_render_args().is_some() {
                target.set_finish().unwrap();
                *target = window.draw();
            }
            *e = Some(new_e);
            rt.push(true);
        } else {
            *e = None;
            rt.push(false);
        }
        Ok(())
    }
}

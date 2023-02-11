#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{any::Any};
use std::ffi::{CStr, c_void};
use gl33::*;
use gl33::{
    global_loader::*,
};
use beryllium::{
    events::*,
    init::*,
    Sdl,
    video::*,
};
use fermium::prelude::*;

fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    // use openGL 3.3
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();

    let win = sdl
        .create_gl_window(CreateWinArgs {
            title: "Example GL Window",
            width: 1280,
            height: 720,
            ..Default::default()
        })
        .unwrap();

    unsafe {
        let gl_loader =
            &(|f_name: *const u8| win.get_proc_address(f_name) as *const c_void)
            as &dyn Fn(*const u8) -> *const c_void; 
        load_global_gl(gl_loader);
    }

    let gl_version: String;
    unsafe {
        let c_str: &CStr = CStr::from_ptr(glGetString(GL_VERSION) as *const i8);
        match c_str.to_str() {
            Ok(str) => gl_version = String::from(str),
            Err(..) => gl_version = String::from("Unknown GL Version")
        }
    }

    println!("GL version: {}", gl_version);
    println!("GL window size: {:?}", win.get_window_size());
    println!("GL drawable size: {:?}", win.get_drawable_size());
    println!("GL_KHR_debug supported: {}", win.supports_extension("GL_KHR_debug"));

    // turn VSync on
    match win.set_swap_interval(GlSwapInterval::Vsync)  {
        Ok(..) => println!("Vsync: on"),
        Err(..) => println!("Vsync: unable to turn on!")
    };
    unsafe {
        glClearColor(0.1, 0.2, 0.3, 1.0);
    }

    let mut controllers = Vec::new();

    '_loop: loop {
        #[allow(clippy::never_loop)]
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break '_loop,
                Event::ControllerAdded { index } => {
                    match sdl.open_game_controller(index) {
                        Ok(controller) => {
                            println!(
                                "Opened `{name}` (type: {type_:?})",
                                name = controller.get_name(),
                                type_ = controller.type_id()
                            );
                            controllers.push(controller);
                        }
                        Err(msg) => println!("Couldn't open {index}: {msg:?}"),
                    }
                },
                Event::JoystickAdded { .. } | Event::ControllerAxis { .. } |
                Event::MouseMotion { .. } => (),
                Event::Key { pressed, keycode, .. } => {
                    if pressed && keycode == SDLK_ESCAPE {
                        break '_loop;
                    }  
                },
                _ => println!("{event:?}"),
            }
        }

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
        }

        // draw stuff here...

        win.swap_window();
    }
}

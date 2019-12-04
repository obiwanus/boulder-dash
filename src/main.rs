extern crate gl;
extern crate sdl2;

#[macro_use]
extern crate failure;

pub mod render_gl;
pub mod resources;

use failure::Error;
use gl::types::{GLint, GLsizeiptr, GLuint, GLvoid};
use resources::Resources;
use std::path::Path;

use render_gl::Program;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let sdl = sdl2::init().map_err(failure::err_msg)?;
    let video_subsystem = sdl.video().map_err(failure::err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Boulder Dash", 900, 700)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().map_err(failure::err_msg)?;
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,      0.0, 0.0, 1.0,
    ];
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                            // target
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const GLvoid,                          // pointer to data
            gl::STATIC_DRAW,                                             // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
    }
    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // layout (location = 0) in vertex shader
        gl::VertexAttribPointer(
            0,                                         // index of the generic vertex attribute
            3,         // number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalised (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as GLint, // stride
            std::ptr::null(), // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // layout (location = 1) in vertex shader
        gl::VertexAttribPointer(
            1,                                                 // index of the generic vertex attribute
            3,         // number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalised (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as GLint, // stride
            (3 * std::mem::size_of::<f32>()) as *const GLvoid, // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let shader_program = Program::from_res(&res, "shaders/triangle")?;

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0, // starting index in the enabled arrays
                3, // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }

    Ok(())
}

fn failure_to_string(e: Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "    Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}

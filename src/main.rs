use gl::types::*;
use std::time::SystemTime;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate stb_image;

use stb_image::image::{self, LoadResult};

#[macro_use]
extern crate failure;

mod shader;
use shader::Program;

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error_into_string(error));
        std::process::exit(1);
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Boulder Dash", 1024, 768)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    println!(
        "Swap interval: {:?}",
        video_subsystem.gl_get_swap_interval()
    );

    unsafe {
        gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        // positions        // tex coords
        0.5, 0.5, 0.0,      0.5, 0.5,       // top right
        0.5, -0.5, 0.0,     0.5, -0.5,      // bottom right
       -0.5, -0.5, 0.0,    -0.5, -0.5,      // bottom left
       -0.5, 0.5, 0.0,     -0.5, 0.5,       // top left
    ];
    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3,
    ];

    let mut vbo_triangle: GLuint = 0;
    let mut vao_triangle: GLuint = 0;
    let mut ebo_triangle: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo_triangle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_triangle);
        gl::GenVertexArrays(1, &mut vao_triangle);
        gl::BindVertexArray(vao_triangle);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        // Positions
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // Texture coordinates
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);

        // Element buffer
        gl::GenBuffers(1, &mut ebo_triangle);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_triangle);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
        gl::BindVertexArray(0);
    }

    unsafe {
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }

    // Load texture 0
    let texture0 = match image::load_with_depth("assets/textures/wall.jpg", 3, false) {
        LoadResult::ImageU8(image) => image,
        LoadResult::ImageF32(_) => panic!("Image format F32 is not supported"),
        LoadResult::Error(msg) => panic!("Couldn't load texture: {}", msg),
    };

    let mut texture0_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture0_id);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture0_id);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            texture0.width as GLint,
            texture0.height as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture0.data.as_ptr() as *const std::ffi::c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    // Load texture 2
    let texture1 = match image::load_with_depth("assets/textures/awesomeface.png", 3, false) {
        LoadResult::ImageU8(image) => image,
        LoadResult::ImageF32(_) => panic!("Image format F32 is not supported"),
        LoadResult::Error(msg) => panic!("Couldn't load texture: {}", msg),
    };

    let mut texture1_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture1_id);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture1_id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            texture1.width as GLint,
            texture1.height as GLint,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            texture1.data.as_ptr() as *const std::ffi::c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    let triangle_program = Program::new()
        .vertex_shader("assets/shaders/triangle/triangle.vert")?
        .fragment_shader("assets/shaders/triangle/triangle.frag")?
        .link()?;
    triangle_program.set_used();

    // Uniforms
    let vertex_x_offset = triangle_program.get_uniform_location("x_offset");
    let vertex_trans = triangle_program.get_uniform_location("trans");

    // Set texture uniforms
    let texture0_location = triangle_program.get_uniform_location("texture0");
    let texture1_location = triangle_program.get_uniform_location("texture1");
    unsafe {
        gl::Uniform1i(texture0_location, 0);
        gl::Uniform1i(texture1_location, 1);
    }

    let start_timestamp = SystemTime::now();

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

        let now = SystemTime::now()
            .duration_since(start_timestamp)?
            .as_secs_f32();

        let x_offset = 0.0; //now.sin() / 2.0;

        let trans = glm::translation(&glm::vec3(0.0, 0.3, 0.0));
        let trans = trans * glm::rotation(now * glm::pi::<f32>(), &glm::vec3(0.0, 0.0, 1.0));

        unsafe {
            gl::Uniform1f(vertex_x_offset, x_offset);
            gl::UniformMatrix4fv(vertex_trans, 1, gl::FALSE, trans.as_ptr());
            gl::BindVertexArray(vao_triangle);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_triangle);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }

    Ok(())
}

fn error_into_string(err: failure::Error) -> String {
    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        if let Some(backtrace) = next.backtrace() {
            pretty.push_str(&backtrace.to_string());
        }
        prev = next;
    }
    pretty
}

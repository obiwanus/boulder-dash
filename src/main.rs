use gl::types::*;
use std::f32::consts::PI;
use std::time::SystemTime;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate stb_image;

use sdl2::keyboard::Scancode;

#[macro_use]
extern crate failure;

mod shader;
use shader::Program;

mod texture;
use texture::Texture;

mod camera;
use camera::Camera;
use camera::Movement::*;

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
    gl_attr.set_depth_size(16);
    gl_attr.set_double_buffer(true);

    const SCREEN_WIDTH: f32 = 1024.0;
    const SCREEN_HEIGHT: f32 = 768.0;

    let window = video_subsystem
        .window("Boulder Dash", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
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
    sdl.mouse().set_relative_mouse_mode(true);

    unsafe {
        gl::Viewport(0, 0, 1024, 768);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        // positions        // tex coords
        0.5, 0.5, 0.5,      0.5, 0.5,       // 0
        0.5, -0.5, 0.5,     0.5, -0.5,      // 1
       -0.5, 0.5, 0.5,     -0.5, 0.5,       // 3
        0.5, -0.5, 0.5,     0.5, -0.5,      // 1
       -0.5, -0.5, 0.5,    -0.5, -0.5,      // 2
       -0.5, 0.5, 0.5,     -0.5, 0.5,       // 3

       -0.5, 0.5, -0.5,     0.5, 0.5,       // 7
        0.5, 0.5, -0.5,     0.5, -0.5,      // 4
       -0.5, -0.5, -0.5,   -0.5, 0.5,       // 6
       -0.5, -0.5, -0.5,    0.5, -0.5,      // 6
        0.5, -0.5, -0.5,   -0.5, -0.5,      // 5
        0.5, 0.5, -0.5,    -0.5, 0.5,       // 4

        0.5, 0.5, -0.5,     0.5, 0.5,       // 4
        0.5, -0.5, -0.5,    0.5, -0.5,      // 5
        0.5, 0.5, 0.5,     -0.5, 0.5,       // 0
        0.5, -0.5, -0.5,    0.5, -0.5,      // 5
        0.5, -0.5, 0.5,    -0.5, -0.5,      // 1
        0.5, 0.5, 0.5,     -0.5, 0.5,       // 0

       -0.5, 0.5, 0.5,      0.5, 0.5,       // 3
       -0.5, -0.5, 0.5,     0.5, -0.5,      // 2
       -0.5, 0.5, -0.5,    -0.5, 0.5,       // 7
       -0.5, -0.5, 0.5,     0.5, -0.5,      // 2
       -0.5, -0.5, -0.5,   -0.5, -0.5,      // 6
       -0.5, 0.5, -0.5,    -0.5, 0.5,       // 7

        0.5, 0.5, -0.5,     0.5, 0.5,       // 4
        0.5, 0.5, 0.5,      0.5, -0.5,      // 0
       -0.5, 0.5, -0.5,    -0.5, 0.5,       // 7
        0.5, 0.5, 0.5,      0.5, -0.5,      // 0
       -0.5, 0.5, 0.5,     -0.5, -0.5,      // 3
       -0.5, 0.5, -0.5,    -0.5, 0.5,       // 7

        0.5, -0.5, 0.5,     0.5, 0.5,       // 1
        0.5, -0.5, -0.5,    0.5, -0.5,      // 5
       -0.5, -0.5, 0.5,    -0.5, 0.5,       // 2
        0.5, -0.5, -0.5,    0.5, -0.5,      // 5
       -0.5, -0.5, -0.5,   -0.5, -0.5,      // 6
       -0.5, -0.5, 0.5,    -0.5, 0.5,       // 2
    ];
    // #[rustfmt::skip]
    // let indices: Vec<u32> = vec![
    //     0, 1, 3, // Front
    //     1, 2, 3,
    //     7, 4, 6, // Back
    //     6, 5, 4,
    //     4, 5, 0, // Right
    //     5, 1, 0,
    //     3, 2, 7, // Left
    //     2, 6, 7,
    //     4, 0, 7, // Top
    //     0, 3, 7,
    //     1, 5, 2, // Bottom
    //     5, 6, 2,
    // ];

    let mut vbo_triangle: GLuint = 0;
    let mut vao_triangle: GLuint = 0;
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

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
        gl::BindVertexArray(0);
    }

    // let mut vao

    let wall_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/wall.jpg")?;
    let face_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/awesomeface.png")?;

    let triangle_program = Program::new()
        .vertex_shader("assets/shaders/triangle/triangle.vert")?
        .fragment_shader("assets/shaders/triangle/triangle.frag")?
        .link()?;
    triangle_program.set_used();
    triangle_program.set_texture_uniform("wall", 0);
    triangle_program.set_texture_uniform("face", 1);

    // Uniforms
    let vertex_proj = triangle_program.get_uniform_location("proj");
    let vertex_view = triangle_program.get_uniform_location("view");
    let vertex_model = triangle_program.get_uniform_location("model");

    let model = glm::rotation(-0.25 * PI, &glm::vec3(0.0, 0.0, 1.0));

    let cube_positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let mut camera = Camera::new()
        .set_position(glm::vec3(0.0, 0.0, 10.0))
        .set_aspect_ratio(SCREEN_WIDTH / SCREEN_HEIGHT)
        .look_at(glm::vec3(0.0, 0.0, 0.0));

    let start_timestamp = SystemTime::now();
    let mut frame_start = SystemTime::now();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        let now = SystemTime::now();
        let delta_time = now.duration_since(frame_start).unwrap().as_secs_f32();
        frame_start = now;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseWheel { y, .. } => camera.adjust_zoom(y),
                _ => {}
            }
        }

        // Look around
        let mouse_state = event_pump.relative_mouse_state();
        camera.rotate(mouse_state.x(), mouse_state.y());

        // Move camera
        let keyboard = event_pump.keyboard_state();
        if keyboard.is_scancode_pressed(Scancode::W) {
            camera.go(Forward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            camera.go(Backward, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            camera.go(Left, delta_time);
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            camera.go(Right, delta_time);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindVertexArray(vao_triangle);
        }

        // Transformations
        let proj = camera.get_projection_matrix();
        let view = camera.get_view_matrix();
        unsafe {
            gl::UniformMatrix4fv(vertex_proj, 1, gl::FALSE, proj.as_ptr());
            gl::UniformMatrix4fv(vertex_view, 1, gl::FALSE, view.as_ptr());
        }

        // Draw rotating cubes
        wall_texture.bind(0);
        face_texture.bind(1);
        let seconds_elapsed = SystemTime::now()
            .duration_since(start_timestamp)
            .unwrap()
            .as_secs_f32();
        let angle = seconds_elapsed * PI / 5.0;
        for pos in cube_positions.iter() {
            let model = glm::translate(&model, pos);
            let model = glm::rotate(&model, angle, pos); // rotate around position to get different directions
            unsafe {
                gl::UniformMatrix4fv(vertex_model, 1, gl::FALSE, model.as_ptr());
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // // Rendering time
        // let render_ms = SystemTime::now()
        //     .duration_since(frame_start)
        //     .unwrap()
        //     .as_micros() as f32
        //     / 1000.0;
        // println!("rendering time: {} ms", render_ms);

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

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

mod buffers;
use buffers::{VertexArray, VertexBuffer};

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

    let window = video_subsystem
        .window("Boulder Dash", 1024, 768)
        .opengl()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let (window_width, window_height) = window.size();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    println!(
        "Swap interval: {:?}",
        video_subsystem.gl_get_swap_interval()
    );
    sdl.mouse().set_relative_mouse_mode(true);

    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.05, 0.05, 0.05, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    #[rustfmt::skip]
    let cube_vertices: Vec<f32> = vec![
        // positions        // tex coords   // normals
        0.5, 0.5, 0.5,      1.0, 1.0,       0.0, 0.0, 1.0,      // 0
        0.5, -0.5, 0.5,     1.0, 0.0,       0.0, 0.0, 1.0,      // 1
       -0.5, 0.5, 0.5,      0.0, 1.0,       0.0, 0.0, 1.0,      // 3
        0.5, -0.5, 0.5,     1.0, 0.0,       0.0, 0.0, 1.0,      // 1
       -0.5, -0.5, 0.5,     0.0, 0.0,       0.0, 0.0, 1.0,      // 2
       -0.5, 0.5, 0.5,      0.0, 1.0,       0.0, 0.0, 1.0,      // 3

       -0.5, 0.5, -0.5,     1.0, 1.0,       0.0, 0.0, -1.0,     // 7
        0.5, 0.5, -0.5,     1.0, 0.0,       0.0, 0.0, -1.0,     // 4
       -0.5, -0.5, -0.5,    0.0, 1.0,       0.0, 0.0, -1.0,     // 6
       -0.5, -0.5, -0.5,    1.0, 0.0,       0.0, 0.0, -1.0,     // 6
        0.5, -0.5, -0.5,    0.0, 0.0,       0.0, 0.0, -1.0,     // 5
        0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 0.0, -1.0,     // 4

        0.5, 0.5, -0.5,     1.0, 1.0,       1.0, 0.0, 0.0,      // 4
        0.5, -0.5, -0.5,    1.0, 0.0,       1.0, 0.0, 0.0,      // 5
        0.5, 0.5, 0.5,      0.0, 1.0,       1.0, 0.0, 0.0,      // 0
        0.5, -0.5, -0.5,    1.0, 0.0,       1.0, 0.0, 0.0,      // 5
        0.5, -0.5, 0.5,     0.0, 0.0,       1.0, 0.0, 0.0,      // 1
        0.5, 0.5, 0.5,      0.0, 1.0,       1.0, 0.0, 0.0,      // 0

       -0.5, 0.5, 0.5,      1.0, 1.0,      -1.0, 0.0, 0.0,      // 3
       -0.5, -0.5, 0.5,     1.0, 0.0,      -1.0, 0.0, 0.0,      // 2
       -0.5, 0.5, -0.5,     0.0, 1.0,      -1.0, 0.0, 0.0,      // 7
       -0.5, -0.5, 0.5,     1.0, 0.0,      -1.0, 0.0, 0.0,      // 2
       -0.5, -0.5, -0.5,    0.0, 0.0,      -1.0, 0.0, 0.0,      // 6
       -0.5, 0.5, -0.5,     0.0, 1.0,      -1.0, 0.0, 0.0,      // 7

        0.5, 0.5, -0.5,     1.0, 1.0,       0.0, 1.0, 0.0,      // 4
        0.5, 0.5, 0.5,      1.0, 0.0,       0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 1.0, 0.0,      // 7
        0.5, 0.5, 0.5,      1.0, 0.0,       0.0, 1.0, 0.0,      // 0
       -0.5, 0.5, 0.5,      0.0, 0.0,       0.0, 1.0, 0.0,      // 3
       -0.5, 0.5, -0.5,     0.0, 1.0,       0.0, 1.0, 0.0,      // 7

        0.5, -0.5, 0.5,     1.0, 1.0,       0.0, -1.0, 0.0,     // 1
        0.5, -0.5, -0.5,    1.0, 0.0,       0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, 0.5,     0.0, 1.0,       0.0, -1.0, 0.0,     // 2
        0.5, -0.5, -0.5,    1.0, 0.0,       0.0, -1.0, 0.0,     // 5
       -0.5, -0.5, -0.5,    0.0, 0.0,       0.0, -1.0, 0.0,     // 6
       -0.5, -0.5, 0.5,     0.0, 1.0,       0.0, -1.0, 0.0,     // 2
    ];

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

    let mut light_position = glm::vec3(0.0, 1.0, 0.0);
    let cube_model = glm::rotation(-0.25 * PI, &glm::vec3(0.0, 0.0, 1.0));

    // Buffers
    let stride = 8;
    let mut cube = VertexBuffer::new();
    cube.bind();
    cube.set_static_data(&cube_vertices, stride);
    let cube_vao = VertexArray::new();
    cube_vao.bind();
    cube_vao.set_attrib(0, 3, stride, 0); // Positions
    cube_vao.set_attrib(1, 2, stride, 3); // Texture coords
    cube_vao.set_attrib(2, 3, stride, 5); // Normals

    let light_vao = VertexArray::new();
    light_vao.bind();
    light_vao.set_attrib(0, 3, stride, 0);
    cube.unbind();

    let crate_texture = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/diffuse.png")?;
    let crate_specular_map = Texture::new()
        .set_default_parameters()
        .load_image("assets/textures/crate/specular.png")?;

    // Cube shader
    let cube_shader = Program::new()
        .vertex_shader("assets/shaders/cube/cube.vert")?
        .fragment_shader("assets/shaders/cube/cube.frag")?
        .link()?;
    cube_shader.set_used();
    // Set default material
    cube_shader.set_texture_unit("material.diffuse", 0)?;
    cube_shader.set_texture_unit("material.specular", 1)?;
    cube_shader.set_float("material.shininess", 32.0)?;

    let light_color = glm::vec3(1.0, 1.0, 1.0);
    cube_shader.set_vec3("light.ambient", &(0.2 * light_color))?;
    cube_shader.set_vec3("light.diffuse", &(0.5 * light_color))?;
    cube_shader.set_vec3("light.specular", &(1.0 * light_color))?;
    cube_shader.set_float("light.attn_linear", 0.09)?;
    cube_shader.set_float("light.attn_quadratic", 0.032)?;

    crate_texture.bind(0);
    crate_specular_map.bind(1);

    // Light shader
    let light_shader = Program::new()
        .vertex_shader("assets/shaders/light/light.vert")?
        .fragment_shader("assets/shaders/light/light.frag")?
        .link()?;

    let mut camera = Camera::new();
    camera.aspect_ratio = (window_width as f32) / (window_height as f32);
    camera.position = glm::vec3(0.0, 2.0, 5.0);
    camera.look_at(glm::vec3(0.0, 2.0, 0.0));

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
        if keyboard.is_scancode_pressed(Scancode::Escape) {
            break 'main;
        }
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
        }

        // Time for rotations etc
        let seconds_elapsed = SystemTime::now()
            .duration_since(start_timestamp)
            .unwrap()
            .as_secs_f32();

        // Transformations
        let proj = camera.get_projection_matrix();
        let view = camera.get_view_matrix();

        // Light cube
        let x_max = 2.0;
        let z_max = 6.0;
        light_position.x = x_max * (seconds_elapsed * 3.0).sin();
        light_position.z = z_max * seconds_elapsed.cos() - 5.0;
        let light_model = glm::translation(&light_position);
        let light_model = glm::scale(&light_model, &glm::vec3(0.1, 0.1, 0.1));

        // Draw light cube
        light_shader.set_used();
        light_shader.set_mat4("proj", &proj)?;
        light_shader.set_mat4("view", &view)?;
        light_shader.set_mat4("model", &light_model)?;
        light_shader.set_vec3("light_color", &light_color)?;
        light_vao.bind();
        cube.draw_triangles();

        // Draw rotating cubes
        cube_shader.set_used();
        cube_shader.set_mat4("proj", &proj)?;
        cube_shader.set_mat4("view", &view)?;
        // Put light into the view space
        let light_pos = glm::vec4_to_vec3(
            &(view * glm::vec4(light_position.x, light_position.y, light_position.z, 1.0)),
        );
        cube_shader.set_vec3("light.position", &light_pos)?;

        cube_vao.bind();

        let angle = seconds_elapsed * PI / 5.0;
        for pos in cube_positions.iter() {
            let cube_model = glm::translate(&cube_model, pos);
            let cube_model = glm::rotate(&cube_model, angle, pos); // rotate around position to get different directions
            cube_shader.set_mat4("model", &cube_model)?;

            cube.draw_triangles();
        }

        #[cfg(feature = "debug")]
        {
            // Display rendering time
            let render_ms = SystemTime::now()
                .duration_since(frame_start)
                .unwrap()
                .as_micros() as f32
                / 1000.0;
            println!("rendering time: {} ms", render_ms);
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

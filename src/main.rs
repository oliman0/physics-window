#![windows_subsystem = "windows"]
#![allow(dead_code)]

use std::fs::File;
use std::io::{ Write, BufReader, prelude::* };
use std::i32;

use window::window;

use glfw::ffi as glfw;

mod window;

fn main() {
    if !std::path::Path::new("./windowfun_config.txt").is_file() {
        let mut file = File::create("./windowfun_config.txt").expect("config create failed");
        file.write("Monitor Size:\n1920\n1080\nMonitors to the right of primary:\n0\nMonitors to the left of primary:\n0\nWindow Size:\n400\n400\nWindow Colour Hex (without # or 0x):\n000000\nDrag (horizontal velocity multipler when coliding with the floor):\n0.99\nHorizontal Bounce Multipler:\n0.99\nVertical Bounce Multiplier:\n0.95\nMax Velocity From Gravity:\n50.0\nGravity:\n2.0".as_bytes()).expect("write failed");
    }
    
    let config_file: Vec<String> = BufReader::new(File::open("./windowfun_config.txt").unwrap()).lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

    let scr_colour = nalgebra_glm::vec3(i32::from_str_radix(&config_file[11][0..2], 16).unwrap() as f32 / 255.0,
                                                                            i32::from_str_radix(&config_file[11][2..4], 16).unwrap() as f32 / 255.0,
                                                                            i32::from_str_radix(&config_file[11][4..6], 16).unwrap() as f32 / 255.0);

    let scr_size: nalgebra_glm::Vec2 = nalgebra_glm::vec2(config_file[1].parse().unwrap(), config_file[2].parse().unwrap());
    let window_size: nalgebra_glm::Vec2 = nalgebra_glm::vec2(config_file[8].parse().unwrap(), config_file[9].parse().unwrap());

    let monitors_r: f32 = config_file[4].parse().unwrap();
    let monitors_l: f32 = config_file[6].parse().unwrap();

    let drag: f32 =  config_file[13].parse().unwrap();
    let bounce_multiplier = nalgebra_glm::vec2(-config_file[15].parse::<f32>().unwrap(), -config_file[17].parse::<f32>().unwrap());
    let terminal_velocity: f32 = config_file[19].parse().unwrap();
    let gravitational_acceleration: f32 = config_file[21].parse().unwrap();

    let jump_height: f32 = 50.0;
    let arrowkeys_add: f32 = 20.0;

    let window = window("Physics Window", window_size.x as i32, window_size.y as i32, window_size.x as i32, window_size.y as i32, 1.0);

    let mut window_velocity = nalgebra_glm::vec2(20.0, 20.0);
    let mut window_pos: nalgebra_glm::Vec2;

    let mut hold_pos = nalgebra_glm::vec2(0.0, 0.0);
    let mut hold = false;
    let mut last_pos = nalgebra_glm::vec2(0.0, 0.0);

    while !window.should_close() {
        window_pos = window.get_window_position() + window_velocity;
        window.set_window_position(&window_pos);

        if window.get_mouse_button(glfw::MOUSE_BUTTON_1) {
            window_velocity = nalgebra_glm::vec2(0.0, 0.0);

            let mouse_pos = window.get_mouse_position_raw();
            if !hold { hold = true; hold_pos = mouse_pos; }
            
            window.set_window_position(&(window_pos + mouse_pos - hold_pos));
            last_pos = window_pos + mouse_pos;
        }
        else {
            if hold {
                hold = false;
                window_velocity = ((window_pos + window.get_mouse_position_raw()) - last_pos) * 0.5;
            }

            if window_pos.y + window_size.y >= scr_size.y { window.set_window_position(&nalgebra_glm::vec2(window_pos.x, scr_size.y - window_size.y)); window_velocity.y *= bounce_multiplier.y; window_velocity.x *= drag; }
            if window_pos.x + window_size.x >= scr_size.x * (monitors_r + 1.0) { window.set_window_position(&nalgebra_glm::vec2((scr_size.x * (monitors_r + 1.0)) - window_size.x, window_pos.y)); window_velocity.x *= bounce_multiplier.x; }
            if window_pos.x <= -(monitors_l * scr_size.x) { window.set_window_position(&nalgebra_glm::vec2(-(monitors_l * scr_size.x), window_pos.y)); window_velocity.x *= bounce_multiplier.x; }

            if window_velocity.y < terminal_velocity { window_velocity.y += gravitational_acceleration; } 

            if window.get_key_down(glfw::KEY_SPACE) { window_velocity.y = -jump_height; }
            if window.get_key_down(glfw::KEY_RIGHT) { window_velocity.x += arrowkeys_add; }
            if window.get_key_down(glfw::KEY_LEFT) { window_velocity.x += -arrowkeys_add; }
            if window.get_key_down(glfw::KEY_Q) { window_velocity = nalgebra_glm::vec2(0.0, 0.0); }
        }

        window.poll_events();
        window.swap_buffers();

        unsafe {
            gl::ClearColor(scr_colour.x, scr_colour.y, scr_colour.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

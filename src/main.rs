#![windows_subsystem = "windows"]

use raylib::prelude::*;
use num_complex::Complex;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Complex Power 2 RS")
        .size(1000, 1000)
        .msaa_4x()
        .vsync()
        .build();


    // Set up Variables needed every loop
    let num_dots = 256;
    let mut modifier: [f32; 2] = [0.0, 0.0];
    let mut c = Complex::new(0.0, 0.0);
    let mut iterations: Vec<Complex<f32>> = Vec::with_capacity(num_dots);

    for _ in 0..num_dots {
        iterations.push(Complex::new(0.0, 0.0));
    }

    let cam = camera::Camera2D {
        offset: Vector2::new((rl.get_screen_width() / 2) as f32, (rl.get_screen_height() / 2) as f32),
        target: Default::default(),
        rotation: 0.0,
        zoom: 1.0
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut mode_2d = d.begin_mode2D(cam);
        mode_2d.clear_background(Color::DARKGRAY);

        let s_width = mode_2d.get_screen_width();
        let s_height = mode_2d.get_screen_height();
        let s_middle = s_width / 2;
        let mouse_pos = mode_2d.get_mouse_position();

        // Mouse Circle
        mode_2d.draw_circle_v(mouse_pos - Vector2::new(s_middle as f32, s_middle as f32), 5.0, Color::PINK);

        c.re = (mouse_pos.x - s_middle as f32) / 500.0;
        c.im = (mouse_pos.y - s_middle as f32) / 500.0;

        if mode_2d.is_key_down(KeyboardKey::KEY_LEFT) {
            modifier[0] -= 0.005;
        } else if mode_2d.is_key_down(KeyboardKey::KEY_RIGHT) {
            modifier[0] += 0.005;
        }

        if mode_2d.is_key_down(KeyboardKey::KEY_UP) {
            modifier[1] -= 0.005;
        } else if mode_2d.is_key_down(KeyboardKey::KEY_DOWN) {
            modifier[1] += 0.005;
        }


        iterations[0] = Complex::new(modifier[0], modifier[1]);
        iterations[0] = (iterations[0].powu(2)) + c;

        for j in 0..num_dots-1 {
            iterations[j+1] = iterations[j];
            iterations[j+1] = iterations[j+1].powu(2) + c;
        }

        mode_2d.draw_line_v(mouse_pos - s_middle as f32, Vector2::new(iterations[0].re * 500.0, iterations[0].im * 500.0), Color::BLACK);

        // Draw all dots to screen and connect them
        for i in 0..iterations.len() {
            let dot = Vector2::new(iterations[i].re * 500.0, iterations[i].im * 500.0);
            mode_2d.draw_circle_v(dot, 5.0, Color::BLUE);
            if i < iterations.len()-1 {
                let next_dot = Vector2::new(iterations[i+1].re * 500.0, iterations[i+1].im * 500.0);
                mode_2d.draw_line_v(dot, next_dot, Color::BLACK);
            }
        }


        // Draw Keyboard-controller Modifier
        mode_2d.draw_circle_v(Vector2::new(modifier[0] * 500.0, modifier[1] * 500.0), 8.0, Color::RED);


        // Coordiante System
        mode_2d.draw_line(0, -s_middle, 0, s_height - s_middle, Color::WHITE);
        mode_2d.draw_line(-s_middle, 0, s_width - s_middle, 0, Color::WHITE);
        mode_2d.draw_circle((s_width / 2) - s_middle, (s_height / 2) - s_middle, 3.0, Color::WHITE);

        // Outer Circle
        mode_2d.draw_circle_lines((s_width / 2) - s_middle, (s_height / 2) - s_middle, (s_width / 2) as f32, Color::WHITE);
    }
}

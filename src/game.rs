
extern crate shader_version;
extern crate sdl2_window;

use opengl_graphics::{ Gl };
use graphics::*;
use event::{ RenderArgs, UpdateArgs };
use input::{ InputEvent };

pub struct Game {
    gl: Gl,
}

impl Game {
    pub fn new() -> Game {
        let opengl = shader_version::opengl::OpenGL_3_2;

        Game {
            gl: Gl::new(opengl),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);

        let c = Context::abs(args.width as f64, args.height as f64);
        c.rgb(1.0, 0.4, 1.0).draw(&mut self.gl);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {

    }

    pub fn input(&mut self, args: &InputEvent) {
        println!("Input: {}", args);
    }
}


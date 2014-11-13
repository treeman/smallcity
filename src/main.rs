
#![feature(globs)]

extern crate shader_version;
extern crate input;
extern crate sprite;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate piston;

use std::cell::RefCell;
//use std::rc::Rc;

use event::{ Events, WindowSettings };
use sprite::*;

use graphics::*;
use event::*;

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    Gl,
    //Texture,
};

fn main() {
    let (width, height) = (300, 300);
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "SmallCity".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut scene = Scene::new();

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 0.4, 1.0).draw(gl);

                scene.draw(&c, gl);
            },
            Update(args) => {

            },
            Input(x) => {
                //println!("{}", x);
            },
        }
        //scene.event(&e);

        //e.render(|args| {
            //gl.viewport(0, 0, args.width as i32, args.height as i32);

            //let c = Context::abs(args.width as f64, args.height as f64);
            //c.rgb(1.0, 1.0, 1.0).draw(gl);

            //scene.draw(&c, gl);
        //});
    }
}



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

use event::Event::{ Render, Update, Input };
use event::{ Events, WindowSettings };
use sdl2_window::Sdl2Window;

use game::*;

mod game;

fn main() {
    let window = Sdl2Window::new(
        shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "SmallCity".to_string(),
            size: [600, 400],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );
    let mut game = Game::new();
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        match e {
            Render(ref args) => game.render(args),
            Update(ref args) => game.update(args),
            Input(ref args) => game.input(args),
        }
    }
}


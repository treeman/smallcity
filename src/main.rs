
#![feature(globs)]

extern crate shader_version;
extern crate sprite;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate ai_behavior;
extern crate piston;

use std::cell::RefCell;

use piston::event::Event::{ Render, Update, Input, Idle };
use piston::event::{ Events };
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;
use shader_version::OpenGL;

use game::*;

mod game;

fn main() {
    let window = Sdl2Window::new(
        OpenGL::_3_2,
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
            Idle(_) => {},
        }
    }
}


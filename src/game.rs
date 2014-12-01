
extern crate shader_version;
extern crate sdl2_window;

use opengl_graphics::{ Gl, Texture };
use graphics::*;
use event::{ Event, RenderArgs, UpdateArgs };
use event::Event::{ Render, Update, Input };
use input::*;
use input::keyboard::Key;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
    //Behavior,
};

use std::rc::Rc;

pub struct Game {
    gl: Gl,
    rot: f64,
    //sprite: Sprite<Texture>,
    scene: Scene<Texture>,
}

impl Game {
    pub fn new() -> Game {
        let opengl = shader_version::opengl::OpenGL_3_2;

        let mut scene = Scene::new();
        let tex = Path::new("./bin/assets/unsupported.jpg");
        let tex = Rc::new(Texture::from_path(&tex).unwrap());
        let mut sprite = Sprite::from_texture(tex.clone());
        //let image = Texture::from_path(&image).unwrap();

        sprite.set_position(100.0, 100.0);

        //let x: int = sprite;

        let id = scene.add_child(sprite);
        println!("{}", id);

        let seq = Sequence(vec![
            Action(Ease(EaseCubicOut, box ScaleTo(2.0, 0.5, 0.5))),
            Action(Ease(EaseBounceOut, box MoveBy(1.0, 0.0, 100.0))),
            Action(Ease(EaseElasticOut, box MoveBy(2.0, 0.0, -100.0))),
            Action(Ease(EaseBackInOut, box MoveBy(1.0, 0.0, -100.0))),
            Wait(0.5),
            Action(Ease(EaseExponentialInOut, box MoveBy(1.0, 0.0, 100.0))),
            Action(Blink(1.0, 5)),
            While(box WaitForever, vec![
                Action(Ease(EaseQuadraticIn, box FadeOut(1.0))),
                Action(Ease(EaseQuadraticOut, box FadeIn(1.0))),
                ]),
        ]);
        scene.run(id, &seq);

        let rotate = Action(Ease(EaseExponentialInOut, box RotateTo(2.0, 360.0)));
        scene.run(id, &rotate);


        Game {
            gl: Gl::new(opengl),
            rot: 0.0,
            //sprite: sprite,
            scene: scene,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);

        let c = Context::abs(args.width as f64, args.height as f64);
        c.color([1.0, 1.0, 1.0, 1.0]).draw(&mut self.gl);

        //self.sprite.draw(&c, &mut self.gl);

        self.scene.draw(&c, &mut self.gl);

        //let x = 20.0;
        //let y = x;

        //c.trans(x, y)
         //.trans((args.width / 2) as f64, (args.height / 2) as f64)
         //.rot_rad(self.rot)
         //.trans(-x, -y)
         //.image(&image)
         //.draw(&mut self.gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let e: Event = Update(*args);
        self.scene.event(&e);

        self.rot += 2.0 * args.dt;
    }

    // http://www.rust-ci.org/PistonDevelopers/piston/doc/piston/input/enum.InputEvent.html
    pub fn input(&mut self, args: &InputEvent) {
        self.scene.event(args);

        //println!("Input: {}", args);
        match args {
            //&Move(MouseCursor(x, y)) => println!("Mouse pos at: {},{}", x, y),
            &Press(Keyboard(Key::A)) => println!("key 'A' pressed!"),
            &Press(Keyboard(x)) => println!("Other key {} pressed!", x),
            _ => {},
        }
    }
}


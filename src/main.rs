mod tree;

use crate::tree::Tree;
use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event::ScanCode;
use ggez::graphics::{Color, DrawParam, Mesh, Rect};
use ggez::input::keyboard::KeyInput;
use ggez::mint::Point2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{
    event,
    graphics::{self},
    Context, GameError, GameResult,
};
use std::time::Instant;

struct State {
    mesh: Mesh,
    tree: Tree,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let tree = Tree::new();

        Ok(State {
            mesh: tree.generate_mesh(ctx),
            tree,
        })
    }
}

impl event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        canvas.set_screen_coordinates(Rect::new(0.0, 720.0, 1280.0, -720.0));

        canvas.draw(
            &self.mesh,
            DrawParam::default().dest(Point2 { x: 640.0, y: 0.0 }),
        );

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        match input.keycode {
            None => {}
            Some(code) => {
                if code == VirtualKeyCode::Space {
                    let instant = Instant::now();
                    self.tree.add_sub_branch();
                    println!("adding {}", instant.elapsed().as_secs_f64());
                    let instant = Instant::now();
                    self.mesh = self.tree.generate_mesh(ctx);
                    println!("meshing {}", instant.elapsed().as_secs_f64());
                }
            }
        };

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("fractal Tree", "ToBinio")
        .window_mode(WindowMode::default().resizable(true))
        .window_setup(WindowSetup::default().samples(NumSamples::Four));

    let (mut ctx, event_loop) = cb.build()?;
    let state = State::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

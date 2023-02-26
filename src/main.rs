use ggez::graphics::{Color, DrawParam, Mesh, Rect};
use ggez::mint::Point2;
use ggez::{
    event,
    graphics::{self},
    Context, GameError, GameResult,
};

struct State {
    mesh: Mesh,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            mesh: State::generate_mesh(ctx)?,
        })
    }

    fn generate_mesh(ctx: &mut Context) -> Result<Mesh, GameError> {
        Mesh::new_line(
            ctx,
            &vec![Point2 { x: 0.0, y: 0.0 }, Point2 { x: 0.0, y: 100.0 }],
            2.0,
            Color::YELLOW,
        )
    }
}

impl event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.set_screen_coordinates(Rect::new(0.0, 720.0, 1280.0, -720.0));

        canvas.draw(
            &self.mesh,
            DrawParam::default().dest(Point2 {
                x: 640.0,
                y: 0.0,
            }),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("fractal Tree", "ToBinio");
    let (mut ctx, event_loop) = cb.build()?;
    let state = State::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

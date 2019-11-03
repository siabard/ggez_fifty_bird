use ggez::graphics;
use ggez::nalgebra as na;
use ggez::*;
use rand::*;

pub const PIPE_SCROLL: f32 = -90.;
pub const PIPE_HEIGHT: f32 = 288.;
pub const PIPE_WIDTH: f32 = 70.;
#[derive(Clone)]
pub struct Pipe {
    image: graphics::Image,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub gap: f32,
}

impl Pipe {
    pub fn new(image: graphics::Image, x: f32, y: f32, gap: f32) -> GameResult<Pipe> {
        let mut rng = rand::thread_rng();

        let y = rng.gen_range((y / 4.0) as i32, (y - 10.0) as i32) as f32;
        let width = image.width() as f32;

        Ok(Pipe {
            image,
            x,
            y,
            width,
            gap,
        })
    }

    pub fn update(&mut self, ctx: &mut ggez::Context, dt: f32) {
        self.x = self.x + PIPE_SCROLL * dt;
    }

    pub fn render(&self, ctx: &mut ggez::Context) {
        graphics::draw(ctx, &self.image, (na::Point2::new(self.x, self.y),));
        graphics::draw(
            ctx,
            &self.image,
            (
                na::Point2::new(self.x + PIPE_WIDTH, self.y - self.gap),
                3.14,
                graphics::WHITE,
            ),
        );
    }
}

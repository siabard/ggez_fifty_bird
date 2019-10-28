use ggez::graphics;
use ggez::nalgebra as na;
use ggez::*;

pub struct Bird {
    image: ggez::graphics::Image,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    dy: f32,
}

impl Bird {
    pub fn new(ctx: &mut ggez::Context, max_x: f32, max_y: f32) -> GameResult<Bird> {
        let image = ggez::graphics::Image::new(ctx, "/bird.png")?;
        let width = image.width() as f32;
        let height = image.height() as f32;
        let x = (max_x - width) / 2.;
        let y = (max_y - height) / 2.;
        Ok(Bird {
            image,
            width,
            height,
            x,
            y,
            dy: 0.,
        })
    }

    pub fn render(&self, ctx: &mut ggez::Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.image,
            (na::Point2::new(self.x, self.y), 0.0, graphics::WHITE),
        )?;
        Ok(())
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context, dt: f32) -> GameResult {
        self.dy = self.dy + super::GRAVITY * dt;

        self.y = self.y + self.dy;
        println!("{}", self.y);
        Ok(())
    }

    pub fn jump(&mut self, _ctx: &mut ggez::Context, _dt: f32) -> GameResult {
        self.dy = -5.;
        Ok(())
    }
}

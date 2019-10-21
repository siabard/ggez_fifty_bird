use ggez::graphics;
use ggez::nalgebra as na;
use ggez::*;

pub struct Bird {
    image: ggez::graphics::Image,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Bird {
    pub fn new(ctx: &mut ggez::Context, max_x: u16, max_y: u16) -> GameResult<Bird> {
        let image = ggez::graphics::Image::new(ctx, "/bird.png")?;
        let width = image.width();
        let height = image.height();
        let x = (max_x - width) / 2;
        let y = (max_y - height) / 2;
        Ok(Bird {
            image,
            width,
            height,
            x,
            y,
        })
    }

    pub fn render(&self, ctx: &mut ggez::Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.image,
            (
                na::Point2::new(self.x as f32, self.y as f32),
                0.0,
                graphics::WHITE,
            ),
        )?;
        Ok(())
    }
}

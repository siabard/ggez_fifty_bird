use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode, KeyMods};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use ggez::nalgebra as na;
use ggez_fifty_bird::*;
use std::collections::HashMap;
use std::vec::Vec;

use rand::*;

const VIRTUAL_WIDTH: f32 = 512.;
const VIRTUAL_HEIGHT: f32 = 288.;

const WINDOW_WIDTH: f32 = 1024.;
const WINDOW_HEIGHT: f32 = 576.;

const X_RATIO: f32 = WINDOW_WIDTH / VIRTUAL_WIDTH;
const Y_RATIO: f32 = WINDOW_HEIGHT / VIRTUAL_HEIGHT;

const GROUND_SPEED: f32 = 30.;
const BACKGROUND_SPEED: f32 = 60.;
const BACKGROUND_LOOPING_POS: f32 = 413.;

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_new", "author_name")
        .add_resource_path("./resources")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Pong: ggez"))
        .build()?;

    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let mut my_game = MyGame::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut my_game);
    Ok(())
}

#[derive(PartialEq)]
enum GameState {
    WAIT,
    IDLE,
    RUNNING,
    END,
}

struct MyGame {
    buffer: ggez::graphics::Canvas,
    message: ggez::graphics::Text,
    background: ggez::graphics::Image,
    ground: ggez::graphics::Image,
    background_pos_x: f32,
    ground_pos_x: f32,
    bird: ggez_fifty_bird::bird::Bird,
    pipe: graphics::Image,
    pipes: Vec<pipe::Pipe>,
    spawn_timer: f32,
    game_state: GameState,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf").unwrap();
        let message = graphics::Text::new(("Hello Pong!", font, 60.0));

        let buffer = ggez::graphics::Canvas::new(
            ctx,
            VIRTUAL_WIDTH as u16,
            VIRTUAL_HEIGHT as u16,
            ggez::conf::NumSamples::One,
        )
        .unwrap();

        //let buffer = graphics::Canvas::with_window_size(ctx)?;
        let background = graphics::Image::new(ctx, "/background.png")?;
        let ground = graphics::Image::new(ctx, "/ground.png")?;

        let bird = bird::Bird::new(ctx, VIRTUAL_WIDTH, VIRTUAL_HEIGHT)?;

        let pipe = graphics::Image::new(ctx, "/pipe.png")?;

        Ok(MyGame {
            buffer,
            message,
            background,
            ground,
            background_pos_x: 0.,
            ground_pos_x: 0.,
            bird,
            pipe,
            pipes: vec![],
            spawn_timer: 0.,
            game_state: GameState::WAIT,
        })
    }
}

impl MyGame {
    fn draw_canvas(&mut self, ctx: &mut Context, dt: f32) -> GameResult {
        graphics::set_canvas(ctx, Some(&self.buffer));

        graphics::clear(ctx, graphics::Color::from_rgba(30, 30, 0, 255));

        self.background_pos_x =
            (self.background_pos_x + BACKGROUND_SPEED * dt) % BACKGROUND_LOOPING_POS;
        self.ground_pos_x = (self.ground_pos_x + GROUND_SPEED * dt) % (VIRTUAL_WIDTH as f32);
        graphics::draw(
            ctx,
            &self.background,
            (
                na::Point2::new(-self.background_pos_x, 0.),
                0.0,
                graphics::WHITE,
            ),
        )?;

        self.bird.render(ctx);

        for pipe in self.pipes.iter() {
            pipe.render(ctx);
        }

        graphics::draw(
            ctx,
            &self.ground,
            (
                na::Point2::new(-self.ground_pos_x, VIRTUAL_HEIGHT as f32 - 16.),
                0.0,
                graphics::WHITE,
            ),
        )?;

        graphics::set_canvas(ctx, None);
        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = rand::thread_rng();

        // dt(delta) 얻어오기
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            if self.game_state == GameState::RUNNING {
                self.bird.update(ctx, dt);

                // Pipe 생성
                self.spawn_timer = self.spawn_timer + dt;
                if self.spawn_timer > 3. {
                    let gap = rng.gen_range(80, 120) as f32;

                    self.pipes.push(pipe::Pipe::new(
                        self.pipe.clone(),
                        VIRTUAL_WIDTH,
                        VIRTUAL_HEIGHT,
                        gap,
                    )?);
                    self.spawn_timer = 0.;
                }

                // 파이프 업데이트
                for pipe in self.pipes.iter_mut() {
                    pipe.update(ctx, dt);

                    self.bird.collide(&pipe);
                }
                // x 좌표가 0보다 작다면 해당 내역을 지워야하는데..
                self.pipes.retain(|p| p.x >= -p.width);
            }

            // Jump
            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.bird.jump(ctx, dt);
                self.game_state = GameState::RUNNING;
            }

            // 그려야할 스크롤의 위치를 계산하기
            self.draw_canvas(ctx, dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        let dest_point = na::Point2::new(0., 0.);
        let scale = na::Vector2::new(X_RATIO * X_RATIO, Y_RATIO * Y_RATIO);

        graphics::draw(
            ctx,
            &self.buffer,
            graphics::DrawParam::new()
                .dest(dest_point)
                .src(graphics::Rect::new(0., 0., 1., 1.))
                .scale(scale),
        )?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape || keycode == KeyCode::Q {
            ggez::event::quit(ctx);
        }
    }
}

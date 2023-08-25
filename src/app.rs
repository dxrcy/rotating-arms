use std::f32::consts::PI;

use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::miniquad::GraphicsContext;
use ggez::Context;
use ggez::GameResult;
use good_web_game::graphics::Transform;
use good_web_game::mint::{Point2, Vector2};

use crate::color;
use crate::debug::draw_debug_text;

#[derive(Default)]
pub struct App {
    frame_count: u32,
    show_debug: bool,
}

impl App {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult<Self> {
        Ok(Self::default())
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        self.frame_count += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, color!(BLACK));

        let canvas = graphics::Canvas::with_window_size(ctx, quad_ctx)?;
        let center = Point2 {
            x: canvas.width() as f32 / 2.0,
            y: canvas.height() as f32 / 2.0,
        };

        #[derive(Clone, Copy, Default)]
        struct Arm {
            width: f32,
            length: f32,
            rotation: f32,
        }

        let mut draw_arm = |child: Arm, ancestors: &[Arm], color: Color| -> GameResult<()> {
            let mut point = center;
            for ancestor in ancestors {
                point.x += ancestor.length * ancestor.rotation.cos();
                point.y += ancestor.length * ancestor.rotation.sin();
            }

            let param = DrawParam::new()
                .dest(point)
                .rotation(child.rotation - PI / 2.0);

            let points = [
                Point2 { x: 0.0, y: 0.0 },
                Point2 {
                    x: 0.0,
                    y: child.length,
                },
            ];

            let mesh = graphics::MeshBuilder::new()
                .line(&points, child.width, color)?
                .build(ctx, quad_ctx)?;
            graphics::draw(ctx, quad_ctx, &mesh, param)?;

            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                child.width / 2.0,
                0.1,
                color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

            point.x += child.length * child.rotation.cos();
            point.y += child.length * child.rotation.sin();

            let mesh = graphics::Mesh::new_circle(
                ctx,
                quad_ctx,
                DrawMode::fill(),
                point,
                child.width / 2.0,
                0.1,
                color,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

            Ok(())
        };

        let colors = [
            color!(RED),
            color!(GREEN),
            color!(BLUE),
            color!(CYAN),
            color!(YELLOW),
            color!(MAGENTA),
        ];

        let mut ancestors = Vec::new();
        for (i, color) in colors.into_iter().enumerate() {
            let alpha = (colors.len() - i) as f32;
            let omega = (i + 1) as f32;

            let arm = Arm {
                width: alpha * 3.0,
                length: alpha * 30.0,
                rotation: self.frame_count as f32 * omega.powf(1.3) / 100.0,
            };

            draw_arm(arm, &ancestors, color)?;
            ancestors.push(arm);
        }

        if self.show_debug {
            draw_debug_text(
                ctx,
                quad_ctx,
                [format!("Total frames: {}", self.frame_count)],
            )?;
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        use KeyCode::*;

        match (keymod, keycode) {
            (KeyMods::NONE, F3) => self.show_debug ^= true,
            _ => (),
        }
    }
}

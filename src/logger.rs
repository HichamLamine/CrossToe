use std::io::Write;
use crate::renderer::Area;
use crate::renderer::CursorUpdate;

use crate::renderer::GameRenderer;
use crate::renderer::Vec2;

pub enum Log {
    Info(String),
    Warning(String),
    Error(String),
}

pub struct Logger<'a, T: Write> {
    pub game_renderer: &'a mut GameRenderer<T>,
    pub area: Area,
}

impl<'a, T: Write> Logger<'a, T> {
    pub fn new(game_renderer: &'a mut GameRenderer<T>, area: Area) -> Self {
        Self {game_renderer, area}
    }

    pub fn log(&mut self, log: Log) {

        match log {
            Log::Info(info) => {
                // a shape has an Option<BorderShape>
                // self.game_renderer.update_cursor_position(CursorUpdate::MoveTo(, ()))
                todo!();
            },
            Log::Warning(_) => todo!(),
            Log::Error(_) => todo!(),
        }
    }
}

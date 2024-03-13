use crossterm::style::{PrintStyledContent, StyledContent};
use crossterm::QueueableCommand;
use crossterm::{cursor, terminal};
use std::fmt::Display;
use std::io::Write;
use std::usize;

use crate::{
    game_state::{GameState, RunMode},
    shapes::BoardShape,
};

#[derive(Clone)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct Vec2 {
    x: u16,
    y: u16,
}

impl Vec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub trait Drawable {
    fn draw<T: Write>(
        &self,
        renderer: &mut GameRenderer<T>,
        run_mode: RunMode,
        game_state: &GameState,
    ) {
    }
}

pub struct GameRenderer<T: Write> {
    pub current_x: u16,
    pub current_y: u16,
    renderable_cell: Vec<Vec2>,
    pub buffer: T,
}

pub enum CursorUpdate {
    MoveTo(u16, u16),
    MoveUp(u16),
    MoveDown(u16),
    MoveRight(u16),
    MoveLeft(u16),
}

impl<T: Write> GameRenderer<T> {
    pub fn new(buffer: T) -> Self {
        Self {
            current_x: cursor::position().unwrap().0,
            current_y: cursor::position().unwrap().1,
            buffer,
            renderable_cell: Vec::new(),
        }
    }

    pub fn render<D: Drawable>(&mut self, game_state: &GameState, shape: D) {
        shape.draw(self, RunMode::Start, &game_state);
    }
    pub fn render_ref<D: Drawable>(&mut self, game_state: &GameState, shape: &D) {
        shape.draw(self, RunMode::Start, &game_state);
    }
    pub fn center_cursor(&mut self, width: u16, height: u16) {
        let (rows, cols) = terminal::size().expect("Failed to get the size of the terminal");
        self.current_x = (rows / 2) - width;
        self.current_y = (cols / 2) - height;
        self.buffer
            .queue(cursor::MoveTo(self.current_x, self.current_y))
            .unwrap();
    }
    pub fn update_cursor_data(&mut self) {
        (self.current_x, self.current_y) =
            cursor::position().expect("Failed to retreive the current cursor x and y");
    }
    pub fn update_cursor_position(&mut self, position: CursorUpdate) {
        match position {
            CursorUpdate::MoveTo(x, y) => self
                .buffer
                .queue(cursor::MoveTo(x, y))
                .expect("Failed to move the cursor to the specefied x and y"),
            CursorUpdate::MoveUp(steps) => self
                .buffer
                .queue(cursor::MoveUp(steps))
                .expect("Failed to move the cursor up"),
            CursorUpdate::MoveDown(steps) => self
                .buffer
                .queue(cursor::MoveDown(steps))
                .expect("Failed to move the cursor down"),
            CursorUpdate::MoveLeft(steps) => self
                .buffer
                .queue(cursor::MoveLeft(steps))
                .expect("Failed to move the cursor left"),
            CursorUpdate::MoveRight(steps) => self
                .buffer
                .queue(cursor::MoveRight(steps))
                .expect("Failed to move the cursor right"),
        };
    }

    pub fn print_styled<D: Display>(&mut self, contents: Vec<StyledContent<D>>) {
        for content in contents.into_iter() {
            self.buffer
                .queue(PrintStyledContent(content))
                .expect("Failed to print styled content to the terminal");
            // self.update_cursor_position(CursorUpdate::MoveRight(0));
        }
    }

    pub fn hcenter(&mut self, area: &Area) {
        let (rows, _) = terminal::size().unwrap();
        let x = (rows / 2 - area.width / 2) as u16;
        self.update_cursor_position(CursorUpdate::MoveTo(x, area.y));
    }
    pub fn vcenter(&mut self, area: &Area) {
        let (_, cols) = terminal::size().unwrap();
        let y = (cols / 2 - area.width / 2) as u16;
        self.update_cursor_position(CursorUpdate::MoveTo(area.x, y));
    }

    fn set_renderable(&mut self, x: u16, y: u16) {
        self.renderable_cell.push(Vec2::new(x, y));
        // makes a position always renderable
    }

    pub fn is_renderable(&self, cell: Vec2) -> bool {
        self.renderable_cell
            .contains(&Vec2::new(self.current_x, self.current_y))
    }
}

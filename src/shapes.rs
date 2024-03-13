use crossterm::cursor::{self, MoveTo};
use crossterm::style::Stylize;
use std::collections::HashMap;
use std::io::Write;

use crossterm::{style, terminal};

use crate::game_state::{GameState, RunMode};
use crate::renderer::{Area, CursorUpdate, Drawable, GameRenderer, Vec2};
use crossterm::QueueableCommand;

pub struct BoardShape {
    width: u16,
    height: u16,
}
impl BoardShape {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Drawable for BoardShape {
    fn draw<T: Write>(
        &self,
        renderer: &mut GameRenderer<T>,
        run_mode: RunMode,
        game_state: &GameState,
    ) {
        for i in 0..7 {
            let (rows, cols) = terminal::size().unwrap();

            let start_row = rows / 2 - 7;
            let start_col = cols / 2 - 4;
            renderer.update_cursor_position(CursorUpdate::MoveTo(start_row, start_col + i));
            if i % 2 == 0 && i == 0
            //     || (i % 2 == 0 && matches!(run_mode, RunMode::Start))
            // // TODO: Fix this
            //     || (i % 2 == 0)
            {
                renderer
                    .buffer
                    .queue(style::Print(format!("┌───┬───┬───┐")))
                    .unwrap();
            } else if i % 2 == 0 && i == 6 {
                renderer
                    .buffer
                    .queue(style::Print(format!("└───┴───┴───┘")))
                    .unwrap();
            } else if i % 2 == 0 {
                renderer.print_styled(vec!["├───┼───┼───┤".white()])
            } else {
                for j in 0..3 {
                    if i == 1 {
                        let player_action = if game_state.board[(i + j - 1) as usize] == 'X' {
                            game_state.board[(i + j - 1) as usize].blue()
                        } else if game_state.board[(i + j - 1) as usize] == 'O' {
                            game_state.board[(i + j - 1) as usize].green()
                        } else {
                            (((i + j) as u8 + b'0') as char).black()
                        };

                        renderer
                            .buffer
                            .queue(style::Print(format!("│ ")))
                            .unwrap()
                            .queue(style::PrintStyledContent(player_action))
                            .unwrap();
                        renderer.update_cursor_position(CursorUpdate::MoveRight(0));
                    } else if i == 3 {
                        let player_action = if game_state.board[(i + j) as usize] == 'X' {
                            game_state.board[(i + j) as usize].blue()
                        } else if game_state.board[(i + j) as usize] == 'O' {
                            game_state.board[(i + j) as usize].green()
                        } else {
                            (((i + j + 1) as u8 + b'0') as char).black()
                        };
                        renderer
                            .buffer
                            .queue(style::Print(format!("│ ")))
                            .unwrap()
                            .queue(style::PrintStyledContent(player_action))
                            .unwrap();
                        renderer.update_cursor_position(CursorUpdate::MoveRight(0));
                    } else {
                        let player_action = if game_state.board[(i + j + 1) as usize] == 'X' {
                            game_state.board[(i + j + 1) as usize].blue()
                        } else if game_state.board[(i + j + 1) as usize] == 'O' {
                            game_state.board[(i + j + 1) as usize].green()
                        } else {
                            (((i + j + 2) as u8 + b'0') as char).black()
                        };
                        renderer.print_styled(vec!["│ ".to_string().white()]);
                        renderer.print_styled(vec![player_action]);
                        renderer.update_cursor_position(CursorUpdate::MoveRight(0));
                    }
                }
                renderer.buffer.queue(style::Print(format!("│"))).unwrap();
            }
        }
    }
}

// Todo work on a text ascii generator
pub struct TitleShape {}

// Drawable trait can take an area argument to draw onto it
impl Drawable for TitleShape {
    fn draw<T: Write>(
        &self,
        renderer: &mut GameRenderer<T>,
        run_mode: RunMode,
        game_state: &GameState,
        // area: &Area,
    ) {
        let (x_size, y_size) = terminal::size().unwrap();

        let shape_area = Area::new(0, 0, 58, 3);

        renderer.update_cursor_position(CursorUpdate::MoveTo(
            (x_size / 2 - 58 / 2) as u16,
            (3 / 2) as u16,
        ));
        renderer.print_styled(vec![
            "▀▀█▀▀ ▀█▀ ▒█▀▀█ 　 ▀▀█▀▀ ░█▀▀█ ▒█▀▀█ 　 ▀▀█▀▀ ▒█▀▀▀█ ▒█▀▀▀".white(),
        ]);
        renderer.update_cursor_position(CursorUpdate::MoveTo(
            (x_size / 2 - 58 / 2) as u16,
            (3 / 2 + 1) as u16,
        ));
        renderer.print_styled(vec![
            "░▒█░░ ▒█░ ▒█░░░ 　 ░▒█░░ ▒█▄▄█ ▒█░░░ 　 ░▒█░░ ▒█░░▒█ ▒█▀▀▀".white(),
        ]);
        renderer.update_cursor_position(CursorUpdate::MoveTo(
            (x_size / 2 - 58 / 2) as u16,
            (3 / 2 + 2) as u16,
        ));
        renderer.print_styled(vec![
            "░▒█░░ ▄█▄ ▒█▄▄█ 　 ░▒█░░ ▒█░▒█ ▒█▄▄█ 　 ░▒█░░ ▒█▄▄▄█ ▒█▄▄▄".white(),
        ]);
    }
}

enum Log {
    Info,
    Warning,
    Error,
}

pub struct LoggerShape {
    area: Area,
    logs: HashMap<String, Log>,
    border: Option<BorderShape>,
}

impl LoggerShape {
    pub fn new(area: Area) -> Self {
        Self {
            area,
            logs: HashMap::new(),
            border: None,
        }
    }
    // for chaining functions
    pub fn set_border(mut self, border: BorderShape) -> Self {
        self.border = Some(border);
        self
    }
    pub fn log_info(&mut self, info: String) {
        if self.logs.len() > self.area.height as usize - 1 {
            self.logs.clear();
        }
        self.logs.insert(info, Log::Info);
    }
    pub fn log_warning(&mut self, warning: String) {
        if self.logs.len() > self.area.height as usize - 1 {
            self.logs.clear();
        }
        self.logs.insert(warning, Log::Warning);
    }
    pub fn log_error(&mut self, error: String) {
        if self.logs.len() > self.area.height as usize - 1 {
            self.logs.clear();
        }
        self.logs.insert(error, Log::Error);
    }
}

impl Drawable for LoggerShape {
    fn draw<T: Write>(
        &self,
        renderer: &mut GameRenderer<T>,
        run_mode: RunMode,
        game_state: &GameState,
    ) {
        if let Some(border) = self.border.clone() {
            renderer.render(&game_state, border);
        }
        for (index, log) in self.logs.keys().enumerate() {
            renderer.update_cursor_position(CursorUpdate::MoveTo(
                self.area.x + 1,
                self.area.y + index as u16 + 1,
            ));
            let short_log = if log.len() > self.area.width as usize * 2 - 12 {
                format!("{}", &log[..self.area.width as usize * 2 - 12])
            } else {
                log.to_string()
            };
            match self.logs.get(&log.clone()).unwrap() {
                Log::Info =>
                // to check the contents of the board and the players
                {
                    renderer.print_styled(vec!["[INFO]: ".green(), short_log[..].white()])
                }

                Log::Warning => {
                    renderer.print_styled(vec!["[WARNING]: ".yellow(), short_log[..].white()])
                }
                Log::Error => renderer.print_styled(vec!["[ERROR]: ".red(), short_log[..].white()]),
            }
        }
    }
}

#[derive(Clone)]
pub struct BorderShape {
    area: Area,
}

impl BorderShape {
    pub fn new(area: Area) -> Self {
        Self { area }
    }
}

impl Drawable for BorderShape {
    fn draw<T: Write>(
        &self,
        renderer: &mut GameRenderer<T>,
        run_mode: RunMode,
        game_state: &GameState,
    ) {
        // renderer.update_cursor_position(CursorUpdate::MoveTo(4, self.area.y+ 1));
        renderer
            .buffer
            .queue(cursor::MoveTo(self.area.x, self.area.y))
            .unwrap();
        renderer.print_styled(vec![
            "┌".to_string().white(),
            "─".repeat((self.area.width as usize) * 2 - 1).white(),
            "┐".to_string().white(),
        ]);
        for h in 0..self.area.height - 1 {
            renderer.update_cursor_position(CursorUpdate::MoveTo(self.area.x, self.area.y + h + 1));
            renderer.print_styled(vec!["│".white()]);
            renderer.update_cursor_position(CursorUpdate::MoveTo(
                self.area.x + self.area.width * 2,
                self.area.y + h + 1,
            ));
            renderer.print_styled(vec!["│".white()]);
        }
        renderer.update_cursor_position(CursorUpdate::MoveTo(
            self.area.x,
            self.area.y + self.area.height,
        ));
        renderer.print_styled(vec![
            "└".to_string().white(),
            "─".repeat((self.area.width as usize) * 2 - 1).white(),
            "┘".to_string().white(),
        ]);
        // renderer.update_cursor_position(CursorUpdate::MoveTo(10, 15));
        // renderer.print_styled(vec!["─".repeat(self.area.width as usize).white()]);

        // for h in 0..self.area.height {
        //     for w in 0..self.area.width {
        //         if h == 0 || h == self.area.height - 1 {
        //             renderer.print_styled(vec!["─".repeat(self.area.width as usize).white()]);
        //         } else if w == 0 || w == self.area.width - 1 {
        //             renderer.print_styled(vec!["│".repeat(self.area.height as usize).white()]);
        //         }
        //         renderer.update_cursor_position(CursorUpdate::MoveTo(self.area.x, self.area.y + h));
        //     }
        // }
    }
}

// prompt that has a border
// // implement enums for way big medium size in area struct

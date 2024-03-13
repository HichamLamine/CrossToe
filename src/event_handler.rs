use std::time::Duration;

use crossterm::event;

use crate::game_state::GameState;

enum Event {
    BoardChange(char),
    Quit,
    Resize(u16, u16),
}

pub struct EventHandler<'a> {
    game_state: &'a mut GameState,
}

impl<'a> EventHandler<'a> {
    pub fn new(game_state: &'a mut GameState) -> Self {
        Self { game_state }
    }
    pub fn run(&mut self, timeout_millis: u64) {
        if event::poll(Duration::from_millis(timeout_millis)).expect("Failed to return an event") {
            match event::read().expect("Failed to read event") {
                event::Event::Key(key) => match key.code {
                    event::KeyCode::Char(c) => match c {
                        'q' => self.handle_events(Event::Quit),
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            self.handle_events(Event::BoardChange(c))
                        }
                        _ => {}
                    },
                    _ => {}
                },
                event::Event::Resize(x, y) => self.handle_events(Event::Resize(x, y)),
                _ => {}
            }
        }
    }

    fn handle_events(&mut self, event: Event) {
        match event {
            Event::BoardChange(ch) => {
                let mut x_counter: u8 = 0;
                let mut o_counter: u8 = 0;
                for i in 0..self.game_state.board.len() {
                    if self.game_state.board[i] == 'X' {
                        x_counter += 1;
                    } else if self.game_state.board[i] == 'O' {
                        o_counter += 1;
                    }
                }
                self.game_state.round = if x_counter <= o_counter { 0 } else { 1 };
                self.game_state.turn = self
                    .game_state
                    .get_player_name((self.game_state.round + 1) % 2);
                let target_index = ch as u8 - '0' as u8 - 1;
                // Replace the element in target_index of the board with either X or O
                // depending on the current round thus the current player
                if let Some(element) = self.game_state.board.get_mut(target_index as usize) {
                    *element = if self.game_state.round % 2 == 0 {
                        'X'
                    } else {
                        'O'
                    }
                } else {
                    println!("Failed to get the nth element of the board and replace it.");
                }
                // self.game_state.determine_winner();
            }
            Event::Quit => self.game_state.should_quit = true,
            Event::Resize(x, y) => todo!(),
        }
    }
}

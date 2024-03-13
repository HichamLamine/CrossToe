struct Player {
    name: String,
    score: u8,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
        }
    }
}

pub struct GameState {
    pub board: Vec<char>,
    // can use a hashmap to store the players with the score
    pub players: Vec<Player>,
    pub round: u8,
    pub turn: String,
    pub should_quit: bool,
    pub winner: Option<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: vec![' '; 9],
            players: vec![Player::new("Hicham"), Player::new("Lamine")],
            round: 0,
            turn: String::new(),
            should_quit: false,
            winner: None,
        }
    }
    pub fn get_player_name(&self, player_index: u8) -> String {
        self.players[player_index as usize].name.clone()
    }
    pub fn get_player_score(&self, player_index: u8) -> u8 {
        self.players[player_index as usize].score.clone()
    }
    pub fn clear_board(&mut self) {
        self.board = vec![' '; 9];
    }
    pub fn determine_winner(&mut self) {
        if (self.board[0] == self.board[4]
            && self.board[4] == self.board[8]
            && self.board[0] != ' ')
            || (self.board[2] == self.board[4]
                && self.board[4] == self.board[6]
                && self.board[6] != ' ')
        {
            self.winner = Some(self.get_player_name(self.round % 2));
            self.players[self.round as usize % 2].score += 1;
            self.clear_board();
            // self.winner = Some(self.turn.clone());
        }

        for i in 0..3 {
            if self.board[i * 3] == self.board[i * 3 + 1]
                && self.board[i * 3 + 1] == self.board[i * 3 + 2]
                && self.board[i * 3] != ' '
            {
                self.winner = Some(self.get_player_name(self.round % 2));
                self.players[self.round as usize % 2].score += 1;
                self.clear_board();
                // self.winner = Some(self.turn.clone());
                // self.print_winner(self.board, i, players);
            }
        }
        for i in 0..3 {
            if self.board[i] == self.board[i + 3]
                && self.board[i + 3] == self.board[i + 6]
                && self.board[i] != ' '
            {
                self.winner = Some(self.get_player_name(self.round % 2));
                self.players[self.round as usize % 2].score += 1;
                self.clear_board();
                // self.winner = Some(self.turn.clone());
            }
        }
    }
}

pub enum RunMode {
    Start,
    Update,
}

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{self, read, Event},
    style::Stylize,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use event_handler::EventHandler;
use renderer::Area;
use shapes::{BorderShape, LoggerShape, TitleShape};

mod event_handler;
mod game_state;
mod logger;
mod renderer;
mod shapes;
// mod ui;

use crate::game_state::GameState;
use crate::renderer::GameRenderer;
use crate::shapes::BoardShape;

// move to the positions in the renderable_cells vec and update them accordignly

fn start() {}

fn update() {}

fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();

    start();

    let mut game_state = GameState::new();
    let mut game_renderer = GameRenderer::new(&mut stdout);
    let board_shape = BoardShape::new(3, 3);
    game_renderer.render(&game_state, TitleShape {});
    game_renderer.render(&game_state, board_shape);

    let border_area = Area::new(11, 17, 40, 6);
    let border_shape = BorderShape::new(border_area);

    let logger_area = Area::new(11, 17, 40, 6);
    let mut logger_shape = LoggerShape::new(logger_area).set_border(border_shape);
    // logger_shape.log_info("Hi".to_string());

    while !game_state.should_quit {
        let board_shape = BoardShape::new(3, 3);
        game_renderer.render(&game_state, board_shape);

        game_renderer.render_ref(&game_state, &logger_shape);

        let mut board_content = String::new();
        for i in 0..game_state.board.len() {
            board_content.push_str(&format!("{}: {}, ", i, game_state.board[i]));
        }
        // logger_shape.log_info(board_content);
        // logger_shape.log_info(format!("Current Player: {}", game_state.turn));
        // logger_shape.log_info(format!("Current round: {}", game_state.round));

        logger_shape.log_info("Hello! This is an info!".to_string());
        logger_shape.log_warning("Something is wrong! This is an warning!".to_string());
        logger_shape.log_error("A crucial operation has failed! This is an error!".to_string());

        let mut event_handler = EventHandler::new(&mut game_state);

        event_handler.run(500);

        // if let Some(ref winner) = game_state.winner {
        //     logger_shape.log_info(format!("Current winner: {}", winner));
            // game_renderer.update_cursor_position(renderer::CursorUpdate::MoveTo(0, 0));
            // game_renderer.print_styled(vec![format!("the winner is {}", winner).white()]);
        // }
        // else {
        //
        //     game_renderer.update_cursor_position(renderer::CursorUpdate::MoveTo(0, 0));
        //     game_renderer.print_styled(vec![format!("the winner is suiii" ).white()]);
        // }

        game_renderer.update_cursor_position(renderer::CursorUpdate::MoveTo(0, 0));
        game_renderer.print_styled(vec![
            format!("{}: ", game_state.get_player_name(0)).blue(),
            format!("{}", game_state.get_player_score(0)).white(),
        ]);
        game_renderer.update_cursor_position(renderer::CursorUpdate::MoveTo(0, 1));
        game_renderer.print_styled(vec![
            format!("{}: ", game_state.get_player_name(1)).green(),
            format!("{}", game_state.get_player_score(1)).white(),
        ]);
        game_renderer.update_cursor_position(renderer::CursorUpdate::MoveTo(0, 2));
        game_renderer.print_styled(vec!["turn  : ".cyan(), game_state.turn[..].white()]);

        // if (game_state.board[0] == game_state.board[4]
        //     && game_state.board[4] == game_state.board[8]
        //     && game_state.board[0] != ' ')
        //     || (game_state.board[2] == game_state.board[4]
        //         && game_state.board[4] == game_state.board[6]
        //         && game_state.board[6] != ' ')
        // {
        //     game_renderer.print_styled(vec![format!("Winner is: {}", game_state.turn).white()]);
        // }

        // if (game_state.board[0 as usize][0 as usize] == game_state.board[1 as usize][1 as usize]
        //     && game_state.board[1 as usize][1 as usize] == game_state.board[2 as usize][2 as usize]
        //     && game_state.board[0 as usize][0 as usize] != ' ')
        //     || (game_state.board[0 as usize][2 as usize] == game_state.board[1 as usize][1 as usize]
        //         && game_state.board[1 as usize][1 as usize] == game_state.board[2 as usize][0 as usize]
        //         && game_state.board[0 as usize][2 as usize] != ' ')
        // {
        //     print_winner(game_state.board, i, players);
        // }

        // for i in 0..3 {
        //     if ((game_state.board[i as usize][0 as usize] == game_state.board[i as usize][1 as usize]
        //         && game_state.board[i as usize][1 as usize] == game_state.board[i as usize][2 as usize]
        //         && game_state.board[i as usize][0 as usize] != ' ')
        //         || (game_state.board[0 as usize][i as usize] == game_state.board[1 as usize][i as usize]
        //             && game_state.board[1 as usize][i as usize] == game_state.board[2 as usize][i as usize]
        //             && game_state.board[0 as usize][i as usize] != ' '))
        //     {
        //         print_winner(game_state.board, i, players);
        //     }
        // }

        game_state.determine_winner();

        game_renderer.buffer.flush().unwrap();
    }

    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}

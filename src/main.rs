use chess::{Board, BoardStatus, ChessMove, Color};
use std::str::FromStr;

use std::io::{self, Write};

mod engine;
mod piece_values; // Declare the piece_values module

fn main() {
    // init board, human player color
    // TODO: Use "game" instead of jhust board
    // for UCI and 3-fold repetitions
    let mut board: Board = Board::from_str("rr4k1/5pp1/3b3p/p1p2B2/P1Pp4/4PP2/1P4PP/RR4K1 w - - 0 24").expect("invalid FEN");
    let opp: Color = Color::White;

    // init human move and engine move
    let mut input = String::new();

    // while the there are still moves to be made
    // (not a checkmate or a stalemate)
    while board.status() == BoardStatus::Ongoing {
        if board.side_to_move() == opp {
            // if it's human player's turn, ask for move and TODO: play it if legal
            io::stdout()
                .write_all(b"Enter Move: \n")
                .expect("Failed to write to stdout");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // attempt to take human move and play it if from_san() is successful
            // throw error if from_san() fails
            match ChessMove::from_san(&board, &input.trim()) {
                Ok(chess_move) => {
                    board = board.make_move_new(chess_move);
                    println!("Player played: {}", chess_move.to_string());
                }
                Err(parse_error) => {
                    // Failed to parse the chess move
                    println!("Error: {}", parse_error);
                    // Handle the error appropriately, such as displaying an error message or taking corrective action
                }
            }
        } else {
            // if it's the engine's move, play the engine's best move
            assert_eq!(board.side_to_move(), Color::Black);
            match engine::get_best_move(board, 4) {
                Some(best_move) => {
                    board = board.make_move_new(best_move);
                    println!("Engine played: {}", best_move.to_string());
                } None => {
                    panic!("Couldn't find a good move?");
                }
            }

        }
    }
}

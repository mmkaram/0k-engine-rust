use chess::{Board, BoardStatus, ChessMove, Color};
use std::str::FromStr;

use std::io::{self, Write};

mod engine;
mod piece_values; // Declare the piece_values module

/// # Params
/// * 'fen' - the FEN string to parse (put it in "quotes")
/// # Example
/// ./target/debug/rust-engine "rr4k1/5pp1/3b3p/p1p2B2/P1Pp4/4PP2/1P4PP/RR4K1 w - - 0 24"
fn main() {
    // TODO: Make it so that the computer can play as white
    // get args
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let fen_string = &args[1];
        fen = fen_string;
        // Now you can use fen_string
        println!("FEN string: {}", fen_string);
    } else {
        println!("No FEN string provided");
    }

    // init board, human player color
    // TODO: Use "game" instead of jhust board
    // for UCI and 3-fold repetitions
    let mut board: Board = Board::from_str(fen).expect("invalid FEN");
    let opp: Color = Color::White;

    // init human move and engine move
    let mut input = String::new();

    // while the there are still moves to be made
    // (not a checkmate or a stalemate)
    while board.status() == BoardStatus::Ongoing {
        // if it's the humans player's turn, take the human's move
        if board.side_to_move() == opp {
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
                }
                None => {
                    panic!("FAILED: Engine could not find a move");
                }
            }
        }
    }
}

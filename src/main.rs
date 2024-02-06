use chess::{Board, BoardStatus, ChessMove, Color};

use std::io::{self, Write};

mod engine;

fn main() {
    // init board, human player color
    let mut board: Board = Board::default();
    let opp: Color = Color::White;

    // init human move and engine move
    let mut input = String::new();
    let mut best_move: ChessMove;

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
        } else {
            // if it's the engine's move, play the engine's best move
            best_move = engine::get_best_move(board);
            board = board.make_move_new(best_move);
        }
    }
}

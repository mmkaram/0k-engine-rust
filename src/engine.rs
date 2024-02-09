use chess::{Board, ChessMove, Color, MoveGen, Piece};

use crate::piece_values;

const PAWN_VALUE: f64 = 100.0;
const BISHOP_VALUE: f64 = 300.0;
const KNIGHT_VALUE: f64 = 300.0;
const ROOK_VALUE: f64 = 500.0;
const QUEEN_VALUE: f64 = 900.0;

pub fn get_best_move(board: Board) -> Option<ChessMove> {
    let mut test_board = board;
    // let mut best_move;
    let mut best_move: Option<ChessMove> = None;
    let mut best_score = -f64::INFINITY;
    let moves = MoveGen::new_legal(&board);

    for a in moves {
        test_board = test_board.make_move_new(a);
        if eval_pos(test_board) < best_score {
            best_move = Some(a);
            best_score = eval_pos(test_board);
        }
    }

    match best_move {
        Some(chess_move) => {
            // Use chess_move here...
            return Some(chess_move);
        }
        None => {
            // Handle the case where best_move was not set...
            println!("couldn't find move");
            return None;
        }
    }
}

fn eval_pos(board: Board) -> f64 {
    let mut final_score: f64 = 0.0;

    for square in chess::ALL_SQUARES {
        // Find a piece on the square and its side
        let piece: Option<Piece> = board.piece_on(square);
        let piece_side: Option<Color> = board.color_on(square);

        // Get the side multiplier
        let side_multiplier: f64 = match piece_side {
            Some(Color::White) => 1.0,
            Some(Color::Black) => -1.0,
            None => 0.0,
        };

        // Add the piece's values * its side to the final score
        if let Some(piece) = piece {
            final_score += match piece {
                Piece::Pawn => (PAWN_VALUE + piece_values::WHITE_PAWN.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Knight => (KNIGHT_VALUE + piece_values::KNIGHT.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Bishop => (BISHOP_VALUE + piece_values::BISHOP.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Rook => (ROOK_VALUE + piece_values::ROOK.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Queen => (QUEEN_VALUE /* + piece_values::QUEEN.get(square.to_index()).unwrap() */) * side_multiplier,
                Piece::King => 0.0,
            };
        } else {
            // println!("No piece")
        }
    }
    // println!("Score: {}", final_score);
    return final_score;
}
use chess::{Board, ChessMove, Color, MoveGen, Piece};

use crate::piece_values;

const PAWN_VALUE: f64 = 100.0;
const BISHOP_VALUE: f64 = 300.0;
const KNIGHT_VALUE: f64 = 300.0;
const ROOK_VALUE: f64 = 500.0;
const QUEEN_VALUE: f64 = 900.0;

pub fn minimax(board: Board, depth: u8, maximizing_player: bool) -> f64 {
    // return eval_pos
    if depth == 0 {
        return eval_pos(board);
    }

    let moves = MoveGen::new_legal(&board);
    if maximizing_player {
        let mut best_score = -f64::INFINITY;
        for a in moves {
            let test_board = board.make_move_new(a);
            best_score = best_score.max(minimax(test_board, depth - 1, false));
        }
        best_score
    } else {
        let mut best_score = f64::INFINITY;
        for a in moves {
            let test_board = board.make_move_new(a);
            best_score = best_score.min(minimax(test_board, depth - 1, true));
        }
        best_score
    }
}

pub fn get_best_move(board: Board, depth: u8) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut best_score = -f64::INFINITY;
    let moves = MoveGen::new_legal(&board);

    for a in moves {
        let test_board = board.make_move_new(a);
        let score = minimax(test_board, depth - 1, false);
        if score > best_score {
            best_move = Some(a);
            best_score = score;
        }
    }

    best_move
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
                Piece::Pawn => (PAWN_VALUE + piece_values::PAWN.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Knight => (KNIGHT_VALUE + piece_values::KNIGHT.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Bishop => (BISHOP_VALUE + piece_values::BISHOP.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Rook => (ROOK_VALUE + piece_values::ROOK.get(square.to_index()).unwrap()) * side_multiplier,
                Piece::Queen => (QUEEN_VALUE /* + piece_values::QUEEN.get(square.to_index()).unwrap() */) * side_multiplier,
                Piece::King => 0.0,
            };
        } else { }
    }
    // println!("Score: {}", final_score);
    final_score
}
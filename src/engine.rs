use chess::{Board, ChessMove, Color, MoveGen, Piece};

// I'm not sure I understand this syntax, but it seems to be a way to import a module from another file
// I had to import the actual module in the main.rs file, so I'm not sure why I'm now importing a crate?
use crate::piece_values;

// init piece values
const PAWN_VALUE: f64 = 100.0;
const BISHOP_VALUE: f64 = 300.0;
const KNIGHT_VALUE: f64 = 300.0;
const ROOK_VALUE: f64 = 500.0;
const QUEEN_VALUE: f64 = 900.0;

/// minimax algorithm
/// # Parameters
/// * 'board' - the current board state
/// * 'depth' the depth left to search
/// * 'maximizing_player' - whether the current player is the maximizing player
/// # Returns
/// the score of the best move as an f64
pub fn minimax(board: Board, depth: u8, maximizing_player: bool) -> f64 {
    // TODO: Implement qsearch
    // return eval_pos if final depth reached
    if depth == 0 {
        return eval_pos(board);
    }
    // create list of all legal moves
    let moves = MoveGen::new_legal(&board);
    if maximizing_player {
        // if the current player is the maximizing player
        // set the lowest score to negative infinity
        // loop through all the moves and find the best score
        // recursively call minimax with the new board state
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

/// get the best move for the current board state
/// # Parameters
/// * 'board' - the current board state
/// * 'depth' - the depth to search
/// # Returns
/// the best move as an Option<ChessMove> in case it can't find a move
/// # Panics
/// if the engine can't find a move
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
/// evaluate the position of the board
/// # Parameters
/// * 'board' - the current board state
/// # Returns
/// the score of the board as an f64
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
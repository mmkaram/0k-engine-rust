use chess::{Board, ChessMove, MoveGen, Square};

pub fn get_best_move(board: Board) -> ChessMove {
    return ChessMove::new(Square::E2, Square::E5, None);
}

fn eval_pos(board: Board) -> f64 {
    return 2.7;
}
fn get_moves(board: Board) -> MoveGen {
    let movegen: MoveGen = MoveGen::new_legal(&board);
    return movegen;
}

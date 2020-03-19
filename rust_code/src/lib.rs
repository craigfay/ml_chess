use chess_engine::{
    GameState,
    Piece,
    PieceName::{
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,
    },
    Color::{
        Black,
        White,
    },
};



pub extern "C" fn bitboard() -> Vec<i32> {
    let state = GameState::new();
    vec![1,2,3]
}

#[test]
fn bitboard_test() {
    let b = bitboard();
    assert_eq!(b, vec![1,2,3]);
}

fn convert_piece_to_int(maybe_piece: Option<Piece>) -> i32 {
    match maybe_piece {
        None => 0,
        Some(piece) => {
            match (piece.color, piece.name) {
                (White, Pawn) => 1,
                (White, Bishop) => 2,
                (White, Knight) => 3,
                (White, Rook) => 4,
                (White, Queen) => 5,
                (White, King) => 6,
                (Black, Pawn) => 7,
                (Black, Bishop) => 8,
                (Black, Knight) => 9,
                (Black, Rook) => 10,
                (Black, Queen) => 11,
                (Black, King) => 12,
            }
        }
    }
}

#[test]
fn convert_piece_to_int_test() {
    let piece = Piece { color: White, name: Pawn };
    assert_eq!(1, convert_piece_to_int(Some(piece)));
}


#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}


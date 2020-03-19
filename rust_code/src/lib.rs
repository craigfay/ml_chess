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

pub extern "C" fn gamestate_as_ints() -> Vec<i32> {
    let mut result = vec![];

    let state = GameState::new();
    for index in 0..64 {
        let maybe_piece = state.squares[index];
        result.push(piece_as_int(maybe_piece));
    }
    result
}

#[test]
fn gamestate_as_ints_test() {
    let ints = gamestate_as_ints();
    let expected = vec![4, 3, 2, 5, 6, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 7, 7, 7, 7, 7, 10, 9, 8, 11, 12, 8, 9, 10];

    assert_eq!(ints, expected);
}

fn piece_as_int(maybe_piece: Option<Piece>) -> i32 {
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
fn piece_as_int_test() {
    assert_eq!(0, piece_as_int(None));

    let piece = Piece { color: White, name: Pawn };
    assert_eq!(1, piece_as_int(Some(piece)));

    let piece = Piece { color: White, name: Bishop };
    assert_eq!(2, piece_as_int(Some(piece)));

    let piece = Piece { color: White, name: Knight };
    assert_eq!(3, piece_as_int(Some(piece)));

    let piece = Piece { color: White, name: Rook };
    assert_eq!(4, piece_as_int(Some(piece)));

    let piece = Piece { color: White, name: Queen };
    assert_eq!(5, piece_as_int(Some(piece)));

    let piece = Piece { color: White, name: King };
    assert_eq!(6, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: Pawn };
    assert_eq!(7, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: Bishop };
    assert_eq!(8, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: Knight };
    assert_eq!(9, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: Rook };
    assert_eq!(10, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: Queen };
    assert_eq!(11, piece_as_int(Some(piece)));

    let piece = Piece { color: Black, name: King };
    assert_eq!(12, piece_as_int(Some(piece)));
}


#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}


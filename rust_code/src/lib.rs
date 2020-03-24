
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
    legal_next_states,
    is_checkmate,
    is_stalemate,
    relative_material_values,
};

type NumericGameState = [i32; 70];

#[no_mangle]
pub extern "C" fn numeric_gamestate_is_checkmate(ints: NumericGameState) -> bool {
    let state = denumeralize_gamestate(ints);
    is_checkmate(&state)
}

#[no_mangle]
pub extern "C" fn numeric_gamestate_is_stalemate(ints: NumericGameState) -> bool {
    let state = denumeralize_gamestate(ints);
    is_stalemate(&state)
}

#[no_mangle]
pub extern "C" fn numeric_gamestate_material_values(ints: &NumericGameState, target: &mut [i32; 2]) {
    let state = denumeralize_gamestate(*ints);
    let (white_value, black_value) = relative_material_values(&state);
    target[0] = white_value as i32;
    target[1] = black_value as i32;
}

#[no_mangle]
pub extern "C" fn enumerate_legal_numeric_gamestates(ints: NumericGameState) -> Vec<NumericGameState> {
    let state = denumeralize_gamestate(ints);
    let next_states = legal_next_states(&state);

    let mut result = vec![];
    
    for state in next_states {
        result.push(numeralize_gamestate(&state));
    }

    result
}

fn denumeralize_gamestate(ints: NumericGameState) -> GameState {
    let mut state = GameState::with_placements(vec![]);

    for index in 0..64 {
        state.squares[index] = int_as_piece(ints[index]);
    }

    if ints[64] == 1 {
        state.to_move = Black;
    }

    if ints[65] == 1 {
        state.white_can_castle_kingside = true;
    }
    
    if ints[66] == 1 {
        state.white_can_castle_queenside = true;
    }
    if ints[67] == 1 {
        state.black_can_castle_kingside = true;
    }
    if ints[68] == 1 {
        state.black_can_castle_kingside = true;
    }

    if ints[69] > 0 && ints[69] < 64 {
        state.en_passant_square = Some(ints[69] as usize);
    }

    state
}

fn numeralize_gamestate(state: &GameState) -> NumericGameState {
    let mut result = [0; 70];

    for index in 0..64 {
        let maybe_piece = state.squares[index];
        result[index] = piece_as_int(maybe_piece);
    }

    if state.to_move == White { result[64] = 0 }
    else { result[64] = 1 }

    if state.white_can_castle_kingside { result[65] = 1 }
    else { result[65] = 0 }

    if state.white_can_castle_queenside { result[66] = 1 }
    else { result[66] = 0 }

    if state.black_can_castle_kingside { result[67] = 1 }
    else { result[67] = 0 }

    if state.black_can_castle_queenside { result[68] = 1 }
    else { result[68] = 0 }

    match state.en_passant_square {
        None => result[69] = 0,
        Some(index) => result[69] = index as i32 + 1,
    }

    result
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

fn int_as_piece(int: i32) -> Option<Piece> {
    match int {
        1 => Some(Piece { color: White, name: Pawn }),
        2 => Some(Piece { color: White, name: Bishop }),
        3 => Some(Piece { color: White, name: Knight }),
        4 => Some(Piece { color: White, name: Rook }),
        5 => Some(Piece { color: White, name: Queen }),
        6 => Some(Piece { color: White, name: King }),
        7 => Some(Piece { color: Black, name: Pawn }),
        8 => Some(Piece { color: Black, name: Bishop }),
        9 => Some(Piece { color: Black, name: Knight }),
        10 => Some(Piece { color: Black, name: Rook }),
        11 => Some(Piece { color: Black, name: Queen }),
        12 => Some(Piece { color: Black, name: King }),
        _ => None,
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
pub extern "C" fn fill_array_with_new_gamestate(target: &mut NumericGameState) {
    let state = GameState::new();
    let ints = numeralize_gamestate(&state);
    for index in 0..70 {
        target[index] = ints[index];
    }
}

#[test]
fn fill_array_with_new_gamestate_test() {
    let mut ints: NumericGameState = [0; 70];
    fill_array_with_new_gamestate(&mut ints);

    let expected = [4, 3, 2, 5, 6, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 7, 7, 7, 7, 7, 10, 9, 8, 11, 12, 8, 9, 10, 0, 1, 1, 1, 1, 0];
    
    for index in 0..expected.len() {
        assert_eq!(ints[index], expected[index]);
    }
}


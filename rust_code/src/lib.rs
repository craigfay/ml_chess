use chess_engine::GameState;

#[test]
fn bitboard_test() {
    let b = bitboard();
    assert_eq!(b, vec![1,2,3]);
}

pub extern "C" fn bitboard() -> Vec<i32> {
    let state = GameState::new();
    vec![1,2,3]
}


#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}


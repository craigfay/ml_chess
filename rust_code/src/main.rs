use chess_engine::GameState;

fn main() {
    let state = GameState::new();
    println!("{}", state.to_string());
}

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}


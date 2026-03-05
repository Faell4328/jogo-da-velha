// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use serde::Serialize;

static PLAYED: Mutex<[i32; 9]> = Mutex::new([0; 9]);
static TURN: Mutex<i32> = Mutex::new(0);

#[derive(Serialize)]
struct GameState {
    played: [i32; 9],
    turn: i32,
    winner: Option<i32>,
    is_draw: bool,
}

fn check_winner(board: &[i32; 9]) -> Option<i32> {
    let lines = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    for (a, b, c) in lines {
        if board[a] != 0 && board[a] == board[b] && board[b] == board[c] {
            return Some(board[a]);
        }
    }
    None
}

#[tauri::command]
fn play(clicked: String) -> Result<String, String> {
    let mut played = PLAYED.lock().unwrap();
    let mut turn = TURN.lock().unwrap();
    let clicked: usize = match clicked.parse::<usize>() {
        Ok(num) if num >= 1 && num <= 9 => num - 1,
        _ => {
            println!("Invalid element: {}", clicked);
            return Err("Invalid element".to_string());
        }
    };
    if played[clicked] != 0 {
        println!("Elemento {} já foi jogado!", clicked + 1);
        return Err("Elemento já jogado".to_string());
    }
    let player = *turn + 1; // turn: 0 -> player 1, 1 -> player 2
    played[clicked] = player;

    let winner = check_winner(&played);
    let is_draw = played.iter().all(|&v| v != 0) && winner.is_none();

    // Only advance turn if game continues
    if winner.is_none() && !is_draw {
        *turn = (*turn + 1) % 2;
    }

    let state = GameState {
        played: *played,
        turn: *turn,
        winner,
        is_draw,
    };

    match serde_json::to_string(&state) {
        Ok(json) => {
            println!("Played[{}]: {}", clicked + 1, played[clicked]);
            println!("Turn: {}", turn);
            Ok(json)
        }
        Err(e) => Err(format!("Serialization error: {}", e)),
    }
}

#[tauri::command]
fn get_state() -> Result<String, String> {
    let played = PLAYED.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let winner = check_winner(&played);
    let is_draw = played.iter().all(|&v| v != 0) && winner.is_none();
    let state = GameState {
        played: *played,
        turn: *turn,
        winner,
        is_draw,
    };
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

#[tauri::command]
fn reset() -> Result<String, String> {
    let mut played = PLAYED.lock().unwrap();
    let mut turn = TURN.lock().unwrap();
    *played = [0; 9];
    *turn = 0;
    let state = GameState {
        played: *played,
        turn: *turn,
        winner: None,
        is_draw: false,
    };
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // register all of our commands so they are callable from JS
        .invoke_handler(tauri::generate_handler![play, get_state, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

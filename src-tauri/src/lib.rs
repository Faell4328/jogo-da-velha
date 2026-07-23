// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use std::sync::Mutex;

#[derive(Serialize)]
struct GameState<'a> {
    squares: [&'a str; 9],
    turn: i8,
    winner: Option<&'a str>,
    is_draw: bool,
    err: &'a str,
}

static GAMESTATE: Mutex<GameState<'static>> = Mutex::new(GameState {
    squares: [" "; 9],
    turn: 0,
    winner: None,
    is_draw: false,
    err: "",
});

fn check_winner(game_state: &mut GameState) {
    // Todas possibilidades para ganhar
    let possibilities = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    for (a, b, c) in possibilities {
        if game_state.squares[a] != " "
            && game_state.squares[a] == game_state.squares[b]
            && game_state.squares[b] == game_state.squares[c]
        {
            game_state.winner = Some(game_state.squares[a]);
        }
    }
}

fn check_the_play(game_state: &mut GameState, clicked_square: usize) {
    // Verificando se foi escolhido uma casa inválida e não ocupada
    if clicked_square == 0 || clicked_square > 9 {
        game_state.err = "Você escolheu uma posição inválida";
    } else if game_state.squares[clicked_square - 1] != " " {
        game_state.err = "Você escolheu uma posição ocupada";
    }
}

#[tauri::command]
fn apply_move(clicked_square: usize) -> String {
    println!("Recebi o clicked? {}", clicked_square);

    let mut game_state = GAMESTATE.lock().unwrap();

    check_the_play(&mut game_state, clicked_square);
    if game_state.err != "" {
        let state = serde_json::to_string(&*game_state).unwrap();
        game_state.err = "";
        return state;
    }

    game_state.squares[clicked_square - 1] = {
        if game_state.turn % 2 == 0 {
            "X"
        } else {
            "O"
        }
    };
    game_state.turn += 1;

    check_winner(&mut game_state);
    if game_state.winner != None {
        let state = serde_json::to_string(&*game_state).unwrap();
        check_reset(&mut game_state);
        return state;
    }
    check_reset(&mut game_state);
    if game_state.is_draw == true {
        let state = serde_json::to_string(&*game_state).unwrap();
        game_state.is_draw = false;
        return state;
    }

    return serde_json::to_string(&*game_state).unwrap();
}

#[tauri::command]
fn get_state() -> String {
    let game_state = GAMESTATE.lock().unwrap();
    return serde_json::to_string(&*game_state).unwrap();
}

fn check_reset(game_state: &mut GameState) {
    if game_state.turn >= 9 {
        game_state.squares = [" "; 9];
        game_state.turn = 0;
        game_state.is_draw = true;
        game_state.winner = None;
        game_state.err = "";
    } else if game_state.winner != None {
        game_state.squares = [" "; 9];
        game_state.turn = 0;
        game_state.is_draw = false;
        game_state.winner = None;
        game_state.err = "";
    }
}

#[tauri::command]
fn reset() {
    let mut game_state = GAMESTATE.lock().unwrap();
    game_state.squares = [" "; 9];
    game_state.turn = 0;
    game_state.is_draw = false;
    game_state.winner = None;
    game_state.err = "";
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // register all of our commands so they are callable from JS
        .invoke_handler(tauri::generate_handler![apply_move, get_state, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

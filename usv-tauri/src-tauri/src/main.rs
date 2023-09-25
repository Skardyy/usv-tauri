// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

#[derive(Serialize)]
struct Action {
    taunter: String,
    word: String,
}

#[derive(Serialize)]
struct Block {
    zone_holder: String,
    actions: Vec<Action>
}

#[tauri::command]
fn get_blocks_mole() -> Vec<Block> {
    let start = Block {
        zone_holder: String::from("SC"),
        actions: vec![
            Action { taunter: String::from("LR"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Listen") },
        ]
    };
    let block1 = Block {
        zone_holder: String::from("LR"),
        actions: vec![
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("SC"), word: String::from("Listen") },
            Action { taunter: String::from("SC"), word: String::from("Truth") },
        ]
    };
    let block2 = Block {
        zone_holder: String::from("AP"),
        actions: vec![
            Action { taunter: String::from("LR"), word: String::from("Listen") },
            Action { taunter: String::from("LR"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Listen") },
        ]
    };
    let block3 = Block {
        zone_holder: String::from("LOO"),
        actions: vec![
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("SC"), word: String::from("Listen") },
            Action { taunter: String::from("SC"), word: String::from("Truth") },
        ]
    };
    let block4 = Block {
        zone_holder: String::from("SC"),
        actions: vec![
            Action { taunter: String::from("LR"), word: String::from("Listen") },
            Action { taunter: String::from("LR"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Listen") },
        ]
    };

    return vec![start, block1, block2, block3, block4];
}

#[tauri::command]
fn get_block_3man() -> Vec<Block> {
    let start = Block {
        zone_holder: String::from("LR"),
        actions: vec![
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("AP"), word: String::from("Listen") },
        ]
    };
    let block1 = Block {
        zone_holder: String::from("AP"),
        actions: vec![
            Action { taunter: String::from("LOO"), word: String::from("Truth") },
            Action { taunter: String::from("LR"), word: String::from("Listen") },
            Action { taunter: String::from("LR"), word: String::from("Truth") },
        ]
    };
    let block2 = Block {
        zone_holder: String::from("DPS"),
        actions: vec![
            Action { taunter: String::from("AP"), word: String::from("Listen") },
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Truth") },
            Action { taunter: String::from("LOO"), word: String::from("Listen") },
        ]
    };
    let block3 = Block {
        zone_holder: String::from("LOO"),
        actions: vec![
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("LR"), word: String::from("Listen") },
            Action { taunter: String::from("LR"), word: String::from("Truth") },
        ]
    };
    let block4 = Block {
        zone_holder: String::from("LR"),
        actions: vec![
            Action { taunter: String::from("LOO"), word: String::from("Listen") },
            Action { taunter: String::from("LOO"), word: String::from("Truth") },
            Action { taunter: String::from("AP"), word: String::from("Truth") },
            Action { taunter: String::from("AP"), word: String::from("Listen") },
        ]
    };

    return vec![start, block1, block2, block3, block4];
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_blocks_mole, get_block_3man])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

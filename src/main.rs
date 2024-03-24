mod eight_puzzle;
use eight_puzzle::eight_puzzle::EightPuzzle;
use std::{process, thread, time::Duration};

fn clear_console() {
    let _ = process::Command::new("cmd").args(&["/c", "cls"]).status();
}
fn main() {
    let mut puzzle = EightPuzzle::new();
    let solution = puzzle.solve();
    puzzle.movepos((1,2));

    if !solution.is_empty() {
        for state in solution {
            clear_console();
            state.print();
            // thread::sleep(Duration::from_secs(1)); 
        }
        println!("Solución encontrada!!.")
    } else {
        println!("No se encontró solución.");
    }

    // println!("Cantidad de movimientos: {}", puzzle.count);  
}
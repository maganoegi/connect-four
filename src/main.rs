
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*-------------------------- Puissance-4 -------------------------------------*/
/*----------------------------------------------------------------------------*/

extern crate c4_lib;
use c4_lib::*;

fn main() {
    let mut grid = init_grid();
    let mode = select_mode();
    let mut current_turn = 0;
    let mut score = 0; // values {-1, 0, 1} for Player2 win, No win and P1 win, respectively
    
    while score == 0 && current_turn < max_turns() {
        draw_grid(&grid);
        let played_at = play(&mut grid, mode, current_turn); // Different functionalities depending on mode
        score += judge(&grid, played_at); 
        current_turn += 1;
    }

    match score {
        1 => p1_won(),
        -1 => p2_won(),
        0 => no_winner(),
        _ => {},
    }
    draw_grid(&grid);
}
    

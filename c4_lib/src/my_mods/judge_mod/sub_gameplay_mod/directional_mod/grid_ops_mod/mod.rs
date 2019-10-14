
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*-------------------------- Puissance-4 -------------------------------------*/
/*----------------------------------------------------------------------------*/

pub mod constants;
use Statuses;
use NCOL;
use NLIGN;

pub fn insert_to_col(grid: &mut [[Statuses; NCOL]; NLIGN], current_turn: i32, selected_col: usize) -> Option<[usize;2]> {
    let mut i: usize = 0;
    while i != NLIGN {
        match grid[i][selected_col] {
            Statuses::NULL => {
                i += 1;
                continue;
            },
            _ => break,
        }
    }
    if i == 0 {
        return None;
    } else {
        if current_turn%2 == 0 { // player 1
            grid[i-1][selected_col] = Statuses::P1;
        } else { // player 2
            grid[i-1][selected_col] = Statuses::P2;
        } 
        return Some([i-1, selected_col]);
    }
}

pub fn is_inside(line: i32, col: i32) -> bool {
    if line >= 0 && line < (NLIGN as i32) && col < (NCOL as i32) && col >= 0 {
        return true;
    } else {
        return false;
    }
}

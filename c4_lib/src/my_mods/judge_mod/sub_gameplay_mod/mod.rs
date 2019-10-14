extern crate rust_hepia_lib;
use rust_hepia_lib::*;

pub mod directional_mod;
use self::directional_mod::*;
use self::directional_mod::grid_ops_mod::*;
use self::directional_mod::grid_ops_mod::constants::*;

pub fn smarter_computer(grid: &mut [[Statuses; NCOL]; NLIGN], player: i32) -> Option<[usize; 2]> {
    let mut result: Option<[usize; 2]> = None;
    let mut c: usize = 0;
    let mut l = 0;

    loop {
        if c < NCOL && (l as usize) < NLIGN {
            match result {
                None => {
                    if player == 1 {
                        match &grid[l as usize][c] {
                            Statuses::NULL => {
                                if l as usize == (NLIGN - 1) {
                                    if is_win_position(&grid, l, c as i32, player) {
                                        result = Some([l as usize, c]);
                                        break; 
                                    }
                                    c += 1;
                                    l = 0;
                                } else {
                                    l += 1;
                                }
                            },
                            Statuses::P2 => {
                                c += 1;
                                l = 0;
                            },
                            Statuses::P1 => {
                                if is_win_position(&grid, l-1, c as i32, player) {
                                    result = Some([(l-1) as usize, c]);
                                    break;
                                } else {
                                    l = 0;
                                    c+=1;
                                }
                            }
                        } 
                    } else {
                        match &grid[l as usize][c] {
                            Statuses::NULL => {
                                if l as usize == (NLIGN - 1) {
                                    if is_win_position(&grid, l, c as i32, player) {
                                        result = Some([l as usize, c]);
                                        break; 
                                    }
                                    c += 1;
                                    l = 0;
                                } else {
                                    l += 1;
                                }
                            },
                            Statuses::P1 => {
                                c += 1;
                                l = 0;
                            },
                            Statuses::P2 => {
                                if is_win_position(&grid, l-1, c as i32, player) {
                                    result = Some([(l-1) as usize, c]);
                                    break;
                                } else {
                                    l = 0;
                                    c+=1;
                                }
                            }
                        } 
                    }
                },
                Some(_res) => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    return result;
}

pub fn is_win_position(&grid: &[[Statuses; NCOL]; NLIGN], l: i32, c: i32, player: i32) -> bool {
    /* let mut result: bool = false; */
    if is_inside(l, c){
        if (east(&grid, l, c+1, player) + west(&grid, l, c-1, player)) >= 3 {
            return true;
        } else if south(&grid, l+1, c, player) >= 3 {
            return true;
        } else if (northeast(&grid, l-1, c+1, player) + southwest(&grid, l+1, c-1, player)) >= 3 {
            return true;
        } else if (southeast(&grid, l+1, c+1, player) + northwest(&grid, l-1, c-1, player)) >= 3 {
            return true;                    
        } else {
            return false;
        }
    } else {
        return false;
    }
    /* return result; */
}

pub fn rdm_computer(mut grid: &mut[[Statuses; NCOL]; NLIGN]) -> Option<[usize; 2]> {
    let random_col = gen(0, NCOL);
    match random_col {
        0 ..= NCOL => { // When valid column number is chosen
            let mut where_played = insert_to_col(&mut grid, 1, random_col as usize);
            return where_played;
        },
        _ => { // When anything else is chosen
            return None;
        }
    }
}

pub fn player_play(mut grid: &mut[[Statuses; NCOL]; NLIGN],current_turn: i32) -> Option<[usize; 2]> {
    let selected_col = read_int();
    match selected_col as usize {
        1 ..= NCOL => { // When valid column number is chosen
            let where_played = insert_to_col(&mut grid, current_turn, (selected_col - 1) as usize);
            return where_played;
        },
        _ => { // When anything else is chosen
            return None;
        }
    }
}
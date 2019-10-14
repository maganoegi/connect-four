
/*----------------------------------------------------------------------------*/
/*------ Sergey Platonov --------------------- HEPIA, ITI 2018-2019 sem.1 ----*/
/*-------------------------- Puissance-4 -------------------------------------*/
/*----------------------------------------------------------------------------*/

extern crate rust_hepia_lib;
use rust_hepia_lib::*;

mod my_mods;
use my_mods::judge_mod::*;
use my_mods::judge_mod::sub_gameplay_mod::*;
use my_mods::judge_mod::sub_gameplay_mod::directional_mod::grid_ops_mod::*;
use my_mods::judge_mod::sub_gameplay_mod::directional_mod::grid_ops_mod::constants::*;

/* use my_mods::grid_ops_mod::constants::*;
use my_mods::grid_ops_mod::*; */


pub fn max_turns() -> i32 {
    let result = (NCOL*NLIGN) as i32;
    return result;
}

pub fn select_mode() -> i32 {
    // Determining mode
    println!("\nWelcome to a game of Connect Four.\nPlease select a GAME MODE:\n(1) for 2-player, (2) for computer randomness and (3) for smarter computer adversary:");
    loop {
        let mode = read_int();
        match mode {
            1 ..=3 => {
                return mode;
                /* break; */
            },
            _ => {
                println!("\nINVALID INPUT.\nPlease select a GAME MODE:\n(1) for 2-player, (2) for computer randomness and (3) for smarter computer adversary:");
                continue;
            },
        }
    }
}

pub fn init_grid() -> [[Statuses; NCOL]; NLIGN] {
   let my_grid: [[Statuses; NCOL]; NLIGN] = [[Statuses::NULL; NCOL]; NLIGN];
   return my_grid;
}

// Function that handles the printing of the grid to the terminal.
pub fn draw_grid(&grid: &[[Statuses; NCOL]; NLIGN]) {
    let mut printable_grid/* : [[String; NCOL]; NLIGN] */ = [[""; NCOL]; NLIGN];
    println!("      ┌───┬───┬───┬───┬───┬───┬───┐");
    for l in 0..NLIGN {
        let mut owned_string = "      ".to_owned(); // offset right: 6
        for c in 0..NCOL {
            match &grid[l][c] {
                Statuses::NULL => printable_grid[l][c] = "|   ",
                Statuses::P1 => printable_grid[l][c] = "| O ",
                Statuses::P2 => printable_grid[l][c] = "| X ",
            }
            let borrowed_string: &str = printable_grid[l][c];
            owned_string += borrowed_string;
            
        }
        owned_string += "|" as &str;
        println!("{}", owned_string);
        if l < NLIGN-1 {
            println!("      ├───┼───┼───┼───┼───┼───┼───┤");
        }
    }
    println!("      └───┴───┴───┴───┴───┴───┴───┘"); // offset right: 6
    println!("        1   2   3   4   5   6   7                    P1: O       P2: X"); // offset to the right: 8
}

pub fn play(mut grid: &mut[[Statuses; NCOL]; NLIGN], mode: i32, current_turn: i32 ) -> [usize; 2]  {
    match mode {
        1 => { // 2 player mode
            println!("\nSelect column:");
            loop {
                let mut where_played = player_play(&mut grid, current_turn);
                match where_played {
                    None => {
                        println!("Selected column is either full or is invalid - select another");
                        continue;
                    },
                    Some(result) => {
                        return result;
                    }
                }
            }
        },
        2 => { // against random computer turns
            if current_turn % 2 == 0 {
                println!("\nSelect column:");
                loop {
                    let mut where_played = player_play(&mut grid, current_turn);
                    match where_played {
                        None => {
                            println!("Selected column is either full or is invalid - select another");
                            continue;
                        },
                        Some(result) => {
                            return result;
                        }
                    }
                }
            } else {
                loop {
                    let mut where_played = rdm_computer(&mut grid);
                    match where_played {
                        None => continue,
                        Some(rdm) => {
                            return rdm;
                        }
                    }
                }
            }
        },
        3 => {
            if current_turn % 2 == 0 {
                println!("\nSelect column:");
                loop {
                    let mut where_played = player_play(&mut grid, current_turn);
                    match where_played {
                        None => {
                            println!("Selected column is either full or is invalid - select another");
                            continue;
                        },
                        Some(result) => {
                            return result;
                        }
                    }
                }
            } else {
                if current_turn >= 5 {
                    let p2_win_pos = smarter_computer(&mut grid, 2);
                    let p1_block_pos = smarter_computer(&mut grid, 1);

                    match p2_win_pos {
                        None => {
                            match p1_block_pos {
                                None => {
                                    loop {
                                        let mut where_played = rdm_computer(&mut grid);
                                        match where_played {
                                            None => continue,
                                            Some(rdm) => {
                                                return rdm;
                                            }
                                        }
                                    }
                                },
                                Some(res1) => {
                                    insert_to_col(&mut grid, current_turn, res1[1]);
                                    return res1;
                                }
                            }
                        },
                        Some(res2) => {
                            insert_to_col(&mut grid, current_turn, res2[1]);
                            return res2;
                        }
                    }
                } else {
                    loop {
                        let mut where_played = rdm_computer(&mut grid);
                        match where_played {
                            None => continue,
                            Some(rdm) => {
                                return rdm;
                            }
                        }
                    }
                }              
            }
        },
        _ => panic!("invalid mode"),
    }
}

pub fn judge(&grid: &[[Statuses; NCOL]; NLIGN], played_at: [usize; 2]) -> i32 {
    let v_rez = vertical_judge(&grid, played_at);
    let h_rez = horizontal_judge(&grid, played_at);
    let d_rez = diagonal_judge(&grid, played_at);
    
    if v_rez != 0 {
        return v_rez;
    } else if h_rez != 0 {
        return h_rez;
    } else if d_rez != 0 {
        return d_rez;
    } else {
        return 0;
    }
}


pub fn p1_won() {
    println!("\n     //////   //     //   //    //  //////  //    //     ");
    println!("    //   //  //      //  ////  //  //  //  ////  //   ");
    println!("   ///////  //       // // // //  //  //  // // //   ");
    println!("  //       //        ////  ////  //  //  //  ////        ");
    println!(" //       //         ///   ///  //////  //   ///    ");
}

pub fn no_winner() {
    println!("\n     //    //  //////    //   //    //  //////  //    //     ");
    println!("    ////  //  //  //     //  ////  //    //    ////  //   ");
    println!("   // // //  //  //      // // // //    //    // // //   ");
    println!("  //  ////  //  //       ////  ////    //    //  ////        ");
    println!(" //   ///  //////        ///   ///  //////  //   ///    ");
}

pub fn p2_won() {
    println!("\n     //////   //////    //   //    //  //////  //    //     ");
    println!("    //   //      //     //  ////  //  //  //  ////  //   ");
    println!("   ///////  //////      // // // //  //  //  // // //   ");
    println!("  //       //           ////  ////  //  //  //  ////        ");
    println!(" //       //////        ///   ///  //////  //   ///    ");
}















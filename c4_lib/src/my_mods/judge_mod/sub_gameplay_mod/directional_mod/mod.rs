pub mod grid_ops_mod;
use self::grid_ops_mod::*;
use  self::grid_ops_mod::constants::*;

pub fn west(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line, col-1) && result == 1 {
                match &grid[line as usize][(col-1) as usize]{
                    Statuses::P1 => result += west(&grid, line, col-1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line, col-1) && result == 1 {
                match &grid[line as usize][(col-1) as usize]{
                    Statuses::P2 => result += west(&grid, line, col-1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn east(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0, // IF RESULT=0 DO NOT PLAY FURTHER!?
            }
            if is_inside(line, col+1) && result == 1 {
                match &grid[line as usize][(col+1) as usize]{
                    Statuses::P1 => result += east(&grid, line, col+1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
            
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line, col+1) && result == 1 {
                match &grid[line as usize][(col+1) as usize]{
                    Statuses::P2 => result += east(&grid, line, col+1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn south(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col) && result == 1 {
                match &grid[(line+1) as usize][col as usize]{
                    Statuses::P1 => result += south(&grid, line+1, col, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col) && result == 1 {
                match &grid[(line+1) as usize][col as usize]{
                    Statuses::P2 => result += south(&grid, line+1, col, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn northeast(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line-1, col+1) && result == 1 {
                match &grid[(line-1) as usize][(col+1) as usize]{
                    Statuses::P1 => result += northeast(&grid, line-1, col+1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line-1, col+1) && result == 1 {
                match &grid[(line-1) as usize][(col+1) as usize]{
                    Statuses::P2 => result += northeast(&grid, line-1, col+1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn southeast(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col+1) && result == 1 {
                match &grid[(line+1) as usize][(col+1) as usize]{
                    Statuses::P1 => result += southeast(&grid, line+1, col+1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col+1) && result == 1 {
                match &grid[(line+1) as usize][(col+1) as usize]{
                    Statuses::P2 => result += southeast(&grid, line+1, col+1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn northwest(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line-1, col-1) && result == 1 {
                match &grid[(line-1) as usize][(col-1) as usize]{
                    Statuses::P1 => result += northwest(&grid, line-1, col-1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line-1, col-1) && result == 1 {
                match &grid[(line-1) as usize][(col-1) as usize]{
                    Statuses::P2 => result += northwest(&grid, line-1, col-1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}

pub fn southwest(&grid: &[[Statuses; NCOL]; NLIGN], line: i32, col: i32, player: i32) -> i32 {
    let mut result = 0;
    if is_inside(line, col) {
        if player == 1 {
            match &grid[line as usize][col as usize] {
                Statuses::P1 => result += 1,
                Statuses::P2 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col-1) && result == 1 {
                match &grid[(line+1) as usize][(col-1) as usize]{
                    Statuses::P1 => result += southwest(&grid, line+1, col-1, 1),
                    Statuses::P2 | Statuses::NULL => result += 0,
                }
            }
        } else {
            match &grid[line as usize][col as usize] {
                Statuses::P2 => result += 1,
                Statuses::P1 | Statuses::NULL => result = 0,
            }
            if is_inside(line+1, col-1) && result == 1{
                match &grid[(line+1) as usize][(col-1) as usize]{
                    Statuses::P2 => result += southwest(&grid, line+1, col-1, 2),
                    Statuses::P1 | Statuses::NULL => result += 0,
                }
            }
        }
    }
    return result;
}
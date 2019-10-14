

pub mod sub_gameplay_mod;
use self::sub_gameplay_mod::directional_mod::*;
use self::sub_gameplay_mod::directional_mod::grid_ops_mod::constants::*;

pub fn diagonal_judge(&grid: &[[Statuses; NCOL]; NLIGN], played_at: [usize; 2]) -> i32 {
    let mut counter_p1_positive = 0;
    let mut counter_p1_negative = 0;
    let mut counter_p2_positive = 0;
    let mut counter_p2_negative = 0;

    match &grid[played_at[0]][played_at[1]] {
        Statuses::P1 => {
            counter_p1_positive = 1 + (northeast(&grid, (played_at[0] as i32)-1, (played_at[1] as i32)+1, 1) + southwest(&grid, (played_at[0] as i32)+1, (played_at[1] as i32)-1, 1)) ;
            counter_p1_negative = 1 + (northwest(&grid, (played_at[0] as i32)-1, (played_at[1] as i32)-1, 1) + southeast(&grid, (played_at[0] as i32)+1, (played_at[1] as i32)+1, 1));
        },
        Statuses::P2 => {
            counter_p2_positive = 1 + (northeast(&grid, (played_at[0] as i32)-1, (played_at[1] as i32)+1, 2) + southwest(&grid, (played_at[0] as i32)+1, (played_at[1] as i32)-1, 2) );
            counter_p2_negative = 1 + (northwest(&grid, (played_at[0] as i32)-1, (played_at[1] as i32)-1, 2) + southeast(&grid, (played_at[0] as i32)+1, (played_at[1] as i32)+1, 2) );
        },
        Statuses::NULL => {},
    }

    if counter_p1_positive >= 4 {
        return 1;
    } else if counter_p2_positive >= 4 {
        return -1;
    } else if counter_p1_negative >= 4 {
        return 1;
    } else if counter_p2_negative >= 4 {
        return -1;
    } else {
        return 0;
    }
}

pub fn horizontal_judge(&grid: &[[Statuses; NCOL]; NLIGN], played_at: [usize; 2]) -> i32 {
    let mut counter_p1 = 0;
    let mut counter_p2 = 0;

    match &grid[played_at[0]][played_at[1]] {
            Statuses::P1 => {
                counter_p1 = 1 + (east(&grid, played_at[0] as i32, (played_at[1] as i32)+1, 1) + west(&grid, played_at[0] as i32, (played_at[1] as i32)-1, 1));
            },
            Statuses::P2 => {
                counter_p2 = 1 + east(&grid, played_at[0] as i32, (played_at[1] as i32)+1, 2) + west(&grid, played_at[0] as i32, (played_at[1] as i32)-1, 2);
            },
            Statuses::NULL => {},
        }

    if counter_p1 >= 4 {
        return 1;
    } else if counter_p2 >= 4 {
        return -1;
    } else {
        return 0;
    }
}

pub fn vertical_judge(&grid: &[[Statuses; NCOL]; NLIGN], played_at: [usize;2]) -> i32 {
    let mut counter_p1 = 0;
    let mut counter_p2 = 0;

    match &grid[played_at[0]][played_at[1]] {
            Statuses::P1 => {
                counter_p1 = 1 + (south(&grid, (played_at[0] as i32)+1, played_at[1] as i32, 1));
            }, 
            Statuses::P2 => {
                counter_p2 = 1 + (south(&grid, (played_at[0] as i32)+1, played_at[1] as i32, 2));
            }, 
            Statuses::NULL => {},
        }

    if counter_p1 >= 4 {
        return 1;
    } else if counter_p2 >= 4 {
        return -1;
    } else {
        return 0;
    }
}
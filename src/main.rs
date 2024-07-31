use std::{cell::Cell, collections::HashSet, hash::Hash, thread, time};

use rand::Rng;
use termgrid::{grid::Grid, print::Printer};

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Snow,
    Empty
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CellState::Empty => " ",
            CellState::Snow => "*"
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let mut grid = random_snow_grid();
    let mut printer = Printer::new(grid.clone());
    printer.start();

    // lets just start by creating a random grid
    loop {
        let time = time::Duration::from_millis(100);
        thread::sleep(time);
        grid = apply_gravity(grid.clone());
        printer = match printer.update(grid.clone()) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
    }
}

fn apply_gravity(grid: Grid<CellState>) -> Grid<CellState> {
    let mut new_grid = grid.clone();
    let mut set: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..grid.grid.len() {
        for col in 0..new_grid.grid[0].len() {
            new_grid = apply_gravity_to_cell((row, col), new_grid, &mut set);
        }
    }

    new_grid
}

fn apply_gravity_to_cell(indices: (usize, usize), grid: Grid<CellState>, set: &mut HashSet<(usize, usize)>) -> Grid<CellState> {
    if set.contains(&indices) {
        return grid;
    }

    if grid.grid[indices.0][indices.1] == CellState::Empty {
        return grid;
    }

    if grid.grid.len() - 1 <= indices.0 {
        return grid;
    }

    set.insert(indices);
    let mut new_grid = grid.clone();

    let mut below = indices.0 + 1;

    while below < new_grid.grid.len() && new_grid.grid[below][indices.1] != CellState::Empty {
        set.insert((below, indices.1));
        below = below + 1;
    }

    if below < new_grid.grid.len() {
        set.insert((below, indices.1));
        new_grid.grid[indices.0][indices.1] = CellState::Empty;
        new_grid.grid[below][indices.1] = CellState::Snow;
    }

    new_grid
}

fn create_single_snow_grid() -> Grid<CellState> {
    let mut grid = Grid::new(CellState::Empty);
    grid.grid[0][0] = CellState::Snow;
    grid.grid[1][0] = CellState::Snow;
    grid.grid[2][0] = CellState::Snow;
    return grid;
}

fn random_snow_grid() -> Grid<CellState> {
    let mut grid = Grid::new(CellState::Empty);
    
    let row_count = grid.grid.len();
    let column_count = grid.grid[0].len();

    for row_index in 0..row_count / 3 {
        for column_index in 0..column_count {
            if fifty_fifty_chance() {
                grid.grid[row_index][column_index] = CellState::Snow;
            }
        }
    }

    return grid;
}

fn fifty_fifty_chance() -> bool {
    let num = rand::thread_rng().gen_range(0..3);
    return (num % 3) == 0;
}

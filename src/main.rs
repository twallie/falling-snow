use std::{collections::HashSet, thread, time};

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
    let mut grid = Grid::new(CellState::Empty);
    let mut printer = Printer::new(grid.clone());
    printer.start();

    // lets just start by creating a random grid
    loop {
        let time = time::Duration::from_millis(1);
        thread::sleep(time);
        let results = generate_next_grid(grid.clone());
        if results.1 {
            break;
        }
        grid = results.0;
        printer = match printer.update(grid.clone()) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
    }

    termgrid::print::reset_cursor();
}

fn generate_next_grid(grid: Grid<CellState>) -> (Grid<CellState>, bool) {
    let mut new_grid = grid.clone();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut all_filled = true;

    for row in 0..grid.grid.len() {
        for col in 0..new_grid.grid[0].len() {
            if new_grid.grid[row][col] == CellState::Empty {
                all_filled = false;
                continue;
            }
            new_grid = apply_gravity_to_cell((row, col), new_grid, &mut set);
        }
    }

    if all_filled {
        return (new_grid, true);
    }

    for col in 0..new_grid.grid[0].len() {
        if chance(1) {
            new_grid.grid[0][col] = CellState::Snow;
        }
    }

    (new_grid, false)
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

fn chance(percent: usize) -> bool {
    let num = rand::thread_rng().gen_range(0..100);
    return num < percent;
}

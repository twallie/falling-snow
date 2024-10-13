use std::{collections::HashSet, thread, time};

use rand::Rng;
use termgrid::controller::TermGrid;

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Snow,
    Empty,
}

fn main() {
    let mut termgrid = TermGrid::new(&CellState::Snow, &CellState::Empty);
    termgrid.start();
    termgrid.num_rows();

    // lets just start by creating a random grid
    loop {
        let time = time::Duration::from_millis(1);
        thread::sleep(time);
        let res = generate_next_grid(termgrid);
        termgrid = res.0;
        if res.1 {
            break;
        };
        termgrid.update();
    }

    termgrid.end();
}

fn generate_next_grid(grid: TermGrid<CellState>) -> (TermGrid<CellState>, bool) {
    let mut new_grid = grid.clone();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut all_filled = true;

    for row in 0..grid.num_rows() {
        for column in 0..grid.num_columns() {
            if *new_grid.get(column, row).unwrap() == CellState::Empty {
                all_filled = false;
                continue;
            }
            new_grid = apply_gravity_to_cell((row, column), new_grid, &mut set);
        }
    }

    if all_filled {
        return (new_grid, true);
    }

    for column in 0..new_grid.num_columns() {
        if chance(1) {
            new_grid.set(column, new_grid.num_rows() - 1);
        }
    }

    (new_grid, false)
}

fn apply_gravity_to_cell(
    indices: (usize, usize),
    grid: TermGrid<CellState>,
    set: &mut HashSet<(usize, usize)>,
) -> TermGrid<CellState> {
    // if we have already applied gravity to this cell
    if set.contains(&indices) {
        return grid;
    }

    // If this cell is empty, we cant apply gravity
    if *grid.get(indices.1, indices.0).unwrap() == CellState::Empty {
        return grid;
    }

    // If the cell is at the bottom of the screen, we cant apply gravity
    if 0 == indices.0 {
        return grid;
    }

    // Otherwise, we are going to apply gravity to this cell
    set.insert(indices);
    let mut new_grid = grid.clone();

    let mut below = indices.0 - 1;

    while below > 0 && *new_grid.get(indices.1, below).unwrap() != CellState::Empty {
        set.insert((below, indices.1));
        below = below - 1;
    }

    if below < new_grid.num_rows() && *new_grid.get(indices.1, below).unwrap() != CellState::Snow {
        set.insert((below, indices.1));
        new_grid.unset(indices.1, indices.0);
        new_grid.set(indices.1, below);
    }

    new_grid
}

fn chance(percent: usize) -> bool {
    let num = rand::thread_rng().gen_range(0..100);
    return num < percent;
}

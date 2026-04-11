use crate::{
    AnalyzeCells,
    grid::{Cell, Grid},
};

#[derive(Debug)]
pub struct AnalyzePuzzle {
    pub grid: Grid,
}

pub fn analyze_puzzle(cells: &AnalyzeCells) -> AnalyzePuzzle {
    let empty0 = make_grid(cells, 0, 1, 2);
    let empty1 = make_grid(cells, 1, 2, 0);
    let empty2 = make_grid(cells, 2, 0, 1);

    let empty0_valid = empty0.is_valid();
    let empty1_valid = empty1.is_valid();
    let empty2_valid = empty2.is_valid();

    let grid = match (empty0_valid, empty1_valid, empty2_valid) {
        (true, false, false) => empty0,
        (false, true, false) => empty1,
        (false, false, true) => empty2,
        (true, _, _) => {
            log::warn!("ambiguous grid");
            empty0
        }
        (false, true, _) => {
            log::warn!("ambiguous grid");
            empty1
        }
        (false, false, false) => {
            panic!("no assignment of colors produces a valid grid");
        }
    };

    AnalyzePuzzle { grid }
}

fn make_grid(cells: &AnalyzeCells, empty: usize, color_a: usize, color_b: usize) -> Grid {
    let (black, white) = {
        let color_a_sum: f32 = cells.centroids[color_a].data.iter().sum();
        let color_b_sum: f32 = cells.centroids[color_b].data.iter().sum();
        match color_a_sum > color_b_sum {
            true => (color_b, color_a),
            false => (color_a, color_b),
        }
    };

    let mut grid = Grid::new(cells.cells.len() / cells.cols, cells.cols);

    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            let i = r * grid.cols() + c;
            grid[(r, c)] = match cells.cell_classes[i] {
                i if i == empty => Cell::Empty,
                i if i == black => Cell::Black,
                i if i == white => Cell::White,
                i => panic!("unknown cell class: {i}"),
            }
        }
    }

    grid
}

use std::fmt;

use pzpr_codec::{
    grid::{Grid, Gridlike},
    variety::yinyang::Cell,
};

use crate::{AnalyzeCells, AnalyzeResult};

pub struct AnalyzePuzzle {
    pub grid: Grid<Cell>,
}

impl fmt::Debug for AnalyzePuzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalyzePuzzle").finish_non_exhaustive()
    }
}

pub fn analyze_puzzle(cells: &AnalyzeCells) -> AnalyzeResult<AnalyzePuzzle> {
    let empty0 = make_grid(cells, 0, 1, 2)?;
    let empty1 = make_grid(cells, 1, 2, 0)?;
    let empty2 = make_grid(cells, 2, 0, 1)?;

    let empty0_valid = is_valid(&empty0);
    let empty1_valid = is_valid(&empty1);
    let empty2_valid = is_valid(&empty2);

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
            return Err("no assignment of colors produces a valid grid".into());
        }
    };

    Ok(AnalyzePuzzle { grid })
}

fn make_grid(
    cells: &AnalyzeCells,
    empty: usize,
    color_a: usize,
    color_b: usize,
) -> AnalyzeResult<Grid<Cell>> {
    let (black, white) = {
        let color_a_sum: f32 = cells.centroids[color_a].data.iter().sum();
        let color_b_sum: f32 = cells.centroids[color_b].data.iter().sum();
        match color_a_sum > color_b_sum {
            true => (color_b, color_a),
            false => (color_a, color_b),
        }
    };

    let rows = (cells.cells.len() / cells.cols) as isize;
    let cols = cells.cols as isize;

    cells
        .cell_classes
        .iter()
        .map(|cls| match *cls {
            i if i == empty => Ok(Cell::Empty),
            i if i == black => Ok(Cell::Black),
            i if i == white => Ok(Cell::White),
            i => Err(format!("unknown cell class {i}").into()),
        })
        .collect::<AnalyzeResult<Grid<Cell>>>()?
        .reshape(rows, cols)
        .map_err(|e| format!("reshape failed: {e:?}").into())
}

fn is_valid(grid: &Grid<Cell>) -> bool {
    for r in 1..grid.shape().rows() {
        for c in 1..grid.shape().cols() {
            let g = grid.view(r - 1, c - 1, 2, 2);
            if g[0] == g[1] && g[0] == g[2] && g[0] == g[3] && g[0] != Cell::Empty {
                return false;
            }
        }
    }
    true
}

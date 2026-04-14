use std::fmt;

use image::GrayImage;

use crate::{AnalyzeGridBounds, AnalyzeGridPitch, AnalyzeResult, pixel::PixelExt};

pub struct AnalyzeCells {
    pub cols: usize,
    pub centroids: Vec<AnalyzeCell>,
    pub cells: Vec<AnalyzeCell>,
    pub cell_classes: Vec<usize>,
}

#[derive(Clone)]
pub struct AnalyzeCell {
    pub data: Vec<f32>,
}

pub fn analyze_cells(
    img: &GrayImage,
    pitch: &AnalyzeGridPitch,
    bounds: &AnalyzeGridBounds,
) -> AnalyzeResult<AnalyzeCells> {
    let crop = 0.06;

    let cell_w = (pitch.size.w * (1.0 - 2.0 * crop)).round() as u32;
    let cell_h = (pitch.size.h * (1.0 - 2.0 * crop)).round() as u32;

    let cell_cols = ((bounds.bounds.x1 - bounds.bounds.x0) as f32 / pitch.size.w).round() as usize;
    let cell_rows = ((bounds.bounds.y1 - bounds.bounds.y0) as f32 / pitch.size.h).round() as usize;

    let mut cells: Vec<AnalyzeCell> = Vec::new();
    for i in 0..cell_rows {
        for j in 0..cell_cols {
            let cell_x =
                bounds.bounds.x0 as u32 + ((j as f32 + crop) * pitch.size.w).round() as u32;
            let cell_y =
                bounds.bounds.y0 as u32 + ((i as f32 + crop) * pitch.size.h).round() as u32;
            let mut cell: Vec<f32> = Vec::new();
            for y in cell_y..cell_y + cell_h {
                for x in cell_x..cell_x + cell_w {
                    cell.push(img.get_pixel(x, y).to_luma_f32());
                }
            }
            cells.push(AnalyzeCell::new(cell));
        }
    }

    if cells.is_empty() {
        return Err("no cells extracted from image".into());
    }

    let (centroids, classes) = make_centroids(&cells[..]);

    Ok(AnalyzeCells {
        cols: cell_cols,
        centroids,
        cells,
        cell_classes: classes,
    })
}

fn make_centroids(cells: &[AnalyzeCell]) -> (Vec<AnalyzeCell>, Vec<usize>) {
    let dim = cells[0].data.len();

    let mut centroids = CentroidSet::new(cells);
    let mut classes: Vec<usize> = (0..cells.len()).collect();

    while centroids.num_centroids() > 3 {
        let (i, j) = centroids.argmin_distinct();
        assert!(i < j);

        // merge class j into class i
        for cls in classes.iter_mut() {
            if *cls == j {
                *cls = i;
            }
        }

        // delete centroid for class j
        let k = centroids.num_centroids() - 1;
        centroids.swap(j, k);
        centroids.truncate(k);
        for cls in classes.iter_mut() {
            if *cls == k {
                *cls = j;
            }
        }

        // update centroid for class i
        let updated = {
            let mut count = 0;
            let mut updated = AnalyzeCell::zero(dim);
            for (idx, cls) in classes.iter().enumerate() {
                if *cls == i {
                    count += 1;
                    updated.add(&cells[idx]);
                }
            }
            if count == 0 {
                eprintln!("nobody in {i}");
            }
            updated.mul_scalar(1.0 / count as f32);
            updated
        };
        centroids.set(i, updated);
    }

    (centroids.centroids, classes)
}

struct CentroidSet {
    centroids: Vec<AnalyzeCell>,
    distances: Vec<Vec<f32>>,
}

impl CentroidSet {
    fn new(cells: &[AnalyzeCell]) -> CentroidSet {
        let centroids: Vec<AnalyzeCell> = cells.iter().cloned().collect();

        let distances = {
            let mut distances: Vec<Vec<f32>> = Vec::new();
            for i in 0..cells.len() {
                let mut row: Vec<f32> = Vec::new();
                for j in 0..cells.len() {
                    let dist = centroids[i].distance(&centroids[j]);
                    row.push(dist);
                }
                distances.push(row);
            }
            distances
        };

        CentroidSet {
            centroids,
            distances,
        }
    }

    #[allow(unused)]
    fn dump_distances(&self) {
        eprint!("[");
        for (i, row) in self.distances.iter().enumerate() {
            if i != 0 {
                eprint!(",");
            }
            eprint!("[");
            for (j, col) in row.iter().enumerate() {
                if j != 0 {
                    eprint!(",");
                }
                eprint!("{col:6.1}");
            }
            eprintln!("]");
        }
        eprintln!("]");
    }

    fn num_centroids(&self) -> usize {
        self.centroids.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }

        self.centroids.swap(i, j);

        for k in 0..self.num_centroids() {
            if k != i && k != j {
                let self_distances_x = self.distances[i][k];
                self.distances[i][k] = self.distances[j][k];
                self.distances[j][k] = self_distances_x;

                let self_distances_x = self.distances[k][i];
                self.distances[k][i] = self.distances[k][j];
                self.distances[k][j] = self_distances_x;
            } else {
                // the (i,i) and (i,j) entries do not need to be changed
            }
        }
    }

    fn truncate(&mut self, n: usize) {
        self.distances.truncate(n);
        self.centroids.truncate(n);
        for dist in self.distances.iter_mut() {
            dist.truncate(n);
        }
    }

    fn set(&mut self, i: usize, data: AnalyzeCell) {
        self.centroids[i] = data;

        for j in 0..self.num_centroids() {
            if i != j {
                let dist = self.centroids[i].distance(&self.centroids[j]);
                self.distances[i][j] = dist;
                self.distances[j][i] = dist;
            }
        }
    }

    fn argmin_distinct(&self) -> (usize, usize) {
        let mut best_i = (0, 0);
        let mut best_x = std::f32::INFINITY;
        for i in 0..self.num_centroids() {
            for j in i + 1..self.num_centroids() {
                let dist = self.distances[i][j];
                if dist < best_x {
                    best_i = (i, j);
                    best_x = dist;
                }
            }
        }
        best_i
    }
}

impl fmt::Debug for AnalyzeCells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalyzeCells")
            .field("cols", &self.cols)
            .finish_non_exhaustive()
    }
}

impl AnalyzeCell {
    fn new(data: Vec<f32>) -> AnalyzeCell {
        AnalyzeCell { data }
    }

    fn zero(n: usize) -> AnalyzeCell {
        AnalyzeCell::new((0..n).map(|_| 0.0).collect())
    }

    fn add(&mut self, other: &AnalyzeCell) {
        assert_eq!(self.data.len(), other.data.len());
        for (x, y) in self.data.iter_mut().zip(other.data.iter()) {
            *x += *y;
        }
    }

    fn mul_scalar(&mut self, other: f32) {
        for x in self.data.iter_mut() {
            *x *= other;
        }
    }

    fn distance(&self, other: &AnalyzeCell) -> f32 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }
}

impl fmt::Debug for AnalyzeCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalyzeCell").finish_non_exhaustive()
    }
}

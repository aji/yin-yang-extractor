use std::fmt;

use image::GrayImage;
use rand::seq::IndexedRandom;

use crate::{AnalyzeGridBounds, AnalyzeGridPitch, pixel::PixelExt};

pub struct AnalyzeCells {
    pub cols: usize,
    pub centroids: Vec<AnalyzeCell>,
    pub cells: Vec<AnalyzeCell>,
    pub cell_classes: Vec<usize>,
}

pub struct AnalyzeCell {
    pub data: Vec<f32>,
}

pub fn analyze_cells(
    img: &GrayImage,
    pitch: &AnalyzeGridPitch,
    bounds: &AnalyzeGridBounds,
) -> AnalyzeCells {
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

    let mut centroids = vec![
        AnalyzeCell::new((0..cell_w * cell_h).map(|_| rand::random()).collect()),
        AnalyzeCell::new((0..cell_w * cell_h).map(|_| rand::random()).collect()),
        AnalyzeCell::new((0..cell_w * cell_h).map(|_| rand::random()).collect()),
    ];
    log::debug!("{}", cells[0].data.len());
    log::debug!("{}", centroids[0].data.len());
    let classes = k_means(&mut centroids[..], &cells[..]);

    AnalyzeCells {
        cols: cell_cols,
        centroids,
        cells,
        cell_classes: classes,
    }
}

fn k_means(centroids: &mut [AnalyzeCell], cells: &[AnalyzeCell]) -> Vec<usize> {
    let mut classes: Vec<usize> = (0..cells.len()).map(|_| centroids.len()).collect();

    loop {
        let mut changed = false;
        for (i, cell) in cells.iter().enumerate() {
            let closest = cell.closest(centroids);
            if closest != classes[i] {
                classes[i] = closest;
                changed = true;
            }
        }
        for (i, centroid) in centroids.iter_mut().enumerate() {
            let mut n = 0;
            for x in centroid.data.iter_mut() {
                *x = 0.0;
            }
            for (class, cell) in classes.iter().zip(cells.iter()) {
                if *class == i {
                    for (x, y) in centroid.data.iter_mut().zip(cell.data.iter()) {
                        *x += y;
                    }
                    n += 1;
                }
            }
            if n == 0 {
                changed = true;
                centroid.data = cells.choose(&mut rand::rng()).unwrap().data.clone();
            } else {
                for x in centroid.data.iter_mut() {
                    *x /= n as f32;
                }
            }
        }
        if !changed {
            break;
        }
    }

    classes
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

    fn distance(&self, other: &AnalyzeCell) -> f32 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }

    fn closest(&self, others: &[AnalyzeCell]) -> usize {
        let mut closest_i = 0;
        let mut closest_d = others[0].distance(self);
        for i in 1..others.len() {
            let d = others[i].distance(self);
            if d < closest_d {
                closest_i = i;
                closest_d = d;
            }
        }
        closest_i
    }
}

impl fmt::Debug for AnalyzeCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalyzeCell").finish_non_exhaustive()
    }
}

use std::fmt;

use image::GrayImage;

use crate::{
    math::{self, Array, HoldSampler},
    pixel::PixelExt,
};

#[derive(Debug)]
pub struct AnalyzeGridCommon {
    /// The image reduced along the x axis. Indices into these vectors are rows
    pub reduced_col: AnalyzeReducedAxis,

    /// The image reduced along the y axis. Indices into these vectors are cols
    pub reduced_row: AnalyzeReducedAxis,
}

pub struct AnalyzeReducedAxis {
    /// The average taken across the reduced axis, e.g. if this is the x axis
    /// reduction, then there is one entry per row and this is the average value
    /// for that row.
    pub values: Array,

    /// `reduced` with a Laplace filter applied. Has the same size.
    pub laplace: Array,

    /// The square of `laplace`
    pub laplace_sq: Array,

    /// Autocorrelation of `laplace`
    pub laplace_autocorr: Array,
}

pub fn analyze_grid_common(img: &GrayImage) -> AnalyzeGridCommon {
    let mut reduced_x: Vec<f32> = Vec::new();
    let mut reduced_y: Vec<f32> = Vec::new();

    for y in 0..img.height() {
        let sum: f32 = (0..img.width())
            .map(|x| img.get_pixel(x, y).to_luma_f32())
            .sum();
        reduced_x.push(sum / img.width() as f32);
    }
    for x in 0..img.width() {
        let sum: f32 = (0..img.height())
            .map(|y| img.get_pixel(x, y).to_luma_f32())
            .sum();
        reduced_y.push(sum / img.height() as f32);
    }

    AnalyzeGridCommon {
        reduced_col: AnalyzeReducedAxis::new(reduced_x),
        reduced_row: AnalyzeReducedAxis::new(reduced_y),
    }
}

impl AnalyzeReducedAxis {
    fn new(values: Vec<f32>) -> AnalyzeReducedAxis {
        if values.len() == 0 {
            panic!("empty reduced axis analysis input");
        }

        let sampler = HoldSampler::new(&values[..]);
        let laplace: Vec<f32> = (0..values.len() as isize)
            .map(|i| sampler.get(i - 1) + sampler.get(i + 1) - 2.0 * sampler.get(i))
            .collect();

        let laplace_sq: Vec<f32> = laplace.iter().map(|x| x.powi(2)).collect();
        let laplace_autocorr: Vec<f32> = math::naive_forward_autocorr(&laplace[..]);

        AnalyzeReducedAxis {
            values: Array::new(values),
            laplace: Array::new(laplace),
            laplace_sq: Array::new(laplace_sq),
            laplace_autocorr: Array::new(laplace_autocorr),
        }
    }
}

impl fmt::Debug for AnalyzeReducedAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalyzeReducedAxis").finish_non_exhaustive()
    }
}

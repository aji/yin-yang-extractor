use image::GrayImage;

use crate::{
    AnalyzeGridCommon, AnalyzeGridPitch, AnalyzeReducedAxis, geom, math::standard_normal,
    signal::ZeroSampler,
};

#[derive(Debug)]
pub struct AnalyzeGridBounds {
    pub bounds: crate::geom::Rect<usize>,
}

pub fn analyze_grid_bounds(
    _img: &GrayImage,
    common: &AnalyzeGridCommon,
    pitch: &AnalyzeGridPitch,
) -> AnalyzeGridBounds {
    let (offset_x, count_x) = analyze_grid_bounds_axis(&common.reduced_row, pitch.size.w);
    let (offset_y, count_y) = analyze_grid_bounds_axis(&common.reduced_col, pitch.size.h);

    let offset: geom::Point<usize> = (offset_x, offset_y).into();
    let size: geom::Size<usize> = (
        (count_x as f32 * pitch.size.w).round() as usize,
        (count_y as f32 * pitch.size.h).round() as usize,
    )
        .into();

    AnalyzeGridBounds {
        bounds: (offset, size).into(),
    }
}

fn analyze_grid_bounds_axis(axis: &AnalyzeReducedAxis, pitch: f32) -> (usize, usize) {
    let mut best_params: (usize, usize) = (0, 0);
    let mut best_score: f32 = 0.0;

    // Slight heuristic here to only consider grid dimensions between 4 and 50,
    // I happen to know Yin-Yangs with dimensions outside these are not exactly
    // realistic.
    let min_count = 4;
    let max_count = 50;

    for count in min_count..max_count {
        let this_width = pitch * count as f32;
        let max_offset = axis.values.data.len() as f32 - this_width;
        if max_offset < 0.0 {
            continue;
        }
        for offset in 0..=(max_offset as usize) {
            let this_params = (offset, count);
            let this_score = score_params(axis, pitch, offset, count);
            if this_score > best_score {
                best_params = this_params;
                best_score = this_score;
            }
        }
    }

    best_params
}

fn score_params(axis: &AnalyzeReducedAxis, pitch: f32, offset: usize, count: usize) -> f32 {
    let sampler = ZeroSampler::new(0.0, &axis.laplace_sq.data[..]);
    let mut total = 0.0;
    for gridline_index in 0..=count {
        let i = (offset as f32 + gridline_index as f32 * pitch).round() as isize;
        total += (-10..=10)
            .map(|j| {
                let val = if *sampler.get(i + j) < axis.laplace_sq.mean {
                    0.0
                } else {
                    1.0
                };
                val * standard_normal(j as f32)
            })
            .sum::<f32>();
    }
    total
}

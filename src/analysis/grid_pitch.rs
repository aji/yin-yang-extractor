use image::GrayImage;

use crate::{
    AnalyzeGridCommon, AnalyzeReducedAxis,
    math::{self, ZeroSampler},
};

#[derive(Debug)]
pub struct AnalyzeGridPitch {
    pub size: crate::geom::Size<f32>,
}

pub fn analyze_grid_pitch(_img: &GrayImage, common: &AnalyzeGridCommon) -> AnalyzeGridPitch {
    let pitch_x = analyze_grid_pitch_axis(&common.reduced_row);
    let pitch_y = analyze_grid_pitch_axis(&common.reduced_col);

    let pitch = make_square(pitch_x, pitch_y);

    AnalyzeGridPitch {
        size: (pitch, pitch).into(),
    }
}

fn analyze_grid_pitch_axis(axis: &AnalyzeReducedAxis) -> f32 {
    let peak = (math::argmax(&axis.laplace_autocorr.data[2..]) + 2) as f32;
    let subharms = ((peak / 10.0).round() as usize).max(1).min(10);

    let data = ZeroSampler::new(0.0, &axis.laplace_autocorr.data[..]);
    let (pitch, score) = (1..=subharms)
        .map(|x| {
            let x = x as f32;
            let (pitch, score) = refine_pitch(&data, peak / x);
            (pitch, score * x)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    log::debug!("done: pitch={} score={}", pitch, score);
    pitch
}

fn refine_pitch(data: &ZeroSampler<f32>, init: f32) -> (f32, f32) {
    let mut pitch = init;

    for iter in 0.. {
        let mut total_y = 0.0;
        let mut total_dy = 0.0;

        for i in 1.. {
            let x = pitch * i as f32;
            if x > data.len() as f32 {
                break;
            }
            let (y, dy) = data.get_linear_grad(pitch);
            total_y += y;
            total_dy += dy;
        }

        if iter > 1000 {
            return (pitch, total_y);
        } else {
            pitch += 0.001 * total_dy;
        }
    }

    unreachable!()
}

fn make_square(a: f32, b: f32) -> f32 {
    if b < a {
        return make_square(b, a);
    }
    // b is approximately n*a for integer n
    let n = (b / a).round();
    (a + b / n) / 2.0
}

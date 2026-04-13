use image::GrayImage;

use crate::{
    AnalyzeGridCommon, AnalyzeReducedAxis,
    math::{self},
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
    let mut tot_pitch = (math::argmax(&axis.laplace_autocorr.data[2..100]) + 2) as f32;
    let mut num_pitch = 1.0;

    let _ = {
        // dumb hack to check if pitch/2 is a possibility. sometimes the argmax is the 2nd peak
        let pitch = tot_pitch / num_pitch;
        let idx_a = (pitch * 0.4).round() as usize;
        let idx_b = (pitch * 0.6).round() as usize;
        let idx = math::argmax(&axis.laplace_autocorr.data[idx_a..idx_b]) + idx_a;
        if axis.laplace_autocorr.data[idx] * 3.0 > axis.laplace_autocorr.data[tot_pitch as usize] {
            tot_pitch = idx as f32;
        }
    };

    log::debug!("pitch={} (init)", tot_pitch / num_pitch);

    let mut refine = 2.0f32;
    let max_refine = (axis.laplace_autocorr.data.len() as f32) / 2.0;
    while (tot_pitch / num_pitch) * refine < max_refine {
        let pitch = tot_pitch / num_pitch;
        let idx_a = (pitch * (refine - 0.25)).round() as usize;
        let idx_b = (pitch * (refine + 0.25)).round() as usize;
        let next = math::argmax(&axis.laplace_autocorr.data[idx_a..idx_b]) + idx_a;
        tot_pitch += next as f32 / refine;
        num_pitch += 1.0;
        log::debug!("refine={} pitch={}", refine, tot_pitch / num_pitch);
        refine += 1.0;
    }

    tot_pitch / num_pitch
}

fn make_square(a: f32, b: f32) -> f32 {
    if b < a {
        return make_square(b, a);
    }
    // b is approximately n*a for integer n
    let n = (b / a).round();
    (a + b / n) / 2.0
}

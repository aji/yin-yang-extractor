mod analysis;
mod error;
mod geom;
mod math;
mod pixel;

use std::path::Path;

pub use analysis::*;
pub use error::*;

pub use image::GrayImage;
pub use puzzle_grid::array::ArrayBuffer;
pub use pzpr_codec::yinyang::Cell;

pub fn extract_from_image(img: &GrayImage) -> AnalyzeResult<ArrayBuffer<Cell>> {
    let x0 = analyze_grid_common(img)?;
    let x1 = analyze_grid_pitch(img, &x0)?;
    let x2 = analyze_grid_bounds(img, &x0, &x1)?;
    let x3 = analyze_cells(img, &x1, &x2)?;
    let x4 = analyze_puzzle(&x3)?;
    Ok(x4.grid)
}

pub fn extract_from_image_file(fname: impl AsRef<Path>) -> AnalyzeResult<ArrayBuffer<Cell>> {
    let img = image::ImageReader::open(fname)
        .map_err(|e| format!("could not open image: {e}"))?
        .decode()
        .map_err(|e| format!("could not decode image: {e}"))?;
    extract_from_image(&img.into_luma8())
}

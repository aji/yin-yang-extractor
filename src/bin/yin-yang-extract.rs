use clap::Parser;
use image::{ImageFormat, ImageReader, Rgb, RgbImage, buffer::ConvertBuffer};
use yin_yang_extractor::Cell;

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: String,

    #[arg(long)]
    debug_output: Option<String>,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    log::debug!("loading {}", cli.input);
    let img = ImageReader::open(&cli.input)
        .expect("could not open image")
        .decode()
        .expect("could not decode image")
        .into_luma8();

    log::info!("analyzing {}", cli.input);
    let grid_common = yin_yang_extractor::analyze_grid_common(&img);
    log::debug!("analyze_grid_common() -> {:?}", grid_common);
    let grid_pitch = yin_yang_extractor::analyze_grid_pitch(&img, &grid_common);
    log::debug!("analyze_grid_pitch() -> {:?}", grid_pitch);
    let grid_bounds = yin_yang_extractor::analyze_grid_bounds(&img, &grid_common, &grid_pitch);
    log::debug!("analyze_grid_bounds() -> {:?}", grid_bounds);
    let cells = yin_yang_extractor::analyze_cells(&img, &grid_pitch, &grid_bounds);
    log::debug!("analyze_cells() -> {:?}", cells);
    let puzzle = yin_yang_extractor::analyze_puzzle(&cells);
    log::debug!("analyze_puzzle() -> {:?}", puzzle);

    if let Some(out_fname) = cli.debug_output {
        let mut out: RgbImage = img.convert();
        let cell = grid_pitch.size;
        let rect = grid_bounds.bounds;

        let blue: Rgb<u8> = [0, 0, 255].into();
        let rows = ((rect.y1 - rect.y0) as f32 / cell.h).round() as usize;
        for row in 1..rows {
            let y = rect.y0 + (row as f32 * cell.h).round() as usize;
            for x in rect.x0..rect.x1 {
                out.put_pixel(x as u32, y as u32, blue);
            }
        }
        let cols = ((rect.x1 - rect.x0) as f32 / cell.w).round() as usize;
        for col in 1..cols {
            let x = rect.x0 + (col as f32 * cell.w).round() as usize;
            for y in rect.y0..rect.y1 {
                out.put_pixel(x as u32, y as u32, blue);
            }
        }

        let red: Rgb<u8> = [255, 0, 0].into();
        for x in rect.x0..rect.x1 {
            out.put_pixel(x as u32, rect.y0 as u32, red);
            out.put_pixel(x as u32, rect.y1 as u32, red);
        }
        for y in rect.y0..rect.y1 {
            out.put_pixel(rect.x0 as u32, y as u32, red);
            out.put_pixel(rect.x1 as u32, y as u32, red);
        }

        for (i, _) in cells.cell_classes.iter().enumerate() {
            let cell_row = i / cells.cols;
            let cell_col = i % cells.cols;
            let color: Rgb<u8> = match puzzle.grid[(cell_row, cell_col)] {
                Cell::Empty => [0, 255, 0].into(),
                Cell::Black => [0, 0, 255].into(),
                Cell::White => [255, 0, 0].into(),
            };
            let cell_x = (rect.x0 as f32 + cell.w * (cell_col as f32 + 0.3)) as u32;
            let cell_y = (rect.y0 as f32 + cell.h * (cell_row as f32 + 0.3)) as u32;
            let cell_w = (cell.w * 0.4) as u32;
            let cell_h = (cell.h * 0.4) as u32;
            for x in cell_x..(cell_x + cell_w) {
                for y in cell_y..(cell_y + cell_h) {
                    out.put_pixel(x, y, color);
                }
            }
        }

        out.save_with_format(&out_fname, ImageFormat::Png)
            .expect("could not save debug output");
        log::info!("wrote debug output to {out_fname}");
    }

    print!("{}", puzzle.grid);
}

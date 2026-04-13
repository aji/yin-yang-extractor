use clap::{Parser, ValueEnum};
use image::{GrayImage, ImageFormat, ImageReader, Rgb, RgbImage, buffer::ConvertBuffer};
use pzpr_codec::{
    grid::{Grid, Gridlike},
    variety::yinyang::{self, Cell},
};
use yin_yang_extractor::{AnalyzeCells, AnalyzeGridBounds, AnalyzeGridPitch, AnalyzePuzzle};

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: String,

    #[arg(short, long)]
    format: Option<OutputFormat>,

    #[arg(long)]
    debug_output: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Ascii,
    Url,
}

impl OutputFormat {
    fn display(&self, grid: &Grid<Cell>) {
        match self {
            OutputFormat::Ascii => {
                for r in 0..grid.shape().rows() {
                    for c in 0..grid.shape().cols() {
                        if c != 0 {
                            print!(" ");
                        }
                        match grid.rc(r, c) {
                            Cell::Empty => print!("."),
                            Cell::Black => print!("B"),
                            Cell::White => print!("W"),
                        }
                    }
                    println!("");
                }
            }
            OutputFormat::Url => {
                let pzpr = yinyang::encode(grid).unwrap();
                println!("https://puzz.link/p?{pzpr}");
            }
        }
    }
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

    debug_output(
        &img,
        cli.debug_output.as_ref(),
        &grid_pitch,
        &grid_bounds,
        None,
        None,
    );

    let cells = yin_yang_extractor::analyze_cells(&img, &grid_pitch, &grid_bounds);
    log::debug!("analyze_cells() -> {:?}", cells);

    debug_output(
        &img,
        cli.debug_output.as_ref(),
        &grid_pitch,
        &grid_bounds,
        Some(&cells),
        None,
    );

    let puzzle = yin_yang_extractor::analyze_puzzle(&cells);
    log::debug!("analyze_puzzle() -> {:?}", puzzle);

    debug_output(
        &img,
        cli.debug_output.as_ref(),
        &grid_pitch,
        &grid_bounds,
        Some(&cells),
        Some(&puzzle),
    );

    cli.format
        .unwrap_or(OutputFormat::Ascii)
        .display(&puzzle.grid);
}

fn debug_output(
    img: &GrayImage,
    out_fname: Option<&String>,
    grid_pitch: &AnalyzeGridPitch,
    grid_bounds: &AnalyzeGridBounds,
    cells: Option<&AnalyzeCells>,
    puzzle: Option<&AnalyzePuzzle>,
) {
    let Some(out_fname) = out_fname else {
        return;
    };

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

    if let Some(cells) = cells {
        for (i, _) in cells.cell_classes.iter().enumerate() {
            let cell_row = (i / cells.cols) as isize;
            let cell_col = (i % cells.cols) as isize;
            let color: Rgb<u8> = if let Some(puzzle) = puzzle {
                match puzzle.grid.rc(cell_row, cell_col) {
                    Cell::White => [255, 0, 0].into(),
                    Cell::Empty => [0, 255, 0].into(),
                    Cell::Black => [0, 0, 255].into(),
                }
            } else {
                match cells.cell_classes[i] {
                    0 => [255, 0, 0].into(),
                    1 => [0, 255, 0].into(),
                    2 => [0, 0, 255].into(),
                    _ => [255, 255, 0].into(),
                }
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
    }

    out.save_with_format(&out_fname, ImageFormat::Png)
        .expect("could not save debug output");
    log::info!("wrote debug output to {out_fname}");
}

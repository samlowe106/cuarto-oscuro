use std::path::PathBuf;

use walkdir::WalkDir;

use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Input folder path
    path: PathBuf,
    /// Input image formats
    #[arg(short, long, default_values = ["jpg", "jpeg", "png", "bmp", "tiff"])]
    input_formats: Vec<String>,
    /// Target format
    #[arg(short, long, default_value = "webp")]
    out_format: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let inpaths_outpaths: Vec<(PathBuf, PathBuf)> = WalkDir::new(args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| args.input_formats.contains(&ext.to_lowercase()))
        })
        .map(|e| e.path().to_owned())
        .map(|input| {
            let output = input.with_extension(&args.out_format);
            (input, output)
        })
        .collect();

    for (inpath, outpath) in inpaths_outpaths {
        image::open(inpath)?.save(outpath)?;
    }

    Ok(())
}

use clap::{Arg, Command};
use crate::ascii::convert;
use std::io;

pub fn run() -> io::Result<()> {
    let matches = Command::new("ASCII Art Generator")
            .version("0.0.1")
            .author("Maddie Albu")
            .about("Convert images into ASCII art.")
            .arg(
                Arg::new("image-path")
                    .short('i')
                    .long("image-path")
                    .required(true)
                    .value_name("IMAGE")
                    .help("Path to the input image"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Path to the output ASCII text file (default: output.txt)"),
            )
            .arg(
                Arg::new("scale")
                    .short('s')
                    .long("scale")
                    .default_value("0.43")
                    .value_name("SCALE")
                    .help("Scale factor for adjusting aspect ratio"),
            )
            .arg(
                Arg::new("columns")
                    .short('c')
                    .long("columns")
                    .default_value("70")
                    .value_name("COLUMNS")
                    .help("Number of ASCII characters (columns) per row"),
            )
            .get_matches();


    let image_path = matches.get_one::<String>("image-path").unwrap();
    let fallback = "output.txt".to_string();
    let output = matches.get_one::<String>("output").unwrap_or(&fallback);
    let scale: f32 = matches.get_one::<String>("scale").unwrap().parse().expect("Invalid scale");
    let columns: u32 = matches.get_one::<String>("columns").unwrap().parse().expect("Invalid columns");

    let img = image::open(image_path).expect("Failed to open image.");
    let ascii_image = convert(img, columns, scale);

    std::fs::write(output, ascii_image.join("\n"))?;
    println!("{}", ascii_image.join("\n"));
    Ok(())
}
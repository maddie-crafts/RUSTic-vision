use leptos::html::Output;
use rustic_vision::ascii;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    image_path: String,
}

fn main() {
    let args = Cli::parse();
    let image = image::open(&args.image_path).expect("Failed to open image");
    let ascii = ascii::convert(image, 80, 2.0);
    for line in ascii {
        println!("{}", line);
    }
}
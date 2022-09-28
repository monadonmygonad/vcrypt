use std::path::Path;

use clap::{Parser, Subcommand};
use image::{self, imageops::*, RgbImage};

mod crypto;
mod img;

const ENCRYPTED_WIDTH: usize = IMG_WIDTH * 2;
const ENCRYPTED_HEIGHT: usize = IMG_HEIGHT * 2;
const IMG_HEIGHT: usize = 500;
const IMG_WIDTH: usize = 500;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image path
    #[clap(short, long, value_parser)]
    image_path: String,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Encrypt stuff
    Encrypt {
        file: String,
    },
    // Decrypt stuff
    Decrypt {
        ciphered: String,
        shared: String,
        output: String,
    },
}

fn encrypt(file: &str) {
    let sourceimage = image::open(file).unwrap();
    let grayscale_image = sourceimage.grayscale();
    let mut targetimage = grayscale_image.to_luma8();
    dither(&mut targetimage, &BiLevel);

    let mut pixels = targetimage.pixels().map(img::to_one_or_zero);

    let mut image = img::create_grid(IMG_WIDTH, IMG_HEIGHT);

    for i in 0..IMG_HEIGHT {
        for j in 0..IMG_WIDTH {
            let pixel = pixels.next().unwrap();
            image[j][i] = pixel;
        }
    }

    let share = crypto::random::<u8>(ENCRYPTED_WIDTH, ENCRYPTED_HEIGHT);
    let ciphered = crypto::ciphered(&image, &share);

    let base_file = Path::new(file);

    save_with_suffix(&share, &base_file, ".share.png");
    save_with_suffix(&ciphered, &base_file, ".ciphered.png");
}

fn save_with_suffix(vec: &Vec<Vec<u8>>, base_file: &Path, suffix: &str) {
    let mut filename = String::from("");
    filename.push_str(base_file.parent().unwrap().to_str().unwrap());
    filename.push_str("/");
    filename.push_str(base_file.file_stem().unwrap().to_str().unwrap());
    filename.push_str(&suffix);

    let imagebuffer = RgbImage::from_raw(
        ENCRYPTED_WIDTH as u32,
        ENCRYPTED_HEIGHT as u32,
        img::flatten_rgb(&vec),
    )
    .unwrap();
    imagebuffer.save(filename).unwrap();
}

fn decrypt(ciphered: &str, shared: &str, output: &str) {
    let ciphered_image = image::open(ciphered).unwrap();
    let shared_image = image::open(shared).unwrap();

    let ciphered = img::to_ones_and_zeros(ciphered_image);
    let share = img::to_ones_and_zeros(shared_image);
    let recovered = crypto::recover(&ciphered, &share);

    let imagebuffer = RgbImage::from_raw(
        ENCRYPTED_WIDTH as u32,
        ENCRYPTED_HEIGHT as u32,
        img::flatten_rgb(&recovered),
    )
    .unwrap();
    imagebuffer.save(&output).unwrap();
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { file } => {
            encrypt(file);
        }
        Commands::Decrypt {
            ciphered,
            shared,
            output,
        } => {
            decrypt(ciphered, shared, output);
        }
    }
}

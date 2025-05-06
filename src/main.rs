use clap::{ArgAction, FromArgMatches, Parser};
use image::{ImageBuffer, ImageReader, Pixel};
use std::{fs, path::{self, Path}, str::FromStr};
use crate::processor::Processor;

mod processor;

#[derive(Parser)]
struct Cli {
    #[clap(long, short, action)] // ArgAction::SetFalse
    debug: bool,
    #[clap(long, short, action)]
    transparency_test: bool,
    #[clap(long, short, action)]
    set_black: bool,
    path: String,
}

fn main() {
    let args = Cli::parse();

    println!("wsm creates worn textures!");
    println!("path: {:?}", args.path);
    let mut image = ImageReader::open(args.path.clone()).unwrap().decode().unwrap().into_rgba8();
    let image_processor = Processor::new(image.width() as u64, image.height() as u64);

    println!("Number of points: {:?}", image_processor.points.len());

    if args.debug == true && args.transparency_test == true {
        print!("[WARN]: debug mode and transparency test is both on, only debug will be used.")
    }

    if args.set_black == true {
        for x in 0..image.width() {
            for y in 0..image.height() {
                image.put_pixel(x, y, image_processor.set_black([x as u64, y as u64], image.get_pixel(x, y)));
            }
        }
    }
    if args.debug == true {
        for x in 0..image.width() {
            for y in 0..image.height() {
                image.put_pixel(x, y, image_processor.process_pixel_debug([x as u64, y as u64], image.get_pixel(x, y)));
            }
        }
    } else if args.transparency_test == true {
        for x in 0..image.width() {
            for y in 0..image.height() {
                image.put_pixel(x, y, image_processor.transparency_test([x as u64, y as u64], image.get_pixel(x, y)));
            }
        }
    } else {
        for x in 0..image.width() {
            for y in 0..image.height() {
                image.put_pixel(x, y, image_processor.process_pixel([x as u64, y as u64], image.get_pixel(x, y)));
            }
        }
    }

    image.save(args.path).unwrap();
}
use image::{ImageReader, Rgba};
use noise::{NoiseFn, Simplex, Vector2};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{vec};

pub const POINT_FREQUENCY: u16 = 10; // for every x pixels, spawn a point
pub const POINT_SIZE_RANGE: [u8; 2] = [1, 2]; // how big points can be (in pixels)
pub const NOISE_DIVISION: f64 = 20.0;

pub struct PointData {
  position: [u64; 2],
  size: u8,
}
pub struct Processor {
  rng: ThreadRng,
  noise: Simplex,
  pub points: Vec<PointData>,
}

impl Processor {
  pub fn new(image_width: u64, image_height: u64) -> Processor
  {
    let mut rng = rand::rng();
    let mut points = vec![];
    for _ in 0..(image_width*image_height/POINT_FREQUENCY as u64) {
      let size = rng.random_range(POINT_SIZE_RANGE[0]..POINT_SIZE_RANGE[1]);
      let position_x = rng.random_range(0..image_width);
      let position_y = rng.random_range(0..image_height);
      
      points.push(PointData {
        position: [position_x, position_y], 
        size: size
      });
    }
    
    Processor {
      rng: rng,
      noise: Simplex::new(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32), // todo make this random
      points: points,
    }
  }
  
  fn get_magnitude(p1: [f64; 2], p2: [f64; 2]) -> f64
  {
    let p1_vec = Vector2::new(p1[0], p1[1]);
    let p2_vec = Vector2::new(p2[0], p2[1]);
    (p1_vec-p2_vec).magnitude()
  }
  
  pub fn process_pixel(self: &Processor, position: [u64; 2], pixel: &Rgba<u8>) -> Rgba<u8> {
    // do some noise on it for distributed wearing
    let mut output_pixel: Rgba<u8> = pixel.clone();
    output_pixel.0[3] = (output_pixel.0[3] as f64 * ((self.noise.get([position[0] as f64 / NOISE_DIVISION, position[1] as f64 / NOISE_DIVISION]).abs()/3.0) +
      (self.noise.get([position[0] as f64 / NOISE_DIVISION * 2.0, position[1] as f64 / NOISE_DIVISION * 2.0]).abs()/3.0) + // octave 2 
      (self.noise.get([position[0] as f64 / NOISE_DIVISION * 4.0, position[1] as f64 / NOISE_DIVISION * 4.0]).abs()/3.0)) // octave 3
    ) as u8;

    for point in &self.points {
      let dist = Processor::get_magnitude([point.position[0] as f64, point.position[1] as f64], [position[0] as f64, position[1] as f64]);
      if dist < point.size as f64 {
        output_pixel.0[3] = 0;
      }
    }

    output_pixel
  }

  pub fn process_pixel_debug(self: &Processor, position: [u64; 2], pixel: &Rgba<u8>) -> Rgba<u8> {
        // do some noise on it for distributed wearing
        // g
        let mut output_pixel: Rgba<u8> = pixel.clone();
        output_pixel.0[1] = (output_pixel.0[1] as f64 + 
          255.0 * ((self.noise.get([position[0] as f64 / NOISE_DIVISION, position[1] as f64 / NOISE_DIVISION]).abs()/3.0) +
          (self.noise.get([position[0] as f64 / NOISE_DIVISION * 2.0, position[1] as f64 / NOISE_DIVISION * 2.0]).abs()/3.0) + // octave 2 
          (self.noise.get([position[0] as f64 / NOISE_DIVISION * 4.0, position[1] as f64 / NOISE_DIVISION * 4.0]).abs()/3.0)) // octave 3
        ) as u8;
        println!("{}", output_pixel.0[1]);
        
        // r and b (r for points)
        for point in &self.points {
          let dist = Processor::get_magnitude([point.position[0] as f64, point.position[1] as f64], [position[0] as f64, position[1] as f64]);
          if dist < point.size as f64 {
            output_pixel.0[2] = 255;
          }
          if position == point.position {
            output_pixel.0[0] = 255;
          }
        }
    
        output_pixel
  }

  pub fn transparency_test(self: &Processor, position: [u64; 2], pixel: &Rgba<u8>) -> Rgba<u8> {
    return Rgba([255,255,255,position[0] as u8%255]);
  }

  pub fn set_black(self: &Processor, position: [u64; 2], pixel: &Rgba<u8>) -> Rgba<u8> {
    return Rgba([0,0,0,255]);
  }
}
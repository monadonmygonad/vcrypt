use image::{self, DynamicImage, GenericImageView, Luma};

use crate::{ENCRYPTED_HEIGHT, ENCRYPTED_WIDTH};

pub fn create_grid<T>(width: usize, height: usize) -> Vec<Vec<T>>
where
    T: std::default::Default + Clone,
{
    return vec![vec![T::default(); width]; height];
}

pub fn flatten_rgb(inout_vec: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..ENCRYPTED_HEIGHT {
        for j in 0..ENCRYPTED_WIDTH {
            let pixel: u8 = match inout_vec[j][i] {
                1 => 255,
                0 => 0,
                2..=255 => 0,
            };
            vec.push(pixel);
            vec.push(pixel);
            vec.push(pixel);
        }
    }
    vec
}

pub fn to_one_or_zero(pixel: &Luma<u8>) -> u8 {
    let val = pixel.0[0];
    return match val {
        255 => 1,
        0..=254 => 0,
    };
}

pub fn to_ones_and_zeros(image: DynamicImage) -> Vec<Vec<u8>> {
    let mut pixels = image
        .pixels()
        .map(|x| x.2.clone())
        .map(|x| x.0.get(0).unwrap().clone());

    let mut image = create_grid(ENCRYPTED_HEIGHT, ENCRYPTED_WIDTH);

    for i in 0..ENCRYPTED_HEIGHT {
        for j in 0..ENCRYPTED_WIDTH {
            let pixel = match pixels.next().unwrap() {
                255 => 1,
                0..=254 => 0,
            };
            image[j][i] = pixel;
        }
    }

    image
}

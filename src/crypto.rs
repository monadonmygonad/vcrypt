use rand::Rng;

use crate::{ENCRYPTED_HEIGHT, ENCRYPTED_WIDTH};
const TILE_SIZE: usize = 2;

pub trait Inverser {
    type Output;

    fn inverse(self) -> Self::Output;
}

impl Inverser for u8 {
    type Output = u8;

    fn inverse(self) -> Self::Output {
        match self {
            0 => 1,
            _ => 0,
        }
    }
}

fn assert_double<T>(v1: &Vec<Vec<T>>, v2: &Vec<Vec<T>>) {
    assert!(total_items(v1) == total_items(v2) / 4, "must be double");
}

#[allow(path_statements)]
#[allow(clippy::no_effect)]
pub fn ciphered<T>(a: &Vec<Vec<T>>, b: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy + std::cmp::PartialEq + std::default::Default + Inverser<Output = T>,
{
    assert_double(a, b);

    let mut c = crate::img::create_grid(ENCRYPTED_WIDTH, ENCRYPTED_HEIGHT);
    for i in (0..ENCRYPTED_HEIGHT).step_by(TILE_SIZE) {
        for j in (0..ENCRYPTED_WIDTH).step_by(TILE_SIZE) {
            let (first, second) = (a[i / 2][j / 2], b[i][j]);
            let leading = match first == second {
                true => T::default(),
                false => T::default().inverse(),
            };
            let following = leading.inverse();
            set_tile(&mut c, i, j, leading, following);
        }
    }
    c
}

pub trait RandomGenerator {
    type Output;

    fn random() -> Self::Output;
}

impl RandomGenerator for u8 {
    type Output = u8;

    fn random() -> Self::Output {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=1)
    }
}

pub fn random<T>(width: usize, height: usize) -> Vec<Vec<T>>
where
    T: Copy + std::default::Default + RandomGenerator<Output = T> + Inverser<Output = T>,
{
    let mut c = crate::img::create_grid(width, height);

    for i in (0..height).step_by(TILE_SIZE) {
        for j in (0..width).step_by(TILE_SIZE) {
            let leading = T::random();
            let following = leading.inverse();
            set_tile(&mut c, i, j, leading, following);
        }
    }
    c
}

fn set_tile<T: Copy>(a: &mut Vec<Vec<T>>, i: usize, j: usize, leading: T, following: T) {
    a[i][j] = leading;
    a[i][j + 1] = following;
    a[i + 1][j] = following;
    a[i + 1][j + 1] = leading;
}

fn total_items<T>(v: &Vec<Vec<T>>) -> usize {
    return v.into_iter().flatten().count();
}

pub fn recover(a: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut c = crate::img::create_grid(ENCRYPTED_WIDTH, ENCRYPTED_HEIGHT);
    for i in 0..ENCRYPTED_HEIGHT {
        for j in 0..ENCRYPTED_WIDTH {
            c[i][j] = a[i][j] ^ b[i][j];
        }
    }
    c
}

use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;
use itertools::Itertools;

use crate::{Spec, Pixel, Image};

static RESIZE_FILTER: FilterType = FilterType::Nearest;

pub fn pixelize(image: &DynamicImage, spec: Spec) -> Image {
    let (w, h) = size(image, spec);
    let image = image.resize_exact(2 * w, 2 * h, RESIZE_FILTER);

    let pixels = (0..h).cartesian_product(0..w)
        .map(|(y, x)| [
             image.get_pixel(x*2, y*2).0,
             image.get_pixel(x*2+1, y*2).0,
             image.get_pixel(x*2, y*2+1).0,
             image.get_pixel(x*2+1, y*2+1).0,
        ])
        .map(|block| match spec.block {
            1 => do_pixel_1(&block),
            2 => do_pixel_2(&block),
            _ => do_pixel_4(&block),
        })
    .collect();

    Image {
        size: (w, h),
        pixels: pixels,
    }
}

fn size(image: &DynamicImage, spec: Spec) -> (u32, u32) {
    let (max_w, max_h) = spec.size;
    let (image_w, image_h) = image.dimensions();
    let ratio = (image_h as f32) / (image_w as f32) * spec.font;

    let w = max_h as f32 / ratio;
    let h = max_w as f32 * ratio;

    if h as u32 > max_h {
        (w as u32, max_h as u32)
    } else {
        (max_w as u32, h as u32)
    }
}

fn do_pixel_1(block: &[[u8; 4]; 4]) -> Pixel {
    let color = blend(block);
    Pixel {
        fg: color,
        bg: color,
        ch: ' ',
    }
}

fn do_pixel_2(block: &[[u8; 4]; 4]) -> Pixel {
    let fg = blend(&block[0..2]);
    let bg = blend(&block[2..4]);
    Pixel {
        fg: fg,
        bg: bg,
        ch: '▀',
    }
}

fn do_pixel_4(block: &[[u8; 4]; 4]) -> Pixel {
    static CHARS: [char; 7] = ['▘', '▝', '▖', '▗', '▀', '▌', '▚'];
    let (_, fg, bg, ch) = (0..7)
        .map(|n| {
            let (a, b, c, d) = (block[0], block[1], block[2], block[3]);
            let (a, b, c, d) = match n {
                0 => (a, b, c, d),
                1 => (b, a, c, d),
                2 => (c, b, a, d),
                3 => (d, b, c, a),
                4 => (a, b, c, d),
                5 => (a, c, b, d),
                6 => (a, d, b, c),
                _ => panic!(),
            };

            let block = [a, b, c, d];
            let (diff, fg, bg) = if n < 4 {
                let bcd = blend(&block[1..]);
                let diff = diff(b, bcd) + diff(c, bcd) + diff(d, bcd);
                (diff, a, bcd)
            } else {
                let ab = blend(&block[..2]);
                let cd = blend(&block[2..]);
                let diff = diff(a, ab) + diff(b, ab) + diff(c, cd) + diff(d, cd);
                (diff, ab, cd)
            };

            (diff, fg, bg, CHARS[n])
        })
    .min_by_key(|x| x.0)
        .unwrap();

    Pixel {
        fg: fg,
        bg: bg,
        ch: ch,
    }
}

fn blend(pixels: &[[u8; 4]]) -> [u8; 4] {
    let (mut r, mut g, mut b, mut a) = (0f32, 0f32, 0f32, 0f32);
    if pixels.len() > 0 {
        let k = 1f32 / pixels.len() as f32;

        for p in pixels {
            r += k * p[0] as f32;
            g += k * p[1] as f32;
            b += k * p[2] as f32;
            a += k * p[3] as f32;
        }
    }

    [r as u8, g as u8, b as u8, a as u8]
}

fn diff(a: [u8; 4], b: [u8; 4]) -> i32 {
    (0..3).fold(0i32, |sum, i| sum + (a[i] as i32 - b[i] as i32).pow(2))
}

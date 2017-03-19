use std::ops::{Index, Range};
use itertools::Itertools;
use termion::{color, style};

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub fg: [u8; 4],
    pub bg: [u8; 4],
    pub ch: char,
}

pub struct Image {
    pub size: (u32, u32),
    pub pixels: Vec<Pixel>,
}

impl Pixel {
    pub fn render(&self) -> String {
        let (fg, bg) = (rgb(&self.fg), rgb(&self.bg));
        format!("{}{}{}", color::Fg(fg), color::Bg(bg), self.ch)
    }
}

impl Image {
    pub fn render(&self) -> String {
        let (w, h) = self.size;

        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| self[(x, y)].render())
                    .fold(String::new(), |res, pixel| format!("{}{}", res, pixel))
            })
            .fold(String::new(),
                  |res, line| format!("{}{}{}\r\n", res, line, style::Reset))
    }

    pub fn sub_image(&self, x: Range<u32>, y: Range<u32>) -> Self {
        let (w, h) = ((x.end - x.start), (y.end - y.start));

        let pixels = y.cartesian_product(x)
            .map(|(y, x)| self[(x, y)])
            .collect::<Vec<Pixel>>();

        Image {
            size: (w, h),
            pixels: pixels,
        }
    }
}

impl Index<(u32, u32)> for Image {
    type Output = Pixel;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let (w, _) = self.size;
        let i = (y * w + x) as usize;
        &self.pixels[i]
    }
}

fn rgb(pixel: &[u8; 4]) -> color::Rgb {
    let (r, g, b, a) =
        (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32, pixel[3] as f32 / 255f32);

    color::Rgb((r * a) as u8, (g * a) as u8, (b * a) as u8)
}


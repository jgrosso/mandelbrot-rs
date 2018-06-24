extern crate image;

use image::ImageBuffer;
use std::u8;

static MAX_ITERATIONS: u16 = 10000;
static PALETTE_SIZE: u16 = 40;

static SCREEN_HEIGHT: u16 = 16384;
static SCREEN_WIDTH: u16 = 16384;

struct Interval {
    lower_bound: f32,
    upper_bound: f32,
}

impl Interval {
    fn range(&self) -> f32 {
        self.upper_bound - self.lower_bound
    }
}

fn scale(value: f32, input: Interval, output: Interval) -> f32 {
    ((value / input.range()) * output.range()) + output.lower_bound
}

fn mandelbrot(pixel_x: u16, pixel_y: u16) -> image::Rgb<u8> {
    let x0 = scale(
        pixel_x as f32,
        Interval {
            lower_bound: 0.0,
            upper_bound: SCREEN_WIDTH as f32,
        },
        Interval {
            lower_bound: -2.5,
            upper_bound: 1.0,
        },
    );
    let y0 = scale(
        pixel_y as f32,
        Interval {
            lower_bound: 0.0,
            upper_bound: SCREEN_HEIGHT as f32,
        },
        Interval {
            lower_bound: -1.0,
            upper_bound: 1.0,
        },
    );

    let mut x = x0;
    let mut y = y0;
    let mut iteration: u16 = 0;
    while (x * x) + (y * y) < 4.0 && iteration < MAX_ITERATIONS {
        let temp_x = (x * x) - (y * y) + x0;
        y = (2.0 * x * y) + y0;
        x = temp_x;

        iteration += 1;
    }

    iteration_to_pixel(iteration)
}

struct HexColor(String);
impl HexColor {
    fn to_rgb(&self) -> image::Rgb<u8> {
        let red = self.extract_channel(0);
        let green = self.extract_channel(2);
        let blue = self.extract_channel(4);

        image::Rgb {
            data: [red, green, blue],
        }
    }

    fn extract_channel(&self, start_index: usize) -> u8 {
        let channel = &self.0[start_index..start_index + 2];
        u8::from_str_radix(channel, 16).unwrap()
    }
}

fn iteration_to_pixel(iteration: u16) -> image::Rgb<u8> {
    let palette_choice = (iteration % PALETTE_SIZE) as f32;
    let hex_color = scale(
        palette_choice,
        Interval {
            lower_bound: 0.0,
            upper_bound: PALETTE_SIZE as f32,
        },
        Interval {
            lower_bound: 0x000000 as f32,
            upper_bound: 0xFFFFFF as f32,
        },
    );
    let hex_color = format!("{:0<6x}", hex_color as i32);
    let hex_color = HexColor(hex_color);
    hex_color.to_rgb()
}

fn main() {
    let img = ImageBuffer::from_fn(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, {
        |x, y| mandelbrot(x as u16, y as u16)
    });
    img.save("output.png").unwrap();
}

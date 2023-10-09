use std::{fs::File, io::Write};
#[derive(Clone, Copy, Debug)]
struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Pixel {
    fn to_char(&self) -> (char, char, char) {
        (
            self.char_conversion(self.r),
            self.char_conversion(self.g),
            self.char_conversion(self.b),
        )
    }

    fn char_conversion(&self, pixel_value: f32) -> char {
        (pixel_value.min(1.0).max(0.0) * 255.0) as u8 as char
    }
}

fn render() {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;

    let mut frame_buffer = vec![Pixel::default(); (WIDTH * HEIGHT) as usize];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            frame_buffer[(i + j * WIDTH) as usize] = Pixel {
                r: j as f32 / HEIGHT as f32,
                g: i as f32 / WIDTH as f32,
                b: 0.0,
            }
        }
    }

    let mut file = File::create("./out.ppm").unwrap_or_else(|err| panic!("{err}"));
    let header_info = format!("P6\n{WIDTH} {HEIGHT}\n255\n");
    file.write_all(header_info.as_bytes())
        .unwrap_or_else(|err| panic!("{err}"));

    for pixel in &frame_buffer {
        let (r, g, b) = pixel.to_char();
        let pixel_string = format!("{} {} {}\n", r, g, b);

        file.write_all(pixel_string.as_bytes())
            .unwrap_or_else(|err| panic!("{err}"));
    }
}

fn main() {
    render();
}

use std::{fs::File, io::Write, ops::Index};

#[derive(Clone, Default)]
struct Vec3<T> {
    r: T,
    g: T,
    b: T,
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Index out of bounds"),
        }
    }
}

fn render() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;

    let mut framebuffer = vec![
        Vec3 {
            r: 0.0,
            g: 0.0,
            b: 0.0
        };
        (WIDTH * HEIGHT) as usize
    ];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            framebuffer[i + j * WIDTH] = Vec3 {
                r: j as f32 / HEIGHT as f32,
                g: i as f32 / WIDTH as f32,
                b: 0.0,
            }
        }
    }

    let mut file = File::create("./out.ppm").unwrap_or_else(|err| panic!("{err}"));
    file.write_all(format!("P6\n{WIDTH} {HEIGHT}\n255\n").as_bytes())
        .unwrap_or_else(|err| panic!("{err}"));

    for i in 0..HEIGHT * WIDTH {
        for j in 0..3 {
            let pixel_value = (255.0 * framebuffer[i][j].max(0.0).min(1.0)) as u8;
            file.write_all(&[pixel_value])
                .unwrap_or_else(|err| panic!("{err}"));
        }
    }
}

fn main() {
    render();
}

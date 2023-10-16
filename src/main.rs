mod sphere;
mod vec;

use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

use sphere::Sphere;
use vec::Vec3;

const FOV: f32 = PI / 2.0;
const ORIGIN: Vec3<f32> = Vec3 {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

fn cast_ray(orig: &Vec3<f32>, dir: &Vec3<f32>, sphere: &Sphere) -> Vec3<f32> {
    if sphere.ray_intersect(orig, dir) {
        Vec3 {
            r: 0.4,
            g: 0.4,
            b: 0.3,
        }
    } else {
        Vec3 {
            r: 0.2,
            g: 0.7,
            b: 0.8,
        }
    }
}

fn render(sphere: &Sphere) {
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
            let x =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.0).tan() * WIDTH as f32
                    / HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();

            let mut dir = Vec3 {
                r: x,
                g: y,
                b: -1.0,
            };
            dir.normalize();
            framebuffer[(i + j * WIDTH) as usize] = cast_ray(&ORIGIN, &dir, sphere);
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
    render(&Sphere {
        center: Vec3 {
            r: -3.0,
            g: 0.0,
            b: -16.0,
        },
        radius: 2.0,
    });
}

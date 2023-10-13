mod sphere;
mod vec;

use std::fs::File;
use std::io::Write;

use sphere::Sphere;
use vec::Vec3;

fn cast_ray(orig: &Vec3<f32>, dir: &Vec3<f32>, sphere: &Sphere) -> Vec3<f32> {
    let sphere_dist = f32::MAX;
    if !sphere.ray_intersect(orig, dir, sphere_dist) {
        return Vec3 {
            r: 0.2,
            g: 0.7,
            b: 0.8,
        };
    }
    Vec3 {
        r: 0.4,
        g: 0.4,
        b: 0.3,
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

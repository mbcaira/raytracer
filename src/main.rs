mod light;
mod material;
mod sphere;
mod vec;

use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

use light::Light;
use material::Material;
use sphere::Sphere;
use vec::Vec3f;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = PI / 2.0;
const ORIGIN: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

fn reflect(i: Vec3f, n: Vec3f) -> Vec3f {
    return i - n.scalar_multiply(2.0) * (i * n);
}

fn scene_intersect(
    orig: &Vec3f,
    dir: &mut Vec3f,
    spheres: &Vec<Sphere>,
) -> (Vec3f, Vec3f, bool, Material) {
    let mut spheres_dist = f32::MAX;
    let mut material = Material {
        diffuse_colour: Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        albedo: (1.0, 0.0),
        specular_exponent: 0.0,
    };
    let mut hit = Vec3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut n = Vec3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for sphere in spheres {
        let (intersect, dist_i) = sphere.ray_intersect(orig, dir);
        if intersect && dist_i < spheres_dist {
            spheres_dist = dist_i;
            hit = *orig + dir.scalar_multiply(dist_i);
            n = (hit - sphere.center).normalize();
            material = sphere.material;
        }
    }

    return (hit, n, spheres_dist < 1000.0, material);
}

fn cast_ray(orig: &Vec3f, dir: &mut Vec3f, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3f {
    let (point, n, intersect, material) = scene_intersect(orig, dir, spheres);

    if !intersect {
        return Vec3f {
            x: 0.2,
            y: 0.7,
            z: 0.8,
        };
    }

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in lights {
        let light_dir = (light.position - point).normalize();

        diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir.dot(&n));
        specular_light_intensity += f32::powf(
            f32::max(0.0, reflect(light_dir, n).dot(dir)),
            material.specular_exponent,
        );
    }

    material
        .diffuse_colour
        .scalar_multiply(diffuse_light_intensity)
        .scalar_multiply(material.albedo.0)
        + Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        .scalar_multiply(specular_light_intensity)
        .scalar_multiply(material.albedo.1)
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) {
    let mut framebuffer = vec![
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        (WIDTH * HEIGHT) as usize
    ];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x =
                (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0) * (FOV / 2.0).tan() * WIDTH as f32
                    / HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();

            let mut dir = Vec3f {
                x: x,
                y: y,
                z: -1.0,
            };
            dir = dir.normalize();
            framebuffer[(i + j * WIDTH) as usize] = cast_ray(&ORIGIN, &mut dir, spheres, lights);
        }
    }

    write_image(&mut framebuffer)
}

fn write_image(framebuffer: &mut Vec<Vec3f>) {
    let mut file = File::create("./out.ppm").unwrap_or_else(|err| panic!("{err}"));
    file.write_all(format!("P6\n{WIDTH} {HEIGHT}\n255\n").as_bytes())
        .unwrap_or_else(|err| panic!("{err}"));

    for i in 0..HEIGHT * WIDTH {
        for j in 0..3 {
            let c = &mut framebuffer[i];
            let max = c[0].max(c[1]).max(c[2]);
            if max > 1.0 {
                c.scalar_multiply(1.0 / max);
            }
            let pixel_value = (255.0 * framebuffer[i][j].max(0.0).min(1.0)) as u8;
            file.write_all(&[pixel_value])
                .unwrap_or_else(|err| panic!("{err}"));
        }
    }
}

fn main() {
    let ivory = Material {
        diffuse_colour: Vec3f {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        albedo: (0.6, 0.3),
        specular_exponent: 50.0,
    };
    let red = Material {
        diffuse_colour: Vec3f {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        albedo: (0.9, 0.1),
        specular_exponent: 10.0,
    };
    let spheres = vec![
        Sphere {
            center: Vec3f {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: ivory,
        },
        Sphere {
            center: Vec3f {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: red,
        },
        Sphere {
            center: Vec3f {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 3.0,
            material: red,
        },
        Sphere {
            center: Vec3f {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 4.0,
            material: ivory,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3f {
                x: -20.0,
                y: 20.0,
                z: 20.0,
            },
            intensity: 1.5,
        },
        Light {
            position: Vec3f {
                x: 30.0,
                y: 50.0,
                z: -25.0,
            },
            intensity: 1.8,
        },
        Light {
            position: Vec3f {
                x: 30.0,
                y: 20.0,
                z: 30.0,
            },
            intensity: 1.7,
        },
    ];
    render(&spheres, &lights);
}

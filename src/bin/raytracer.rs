extern crate clap;
extern crate indicatif;
extern crate rayon;

#[macro_use]
extern crate raytracer;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use raytracer::camera::Camera;
use raytracer::hittable::Hittable;
use raytracer::material::{dielectric, lambertian, metal};
use raytracer::ray::Ray;
use raytracer::shape::{hittable_list, sphere};
use raytracer::utils::{rand, randvec};
use raytracer::vec3::{ColorVec, GeomVec, Vec3};
use std::f64;
use std::fs::File;
use std::io::Write;
use std::iter::Iterator;

fn random_scene(ball_density: i64) -> Box<Hittable> {
    let mut list = vec![sphere(
        vec3![0, -1000, 0],
        1000.0,
        lambertian(vec3![0.5, 0.5, 0.5]),
    )];

    for a in -ball_density..ball_density {
        for b in -ball_density..ball_density {
            let choose_mat = rand();
            let center = vec3![a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand()];
            if (center - vec3![4, 0.2, 0]).norm() > 0.9 {
                list.push(if choose_mat < 0.8 {
                    sphere(center, 0.2, lambertian(randvec() * randvec()))
                } else if choose_mat < 0.95 {
                    sphere(center, 0.2, metal((randvec() + 1.0) * 0.5, 0.5 * rand()))
                } else {
                    sphere(center, 0.2, dielectric(1.5))
                });
            }
        }
    }

    list.extend(vec![
        sphere(vec3![-4, 1, 0], 1.0, lambertian(vec3![0.4, 0.2, 0.1])),
        sphere(vec3![0, 1, 0], 1.0, dielectric(1.5)),
        sphere(vec3![4, 1, 0], 1.0, metal(vec3![0.7, 0.6, 0.5], 0.0)),
    ]);

    hittable_list(list)
}

fn color(ray: &Ray, world: &Box<Hittable>, depth: u64) -> Vec3 {
    if let Some(rec) = world.hit(&ray, 0.001, f64::MAX) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(&ray, &rec) {
            if depth < 50 {
                attenuation * color(&scattered, world, depth + 1)
            } else {
                Vec3::zeros()
            }
        } else {
            Vec3::zeros()
        }
    } else {
        Vec3::ones().lerp(
            vec3![0.5, 0.7, 1.0],
            0.5 * (ray.direction().unitize().y() + 1.0),
        )
    }
}

fn main() {
    let app = App::new("raytracer")
        .about("Ray tracing in rust")
        .version("1.0.0")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Image width")
                .default_value("200")
                .required(false),
        )
        .arg(
            Arg::with_name("height")
                .short("H")
                .long("height")
                .value_name("HEIGHT")
                .help("Image height")
                .default_value("100")
                .required(false),
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("SAMPLES")
                .help("Number of antialiasing samples")
                .default_value("100")
                .required(false),
        )
        .arg(
            Arg::with_name("gamma")
                .short("g")
                .long("gamma")
                .value_name("GAMMA")
                .help("Gamma correction to apply")
                .default_value("2.0")
                .required(false),
        )
        .arg(
            Arg::with_name("ball_density")
                .short("d")
                .long("ball-density")
                .value_name("BALL_DENSITY")
                .help("Density of balls")
                .default_value("5")
                .required(false),
        )
        .arg(
            Arg::with_name("lookfrom")
                .short("f")
                .long("look-from")
                .value_name("LOOK_FROM")
                .help("Vantage point")
                .use_delimiter(true)
                .default_value("14,3,2")
                .required(false),
        )
        .arg(
            Arg::with_name("lookat")
                .short("t")
                .long("look-at")
                .value_name("LOOK_AT")
                .help("Where to look")
                .use_delimiter(true)
                .default_value("0,0,-1")
                .required(false),
        )
        .arg(
            Arg::with_name("aperture")
                .short("a")
                .long("aperture")
                .value_name("APERTURE")
                .help("Aperture")
                .default_value("0.5")
                .required(false),
        )
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .help("Output filename")
                .value_name("FILENAME")
                .required(true),
        );
    let matches = app.get_matches();
    let width: usize = matches
        .value_of("width")
        .unwrap_or_default()
        .parse()
        .expect("Unable to parse width value");
    let height: usize = matches
        .value_of("height")
        .unwrap_or_default()
        .parse()
        .expect("Unable to parse height value");
    let nsamples: usize = matches
        .value_of("samples")
        .unwrap_or_default()
        .parse()
        .expect("Unable to parse samples value");
    let gamma: f64 = matches
        .value_of("gamma")
        .unwrap_or_default()
        .parse()
        .expect("Unable to parse gamma value");
    let ball_density: i64 = matches
        .value_of("ball_density")
        .unwrap_or_default()
        .parse()
        .expect("Unable to parse ball density");
    let lookfrom: Vec3 = matches
        .values_of("lookfrom")
        .unwrap_or_default()
        .map(|f| f.parse().unwrap())
        .collect();
    let lookat: Vec3 = matches
        .values_of("lookat")
        .unwrap_or_default()
        .map(|a| a.parse().unwrap())
        .collect();
    let aperture: f64 = matches
        .value_of("aperture")
        .unwrap_or_default()
        .parse()
        .expect("Unable parse aperture argument");
    let filename: &str = matches
        .value_of("filename")
        .expect("Unable to retrive value of filename argument");

    let dist_to_focus = (lookfrom - lookat).norm();
    let fwidth = width as f64;
    let fheight = height as f64;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3![0, 1, 0],
        20.0,
        width as f64 / height as f64,
        aperture,
        dist_to_focus,
    );
    let world = random_scene(ball_density);

    let pb = ProgressBar::new((height * width * nsamples) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    let mut file = File::create(filename).expect("Unable to create file");

    writeln!(file, "P3").expect("Unable to write PPM header");
    writeln!(file, "{} {}", width, height).expect("Unable to write width and height to PPM");
    writeln!(file, "255").expect("Unable to write max pixel color value to PPM");

    let ypoints: Vec<_> = (0..height).collect();
    let mut rows: Vec<_> = ypoints
        .par_iter()
        .flat_map(|y| -> Vec<_> {
            let yi = y;
            let y = height - y;
            let fy = y as f64;
            (0..width)
                .map(|x| {
                    let fx = x as f64;
                    let mut col = Vec3::zeros();
                    for _ in 0..nsamples {
                        let u = (fx + rand()) / fwidth;
                        let v = (fy + rand()) / fheight;
                        let r = camera.ray(u, v);
                        col += color(&r, &world, 0);
                    }
                    col /= nsamples as f64;
                    col = col.powf(1.0 / gamma);
                    pb.inc(nsamples as u64);
                    (yi * width + x, (col.r(), col.g(), col.b()))
                })
                .collect()
        })
        .collect();
    rows.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    for (_, (r, g, b)) in rows {
        writeln!(file, "{} {} {}", r, g, b).expect("Unable to write pixel");
    }
    pb.finish();
}

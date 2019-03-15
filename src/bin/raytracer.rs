#![feature(box_syntax)]
extern crate clap;
extern crate indicatif;
extern crate rayon;

#[macro_use]
extern crate raytracer;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use raytracer::camera::Camera;
use raytracer::mat;
use raytracer::ray::Ray;
use raytracer::shape::{sphere, Hittable, HittableList};
use raytracer::utils::{rand, randvec};
use raytracer::vec3::{ColorVec, GeomVec, Vec3};
use std::f64;
use std::fs::File;
use std::io::Write;
use std::iter::Iterator;
use std::ops::Div;

fn random_scene(ball_density: isize) -> Box<Hittable> {
    let mut list = vec![
        sphere(
            vec3![0, -1000, 0],
            1000.0,
            mat::lambertian(vec3![0.5, 0.5, 0.5]),
        ),
        sphere(vec3![-4, 1, 0], 1.0, mat::lambertian(vec3![0.4, 0.2, 0.1])),
        sphere(vec3![0, 1, 0], 1.0, mat::dielectric(1.5)),
        sphere(vec3![4, 1, 0], 1.0, mat::metal(vec3![0.7, 0.6, 0.5], 0.0)),
    ];
    list.reserve(ball_density.pow(2) as usize);

    for a in -ball_density..ball_density {
        for b in -ball_density..ball_density {
            let choose_mat = rand();
            let center = vec3![a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand()];
            if (center - vec3![4, 0.2, 0]).norm() > 0.9 {
                list.push(if choose_mat < 0.8 {
                    sphere(center, 0.2, mat::lambertian(randvec() * randvec()))
                } else if choose_mat < 0.95 {
                    sphere(
                        center,
                        0.2,
                        mat::metal((randvec() + 1.0) * 0.5, 0.5 * rand()),
                    )
                } else {
                    sphere(center, 0.2, mat::dielectric(1.5))
                });
            }
        }
    }

    box HittableList::new(list)
}

fn color(ray: &Ray, world: &Hittable, depth: usize) -> Vec3 {
    if let Some(rec) = world.hit(&ray, 0.001, f64::MAX) {
        if let Some((attenuation, scattered)) = rec.material().scatter(&ray, &rec) {
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
            Arg::with_name("imagedims")
                .short("d")
                .long("image-dims")
                .value_name("IMAGE_DIMS")
                .help("Image width x height")
                .default_value("400x200")
                .use_delimiter(true)
                .value_delimiter("x")
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
                .short("D")
                .long("ball-density")
                .value_name("BALL_DENSITY")
                .help("Density of balls")
                .default_value("11")
                .required(false),
        )
        .arg(
            Arg::with_name("lookfrom")
                .short("F")
                .long("look-from")
                .value_name("LOOK_FROM")
                .help("Vantage point")
                .use_delimiter(true)
                .default_value("13,2,3")
                .required(false),
        )
        .arg(
            Arg::with_name("lookat")
                .short("t")
                .long("look-at")
                .value_name("LOOK_AT")
                .help("Where to look")
                .use_delimiter(true)
                .default_value("0,0,0")
                .required(false),
        )
        .arg(
            Arg::with_name("aperture")
                .short("a")
                .long("aperture")
                .value_name("APERTURE")
                .help("Aperture")
                .default_value("0.1")
                .required(false),
        )
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .help("Output filename")
                .value_name("FILENAME")
                .required(true),
        )
        .arg(
            Arg::with_name("dist_to_focus")
                .short("x")
                .long("dist-to-focus")
                .help("Distance to focus")
                .value_name("DIST_TO_FOCUS")
                .default_value("10.0")
                .required(false),
        );
    let matches = app.get_matches();
    let dims: Vec<usize> = matches
        .values_of("imagedims")
        .unwrap_or_default()
        .map(|f| f.parse().unwrap())
        .collect();
    let ndims = dims.len();
    if ndims != 2 {
        panic!(
            "{} image dimensions given, must give exactly 2 as MxN",
            ndims
        );
    }

    let (width, height) = (dims[0], dims[1]);
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
    let ball_density: isize = matches
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
    let dist_to_focus: f64 = matches
        .value_of("dist_to_focus")
        .unwrap_or(&(lookfrom - lookat).norm().to_string())
        .parse()
        .expect("Unable to parse value of distance to focus argument");
    let fwidth = width as f64;
    let fheight = height as f64;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3![0, 1, 0],
        20.0,
        fwidth / fheight,
        aperture,
        dist_to_focus,
    );
    let world = &*random_scene(ball_density);
    let pb = ProgressBar::new((height * width * nsamples) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}%")
            .progress_chars("##-"),
    );

    let mut file = File::create(filename).expect("Unable to create file");

    writeln!(file, "P3").expect("Unable to write PPM header");
    writeln!(file, "{} {}", width, height).expect("Unable to write width and height to PPM");
    writeln!(file, "255").expect("Unable to write max pixel color value to PPM");

    let gamma = 1.0 / gamma;

    let mut rows = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let fy = (height - y) as f64;
            let mut res: Vec<(usize, (u8, u8, u8))> = Vec::with_capacity(width);
            for x in 0..width {
                let col = (0..nsamples)
                    .map(|_| {
                        let u = (x as f64 + rand()) / fwidth;
                        let v = (fy + rand()) / fheight;
                        let ray = camera.ray(u, v);
                        color(&ray, world, 0)
                    })
                    .sum::<Vec3>()
                    .div(nsamples as f64)
                    .powf(gamma);
                pb.inc(nsamples as u64);
                let value = (y * width + x, (col.r(), col.g(), col.b()));
                res.push(value);
            }
            res
        })
        .collect::<Vec<_>>();
    rows.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    for (_, (r, g, b)) in rows {
        writeln!(file, "{} {} {}", r, g, b).expect("Unable to write pixel");
    }
    pb.finish();
}

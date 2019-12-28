#![feature(box_syntax)]

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use itertools::Itertools;

use raytracer::shape::{sphere, Hittable};
use raytracer::vec3::{ColorVec, GeomVec, Vec3};
use raytracer::{camera, mat, ray, utils, v3};

use std::f64;
use std::fs::File;
use std::io::Write;
use std::iter::Iterator;
use std::ops::Div;

fn generate_center(a: f64, b: f64) -> Option<Vec3> {
    let center = v3![a + 0.9 * utils::rand(), 0.2, b + 0.9 * utils::rand()];
    if (center - v3![4.0, 0.2, 0.0]).norm() > 0.9 {
        Some(center)
    } else {
        None
    }
}

fn random_scene(ball_density: isize) -> Box<dyn Hittable> {
    assert!(ball_density >= 0);
    box vec![
        sphere(
            v3![0.0, -1000.0, 0.0],
            1000.0,
            mat::lambertian(v3![0.5, 0.5, 0.5]),
        ),
        sphere(
            v3![-4.0, 1.0, 0.0],
            1.0,
            mat::lambertian(v3![0.4, 0.2, 0.1]),
        ),
        sphere(v3![0.0, 1.0, 0.0], 1.0, mat::dielectric(1.5)),
        sphere(v3![4.0, 1.0, 0.0], 1.0, mat::metal(v3![0.7, 0.6, 0.5], 0.0)),
    ]
    .into_iter()
    .chain(
        (-ball_density..ball_density)
            .cartesian_product(-ball_density..ball_density)
            .filter_map(|(a, b)| generate_center(a as f64, b as f64))
            .map(|center| {
                sphere(
                    center,
                    0.2,
                    match utils::rand() {
                        choose_mat if choose_mat < 0.8 => {
                            mat::lambertian(Vec3::rand() * Vec3::rand())
                        },
                        choose_mat if choose_mat < 0.95 => {
                            mat::metal(
                                (Vec3::rand() + 1.0) * 0.5,
                                0.5 * utils::rand(),
                            )
                        },
                        _ => mat::dielectric(1.5),
                    },
                )
            }),
    )
    .collect::<Vec<_>>()
}

fn color(ray: &ray::Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
    world.hit(&ray, 0.001, f64::MAX).map_or_else(
        || {
            Vec3::ones().lerp(
                v3![0.5, 0.7, 1.0],
                0.5 * (ray.direction().unitize().y() + 1.0),
            )
        },
        |rec| {
            rec.material().scatter(&ray, &rec).map_or(
                Vec3::zeros(),
                |(attenuation, scattered)| {
                    if depth < 50 {
                        attenuation * color(&scattered, world, depth + 1)
                    } else {
                        Vec3::zeros()
                    }
                },
            )
        },
    )
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
    let dims = matches
        .values_of("imagedims")
        .unwrap_or_default()
        .map(|s| {
            s.parse()
                .expect(format!("Invalid usize value {}", s).as_str())
        })
        .collect::<Vec<usize>>();
    let ndims = dims.len();
    if ndims != 2 {
        println!("The argument '{}' isn't a valid value", ndims);
        return;
    }

    let (width, height) = (dims[0], dims[1]);
    let nsamples = matches
        .value_of("samples")
        .unwrap_or_default()
        .parse::<usize>()
        .expect("Invalid usize value");
    let gamma = matches
        .value_of("gamma")
        .unwrap_or_default()
        .parse::<f64>()
        .expect("Invalid f64 value");
    let ball_density = matches
        .value_of("ball_density")
        .unwrap_or_default()
        .parse::<isize>()
        .expect("Invalid isize value");
    let lookfrom: Vec3 = matches
        .values_of("lookfrom")
        .unwrap_or_default()
        .map(|s| {
            s.parse()
                .expect(format!("Invalid f64 value {}", s).as_str())
        })
        .collect();
    let lookat: Vec3 = matches
        .values_of("lookat")
        .unwrap_or_default()
        .map(|s| {
            s.parse()
                .expect(format!("Invalid f64 value {}", s).as_str())
        })
        .collect();
    let aperture = matches
        .value_of("aperture")
        .unwrap_or_default()
        .parse::<f64>()
        .expect("Invalid f64 value");
    let filename = matches
        .value_of("filename")
        .expect("Missing filename argument");
    let dist_to_focus = matches
        .value_of("dist_to_focus")
        .unwrap_or(&(lookfrom - lookat).norm().to_string())
        .parse::<f64>()
        .expect("");
    let fwidth = width as f64;
    let fheight = height as f64;

    let camera = camera::Camera::new(
        lookfrom,
        lookat,
        v3![0.0, 1.0, 0.0],
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

    let mut file = File::create(filename).expect("Unable to open file");

    writeln!(file, "P3").expect("Unable to write header line P3");
    writeln!(file, "{} {}", width, height).expect(
        format!("Unable to write header line {} {}", width, height).as_str(),
    );
    writeln!(file, "255").expect("Unable to write header line 255");

    let gamma = 1.0 / gamma;

    let mut rows = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let fy = (height - y) as f64;
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let fx = x as f64;
                let col = (0..nsamples)
                    .map(|_| {
                        let u = (fx + utils::rand()) / fwidth;
                        let v = (fy + utils::rand()) / fheight;
                        let ray = camera.ray(u, v);
                        color(&ray, world, 0)
                    })
                    .sum::<Vec3>()
                    .div(nsamples as f64)
                    .powf(gamma);
                pb.inc(nsamples as u64);
                row.push((y * width + x, col.r(), col.g(), col.b()));
            }
            row
        })
        .collect::<Vec<_>>();
    rows.sort_by(|(left, ..), (right, ..)| left.cmp(right));
    for (i, r, g, b) in rows {
        let line = format!("{} {} {}", r, g, b);
        writeln!(file, "{}", line).expect(
            format!("Unable to write pixel at row {} with value {}", i, line)
                .as_str(),
        );
    }
    pb.finish();
}

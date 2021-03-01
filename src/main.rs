use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use raytracer::{
    utils::{rand, randvec},
    vec3, Camera, ColorVec3, Dielectric, Hittable, HittableList, Lambertian, Metal, Ray, Sphere,
    Vec3,
};
use std::{fs::File, io::Write, ops::Div};
use structopt::StructOpt;

fn random_scene(ball_density: i64) -> impl Hittable {
    let mut list = vec![
        Sphere::new(
            vec3![0, -1000, 0],
            1000.0,
            Lambertian::new(vec3![0.5, 0.5, 0.5]),
        ),
        Sphere::new(vec3![-4, 1, 0], 1.0, Lambertian::new(vec3![0.4, 0.2, 0.1])),
        Sphere::new(vec3![0, 1, 0], 1.0, Dielectric::new(1.5)),
        Sphere::new(vec3![4, 1, 0], 1.0, Metal::new(vec3![0.7, 0.6, 0.5], 0.0)),
    ];
    list.reserve(ball_density.pow(2) as usize);

    for a in -ball_density..ball_density {
        for b in -ball_density..ball_density {
            let choose_mat = rand();
            let center = vec3![a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand()];
            if (center - vec3![4, 0.2, 0]).norm() > 0.9 {
                list.push(if choose_mat < 0.8 {
                    Sphere::new(center, 0.2, Lambertian::new(randvec() * randvec()))
                } else if choose_mat < 0.95 {
                    Sphere::new(
                        center,
                        0.2,
                        Metal::new((randvec() + 1.0) * 0.5, 0.5 * rand()),
                    )
                } else {
                    Sphere::new(center, 0.2, Dielectric::new(1.5))
                });
            }
        }
    }

    HittableList::new(list)
}

fn color(ray: Ray, world: &impl Hittable, depth: usize) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        if let Some((attenuation, scattered)) = rec.material.scatter(&ray, &rec) {
            if depth < 50 {
                attenuation * color(scattered, world, depth + 1)
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

#[derive(structopt::StructOpt)]
struct Opt {
    #[structopt(
        short,
        long,
        required = true,
        default_value = "400x200",
        value_delimiter = "x",
        number_of_values = 1,
        help = "Size of the image to produce"
    )]
    image_dims: Vec<u16>,

    #[structopt(short, long, default_value = "100", help = "Number of samples")]
    nsamples: u32,

    #[structopt(short, long, default_value = "2.0", help = "Gamma")]
    gamma: f64,

    #[structopt(short, long, default_value = "11", help = "Density of balls")]
    ball_density: u32,

    #[structopt(
        short,
        long,
        default_value = "13,2,3",
        value_delimiter = ",",
        help = "Origin of camera viewpoint"
    )]
    look_from: Vec<f64>,

    #[structopt(
        short,
        long,
        default_value = "0,0,0",
        value_delimiter = ",",
        help = "Where the camera is looking"
    )]
    look_at: Vec<f64>,

    #[structopt(short, long, default_value = "0.1", help = "Aperture")]
    aperture: f64,

    #[structopt(required = true, help = "Output filename")]
    filename: std::path::PathBuf,

    #[structopt(short, long, default_value = "10.0", help = "Distance to focus")]
    dist_to_focus: f64,
}

fn main() -> Result<()> {
    let Opt {
        image_dims,
        nsamples,
        gamma,
        ball_density,
        look_from,
        look_at,
        aperture,
        filename,
        dist_to_focus,
    } = Opt::from_args();
    let (width, height) = (image_dims[0], image_dims[1]);

    let camera = Camera::new(
        look_from.into(),
        look_at.into(),
        vec3![0, 1, 0],
        20.0,
        f64::from(width) / f64::from(height),
        aperture,
        dist_to_focus,
    );
    let world = random_scene(i64::from(ball_density));
    let pb = ProgressBar::new(u64::from(u32::from(height) * u32::from(width) * nsamples));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}%")
            .progress_chars("##-"),
    );

    let mut file = File::create(filename).context("Unable to create file")?;

    writeln!(file, "P3").context("Unable to write PPM header")?;
    writeln!(file, "{} {}", width, height).context("Unable to write width and height to PPM")?;
    writeln!(file, "255").context("Unable to write max pixel color value to PPM")?;

    let gamma = gamma.recip();

    let mut rows = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let fy = f64::from(height - y);
            let mut res = Vec::with_capacity(width.into());
            for x in 0..width {
                let col = (0..nsamples)
                    .map(|_| {
                        let u = (f64::from(x) + rand()) / f64::from(width);
                        let v = (fy + rand()) / f64::from(height);
                        color(camera.ray(u, v), &world, 0)
                    })
                    .sum::<Vec3>()
                    .div(f64::from(nsamples))
                    .powf(gamma);
                pb.inc(u64::from(nsamples));
                res.push((y * width + x, ColorVec3::from(col).into_array()));
            }
            res
        })
        .collect::<Vec<_>>();
    rows.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    for (row_index, [r, g, b]) in rows {
        writeln!(file, "{} {} {}", r, g, b)
            .with_context(|| format!("Unable to write pixel at row: {}", row_index))?;
    }
    pb.finish();
    Ok(())
}

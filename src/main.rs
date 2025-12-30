use rayon::iter::{IntoParallelIterator, ParallelIterator};
use raytracer::{
    camera::Camera,
    color::Color,
    ppm::PPMImage,
    ray::Ray,
    scene::{scene::Scene, sphere::Sphere},
    vec3::{Point3, Vec3},
};

fn ray_color(ray: &Ray) -> Color {
    let direction = ray.direction.normalize();
    let a = (direction.y + 1.) * 0.5;
    let v = Vec3::ONE * (1. - a) + (Vec3::new(0.5, 0.7, 1.0) * a);
    Color::from(v)
}

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16. / 9.;
    let img_width = 1366;
    let mut img_height = (img_width as f32 / aspect_ratio) as usize;
    if img_height < 1 {
        img_height = 1;
    }

    let viewport_height = 2;
    let viewport_width = viewport_height * (img_width as usize / img_height);

    let camera = Camera::default();
    let focal_len = 1.;

    let viewport_u = Vec3::new(viewport_width as f32, 0., 0.);
    let viewport_v = Vec3::new(0., -(viewport_height as f32), 0.);

    let pixel_delta_u = viewport_u / img_width as f32;
    let pixel_delta_v = viewport_v / img_height as f32;

    let view_port_upper_left =
        camera.origin - Vec3::new(0., 0., focal_len) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = view_port_upper_left + 0.5 * (pixel_delta_v + pixel_delta_u);

    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point3::new(0., 0., 1.), 0.5)));

    let pixels = (0..img_height)
        .into_par_iter()
        .map(|y| {
            (0..img_width)
                .into_par_iter()
                .map(|x| {
                    let pixel_center =
                        pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
                    let ray_direction = pixel_center - camera.origin;
                    let ray = Ray::new(pixel_center, ray_direction);
                    scene.get_color(&ray).unwrap_or_else(|| ray_color(&ray))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let img = PPMImage::from(pixels);
    println!("Img generated");
    img.save("output.ppm")?;
    Ok(())
}

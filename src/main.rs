use std::{sync::Arc, time::Instant};

use raytracer::{
    camera::Camera,
    materials::Lambertian,
    renderables::{Renderable, Scene, Sphere, Triangle},
    vec3::{Point3, Vec3},
};

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(Vec3::ZERO, 16. / 9., 1920)
        .with_focal_length(0.9)
        .with_anti_aliasing_samples(10)
        .with_max_depth(50);

    let mut scene = Scene::default();
    let material = Arc::new(Lambertian::new(Vec3::new(0.3, 0.25, 0.40)));
    let objs: Vec<Box<dyn Renderable + Sync>> = vec![
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5, material.clone())),
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            material.clone(),
        )),
        Box::new(Triangle::new(
            Point3::new(-0.8, 0.3, -0.5),
            Point3::new(-0.6, 0.6, -1.4),
            Point3::new(-0.4, 0.5, -0.5),
            material.clone(),
        )),
    ];
    scene.add_obects(objs);
    let start = Instant::now();
    let img = camera.get_img(&scene);
    println!(
        "Spent {}ms on a frame",
        Instant::now().duration_since(start).as_millis()
    );
    img.save("output.ppm")?;
    Ok(())
}

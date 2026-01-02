use std::{sync::Arc, time::Instant};

use raytracer::{
    camera::Camera,
    materials::{Dielectric, Lambertian, Metal},
    renderables::{Plane, Renderable, Scene, Sphere, Triangle},
    vec3::{Point3, Vec3},
};

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(Vec3::new(0., 0., 0.4), 16. / 9., 1200)
        .with_focal_length(1.)
        .with_anti_aliasing_samples(20)
        .with_fov(90)
        .with_max_depth(7);
    let mut scene = Scene::default();

    let purple_diffuse = Arc::new(Lambertian::new(Vec3::new(0.3, 0.25, 0.40)));
    let default_metal = Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)).with_fuzz(0.04));
    let gold = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)).with_fuzz(0.3));
    let glass = Arc::new(Dielectric::new(1.5));

    let mut tr = Triangle::new(
        Point3::new(-0.25, 0.0, 0.0),
        Point3::new(0.0, 0.5, 0.0),
        Point3::new(0.25, 0.0, 0.0),
        gold.clone(),
    );

    tr.rotate(glam::Quat::from_rotation_x(50.0f32.to_radians()));
    tr.move_to(Point3::new(-0.2, -0.35, -0.4));

    let objs: Vec<Box<dyn Renderable + Sync>> = vec![
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.), 0.5, glass)),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.), 0.5, default_metal)),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.), 0.5, gold.clone())),
        Box::new(Plane::new(
            Point3::new(0.0, -0.5, 0.0),
            Vec3::ZERO.with_y(1.),
            purple_diffuse,
        )),
        Box::new(tr),
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

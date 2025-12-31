use std::time::Instant;

use raytracer::{
    camera::Camera,
    renderables::{Plane, Renderable, Scene, Sphere, Triangle},
    vec3::{Point3, Vec3},
};

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(Vec3::ZERO, 16. / 10., 1920)
        .with_focal_length(1.)
        .with_anti_aliasing_samples(10);

    let mut scene = Scene::default();
    let objs: Vec<Box<dyn Renderable + Sync>> = vec![
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
        Box::new(Plane::new(
            Point3::new(-1., -1., -1.),
            Vec3::new(0., 1.0, 0.3),
        )),
        Box::new(Triangle::new(
            Point3::new(-0.8, 0.3, -0.5),
            Point3::new(-0.6, 0.6, -1.4),
            Point3::new(-0.4, 0.5, -0.5),
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

use std::time::Instant;

use raytracer::{
    camera::Camera,
    renderables::{Scene, Sphere},
    vec3::{Point3, Vec3},
};

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(Vec3::ZERO, 16. / 10., 1920);
    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    scene.add_object(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    let start = Instant::now();
    let img = camera.get_img(&scene);
    println!(
        "Spent {}ms on a frame",
        Instant::now().duration_since(start).as_millis()
    );
    img.save("output.ppm")?;
    Ok(())
}

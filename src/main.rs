use std::time::Instant;

use raytracer::{
    camera::Camera,
    scene::{scene::Scene, sphere::Sphere},
    vec3::{Point3, Vec3},
};

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16. / 9.;
    let img_width = 1920;
    let mut img_height = (img_width as f32 / aspect_ratio) as usize;
    if img_height < 1 {
        img_height = 1;
    }
    let mut camera = Camera::new(Vec3::ZERO, Vec3::ZERO.with_z(1.), img_width, img_height);
    camera.focal_length = 1.0;
    let mut scene = Scene::default();
    scene.add_object(Box::new(Sphere::new(Point3::new(0., 0., 1.), 0.5)));
    let start = Instant::now();
    let img = camera.get_img(&scene);
    println!(
        "Spent {}ms on a frame",
        Instant::now().duration_since(start).as_millis()
    );
    img.save("output.ppm")?;
    Ok(())
}

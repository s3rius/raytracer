use std::{sync::Arc, time::Instant};

use rand::Rng;
use raytracer::{
    camera::Camera,
    materials::{self, CombineMaterial, Dielectric, Lambertian, Metal},
    renderables::{Plane, Renderable, Scene, Sphere},
    vec3::{Point3, Vec3, Vec3Ext},
};

fn main() -> anyhow::Result<()> {
    let camera = Camera::new(Vec3::new(1.4, 0.0, -0.8), 16. / 9., 800)
        .with_anti_aliasing_samples(0)
        .with_fov(60)
        .with_max_depth(6)
        .with_lookat(Point3::new(0., 0.0, 1.));
    // panic!("Heh");
    let mut scene = Scene::default();

    let purple_diffuse = Arc::new(Lambertian::new(Vec3::new(0.3, 0.25, 0.40)));
    let default_metal = Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.7)).with_fuzz(0.04));
    let gold = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)).with_fuzz(0.3));
    let glass = Arc::new(Dielectric::new(1.5));

    let objs: Vec<Box<dyn Renderable + Sync>> = vec![
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, 1.), 0.5, glass.clone())),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, 1.), 0.5, default_metal)),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, 1.), 0.5, gold.clone())),
        Box::new(Plane::new(
            Point3::new(0.0, -0.5, 0.0),
            Vec3::ZERO.with_y(1.),
            purple_diffuse,
        )),
    ];
    scene.add_obects(objs);
    let mut rng = rand::rng();
    for x in -20..20 {
        for z in -20..20 {
            let rand_color = Vec3::rand_with_range(&mut rng, 0.0..=1.0);
            let mut material = CombineMaterial::new()
                .add_material(Arc::new(materials::Lambertian::new(rand_color)));
            if rand::random_bool(0.7) {
                material = material.add_material(glass.clone());
            }
            let position = Vec3::new(
                x as f32 + 0.9 * rng.random_range(0.0..=1.0),
                -0.4,
                z as f32 + 0.9 * rng.random_range(0.0..=1.0),
            );
            if (position - Vec3::new(0., 0., 1.)).length() <= 0.9 {
                continue;
            }
            scene.add_object(Box::new(Sphere::new(position, 0.1, Arc::new(material))));
        }
    }
    let start = Instant::now();

    let img = camera.get_img(&scene);
    println!(
        "Spent {}ms on a frame",
        Instant::now().duration_since(start).as_millis()
    );
    img.save("output.ppm")?;
    Ok(())
}

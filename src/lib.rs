#![warn(
    // Base lints.
    clippy::all,
    // Some pedantic lints.
    clippy::pedantic,
    // New lints which are cool.
    clippy::nursery,
)]
#![
    allow(
        // I don't care about this.
        clippy::module_name_repetitions, 
        // Yo, the hell you should put
        // it in docs, if signature is clear as sky.
        clippy::missing_errors_doc,
        // Well, yes
        clippy::cast_precision_loss,
    )
]
pub mod color;
pub mod ppm;
pub mod vec3;
pub mod ray;
pub mod camera;
pub mod scene;
pub mod utils;
pub mod interval;

use crate::sphere::Sphere;
use ultraviolet::geometry::Ray;
use ultraviolet::Vec3;

mod sphere;

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 1.0), 0.5);
    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, -10.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let result = sphere.intersect(&ray);
    println!("{}", result);

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let pixel_colour: [u8; 3] = [200, 0, 0];
            image.put_pixel(x, y, image::Rgb(pixel_colour));
        }
    }

    image
        .save("output.png")
        .expect("Couldn't save `output.png`");
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

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

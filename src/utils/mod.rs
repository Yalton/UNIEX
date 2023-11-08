// use chrono::{format::strftime::StrftimeItems, DateTime, Local};
// use rand::seq::SliceRandom;


use image::{GenericImageView, ImageBuffer, RgbaImage, load_from_memory};


#[tracing::instrument(skip_all)]    
pub fn stack_images_vertically(img1_data: &[u8], img2_data: &[u8]) -> anyhow::Result<RgbaImage> {
    // Load the images from memory
    let img1 = load_from_memory(img1_data)?;
    let img2 = load_from_memory(img2_data)?;

    // Ensure the images have the same width
    if img1.width() != img2.width() {
        return Err(anyhow::anyhow!("Images must have the same width"));
    }

    // Create a new image with the combined height of the two images
    let img_width = img1.width();
    let img_height = img1.height() + img2.height();
    let mut combined_image: RgbaImage = ImageBuffer::new(img_width, img_height);

    // Copy the pixels from the first image to the combined image
    for x in 0..img1.width() {
        for y in 0..img1.height() {
            let pixel = img1.get_pixel(x, y);
            combined_image.put_pixel(x, y, pixel);
        }
    }

    // Copy the pixels from the second image to the combined image, offset by the height of the first image
    for x in 0..img2.width() {
        for y in 0..img2.height() {
            let pixel = img2.get_pixel(x, y);
            combined_image.put_pixel(x, y + img1.height(), pixel);
        }
    }

    Ok(combined_image)
}
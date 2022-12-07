use image::GenericImageView;

pub fn get_image_transparent(path: &str) -> Option<bool>
{
    let img = match image::open(path) {
        Err(err) => {
            eprintln!("Failed to open image file {}, error: {}", path, err);
            return None;
        },
        Ok(res) => res,
    };
    
    let opaque_pixels = img.pixels()
        .map(|(_,_,pixel)| if pixel.0[3] != 0 { 1 } else { 0 })
        .sum::<u32>();

    let (width, height) = img.dimensions();
    let transparent = opaque_pixels != width * height;
    Some(transparent)
}

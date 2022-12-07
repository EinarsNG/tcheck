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
    
    let mut transparent = false;
    let opaque = img.pixels()
        .map(|(_,_,pixel)| if pixel.0[3] == 255 { true } else { false })
        .last();

    if let Some(res) = opaque
    {
        transparent = res == false;
    }

    Some(transparent)
}

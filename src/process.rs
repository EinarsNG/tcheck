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
    let last = img.pixels()
        .take_while(|(_,_,pixel)| pixel.0[3] != 0)
        .last();

    if let Some(res) = last
    {
        transparent = res.2.0[3] == 0;
    }

    Some(transparent)
}

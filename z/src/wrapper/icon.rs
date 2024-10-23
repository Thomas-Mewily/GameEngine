use crate::*;

pub fn png_file_to_image(bytes: &'static [u8]) -> Image 
{
    Image::from(
        LibImage::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap()
    )
}

fn populate_array(img: Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.img.get_image_data() 
    {
        for value in pixel.iter() 
        {
            array[index] = *value;
            index += 1;
        }
    }
}

/// Thank to @ranmuran on discord on the MacroQuad server : <https://discord.com/channels/710177966440579103/710180051349405746/1093173735734915092>
pub fn generate_icon(px16 : Image, px32 : Image, px64 : Image) -> Icon
{
    let mut array_small: [u8; 16*16*4] = [0; 16*16*4];
    let mut array_medium: [u8; 32*32*4] = [0; 32*32*4];
    let mut array_big: [u8; 64*64*4] = [0; 64*64*4];

    populate_array(px16, &mut array_small);
    populate_array(px32, &mut array_medium);
    populate_array(px64, &mut array_big);

    Icon {
        small: array_small,
        medium: array_medium,
        big: array_big,
    }
}

#[macro_export]
macro_rules! generate_icon {
    ($path:expr) => {
        $crate::generate_icon(
            ::engine::png_file_to_image(include_bytes!(concat!($path, "_16px.png"))),
            ::engine::png_file_to_image(include_bytes!(concat!($path, "_32px.png"))),
            ::engine::png_file_to_image(include_bytes!(concat!($path, "_64px.png"))),
        )
    };
}
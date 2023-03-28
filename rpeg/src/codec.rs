use csc411_image::{RgbImage, Read, Write, Rgb};
use array2::Array2;
use crate::pixel_converter::*;
use crate::block::*;
use crate::dct::*;
use crate::quantization::*;
use crate::codeword::*;

/// Compresses the image and outputs the compressed image to stdout
/// 
/// # Arguments
/// 
/// * `filename`: An optional file path, if `None` read from standard in
pub fn compress(filename: Option<&str>) {
    let image = RgbImage::read(filename).unwrap();
    let image_as_array2 = trim_image(&image);
    // call upon compression functions
    // function operations in order: convert to CV, create 2x2 blocks, convert to cosine space, 
    // quantization, create codewords and output them
    output_all_codewords(
        convert_to_bits(
            to_cosine_space(
                component_video_to_blocks(
                    rgb_image_to_component_video(image_as_array2, image.denominator)
                )
            )
        )
    );
}
/// Decompresses the image and outputs the decompressed image to stdout
/// 
/// # Arguments
/// 
/// * `filename`: An optional file path, if `None` read from standard in
pub fn decompress(filename: Option<&str>) {
    // call upon decompression functions:
    // function operations in order: Read in 32-bit codewords and convert back to quantized values,
    // convert to cosine space, back to pixel space, unpack the 2x2 blocks, 
    // back to rgb pixels with denominator 255
    let image_as_array2 = component_video_image_to_rgb(
        blocks_to_component_video(
            to_pixels(
                convert_to_cs(
                    read_all_codewords(filename)
                )
            )
        ), 
    255);
    let image = RgbImage{
        pixels: image_as_array2
        .iter_row_major()
        .map(|data| data.2.clone())
        .collect(),
        width: image_as_array2.width() as u32,
        height: image_as_array2.height() as u32,
        denominator: 255
    };
    image.write(None).unwrap();

}

// Trims the image to make sure that the height and width are even, and then stores it in an Array2
fn trim_image(image: &RgbImage) -> Array2<Rgb> {
    let mut new_width = image.width;
    let mut new_height = image.height;
    if image.width % 2 != 0 {
        new_width = image.width-1;
    }
    if image.height % 2 != 0 {
        new_height = image.height-1;
    }

    Array2::from_row_major(image.pixels.clone(), new_width as usize, new_height as usize)
}

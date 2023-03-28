use array2::Array2;
use csc411_image::Rgb;

/// An `ComponentVideo` pixel contains three `f32` values, for luminance, blue-difference chroma,
/// and red-difference chroma respectively
#[derive(Clone)]
pub struct ComponentVideo {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

// Converts an Rgb pixel to a floating point representation using the image denominator
fn pixel_to_float(pixel: Rgb, denominator: u16) -> (f32, f32, f32) {
    (pixel.red as f32/denominator as f32, pixel.green as f32/denominator as f32, pixel.blue as f32/denominator as f32)
}

// Converts a floating point representation of a pixel to an Rgb pixel using the image denominator
fn pixel_to_rgb(pixel: (f32, f32, f32), denominator: u16) -> Rgb {
    Rgb {
        red: (pixel.0 * denominator as f32).round() as u16,
        green: (pixel.1 * denominator as f32).round() as u16,
        blue: (pixel.2 * denominator as f32).round() as u16
    }
}

// Converts an RGB pixel to a ComponentVideo pixel
fn rgb_to_component_video(pixel: Rgb, denominator: u16) -> ComponentVideo {
    let float_pixel = pixel_to_float(pixel, denominator);
    ComponentVideo { 
        y: 0.299 * float_pixel.0 + 0.587 * float_pixel.1 + 0.114 * float_pixel.2, 
        pb: -0.168736 * float_pixel.0 - 0.331264 * float_pixel.1 + 0.5 * float_pixel.2, 
        pr: 0.5 * float_pixel.0 - 0.418688 * float_pixel.1 - 0.081312 * float_pixel.2
    }
}

// Converts a ComponentVideo pixel to an Rgb pixel
fn component_video_to_rgb(pixel: ComponentVideo, denominator: u16) -> Rgb {
    let f32r = 1.0 * pixel.y + 0.0 * pixel.pb + 1.402 * pixel.pr;
    let f32g = 1.0 * pixel.y - 0.344136 * pixel.pb - 0.714136 * pixel.pr;
    let f32b = 1.0 * pixel.y + 1.772 * pixel.pb + 0.0 * pixel.pr;
    pixel_to_rgb((f32r, f32g, f32b), denominator)
}

/// Transforms every Rgb pixel in the image into a ComponentVideo pixel
///
/// # Arguments
///
/// * `image`: an array2 that is storing an image with Rgb pixels
/// * `denominator`: the maximum color value of the image
pub fn rgb_image_to_component_video(mut image: Array2<Rgb>, denominator: u16) ->  Array2<ComponentVideo> {
    let component_video_vec:Vec<_>= image.iter_row_major_mut().map(|(_r, _c, pixel)| (
        rgb_to_component_video(pixel.clone(), denominator))).collect();
    Array2::from_row_major(component_video_vec, image.width(), image.height())
}

/// Transforms every Component Video pixel in the image into an Rgb pixel
///
/// # Arguments
///
/// * `image`: an array2 that is storing an image with ComponentVideo pixels
/// * `denominator`: the maximum color value of the image
pub fn component_video_image_to_rgb(mut image: Array2<ComponentVideo>, denominator: u16) -> Array2<Rgb> {
    let rgb_vec:Vec<_> = image.iter_row_major_mut().map(|(_r, _c, pixel)| (
        component_video_to_rgb(pixel.clone(), denominator))).collect();
    Array2::from_row_major(rgb_vec, image.width(), image.height())
}
use crate::block::Block;
use array2::Array2;
// Calculates the average brightness from the image
fn calculate_a(luminance: [f64; 4]) -> f64 {
    (luminance[3] + luminance[2] + luminance[1] + luminance[0]) / 4.0
}
// Calculates the degree to which the image gets brighter from top to bottom
fn calculate_b(luminance: [f64; 4]) -> f64 {
    (luminance[3] + luminance[2] - luminance[1] - luminance[0]) / 4.0
}
// Calculates the degree to which the image gets brighter from left to right
fn calculate_c(luminance: [f64; 4]) -> f64 {
    (luminance[3] - luminance[2] + luminance[1] - luminance[0]) / 4.0
}
// Calculates the degree to which the pixels on one diagonal are brighter than
// pixels on the other diagonal
fn calculate_d(luminance: [f64; 4]) -> f64 {
    (luminance[3] - luminance[2] - luminance[1] + luminance[0]) / 4.0
}
// Calculates brightness for pixel 1
fn calculate_y1(luminance: [f64; 4]) -> f64 {
    luminance[0] - luminance[1] - luminance[2] + luminance[3]
}
// Calculates brightness for pixel 2
fn calculate_y2(luminance: [f64; 4]) -> f64 {
    luminance[0] - luminance[1] + luminance[2] - luminance[3]
}
// Calculates brightness for pixel 3
fn calculate_y3(luminance: [f64; 4]) -> f64 {
    luminance[0] + luminance[1] - luminance[2] - luminance[3]
}
// Calculates brightness for pixel 4
fn calculate_y4(luminance: [f64; 4]) -> f64 {
    luminance[0] + luminance[1] + luminance[2] + luminance[3]
}

/// Transforms the y-values (pixels) to the respective a,b,c,d values (cosine space) in each block
///
/// # Arguments
///
/// * `blocks`: an Array2 that holds all 2x2 blocks of component video pixels
pub fn to_cosine_space(mut vid_blocks: Array2<Block>) -> Array2<Block> {
    for (_r, _c, vid_2x2) in vid_blocks.iter_row_major_mut() {
        // transforming to cosine space
        vid_2x2.luminance = [calculate_a(vid_2x2.luminance), calculate_b(vid_2x2.luminance),
        calculate_c(vid_2x2.luminance), calculate_d(vid_2x2.luminance)];
    }
    vid_blocks
}

/// Transforms the cosine space values back to the respective y values in each block
///
/// # Arguments
///
/// * `blocks`: an Array2 that holds all 2x2 blocks in the cosine space
pub fn to_pixels(mut cs_blocks: Array2<Block>) -> Array2<Block> {
    for (_r, _c, cs_2x2) in cs_blocks.iter_row_major_mut() {
        // transforming to pixels
        cs_2x2.luminance = [calculate_y1(cs_2x2.luminance), calculate_y2(cs_2x2.luminance),
        calculate_y3(cs_2x2.luminance), calculate_y4(cs_2x2.luminance)];
    }
    cs_blocks
}
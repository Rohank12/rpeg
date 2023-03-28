use crate::pixel_converter::ComponentVideo;
use array2::Array2;

/// A `Block` stores data taken by a 2x2 matrix of component video pixels. It has
/// a 4 element array that holds the different `f64` luminance values of the pixels,
/// the average blue-difference chroma, and the average red-difference chroma
#[derive(Clone)]
pub struct Block {
    pub luminance: [f64; 4],
    pub pb_avg: f32,
    pub pr_avg: f32,
}

// Takes in all chroma values, and averages them from a 2x2 block.
fn get_chroma_avg(values: (f32, f32, f32, f32)) -> f32 {
    (values.0 + values.1 + values.2 + values.3)/4.0
}

// Creates a 2x2 block of average chroma values, and the four luminance values,
fn create_block(pixels: [ComponentVideo; 4]) -> Block {
    Block {
        luminance: [pixels[0].y as f64, pixels[1].y as f64, pixels[2].y as f64, pixels[3].y as f64],
        pb_avg: get_chroma_avg((pixels[0].pb, pixels[1].pb, pixels[2].pb, pixels[3].pb)),
        pr_avg: get_chroma_avg((pixels[0].pr, pixels[1].pr, pixels[2].pr, pixels[3].pr))
    }
}

// Deconstructs a block into 4 component video pixels
fn deconstruct_block(block: Block) -> [ComponentVideo; 4] {
    [ComponentVideo{y: block.luminance[0] as f32, pb: block.pb_avg, pr: block.pr_avg},
    ComponentVideo{y: block.luminance[1] as f32, pb: block.pb_avg, pr: block.pr_avg}, 
    ComponentVideo{y: block.luminance[2] as f32, pb: block.pb_avg, pr: block.pr_avg},
    ComponentVideo{y: block.luminance[3] as f32, pb: block.pb_avg, pr: block.pr_avg}]
}

/// Creates a vector of blocks which holds all the 2x2 blocks for the whole entire image
///
/// # Arguments
///
/// * `image`: an array2 that is holding component video pixels.
pub fn component_video_to_blocks(image: Array2<ComponentVideo>) -> Array2<Block> {
    let mut block_vec = vec![];
    for i in 0..image.height()/2 {
        for j in 0..image.width()/2 {
            // indexing to get the pixels from 2x2 blocks
            let arr = [image.get(2*i, 2*j).unwrap().clone(), image.get(2*i, 2*j+1).unwrap().clone(),
            image.get(2*i+1, 2*j).unwrap().clone(), image.get(2*i+1, 2*j+1).unwrap().clone()];
            // pushes to vector in row-major order
            block_vec.push(create_block(arr));
        }
    }
    Array2::from_row_major(block_vec, image.width()/2, image.height()/2)
}

/// Creates an Array2 of component video pixels from all of the blocks that were created
///
/// # Arguments
///
/// * `blocks`: an array2 that holds all 2x2 blocks
pub fn blocks_to_component_video(blocks: Array2<Block>) -> Array2<ComponentVideo> {
    let mut array2_vid = Array2::from_blank_state(
        ComponentVideo {y: 0.0, pb: 0.0, pr: 0.0}, blocks.width()*2, blocks.height()*2);
    for i in 0..blocks.height() {
        for j in 0..blocks.width() {
            let arr = deconstruct_block(blocks.get(i,j).unwrap().clone());
            // indexing correctly back into the ComponentVideo array2
            array2_vid.insert(2*i, 2*j, arr[0].clone());
            array2_vid.insert(2*i, 2*j+1, arr[1].clone());
            array2_vid.insert(2*i+1, 2*j, arr[2].clone());
            array2_vid.insert(2*i+1, 2*j+1, arr[3].clone());
        }
    }
    array2_vid
}
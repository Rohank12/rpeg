use crate::block::Block;
use array2::Array2;
use csc411_arith::*;

/// A `BitValues` stores the different bit integers where `a` is a 9 bit unsigned scaled integer,
/// `b` is a 5 bit signed scaled integer, `c` is a 5 bit signed scaled integer, `d` is a 5 bit
/// signed scaled integer, `index_pb` is a 4 bit unsigned index, and `index_pr` is a 4 bit
/// unsigned index.
#[derive(Clone)]
pub struct BitValue {
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub index_pb: usize,
    pub index_pr: usize,
}

// Converts a to a 9 bit unsigned integer
fn a_to_bits(a: f64) -> u64 {
    (a*511.0).round() as u64
}

// Converts a to a f64 from a 9 bit unsigned integer
fn a_to_f64(a: u64) -> f64 {
    a as f64/511.0
}

// Converts b, c, and d to a 5 bit signed integers
fn b_c_d_to_bits(b: f64, c:f64, d:f64) -> (i64, i64, i64) {
    (encode(b), encode(c), encode(d))
}

// Converts b, c, and d from a 5 bit signed integers to f64s
fn b_c_d_to_f64(b: i64, c:i64, d:i64) -> (f64, f64, f64) {
    (decode(b), decode(c), decode(d))
}

// Converts the pb avg and pr avg to a 4 bit unsigned integers
fn pb_pr_avg_to_bits(pb_avg: f32, pr_avg: f32) -> (usize, usize) {
    (index_of_chroma(pb_avg), index_of_chroma(pr_avg))
}

// Converts the pb avg and pr avg from a 4 bit unsigned integers to f32s
fn pb_pr_avg_to_f32(pb_avg: usize, pr_avg: usize) -> (f32, f32) {
    (chroma_of_index(pb_avg), chroma_of_index(pr_avg))
}

// Handles the arithmetic to correctly convert b, c, and d to i64s
fn encode(mut x: f64) -> i64 {
    if x > 0.3 {
        x = 0.3;
    } else if x < -0.3 {
        x = -0.3;
    } else {
        // x is valid
    }
    (x*50.0).round() as i64
}

// Handles converting back to f64
fn decode(n: i64) -> f64 {
    n as f64/50.0
}

/// Converts the floating point values from the 2x2 blocks to signed and unsigned
/// integer values
///
/// # Arguments
///
/// * `blocks`: an array2 that holds all 2x2 blocks in cosine space
pub fn convert_to_bits(cs_blocks: Array2<Block>) -> Array2<BitValue> {
    let mut bitval_vec = vec![];
    for (_r, _c, block) in cs_blocks.iter_row_major() {
        let bcd = b_c_d_to_bits(block.luminance[1], block.luminance[2], block.luminance[3]);
        let pb_pr = pb_pr_avg_to_bits(block.pb_avg, block.pr_avg);
        bitval_vec.push(BitValue {a: a_to_bits(block.luminance[0]), b: bcd.0, c: bcd.1, d: bcd.2, 
            index_pb: pb_pr.0, index_pr: pb_pr.1});
    }
    Array2::from_row_major(bitval_vec, cs_blocks.width(), cs_blocks.height())
}

/// Converts the signed and unsigned integer values from the 2x2 blocks back
/// to floating point values
///
/// # Arguments
///
/// * `blocks`: an array2 that holds all of the bit values per 2x2 block
pub fn convert_to_cs(bit_blocks: Array2<BitValue>) -> Array2<Block> {
    let mut cs_block_vec = vec![];
    for (_r, _c, bit_block) in bit_blocks.iter_row_major() {
        let bcd = b_c_d_to_f64(bit_block.b, bit_block.c, bit_block.d);
        let pb_pr = pb_pr_avg_to_f32(bit_block.index_pb, bit_block.index_pr);
        cs_block_vec.push(Block {luminance: [a_to_f64(bit_block.a), bcd.0, bcd.1, bcd.2], pb_avg: pb_pr.0, 
        pr_avg: pb_pr.1});
    }
    Array2::from_row_major(cs_block_vec, bit_blocks.width(), bit_blocks.height())
}
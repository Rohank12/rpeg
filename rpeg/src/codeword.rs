use bitpack::bitpack::{newu, news, gets, getu};
use array2::Array2;
use crate::quantization::*;
use csc411_rpegio::*;

// Prints the codewords utilizing rpegio
fn print_codewords(bytes: Vec<[u8; 4]>, width: usize, height: usize) {
    output_rpeg_data(&bytes, width.try_into().unwrap(), height.try_into().unwrap());
}

// Creates a 32 bit codeword utilizing bitpack, and values found in 2x2 block
fn create_codeword(codes: &BitValue) -> u32 {
    let mut codeword = 0_u64;
    codeword = newu(codeword, 4, 0, codes.index_pr as u64).unwrap();
    codeword = newu(codeword, 4, 4, codes.index_pb as u64).unwrap();
    codeword = news(codeword, 5, 8, codes.d).unwrap();
    codeword = news(codeword, 5, 13, codes.c).unwrap();
    codeword = news(codeword, 5, 18, codes.b).unwrap();
    codeword = newu(codeword, 9, 23, codes.a).unwrap();
    codeword as u32
}

// Deconstructs the codeword back to quantized values in a 2x2 block
fn deconstruct_codeword(codeword: u32) -> BitValue {
    BitValue {
        a: getu(codeword.into(), 9, 23),
        b: gets(codeword.into(), 5, 18),
        c: gets(codeword.into(), 5, 13),
        d: gets(codeword.into(), 5, 8),
        index_pb: getu(codeword.into(), 4, 4) as usize,
        index_pr: getu(codeword.into(), 4, 0) as usize
    }
}

// Converts a 32 bit codeword to an array of 4 bytes
fn codeword_to_bytes(codeword: u32) -> [u8; 4] {
    codeword.to_be_bytes()
}

// Converts an array of 4 bytes to a 32 bit codeword
fn bytes_to_codeword(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

/// Contructs and outputs all codewords from the quantized 2x2 blocks
/// 
/// # Arguments
/// 
/// * `quantized_block`: an array2 of 2x2 blocks of quantized values that make up the 32 bit codeword
pub fn output_all_codewords(quantized_block: Array2<BitValue>) {
    let mut byte_vec = vec![];
    for (_r, _c, block) in quantized_block.iter_row_major() {
        byte_vec.push(codeword_to_bytes(create_codeword(block).try_into().unwrap()));
    }
    print_codewords(byte_vec, quantized_block.width()*2, quantized_block.height()*2);
}

/// Reads the codewords from standard in or a file, 
/// and converts them back to 2x2 blocks of quantized values
/// 
/// # Arguments
/// 
/// * `file_path`: An optional file path, if `None` read from standard in
pub fn read_all_codewords(file_path: Option<&str>) -> Array2<BitValue> {
    let input_data = read_in_rpeg_data(file_path).unwrap();
    let mut quantized_values = vec![];
    for row in 0..input_data.2/2 {
        for col in 0..input_data.1/2 {
            quantized_values.push(deconstruct_codeword(bytes_to_codeword(input_data.0[(row * input_data.1/2 + col) as usize])));
        }
    }
    Array2::from_row_major(quantized_values, (input_data.1/2) as usize, (input_data.2/2) as usize)
}
use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 {
        n==0
    } else {
        n >= -(2_i64.pow((width-1).try_into().unwrap())) && n < 2_i64.pow((width-1).try_into().unwrap())
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 64 {
        n < u64::MAX
    } else {
        n < 2_u64.pow((width).try_into().unwrap())
    }
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    if width > 64 || width+lsb > 64 {
        panic!("width is too large");
    }
    if width == 0 {
        0
    } else {
        let val = shift_left(word, 64-width-lsb);
        sra(val as i64, 64 - width)
    }
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    if width > 64 || width+lsb > 64 {
        panic!("width is too large");
    }
    let val = shift_left(word, 64-width-lsb);
    srl(val, 64 - width)
}

// shifts the word to the left by some number of bits
#[inline]
fn shift_left(word: u64, bits: u64) -> u64 {
    if bits == 64 {
        0
    } else {
        word << bits
    }
}

// shifts the unsigned word to the right by some number of bits
#[inline]
fn srl(word: u64, bits: u64) -> u64 {
    if bits == 64 {
        0
    } else {
        word >> bits
    }
}

// shifts the signed word to the right by some number of bits, copying the leading bit
#[inline]
fn sra(word: i64, bits: u64) -> i64 {
    if bits == 64 && word < 0 {
        i64::MIN
    } else if bits == 64 && word >= 0{
        0
    } else {
        word >> bits
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if width > 64 || width + lsb > 64 || !fitsu(value, width) {
        None
    } else {
        let left = shift_left(srl(word, width+lsb), width+lsb);
        let right = srl(shift_left(word, 64-lsb), 64-lsb);
        Some(left | shift_left(value, lsb) | right)
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if width > 64 || width + lsb > 64 || !fitss(value, width) {
        None
    } else {
        let val; 
        if value < 0 {
            // get the correct u64 respective to the width of the bit field
            val = srl(shift_left(value as u64, 64-width), 64-width);
        } else {
            val = value as u64;
        }
        let left = shift_left(srl(word, width+lsb), width+lsb);
        let right = srl(shift_left(word, 64-lsb), 64-lsb);
        Some(left | shift_left(val, lsb) | right)
    }
}
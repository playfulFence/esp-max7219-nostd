//! Provides mappings for a mono spaced font I created myself.
//! The font is not complete and many symbols are missing.
//! This ist just an demonstration how it could be done.
//!
//! Each symbol is exactly `SingleDisplayData * SingleDisplayData` big.

use crate::LED_SQUARE_MATRIX_DIM;

/// We have 8 rows and 8 bits per row.
pub type SingleDisplayData = [u8; LED_SQUARE_MATRIX_DIM];

// Capital letter A mapping
pub const CAP_A: SingleDisplayData = [
    0b01110000,
    0b10001000,
    0b10001000,
    0b10001000,
    0b11111000,
    0b10001000,
    0b10001000,
    0b10001000,
];
pub const CAP_B: SingleDisplayData = [
    0b01111000,
    0b01000100,
    0b01000100,
    0b01111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01111000,
];
pub const CAP_C: SingleDisplayData = [
    0b01111100,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01111100,
];
pub const CAP_D: SingleDisplayData = [
    0b01111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01111000,
];
pub const CAP_E: SingleDisplayData = [
    0b01111100,
    0b01000000,
    0b01000000,
    0b01111100,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01111100,
];
pub const CAP_F: SingleDisplayData = [
    0b01111100,
    0b01000000,
    0b01000000,
    0b01111000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
];
pub const CAP_G: SingleDisplayData = [
    0b01110000,
    0b10001000,
    0b10001000,
    0b10000000,
    0b10111000,
    0b10001000,
    0b10001000,
    0b01110000,
];
pub const CAP_H: SingleDisplayData = [
    0b01000100,
    0b01000100,
    0b01000100,
    0b01111100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
];
pub const CAP_I: SingleDisplayData = [
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
];
pub const CAP_J: SingleDisplayData = [
    0b00000100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b01000100,
    0b00111000,
];
pub const CAP_K: SingleDisplayData = [
    0b01000100,
    0b01001000,
    0b01010000,
    0b01100000,
    0b01010000,
    0b01001000,
    0b01000100,
    0b01000010,
];
pub const CAP_L: SingleDisplayData = [
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01000000,
    0b01111100,
];
pub const CAP_M: SingleDisplayData = [
    0b10001000,
    0b11011000,
    0b10101000,
    0b10101000,
    0b10001000,
    0b10001000,
    0b10001000,
    0b10001000,
];
pub const CAP_N: SingleDisplayData = [
    0b01000100,
    0b01000100,
    0b01100100,
    0b01010100,
    0b01010100,
    0b01001100,
    0b01000100,
    0b01000100,
];
pub const CAP_O: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b00111000,
];
pub const CAP_P: SingleDisplayData = [
    0b01111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01111000,
    0b01000000,
    0b01000000,
    0b01000000,
];
pub const CAP_Q: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01000100,
    0b01010100,
    0b01001000,
    0b00110100,
];
pub const CAP_R: SingleDisplayData = [
    0b01111000,
    0b01000100,
    0b01000100,
    0b01111000,
    0b01100000,
    0b01010000,
    0b01001000,
    0b01000100
];
pub const CAP_S: SingleDisplayData = [
    0b00011100,
    0b00100010, 
    0b00100000, 
    0b00011000, 
    0b00000100, 
    0b00000010, 
    0b00100010, 
    0b00011100,
];
pub const CAP_T: SingleDisplayData = [
    0b01111100,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
];
pub const CAP_U: SingleDisplayData = [
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00011100,
];
pub const CAP_V: SingleDisplayData = [
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00100010,
    0b00010100,
    0b00001000,
];
pub const CAP_W: SingleDisplayData = [
    0b10001000,
    0b10001000,
    0b10001000,
    0b10001000,
    0b10001000,
    0b10101000,
    0b11011000,
    0b10001000,
];
pub const CAP_X: SingleDisplayData = [
    0b01000100,
    0b01000100,
    0b00101000,
    0b00010000,
    0b00101000,
    0b01000100,
    0b01000100,
    0b01000100,
];
pub const CAP_Y: SingleDisplayData = [
    0b01000100,
    0b01000100,
    0b01000100,
    0b00101000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
];
pub const CAP_Z: SingleDisplayData = [
    0b01111110,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01111110,
];
pub const ZERO: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b01001100,
    0b01011100,
    0b01010100,
    0b01110100,
    0b01100100,
    0b00111000,
];
pub const ONE: SingleDisplayData = [
    0b00000100,
    0b00001100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00001110,
];
pub const TWO: SingleDisplayData = [
    0b00011100,
    0b00100010,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b00111110,
];
pub const THREE: SingleDisplayData = [
    0b00111100,
    0b00000010,
    0b00000010,
    0b00011100,
    0b00000010,
    0b00000010,
    0b00000010,
    0b00111100,
];
pub const FOUR: SingleDisplayData = [
    0b00100010,
    0b00100010,
    0b00100010,
    0b00111110,
    0b00000010,
    0b00000010,
    0b00000010,
    0b00000010,
];
pub const FIVE: SingleDisplayData = [
    0b00111110,
    0b00100000,
    0b00100000,
    0b00111100,
    0b00000010,
    0b00000010,
    0b00000010,
    0b00111100,
];
pub const SIX: SingleDisplayData = [
    0b00011100,
    0b00100000,
    0b00100000,
    0b00100000,
    0b00111100,
    0b00100010,
    0b00100010,
    0b00011100,
];

pub const SEVEN: SingleDisplayData = [
    0b01111100,
    0b00000100,
    0b00000100,
    0b00001000,
    0b01111100,
    0b00100000,
    0b01000000,
    0b01000000,
];

pub const EIGHT: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b01000100,
    0b00111000,
    0b01000100,
    0b01000100,
    0b01000100,
    0b00111000,
];
pub const NINE: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b01000100,
    0b00111100,
    0b00000100,
    0b00000100,
    0b00000100,
    0b00111000,
];

pub const SPACE: SingleDisplayData = [0; 8];
pub const DOT: SingleDisplayData = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0b00010000,
];
pub const EXCLAMATION_MARK: SingleDisplayData = [
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0b00010000,
    0,
    0b00010000,
];
pub const QUESTION_MARK: SingleDisplayData = [
    0b00111000,
    0b01000100,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00010000,
    0,
    0b00010000,
];
pub const COMMA: SingleDisplayData = [
    0,
    0,
    0,
    0,
    0,
    0,
    0b00100000,
    0b01100000,
];

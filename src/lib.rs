//! Provides utility functions on top of "max7219"-crate to display data (like text) on a
//! MAX7219 powered matrix display.

#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use max7219::DataError;
use max7219::connectors::Connector;

use crate::encoding::encode_string;
use crate::mappings::SingleDisplayData;

use max7219::connectors::PinConnector;
use max7219::MAX7219;
use max7219::DecodeMode;

//use esp_hal_common::Delay;
use embedded_hal::{blocking::delay::DelayMs};
use esp_hal_common::prelude::_embedded_hal_blocking_delay_DelayMs;

/// We use 8x8 square matrices (per single display)
pub const LED_SQUARE_MATRIX_DIM: usize = 8;

/// Maximum supported chained displays by MAX7219.
pub const MAX_DISPLAYS: usize = 16;

pub mod encoding;
pub mod mappings;

/// Shift all row bits one to the left (to the next col). This way you can animate a moving text.
///
/// * `moving_bits` Vector with the data of all content to display. Each index describes
///                 the 8x8 bit data for a single display.
/// * `repeat` shift 1 bits on the very left to the ending of the vector. Without repeat
//            the vector will be all zeros after enough iterations.

pub fn shift_all_rows_one_bit_left(moving_bits: &mut [SingleDisplayData], /*, repeat: bool*/) {
    // move all bits to next position

    // so we iterate through the whole vector
    // note that probably only [0]..[DISPLAY_COUNT] are shown; this are the active displays
    // while that bits are shifted though the vec!

    let len = moving_bits.len();
    for display_i in 0..len {
        for row_i in 0..8 {
            // we need to shift to next segment if MSB per row is 1
            if moving_bits[display_i][row_i] & 0b10000000 != 0 {
                if display_i == 0
                /*&& repeat*/
                {
                    // to the last display
                    moving_bits[len - 1][row_i] |= 1;
                } else {
                    // to display from previous iteration
                    moving_bits[display_i - 1][row_i] |= 1;
                }
            }
            // shift all in row on to the left
            moving_bits[display_i][row_i] <<= 1;
        }
    }
}

/// Convenient function that turns on the display, clears the display
/// and sets the brightness to the highest possible value. It also sets
/// the DecodeMode to NoDecode which is necessary for displaying content on
/// the 8x8 matrix display. (Max7219 can also be used for 7 segment displays).
///
/// * `display` - mutable reference to Max7219 display driver
/// * `display_count` - count of displays connected to the MAX7219
/// * `intensity` - brightness for the display; value between `0x00` and `0x0F`
/// 
pub fn prepare_display<T>(
    display: &mut MAX7219<T>,
    display_count: usize,
    intensity: u8
) where T:Connector {
    let display_count = display_count % MAX_DISPLAYS;
    display.power_on().unwrap();
    for i in 0..display_count {
        display.set_decode_mode(i, DecodeMode::NoDecode).unwrap();
        display.clear_display(i).unwrap();
        display.set_intensity(i, intensity).unwrap();
    }
}

/// Shows a moving text in loop. After each iteration all bits are shifted one col to the left.
/// ** Make sure to call `prepare_display()` first! **
///
/// * `display` - mutable reference to Max7219 display driver
/// * `text` - the text to display
/// * `display_count` - count of displays connected to the MAX7219
/// * `ms_sleep` - timeout after each iteration
/// * `max_gap_width` - set's the maximum width/count of empty cols between characters. Recommended is 2. 0 to deactivate.
/// 
pub fn show_moving_text_in_loop<T>(
    display: &mut MAX7219<T>,
    text: &str,
    display_count: usize,
    ms_sleep: u32,
    max_gap_width: usize,
    delay : &mut impl DelayMs<u32>
)
where T:Connector {
    let display_count = display_count % MAX_DISPLAYS;

    let raw_bits = encode_string(text);
    let mut display_bits;
    if max_gap_width > 0 {
        display_bits = remove_gaps_in_display_text(&raw_bits, max_gap_width);
    } else {
        display_bits = raw_bits;
    }
    loop {
        for i in 0..display_count {
            display.write_raw(i, &display_bits[i]).unwrap();
        }
        delay.delay_ms(ms_sleep);
        // shift all rows one bit to the left
        shift_all_rows_one_bit_left(&mut display_bits);
    }
}

pub fn show_moving_text_in_loop_with_joystick_interrupt<T>(
    display: &mut MAX7219<T>,
    text: &str,
    display_count: usize,
    ms_sleep: u32,
    max_gap_width: usize,
    delay : &mut impl DelayMs<u32>,
)
where T:Connector {
    let display_count = display_count % MAX_DISPLAYS;

    let raw_bits = encode_string(text);
    let mut display_bits;
    if max_gap_width > 0 {
        display_bits = remove_gaps_in_display_text(&raw_bits, max_gap_width);
    } else {
        display_bits = raw_bits;
    }
    loop {
        for i in 0..display_count {
            display.write_raw(i, &display_bits[i]).unwrap();
        }
        delay.delay_ms(ms_sleep);
        // shift all rows one bit to the left
        shift_all_rows_one_bit_left(&mut display_bits);
    }
}


/*
    Draws a point to specified display from chain (if only one display - set 0 at calling) 
    according to "coordinates" which were set in function call.

    display_actual_state - [u8, 8] array, which contains an actual state of display
    x - horizontal placement of required point
    y - vertical placement of required point

*/
pub fn draw_point<T>(
    display: &mut MAX7219<T>,
    addr: usize,
    display_actual_state: &mut SingleDisplayData,
    x: usize,                                                          
    y: usize,
) where T:Connector {
    if (x < 1 || x > 8 ) || (y < 1 || y > 8 ) {
        return;
    }
    display_actual_state[y-1] |= (0b00000001 << (8-x));

    display.write_raw(addr, display_actual_state).unwrap();
}

/* This is kinda dumb solution, but why the hell no? :D 
    obviously, it just clears the display like a corresponding function from 
    max7219 crate, but also clears an array with actual state 
*/

pub fn clear_with_state<T>(
    display: &mut MAX7219<T>,
    addr: usize, 
    display_actual_state: &mut SingleDisplayData,
) where T:Connector {
    display.clear_display(addr);

    for i in 0..8 {
        display_actual_state[i] &= 0;
    }
}

/// Iterates through the data and removes all gaps between symbols. A gap is two or more cols
/// after each other that are all zero.
pub fn remove_gaps_in_display_text(display_data: &[SingleDisplayData], max_gap_size: usize) -> Vec<SingleDisplayData> {
    let display_data: Vec<SingleDisplayData> = display_data.iter()
        .map(|x| transpose_single_display_data(x.clone()))
        .collect();
    let mut display_data_expanded = vec![];
    for x in &display_data {
        for y in x {
            display_data_expanded.push(y);
        }
    }

    let mut preserve_at_begin = 0;
    let mut preserve_at_end = 0;
    for i in 0..display_data_expanded.len() {
        if *display_data_expanded[i] == 0 {
            preserve_at_begin += 1;
        } else {
            break;
        }
    }
    for i in 0..display_data_expanded.len() {
        // go backwards
        let i = display_data_expanded.len() - 1 - i;
        if *display_data_expanded[i] == 0 {
            preserve_at_end += 1;
        } else {
            break;
        }
    }


    // keep empty cols at begin
    let mut shrinked_display_data_expanded = vec![0 as u8; preserve_at_begin];

    let mut count_since_last_not_empty = 0;
    for i in preserve_at_begin..display_data_expanded.len() - preserve_at_end {
        if *display_data_expanded[i] == 0 {
            count_since_last_not_empty += 1;
        } else {
            count_since_last_not_empty = 0;
        }

        if count_since_last_not_empty <= max_gap_size {
            shrinked_display_data_expanded.push(*display_data_expanded[i]);
        }
    }
    // keep empty cols at end
    shrinked_display_data_expanded.extend_from_slice(&vec![0; preserve_at_end]);

    // now transform again to Vec<SingleDisplayData>
    // 1) check if length is multiple of eight
    let add_to_be_divider_of_8 = 8 - (shrinked_display_data_expanded.len() % 8);
    shrinked_display_data_expanded.extend_from_slice(&vec![0; add_to_be_divider_of_8]);

    let mut shrinked_display_data_transposed: Vec<SingleDisplayData> = vec![];
    for i in (0..shrinked_display_data_expanded.len()).step_by(8) {
        let mut slice: SingleDisplayData = [0; 8];
        for j in 0..8 {
            slice[j] = shrinked_display_data_expanded[i + j];
        }
        shrinked_display_data_transposed.push(slice);
    }

    // transpose again,so that rows become cols again
    let shrinked_display_data: Vec<SingleDisplayData> = shrinked_display_data_transposed.into_iter()
        .map(|x| transpose_single_display_data(x))
        .collect();

    shrinked_display_data
}

/// This does a transpose operation on the `SingleDisplayData`-Matrix and is a helper function for
/// [`remove_gaps_in_display_text`]. Cols become rows and rows become cols.
/// Example:
/// ```
/// let _ = [
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
///     0b10000000,
/// ];
/// ```
/// becomes
/// ```
/// let _ = [
/// 0b11111111,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// 0,
/// ];
/// ```
pub fn transpose_single_display_data(data: SingleDisplayData) -> SingleDisplayData {
    let mut transposed_data: SingleDisplayData = [0; 8];
    for col_i in 0..8 {
        // the data/bits of the current col
        let mut col = 0;
        for row_i in 0..8 {
            // we get the current col value (bit) at index "data[row_i][col_i]"
            // bit by bit. We move the current bit to the lowest index via
            // bit shifting and do a bitwise and with 1.
            let col_bit = (data[row_i] >> (7 - col_i)) & 1;
            // now we first shift all col bits from previous iterations one index
            // to the left and then we add our current bit to the col at the lowest index.
            col = (col << 1) | col_bit;
        }
        transposed_data[col_i] = col;
    }
    transposed_data
}
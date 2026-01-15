/// EGA 8x14 font handling
///
/// The EGA font is stored as raw binary data with 256 characters,
/// each character is 14 bytes (one byte per scanline, 8 pixels wide).

/// EGA 8x14 font data embedded at compile time
static EGA_8X14_FONT: &[u8] = include_bytes!("../fonts/ega-8x14.bin");

/// Font dimensions
pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 14;
pub const FONT_BYTES_PER_CHAR: usize = FONT_HEIGHT;

/// Get the font data for a specific character
///
/// # Arguments
/// * `char_code` - The CP437 character code (0-255)
///
/// # Returns
/// A slice of 14 bytes representing the character's bitmap
pub fn get_char_bitmap(char_code: u8) -> &'static [u8] {
    let start = (char_code as usize) * FONT_BYTES_PER_CHAR;
    let end = start + FONT_BYTES_PER_CHAR;
    &EGA_8X14_FONT[start..end]
}

/// Check if a specific pixel is set in a font byte
///
/// # Arguments
/// * `byte` - The font byte (one scanline of the character)
/// * `bit_position` - The bit position (0-7, where 0 is leftmost)
///
/// # Returns
/// true if the pixel is set, false otherwise
#[inline]
pub fn is_pixel_set(byte: u8, bit_position: u8) -> bool {
    (byte & (0x80 >> bit_position)) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const FONT_CHAR_COUNT: usize = 256;

    #[test]
    fn test_font_data_size() {
        assert_eq!(
            EGA_8X14_FONT.len(),
            FONT_CHAR_COUNT * FONT_BYTES_PER_CHAR,
            "Font data should be 3584 bytes (256 chars * 14 bytes)"
        );
    }

    #[test]
    fn test_get_char_bitmap() {
        // Test character 0 (null)
        let char0 = get_char_bitmap(0);
        assert_eq!(char0.len(), 14);

        // Test character 65 ('A')
        let char_a = get_char_bitmap(65);
        assert_eq!(char_a.len(), 14);

        // Test last character (255)
        let char255 = get_char_bitmap(255);
        assert_eq!(char255.len(), 14);
    }

    #[test]
    fn test_is_pixel_set() {
        // Test with byte 0xFF (all pixels set)
        assert_eq!(is_pixel_set(0xFF, 0), true);
        assert_eq!(is_pixel_set(0xFF, 7), true);

        // Test with byte 0x00 (no pixels set)
        assert_eq!(is_pixel_set(0x00, 0), false);
        assert_eq!(is_pixel_set(0x00, 7), false);

        // Test with byte 0x80 (leftmost pixel only)
        assert_eq!(is_pixel_set(0x80, 0), true);
        assert_eq!(is_pixel_set(0x80, 1), false);

        // Test with byte 0x01 (rightmost pixel only)
        assert_eq!(is_pixel_set(0x01, 7), true);
        assert_eq!(is_pixel_set(0x01, 6), false);
    }
}

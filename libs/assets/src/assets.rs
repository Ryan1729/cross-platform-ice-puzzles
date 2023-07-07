use platform_types::{ARGB, GFX_LENGTH};

// reportedly colourblind friendly colours
// https://twitter.com/ea_accessible/status/968595073184092160

mod colours {
    pub const BLUE: u32 = 0xFF3352E1;
    pub const GREEN: u32 = 0xFF30B06E;
    pub const RED: u32 = 0xFFDE4949;
    pub const YELLOW: u32 = 0xFFFFB937;
    pub const PURPLE: u32 = 0xFF533354;
    #[allow(unused)]
    pub const GREY: u32 = 0xFF5A7D8B;
    #[allow(unused)]
    pub const GRAY: u32 = GREY;
    pub const WHITE: u32 = 0xFFEEEEEE;
    pub const BLACK: u32 = 0xFF222222;
}

use colours::*;

#[rustfmt::skip]
pub const PALETTE: [u32; 8] = [
    BLUE,
    GREEN,
    RED,
    YELLOW,
    PURPLE,
    GREY,
    WHITE,
    BLACK,
];

/*
    A way to convert an image to an array of bytes:
    Given an image called `image.png` use the following imagemagick command:
    ```
    magick .\image.png -define h:format=rgba -depth 8 -size 128x128  image.h
    ```
    Then use regular expression find-and-replace to convert the array to the format you want.
    For example, you might replace `0x22, 0x22, 0x22, 0xFF,` with `index6,`, then similarly
    replace the rest of the colours with something containing their index value, then remove
    all instances of `index`, leaving just the indices. Format further as needed.
*/

pub const GFX: [ARGB; GFX_LENGTH] = include!("gfx.in");

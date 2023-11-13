use csc411_image::{RgbImage, Rgb};
use bitpack::bitpack::{gets, getu};

/*
    Structures required dervice statement due to using 
    debug statements and clone functions within the 
    compression and decompression operations
*/
#[derive(Clone, Debug)]
pub struct RgbFloat {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct QuantValues{
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub avg_pb: u64,
    pub avg_pr: u64,
}

///Function: `trim_image` -> `Vec<csc411_image::Rgb>`
///
///The `trim_image` function recieves paramters:
///`current_image`: `&RbgImage` -> data type from csc411_image
///`current_width`: `u32` -> value of the `current_image`'s width
///`current_height`: `u32` -> value of the `current_image`'s height
///
///The purpose of this function is to repurpose the expected image from the command line argument
///and trim the image's height and width based on the `codec.rs`'s height and width dimensions checker.
///The function initializes an object of all black `Rgb` pixels of the same dimensions of the passed
///`current_width` and `current_height`. Then the function will iterate based on the all black `Rgb` object
///and use `.clone()` of the passed `current_image` to return a vector of `csc411_image::Rgb>` which is a 
///newer image.
pub fn trim_image(current_image: &RgbImage, current_width: u32, current_height: u32) -> Vec<csc411_image::Rgb>{
    //vector to store values
    let mut trimmed_img: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (current_height * current_width) as usize];

    //trimming last row if needed
    for i in 0..current_height{
        for j in 0..current_width{
            trimmed_img[(current_width as usize * i as usize) + j as usize] = current_image.pixels[(current_image.width as usize * i as usize) + j as usize].clone();
        }
    }
    return trimmed_img;
} //Used by Compression Function


///Function: `rgb_int_to_float` -> `Vec<RgbFloat>`
///
///The `rgb_int_to_float` function recieves paramters:
///`current_img`: `&Vec<csc411_image::Rgb>` -> data type from csc411_image
///`init_img`: `&RgbImage` -> reference to the image selected form user command line
///`width`: `u32` -> value of the `current_image`'s width
///`height`: `u32` -> value of the `current_image`'s height
///
///The purpose of this step is to take the `current_img`'s rgb pixel values into a `f32` values. The function
///will return a vector of structure `RgbFloat` data attributes.
pub fn rgb_int_to_float(current_img: &Vec<csc411_image::Rgb>, init_img: &RgbImage, width: u32, height: u32) -> Vec<RgbFloat>{
    let mut vec: Vec<RgbFloat> = vec![RgbFloat{red: 0.0, green:0.0, blue: 0.0}; width as usize * height as usize].clone();

    //storing each pixel as a decimal value
    for pixel in 0..current_img.len(){
        vec[pixel].red = current_img[pixel].red as f32/(init_img.denominator as f32);
        vec[pixel].green = current_img[pixel].green as f32/init_img.denominator as f32;
        vec[pixel].blue = current_img[pixel].blue as f32/init_img.denominator as f32;
    }
    return vec;
} //Used by Compression Function


///Function: `get_quant_values` -> `Vec<QuantValues>`
///
///The `get_quant_values` function recieves paramters:
///`compressed_bytes`: `&Vec<csc411_image::Rgb>` -> data type from csc411_image
///
///The purpose of this function is to take the words of the compressed values and translate the
///word's values to DCT quanitized values. The function will return a vector of structure `QuantValues`
///that has the data attributes of the compressed 2x2 section of pixels from the initial image.
pub fn get_quant_values(compressed_bytes: Vec<[u8; 4]>) -> Vec<QuantValues> {
    let mut words_vec = Vec::new();
    for byte in compressed_bytes{
        
        let word_of_bytes = u32::from_be_bytes(byte);

        let a = getu(word_of_bytes as u64, 9, 23);
        let b = gets(word_of_bytes as u64, 5, 18);
        let c = gets(word_of_bytes as u64, 5, 13);
        let d = gets(word_of_bytes as u64, 5, 8);

        let avg_pb = getu(word_of_bytes as u64, 4, 4);
        let avg_pr = getu(word_of_bytes as u64, 4, 0);

        let values = QuantValues{
            a: a,
            b: b,
            c: c,
            d: d,
            avg_pb,
            avg_pr,
        };
        // saves values above
        words_vec.push(values);
    }
    return words_vec;
} //Used by Decompression Function
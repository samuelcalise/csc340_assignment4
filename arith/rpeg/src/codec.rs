/*
    Required Imports For Class
*/
use csc411_image::{Read, RgbImage, Write};
use bitpack::bitpack::{newu, news};
use csc411_rpegio::{output_rpeg_data, input_rpeg_data};
use crate::format::{trim_image, rgb_int_to_float, get_quant_values};
use crate::value_conversion::{rgb_to_ypbpr, get_dct_values, dct_function, dct_to_rgb};

/*
    Structures required dervice statement due to using 
    debug statements and clone functions within the 
    compression and decompression operations
*/
#[derive(Clone, Debug)]
pub struct Ypbpr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

#[derive(Clone, Debug)]
pub struct RgbFloat {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct DCTValues{
    pub yval: f32,
    pub avg_pb: f32,
    pub avg_pr: f32,
}

/// Function: Compress
///
///The `compress` function takes in object `filename` that has the type
///`Option<&str>`. The function's operation is to take in an inital img
///`init_image` that is read in through the csc411_image crate as an 
///`RbgImage` object.
pub fn compress(filename: Option<&str>)
{
    //Getting initial image
    let init_image = RgbImage::read(filename).unwrap();

    //Identifying the initial images current width and height
    //in case of need to trim an unfriendly dimensions.
    let mut current_height = init_image.height;
    let mut current_width = init_image.width;
    
    /*
        Boolean statements to check both dimensions if
        they are not divisible by 2. If one or both cases
        are true based on the initial read in image, the
        if statements will decremenet the current height 
        and width by one. At any case of this being true,
        this would relate to information lost due to losing
        either a singular column or row of the image
    */
    if init_image.height % 2 != 0
    {
        current_height -=1;
    }
    if init_image.width % 2 != 0
    {
        current_width -= 1;
    }

    //STEP 1 => Trim image to ensure expected height and width
    let trimmed_img = trim_image(&init_image, current_width, current_height);
    
    //STEP 2 => Taking RGB values from trimmed image and converting into f32 RGB values
    let float_rgb_img = rgb_int_to_float(&trimmed_img, &init_image, current_width, current_height);
    
    //STEP 3 => Take the float rgb image and find Y, P_b, and P_r
    let this_vector = rgb_to_ypbpr(&trimmed_img, &float_rgb_img, current_width, current_height);

    //STEP 4 => Converting image to DCT (discrete cosine transform) format 
    let mut word_vec = Vec::new();
    //2x2 section of pixels
    for row in (0..current_height).step_by(2){
        for col in (0..current_width).step_by(2){
            //discrete cosine transform
            let (avg_pb,avg_pr,a,b,c,d) = get_dct_values(&this_vector, current_width, row, col);
            //defining default word of 0 (u64)
            let mut word = 0_u64;
            word = newu(word, 9, 23, a as u64).unwrap();
            word = news(word, 5, 18, b as i64).unwrap();
            word = news(word, 5, 13, c as i64).unwrap();
            word = news(word, 5, 8, d as i64).unwrap();
            word = newu(word, 4, 4, avg_pb as u64).unwrap();
            word = newu(word, 4, 0, avg_pr as u64).unwrap();
            word_vec.push((word as u32).to_be_bytes());
        }
    }

    //Completed compression
    let _ = output_rpeg_data(&word_vec, current_width as usize, current_height as usize);
}

/// Function: Compress
///
///The `decompress` function takes in object `filename` that has the type
///`Option<&str>`. The function's operation is to take the compressed image from
///the previous `compress` function and translate the compressed image into a new
///developed ppm image.
pub fn decompress(filename: Option<&str>) 
{
    /*
        Using the input_rpeg_data function that returns the compressed bytes
        the compressed ppm images' width and height
    */
    let (compressed_bytes, width, height) = input_rpeg_data(filename).unwrap();//Function located in csc411_rpegio crate online
    
    //STEP 1 => Read compressed data from compressed image
    let decompressed_words = get_quant_values(compressed_bytes); //Function located in format.rs

    //STEP 2 => Codewords and revert to DCT values
    let mut dct_values: Vec<DCTValues> = vec![DCTValues{yval: 0.0, avg_pb: 0.0, avg_pr: 0.0}; (width * height) as usize]; //Similar process recommended from TA Ayman
    dct_values = dct_function(dct_values, height as u32, width as u32, decompressed_words);//Function located             //from locality office hours          
                                                                                           //in value_conversion.rs                               
    //STEP 3 => Reverting DCT values to rgb values
    let rgb_decompressed_values = dct_to_rgb(dct_values); //Function located in value_conversion.rs

    //writing final RGB image out
    let completed_image = RgbImage{
        width: width as u32,
        height: height as u32,
        denominator: 255,
        pixels: rgb_decompressed_values,
    };

    //Completed Image
    completed_image.write(None).unwrap();//Function located in csc411_image crate online
}
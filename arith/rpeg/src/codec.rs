use csc411_image;
//use csc411_rpegio;
use csc411_image::{Read, RgbImage};

pub mod format;
use format::{trim_image, pixels_from_int_to_float};

pub mod value_conversions;
use value_conversions::{rgb_float_to_ypbpr};


struct RgbFloatValues
{
    red: f32,
    green: f32,
    blue: f32,
}

struct YpbprValues
{
    y: f32,
    P_b: f32,
    P_r: f32,
}

pub fn compress(input_name: &str)
{
/*
    Handling reading image from command line in terminal from main.rs
    and having place holders for its current width and height. When
    checking the height and with of the initial image, we may have to 
    trim the pixels of either row and column of the image. This results
    in data lost at the beginning step of the program.
*/
    let init_img = RgbImage::read(Some(input_name)).unwrap();

    let mut current_width = init_img.width;
    let mut current_height = init_img.height;

    if init_img.height % 2 != 0
    {
        current_height -= 1;
    }
    else if init_img.width % 2 != 0
    {
        current_width -= 1;
    }

    //STEP 1: Getting Rgb Image
    let current_img = trim_image(&init_img, current_width, current_height);

    //Step 2: Rgb -> Rgb Float Pixel Values
    let rgb_float_img = pixels_from_int_to_float(&current_img, &init_img, current_width, current_height);

    //Step 3: Rgb Float Pixel Values -> Y, P_b, and P_r
    let Y_Pb_Pr_vec = rgb_float_to_ypbpr(&current_img, &rgb_float_img, current_width, current_height);

}
use csc411_image::{RgbImage, Rgb};

struct rgb_float_values
{
    red: f32,
    green: f32,
    blue: f32,
}

pub fn trim_image(image: &RgbImage, width: u32, height: u32) -> Vec<csc411_image::Rgb>
{
    let this_image: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (width * height) as usize];

    for i in 0..height
    {
        for j in 0..width
        {
            this_image[ (width as usize * i as usize) + j as usize] = image.pixels[ (image.width as usize * i as usize) + j as usize].clone();
        }
    }

    this_image
}

pub fn pixels_from_int_to_float(this_image: &Vec<csc411_image::Rgb>, init_img: &RgbImage, width: u32, height: u32)
{
    let float_img: Vec<rgb_float_values> = vec![rgb_float_values{red: 0.0, green: 0.0, blue: 0.0}; width * height].clone();

    for pixel_element in 0..this_image.len()
    {
        float_img[pixel_element].red = this_image[pixel_element].red as f32/init_img.denominator as f32;
        float_img[pixel_element].green = this_image[pixel_element].green as f32/init_img.denominator as f32;
        float_img[pixel_element].blue = this_image[pixel_element].blue as f32/init_img.denominator as f32;
    }
    
    float_img
}
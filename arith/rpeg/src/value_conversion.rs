
#[derive(Clone)]
struct ypbpr_values
{
    y: f32,
    P_b: f32,
    P_r: f32,
}
#[derive(Clone)]
struct rgb_float_values
{
    red: f32,
    green: f32,
    blue: f32,
}

pub fn rgb_float_to_ypbpr(init_img: Vec<ypbpr_values>, float_img_values: &Vec<rgb_float_values> , width: u32, height: u32) -> Vec<ypbpr_values>
{
    let this_vector: Vec<ypbpr_values> = vec![ypbpr_values{y: 0.0, P_b: 0.0, P_r: 0.0}; (width * height) as usize].clone();

    for pixel_element in 0..init_img.len()
    {
        let this_y = 0.299 * float_img_values[pixel_element].red + 0.587 * float_img_values[pixel_element].green + 0.114 * float_img_values[pixel_element].blue;
        let this_P_b = -0.168736 * float_img_values[pixel_element].red + (-0.331264) * float_img_values[pixel_element].green + 0.5 * float_img_values[pixel_element].blue;
        let this_P_r = 0.5 * float_img_values[pixel_element].red + (-0.418688) * float_img_values[pixel_element].green + (-0.081312) * float_img_values[pixel_element].blue;
    
        this_vector[pixel_element].y = this_y;
        this_vector[pixel_element].P_b = this_P_b;
        this_vector[pixel_element].P_r = this_P_r;
    }

    this_vector
}
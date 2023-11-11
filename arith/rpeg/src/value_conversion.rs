use csc411_arith::index_of_chroma;
use csc411_arith::chroma_of_index;
use crate::format::RgbFloat;
use crate::codec::DCTValues;
use crate::format::QuantValues;
use csc411_image::Rgb;

#[derive(Clone, Debug)]
pub struct Ypbpr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

///Need to Document
pub fn rgb_to_ypbpr(current_img: &Vec<csc411_image::Rgb>, rgb_float_img: &Vec<RgbFloat>, width: u32, height: u32) -> Vec<Ypbpr>{
    let mut vec: Vec<Ypbpr> = vec![Ypbpr{y: 0.0, pb:0.0, pr: 0.0}; width as usize * height as usize].clone();
    
    for pixel in 0..current_img.len(){
        let y = 0.299 * rgb_float_img[pixel].red + 0.587 * rgb_float_img[pixel].green + 0.114 * rgb_float_img[pixel].blue;
        let pb = -0.168736 * rgb_float_img[pixel].red + (-0.331264) * rgb_float_img[pixel].green + 0.5 * rgb_float_img[pixel].blue;
        let pr = 0.5 * rgb_float_img[pixel].red + (-0.418688) * rgb_float_img[pixel].green + (-0.081312) * rgb_float_img[pixel].blue;
        vec[pixel].y = y;
        vec[pixel].pb = pb;
        vec[pixel].pr = pr;

    }

    return vec;
}

///Need to document
pub fn get_dct_values(vec: &Vec<Ypbpr>, width: u32, row: u32, col: u32) -> (usize, usize, f32, f32, f32, f32) {
    
    let top_left = vec[(width * row + col) as usize].clone();
    let top_right = vec[(width * row + (col + 1)) as usize].clone();
    let bot_left = vec[(width * (row+1) + col) as usize].clone();
    let bot_right = vec[(width * (row+1) + (col+1)) as usize].clone();

    let avg_pb = (top_left.pb + top_right.pb + bot_right.pb + bot_left.pb)/4.0;
    let avg_pr = (top_left.pr + top_right.pr + bot_right.pr + bot_left.pr)/4.0;

    let avg_pb = index_of_chroma(avg_pb as f32);
    let avg_pr = index_of_chroma(avg_pr as f32);

    let mut a = (bot_right.y + bot_left.y + top_right.y + top_left.y)/4.0;
    let mut b = (bot_right.y + bot_left.y - top_right.y - top_left.y)/4.0;
    let mut c = (bot_right.y - bot_left.y + top_right.y - top_left.y)/4.0;
    let mut d = (bot_right.y - bot_left.y - top_right.y + top_left.y)/4.0;

    a = (a* 511.0).round();
    b = (b.clamp(-0.3,0.3) * 50.0).round();
    c = (c.clamp(-0.3,0.3) * 50.0).round();
    d = (d.clamp(-0.3,0.3) * 50.0).round();


    return (avg_pb, avg_pr, a, b, c, d);
}

/// Need documentation
pub fn dct_function(mut dct_vec: Vec<DCTValues>, img_height: u32, img_width: u32, word_vec: Vec<QuantValues>) -> Vec<DCTValues>{
    let mut index = 0;
    for i in (0..img_height).step_by(2){
        for j in (0..img_width).step_by(2){


            let a = (word_vec[index].a as f32 / 511.0).clamp(0.0,1.0);
            let b = (word_vec[index].b as f32 / 50.0).clamp(-0.3,0.3);
            let c = (word_vec[index].c as f32 / 50.0).clamp(-0.3,0.3);
            let d = (word_vec[index].d as f32 / 50.0).clamp(-0.3,0.3);


            let pb = chroma_of_index(word_vec[index].avg_pb as usize);
            let pr = chroma_of_index(word_vec[index].avg_pr as usize);


            let y1 = a - b - c + d;
            let y2 = a - b + c - d;
            let y3 = a + b - c - d;
            let y4 = a + b + c + d;

            //2x2 Square 
            dct_vec[(i * img_width + j) as usize] = DCTValues{
                yval: y1,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_vec[(i * img_width + (j+1)) as usize] = DCTValues{
                yval: y2,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_vec[((i+1) * img_width + j) as usize] = DCTValues{
                yval: y3,
                avg_pb: pb,
                avg_pr: pr,
            };
            dct_vec[((i+1) * img_width + (j+1)) as usize] = DCTValues{
                yval: y4,
                avg_pb: pb,
                avg_pr: pr,
            };
            index += 1;
        }
    }
    return dct_vec;
}


///Need documentation
pub fn dct_to_rgb(dct_vec: Vec<DCTValues>) -> Vec<Rgb>{
    //dct to rgb float
    let mut rgb_final = Vec::new();
    for value in dct_vec{
        let rgb_val = Rgb{
            red: ((1.0 * value.yval + 0.0 * value.avg_pb + 1.402 * value.avg_pr) * 255.0) as u16,
            green: ((1.0 * value.yval - 0.344136 * value.avg_pb - 0.714136 * value.avg_pr) * 255.0) as u16,
            blue: ((1.0 * value.yval + 1.772 * value.avg_pb + 0.0 * value.avg_pr) * 255.0) as u16,
        };
        rgb_final.push(rgb_val);
    }
    return rgb_final;
}
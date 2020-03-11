use rand::prelude::*;
use image::{self, imageops::*};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, GrayImage, Luma};



fn generate_image(width: u32, height: u32) -> GrayImage{

    let mut rand = rand::thread_rng();

    let mut rand_image : GrayImage = ImageBuffer::new(width, height);

    for (x,y, pix) in rand_image.enumerate_pixels_mut(){
        if rand.gen(){
            *pix = Luma([255]); 
        }   

        else{
            *pix = Luma([0]);
        }
    }

    rand_image

}

pub fn main(){

    let image_a = generate_image(512, 512);

    let mut image_b = generate_image(256, 256);

    let mut image_l = image_a.clone();
    let mut image_r = image_a;

    for (x, y, pix) in image_b.enumerate_pixels_mut(){
        image_l.put_pixel(x+124, y+128, *pix);
    }
    
    for (x, y, pix) in image_b.enumerate_pixels_mut(){
        image_r.put_pixel(x+132, y+128, *pix);
    }

    image_l.save("./Stereo Pairs/Stereo_gram_rand/left.png").unwrap();
    image_r.save("./Stereo Pairs/Stereo_gram_rand/right.png").unwrap();

}




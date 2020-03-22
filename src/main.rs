use image::{self, imageops::*};
use image::{ ImageBuffer, GrayImage, Luma};

mod stereo_random;

const OCCLUSION: f64 = 1.8; // different occlusion cost form the paper
// to achieve better quality disparity maps

fn match_cost(z1 :f64, z2 :f64) -> f64{
    (z2 - z1) * (z2 - z1) / 10.5
}

fn main() {
    stereo_random::main();
  
    let image1 = image :: open("./Stereo Pairs/Pair 1/view1.png").unwrap().to_rgb();
    let image2 = image :: open("./Stereo Pairs/Pair 1/view2.png").unwrap().to_rgb();
    let width = image1.dimensions().0 as usize;
    let height = image1.dimensions().1 as usize;
    
    let image1 = grayscale(&image1);
    let image2 = grayscale(&image2);
  
    let mut disparity_map : GrayImage = ImageBuffer::new(width as u32, height as u32);

    for a in 0..height{
        
        let mut cost_matrix: Vec<_> = (0..=width+1).map(|_| vec![0.0; width + 1]).collect();
        let mut decision_matrix: Vec<_> = (0..=width+1).map(|_| vec![0; width + 1]).collect();
        
        for i in 1..=width{
            cost_matrix[i][0] = i as f64 * OCCLUSION;
        }

        for i in 1..=width{
            cost_matrix[0][i] = i as f64 * OCCLUSION;
        }

        for i in 1..=width{
            for j in 1..=width{
                let min1 = cost_matrix[i-1][j-1] + match_cost(image1.get_pixel(i as u32 - 1, a as u32)[0] as f64, image2.get_pixel(j as u32 - 1,a as u32)[0] as f64);
                let min2 = cost_matrix[i][j-1] + OCCLUSION;
                let min3 = cost_matrix[i-1][j] + OCCLUSION;
                
                if min1 < min2 && min1 < min3{
                    cost_matrix[i][j] = min1;
                    decision_matrix[i][j] = 1;
                }

                else if min2 < min1 && min2 < min3{
                    cost_matrix[i][j] = min2;
                    decision_matrix[i][j] = 2;
                }

                else{
                    cost_matrix[i][j] = min3;
                    decision_matrix[i][j] = 3;
                }
            }
        }         
           
        let mut i = width;
        let mut j = width;
    
        while i > 0  && j > 0{
            if decision_matrix[i][j] == 1{
                disparity_map.put_pixel(i as u32 - 1, a as u32, Luma([((i as i32 - j as i32).abs() as u8 + 20).saturating_mul(4)]));
                i -= 1;
                j -= 1;
            }
    
            else if decision_matrix[i][j] == 2{
                j -= 1;
            }
    
            else{
                i -= 1;
            }
        }
    }

    disparity_map.save("output/output_pair1.png").unwrap();

}

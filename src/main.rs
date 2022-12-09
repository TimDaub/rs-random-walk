use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;
use rand::Rng;

fn walk_direction(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: f32, y1: f32, x2: f32, y2: f32) {
    draw_line_segment_mut(img, (x1, y1), (x2, y2), Rgb([255, 255, 255]));
}

fn main() {
    let width = 1000;
    let height = 1000;
    let mut img = RgbImage::new(width, height);

    let mut rng = rand::thread_rng();
    let step_size = 4f32;

    let mut x = 500f32;
    let mut y = 500f32;
    for n in 0..50000 {
        // NOTE: 0:=up, 1:=right, ... (clock wise)
        let draw: u32 = rng.gen_range(0..4);
        if draw == 0 {
            let new_y = y-step_size;
            if new_y < 0f32 {
                continue;
            }
            walk_direction(&mut img, x, y, x, y-step_size);
            y = new_y;
        } else if draw == 1 {
            let new_x = x+step_size;
            if new_x > width as f32 {
                continue;
            }
            walk_direction(&mut img, x, y, x+step_size, y);
            x = new_x;
        } else if draw == 2 {
            let new_y = y+step_size;
            if new_y > height as f32 {
                continue;
            }
            walk_direction(&mut img, x, y, x, y+step_size);
            y = new_y;
        } else if draw == 3 {
            let new_x = x-step_size;
            if new_x < 0f32 {
                continue;
            }
            walk_direction(&mut img, x, y, x-step_size, y);
            x = new_x;
        }
    }

    let file_name = "image.png";
    img.save(file_name);
}

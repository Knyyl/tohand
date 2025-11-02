use image::{DynamicImage, GenericImage, GenericImageView, RgbaImage, Pixel};

fn crop_to_letter(img: &RgbaImage) -> RgbaImage {
    let (mut min_x, mut max_x) = (img.width(), 0);
    let (mut min_y, mut max_y) = (img.height(), 0);

    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            // Crop based on brightness, not alpha
            if pixel[0] < 200 && pixel[1] < 200 && pixel[2] < 200 {
                if x < min_x { min_x = x; }
                if x > max_x { max_x = x; }
                if y < min_y { min_y = y; }
                if y > max_y { max_y = y; }
            }
        }
    }

    img.view(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
        .to_image()
}

fn keep_only_black(mut img: RgbaImage) -> RgbaImage {
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel_mut(x, y);
            // Check if the pixel is almost black
            if pixel[0] > 50 || pixel[1] > 50 || pixel[2] > 50 {
                *pixel = image::Rgba([0, 0, 0, 0]); // make non-black pixels transparent
            }
        }
    }
    img
}

fn main() {
    let text: String = "life is a journey that teaches us lessons through both joy and hardship. every moment offers a chance to grow and understand ourselves better. people come and go but each leaves a mark that shapes who we become. the world is full of beauty waiting to be noticed in small details like sunlight through leaves or laughter shared with friends. Though nigga"
        .to_string();

    let c: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let alphabet_char: Vec<char> = c.chars().collect();

    let chars: Vec<char> = text.chars().collect();
    let input_length = chars.len();
    let alphabet_char_len = alphabet_char.len();
    let mut counter: i32 = 0 as i32;

    println!("Total characters: {}", input_length);

    let mut letter_imgs: Vec<RgbaImage> = Vec::new();
    let spacing = 5;

    for i in 0..input_length {
        let ch = chars[i];
        if let Some(pathcount) = alphabet_char.iter().position(|&c| c == ch) {
            counter += 1;
            let calcer = (counter as f64 / input_length as f64)*100 as f64;
            let imgpath = format!("img{}.png", pathcount + 1);
            println!("{} -> {} %", imgpath, calcer as i32);

            let img = image::open(&imgpath).expect("Failed to open").into_rgba8();
            let img = keep_only_black(img);
            let img = crop_to_letter(&img);

            letter_imgs.push(img);
        }    }

    let total_width: u32 = letter_imgs
        .iter()
        .map(|img| img.width() + spacing)
        .sum::<u32>()
        .saturating_sub(spacing);

    let max_height: u32 = letter_imgs
        .iter()
        .map(|img| img.height())
        .max()
        .unwrap_or(0);

    if letter_imgs.is_empty() {
        println!("No valid characters found!");
        return;
    }

    let mut combined = DynamicImage::new_rgba8(total_width, max_height);
    let mut x_offset = 0;

    for img in letter_imgs {
        let y_offset = max_height - img.height(); // baseline align
        combined.copy_from(&img, x_offset, y_offset).unwrap();
        x_offset += img.width() + spacing;
    }

    combined
        .save("output.png")
        .expect("Failed to save combined image");
    println!("Saved output.png");
}

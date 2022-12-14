use image::Rgb;

use image::ImageBuffer;

pub fn apply_conv(
    kernel: [[f32; 3]; 3],
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    should_clamp: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, image.height());
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let mut result = [0.0, 0.0, 0.0];
            for i in 0..=2 {
                for j in 0..=2 {
                    let disloc_x = i as i32 - 1;
                    let disloc_y = j as i32 - 1;

                    let pixel =
                        image.get_pixel((x as i32 + disloc_x) as u32, (y as i32 + disloc_y) as u32);

                    result[0] += pixel[0] as f32 * kernel[i as usize][j as usize];
                    result[1] += pixel[1] as f32 * kernel[i as usize][j as usize];
                    result[2] += pixel[2] as f32 * kernel[i as usize][j as usize];
                }
            }

            if should_clamp {
                result[0] += 127.0;
                result[1] += 127.0;
                result[2] += 127.0;
            }

            result[0] = adjust_pixel_value(result[0]);
            result[1] = adjust_pixel_value(result[1]);
            result[2] = adjust_pixel_value(result[2]);

            if result[0] > 255.0 || result[0] < 0.0 {
                panic!("Incorrect value!");
            }

            output.put_pixel(
                x,
                y,
                Rgb([result[0] as u8, result[1] as u8, result[2] as u8]),
            );
        }
    }
    return output;
}

pub(self) fn adjust_pixel_value(pixel: f32) -> f32 {
    return if pixel > 255.0 {
        255.0
    } else {
        if pixel < 0.0 {
            0.0
        } else {
            pixel
        }
    };
}

pub(crate) fn zoom_in(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let new_width = width * 2;
    let new_height = height * 2;
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(new_width, new_height);

    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            output.put_pixel(x * 2, y * 2, *pixel);
        }
    }

    for x in (1..new_width - 1).step_by(2) {
        for y in (0..new_height).step_by(2) {
            output.put_pixel(
                x,
                y,
                interpole_pixel(output.get_pixel(x - 1, y), output.get_pixel(x + 1, y)),
            );
        }
    }

    for x in 0..new_width {
        for y in (1..new_height - 1).step_by(2) {
            output.put_pixel(
                x,
                y,
                interpole_pixel(output.get_pixel(x, y - 1), output.get_pixel(x, y + 1)),
            );
        }
    }
    output
}

pub(self) fn interpole_pixel(before_pixel: &Rgb<u8>, after_pixel: &Rgb<u8>) -> Rgb<u8> {
    let pixel_0 = (before_pixel[0] as i32 + after_pixel[0] as i32) / 2;
    let pixel_1 = (before_pixel[1] as i32 + after_pixel[1] as i32) / 2;
    let pixel_2 = (before_pixel[2] as i32 + after_pixel[2] as i32) / 2;
    Rgb([pixel_0 as u8, pixel_1 as u8, pixel_2 as u8])
}

pub(crate) fn zoom_out(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    sx: f32,
    sy: f32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let new_width = (width as f32 / sx) as u32;
    let new_height = (height as f32 / sy) as u32;
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(new_width, new_height);
    let num_of_itens = (sx * sy) as i32;

    for x in (0..=(width - sx as u32)).step_by(sx as usize) {
        for y in (0..=(height - sy as u32)).step_by(sy as usize) {
            output.put_pixel(
                x / sx as u32,
                y / sy as u32,
                reduce_pixel_area(sx, sy, image, height, width, x, y, num_of_itens),
            );
        }
    }
    output
}

fn reduce_pixel_area(
    sx: f32,
    sy: f32,
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    height: u32,
    width: u32,
    x: u32,
    y: u32,
    num_of_itens: i32,
) -> Rgb<u8> {
    let mut result = vec![];
    for i in 0..sx as u32 {
        for j in 0..sy as u32 {
            result.push(
                image
                    .get_pixel((x + i).min(width - 1), (y + j).min(height - 1))
                    .0
                    .map(|a| a as i32),
            );
        }
    }
    result
        .into_iter()
        .reduce(|pixel_a, pixel_b| {
            [
                pixel_a[0] + pixel_b[0],
                pixel_a[1] + pixel_b[1],
                pixel_a[2] + pixel_b[2],
            ]
        })
        .map(|accumulated_pixel| {
            Rgb([
                (accumulated_pixel[0] / num_of_itens) as u8,
                (accumulated_pixel[1] / num_of_itens) as u8,
                (accumulated_pixel[2] / num_of_itens) as u8,
            ])
        })
        .expect("Should be able to zoom out image")
}

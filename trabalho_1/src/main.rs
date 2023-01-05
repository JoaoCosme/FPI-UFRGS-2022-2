mod image_ops;
mod matrix_ops;
mod test;
mod kernel;
use image::GenericImageView;
const COLOR_NUMBER: usize = 256;
use fltk::{
    app,
    button::Button,
    dialog::{self, FileChooser, FileChooserType, FileDialog},
    frame::Frame,
    image::SharedImage,
    input::Input,
    prelude::*,
    window::Window,
};

const SAVED_FILE: &'static str = "./loaded_image.jpeg";
const COPIED_FILE: &'static str = "./copy.jpeg";
const HISTOGRAM: &'static str = "./histogram.jpeg";

fn main() {
    make_ui();
}

fn pick_file() {
    let mut file_chooser = FileChooser::new(
        ".",
        "*.{jpeg,jpg}",
        FileChooserType::Single,
        "Select a File!",
    );
    file_chooser.show();
    while file_chooser.shown() {
        app::wait();
    }
    let dynamic_image = image::open(file_chooser.value(0).expect("Should have choosen file"))
        .expect("Should open image");
    dynamic_image
        .save(SAVED_FILE)
        .expect("Should save opened image");
        dynamic_image
        .save(COPIED_FILE)
        .expect("Should save opened image");
}

fn make_ui() {
    pick_file();
    let img = image::open(SAVED_FILE).expect("Should open image");
    let (width, height) = img.dimensions();
    let window_width = (width + 100).max(500) as i32;
    let window_height = (height + 100).max(400) as i32;
    let width = width as i32;
    let height = height as i32;
    let app = app::App::default();
    let mut window = Window::new(0, 0, window_width, window_height + 50, "Base Image");
    let mut frame = Frame::new(20, 10, width, height, "");
    let mut image = SharedImage::load(SAVED_FILE).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    let mut but_equalize = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&frame, 0)
        .with_label("Equalize");
    let mut but_horizontal = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_equalize, 5)
        .with_label("Flip Horizontal");
    let mut but_vertical = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_horizontal, 5)
        .with_label("Flip Vertical");
    let mut but_gray = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_vertical, 5)
        .with_label("Gray Scale");
    let mut save_result = Button::default()
        .size_of(&but_equalize)
        .right_of(&but_gray, 5)
        .with_label("Save Result");
    let mut equalize_val = Input::default()
        .size_of(&but_equalize)
        .below_of(&but_equalize, 1);
    let mut but_bright = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_horizontal, 0)
        .with_label("Bright Up");
    let mut but_contrast = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_bright, 5)
        .with_label("Contrast Up");
    let mut but_negative = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_contrast, 5)
        .with_label("Negative");
    let mut but_histogram = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_negative, 5)
        .with_label("Histogram");
    let mut but_laplacian = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_histogram, 5)
        .with_label("LaPlacian");
    let mut but_gauss = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&equalize_val, 5)
        .with_label("Gaussian");
    let mut but_passa_alta = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_gauss, 5)
        .with_label("Passa Alta");
    let mut but_pw_hx = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_passa_alta, 5)
        .with_label("Prewitt Hx");
    let mut but_pw_hy = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_pw_hx, 5)
        .with_label("Prewitt Hy");
    let mut but_sobel_hx = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .below_of(&but_gauss, 5)
        .with_label("Sobel Hx");
    let mut but_sobel_hy: Button = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_sobel_hx, 5)
        .with_label("Sobel Hy");

        let mut but_reset: Button = Button::default()
        .with_size((window_width - 100) / 5, 20)
        .right_of(&but_sobel_hx, 5)
        .with_label("Reset");

    equalize_val.set_value("0");

    but_horizontal.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::horizontal_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    but_gray.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::make_gray_image(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    but_vertical.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::vertical_flip(&img)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    but_equalize.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::equalize_image(
            &img,
            equalize_val
                .value()
                .trim()
                .parse()
                .expect("Should have number!"),
        )
        .save(SAVED_FILE)
        .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    save_result.set_callback(move |_| {
        let img = image::open(SAVED_FILE).expect("Should open image");
        let mut save = FileDialog::new(dialog::FileDialogType::BrowseSaveFile);

        save.show();
        while Some(save.filename()).is_none() {
            app::wait();
        }
        img.save(save.filename())
            .expect("Should save image correctly");
    });
    but_bright.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, 1.0, 10.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    but_contrast.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, 0.25, 0.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });

    but_negative.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::apply_point_operation(&img, -1.0, 255.0)
            .save(SAVED_FILE)
            .expect("Should save image");
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });
    but_histogram.set_callback(move |_| {
        let img = image::open(SAVED_FILE)
            .expect("Should open image")
            .into_rgb8();
        image_ops::draw_histogram(
            &image_ops::make_histogram(&image_ops::make_gray_image(&img)),
            HISTOGRAM,
        );
        update_frame(img.width() as i32, img.height() as i32, HISTOGRAM);
    });


    but_laplacian.set_callback(move |_| {
        apply_kernel_to_image(kernel::LAPLACIAN,false);
    });

    but_gauss.set_callback(move |_| {
        apply_kernel_to_image(kernel::GAUSS,false);
    });

    but_passa_alta.set_callback(move |_| {
        apply_kernel_to_image(kernel::PASSA_ALTA,true);
    });

    but_pw_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel::PREWITT_HX,true);
    });

    but_pw_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel::PREWITT_HY,true);
    });

    but_sobel_hx.set_callback(move |_| {
        apply_kernel_to_image(kernel::SOBEL_HX,true);
    });

    but_sobel_hy.set_callback(move |_| {
        apply_kernel_to_image(kernel::SOBEL_HY,true);
    });
    but_reset.set_callback(move |_|{
        let img = image::open(COPIED_FILE)
        .expect("Should open image");
        img.save(SAVED_FILE).ok();
        update_frame(img.width() as i32, img.height() as i32, SAVED_FILE);
    });

    window.make_resizable(false);
    window.show();
    app.run().ok();
}

fn apply_kernel_to_image(kernel: [[f32; 3]; 3],should_clamp:bool) {
    let image = image::open(SAVED_FILE)
        .expect("Should open image")
        .into_rgb8();
    image_ops::apply_conv(kernel, &image, should_clamp)
        .save(SAVED_FILE)
        .expect("Should save image");
    update_frame(image.width() as i32, image.height() as i32, SAVED_FILE);
}

fn update_frame(width: i32, height: i32, file_path: &'static str) {
    let window_width = (width + 100).max(500) as i32;
    let window_height = (height).max(400) as i32;
    let width = width as i32;
    let height = height as i32;
    let mut window = Window::new(window_width, 0, window_width, window_height + 50, "Result");
    let mut frame = Frame::new(0, 0, width + 100, height, "").center_of_parent();
    let mut image = SharedImage::load(file_path).unwrap();
    image.scale(width, height, true, true);
    frame.set_image(Some(image));
    window.show();
}

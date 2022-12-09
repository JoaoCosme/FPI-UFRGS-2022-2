use gtk::Align;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Image;
use gtk::prelude::*;
use gtk::Application;

const APP_ID : &str = "fpi.trab1";
const IMAGE_PATH: &str = "./src/test_images/Gramado_22k.jpg";

fn main() {
    
    
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application){
    let button = Button::builder()
    .label("Press me!")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Start)
    .valign(Align::Start)
    .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello World!")
    });
    
    let window = ApplicationWindow::builder()
    .application(app)
    .title("FPI - Joao Cosme")
    .child(&button)
    .build();

    let image = Image::from_file(String::from(IMAGE_PATH));
    let window2 = ApplicationWindow::builder()
    .application(app)
    .title("FPI - Joao Cosme")
    .child(&image)
    .build();    
    app.add_window(&window2);
    app.add_window(&window);
    // window.present();
    window2.present();
}
use log::{info};
use dolrs::parser::{parse, ParseError};
use simple_logger::SimpleLogger;
use dolrs::document::DocumentEntry;
use minifb::{Window, WindowOptions, Key};

fn main() {
    SimpleLogger::new().init();

    info!("Starting dolrs document viewer");

    let mut buffer: Vec<u32> = vec![0; 640 * 480];
    /*let mut window = Window::new(
        "Document Viewer",
        640,
        480,
        WindowOptions::default()
    ).unwrap();*/

    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    /*while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i += 1;
        }

        window
            .update_with_buffer(&buffer, 640, 480)
            .unwrap();
    }*/


    // let entries = parse("Hello World! $CL$ $FG,RED$ What's up?");
    let entries = parse("Hello World! $CL$ $TX+CX,A=C,B=X$ What's up?");
    println!("{:?}", entries);
}

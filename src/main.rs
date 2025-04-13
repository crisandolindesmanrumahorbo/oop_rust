use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use oop::summary::{Summary, notify3};
use oop::{installment::CarInstallment, summary::NewsArticle};

fn main() {
    // There are several error handling:
    // 1. with match,  CarInstallment::new has '?' which throw an error
    // let car = match CarInstallment::new(String::from("x"), 2022, 0.09, 25000000, 100000000, 5) {
    //     Ok(car) => car,
    //     Err(e) => {
    //         print!("error: {}", e);
    //         panic!()
    //     }
    // };
    // car.monlty_installment();
    // 2. unwarp, will execute panic
    // CarInstallment::new(String::from("x"), 2022, 0.09, 25000000, 100000000, 5).unwrap();
    // 3. expect, will panic with message
    CarInstallment::new(String::from("baru"), 2022, 0.09, 25000000, 100000000, 5).expect("Error");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    notify3(&article);

    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind");

    loop {
        let (stream, _) = listener.accept().unwrap();
        std::thread::spawn(move || handle_client(stream));
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            let _string = String::from("ss");
            println!("{}", request);
            stream
                .write_all(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\nhello world!"
                        .as_bytes(),
                )
                .unwrap();
        }
        Err(_) => todo!(),
    };
}

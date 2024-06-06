use std::{
    fs::{ read_to_string},
    // io::{prelude::*,BufReader},
    // net::{TcpListener, TcpStream},
    // thread,
};
#[macro_use]
extern crate rocket;
// use distrivid::ThreadPool;
use rocket::response::content;
#[get("/heart")]
fn heart() -> content::RawHtml<String>{
    content::RawHtml(read_to_string("hello.html").unwrap())
}
#[get("/")]
fn index()->content::RawHtml<String>{
    content::RawHtml(read_to_string("index.html").unwrap())
}
#[get("/styles.css")]
fn style()->content::RawCss<String>{
    content::RawCss(read_to_string("styles.css").unwrap())
}
// #[get("/david")]
// fn pdf() -> PdfRes{
//     let response = Content(ContentType::PDF, file::fs::read("david.pdf"));
//     PdfRes::new("lalala")
// }
#[launch]
fn rocker() ->_{
    let figment = rocket::Config::figment()
        .merge(("port",5290))
        .merge(("address","192.168.43.26"));
    rocket::custom(figment).mount("/",routes![index,heart,style])
}
// fn main() {
//     let listener = TcpListener::bind("192.168.43.26:5290").expect("failed to bind 5290 port, please try to change one ");
//     let pool = ThreadPool::new(4);
//     for stream in listener.incoming(){
//         let stream = stream.expect("failed to establish connection, please check the connection limit of your OS");
        
//         pool.execute(||{
//             handle_connection(stream);
//         });
//     }
//     // for stream in listener.incoming(){
//     //     let stream = stream.unwrap();
//     //     thread::spawn(||{ 
//     //         handle_connection(stream);
//     //     });
        
//     // }
// }
// fn handle_connection(mut stream: TcpStream){
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line= buf_reader.lines().next().expect("no lines to split").expect("not an UTF-8 character");
//     if request_line == "GET /david.pdf HTTP/1.1"{
//         let contents = fs::read("david.pdf").expect("unable to read david.pdf");
//         let length = contents.len();
//         let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/pdf0\r\nContent-Length:{length}\r\n\r\n");
//         stream.write_all(response.as_bytes()).unwrap();
//         stream.write_all(contents.as_slice()).unwrap(); 
//         // let response = response.as_bytes();
//         return ;
//     }
//     let (status_line,filename) = if request_line == "GET / HTTP/1.1"{
//         ("HTTP/1.1 200 OK","hello.html")
//     }else if request_line == "GET /example.txt HTTP/1.1"{
//         ("HTTP/1.1 200 OK","example.txt")
//     }else{
//         ("HTTP/1.1 404 NOT FOUND","404.html")
//     };
//     let contents = fs::read_to_string(filename).expect(format!("failed to read {filename}, please check the file path").as_str());
//     let length = contents.len();
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
//     // println!("Request:{:#?}",http_request);
// }
use std::{
    env,
    io::Write,
    net::TcpListener,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let port = if let Some(arg) = env::args().nth(1) {
        arg
    } else if let Ok(var) = env::var("PORT") {
        var
    } else {
        "8080".to_string()
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).expect("failed to create listener");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("connection failed: {e}"),
            Ok(mut stream) => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("SystemTime set before unix epoch")
                    .as_secs()
                    .to_string();

                let length = timestamp.len();

                let response =
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{timestamp}");

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("write failed: {e}")
                }
            }
        }
    }
}

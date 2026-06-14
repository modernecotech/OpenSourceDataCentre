use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const STYLE_CSS: &str = include_str!("views/style.css");
const USER_HTML: &str = include_str!("views/user.html");
const OPERATOR_HTML: &str = include_str!("views/operator.html");
const RACK_IMAGE: &[u8] = include_bytes!("../../../docs/assets/rack-thermal-spine-cutaway.png");
const EXTERIOR_IMAGE: &[u8] =
    include_bytes!("../../../docs/assets/prefab-panel-datacentre-exterior-02.png");

fn main() -> std::io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8787".to_string());
    let listener = TcpListener::bind(&addr)?;

    println!("osdc-portal listening on http://{addr}");
    println!("tenant portal: http://{addr}/user");
    println!("operator console: http://{addr}/operator");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_connection(stream) {
                    eprintln!("request failed: {err}");
                }
            }
            Err(err) => eprintln!("connection failed: {err}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    let read = stream.read(&mut buffer)?;
    if read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..read]);
    let mut parts = request
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace();
    let method = parts.next().unwrap_or_default();
    let raw_path = parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or("/");

    let response = match (method, path) {
        ("GET", "/") => redirect("/user"),
        ("GET", "/user") => html(USER_HTML),
        ("GET", "/operator") => html(OPERATOR_HTML),
        ("GET", "/styles.css") => bytes("200 OK", "text/css; charset=utf-8", STYLE_CSS.as_bytes()),
        ("GET", "/assets/rack-thermal-spine-cutaway.png") => {
            bytes("200 OK", "image/png", RACK_IMAGE)
        }
        ("GET", "/assets/prefab-panel-datacentre-exterior-02.png") => {
            bytes("200 OK", "image/png", EXTERIOR_IMAGE)
        }
        ("GET", "/health") => bytes("200 OK", "text/plain; charset=utf-8", b"ok\n"),
        _ => bytes("404 Not Found", "text/plain; charset=utf-8", b"not found\n"),
    };

    stream.write_all(&response)?;
    stream.flush()
}

fn html(body: &str) -> Vec<u8> {
    bytes("200 OK", "text/html; charset=utf-8", body.as_bytes())
}

fn redirect(location: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    )
    .into_bytes()
}

fn bytes(status: &str, content_type: &str, body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    response
}

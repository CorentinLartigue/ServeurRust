use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::str;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).expect("Impossible de lier l'adresse");

    println!("Serveur démarré sur {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream);
            }
            Err(e) => {
                eprintln!("Erreur lors de la connexion: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = str::from_utf8(&buffer).unwrap_or("");

        if request.starts_with("GET /ping") {
            let headers = get_headers_from_request(request);

            let response_json = format!(
                "{{\"headers\": {}}}",
                format_headers_as_json(&headers)
            );

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                response_json
            );

            stream.write(response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
        }
    }
}

fn get_headers_from_request(request: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    let mut lines = request.lines().skip(1);
    for line in lines {
        if line.is_empty() {
            break; // Fin des headers
        }

        let mut parts = line.splitn(2, ": ");
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                headers.insert(key.to_string(), value.to_string());
            }
        }
    }

    headers
}

fn format_headers_as_json(headers: &HashMap<String, String>) -> String {
    let mut json_str = String::new();
    json_str.push('{');

    for (i, (key, value)) in headers.iter().enumerate() {
        if i > 0 {
            json_str.push(',');
        }
        json_str.push_str(&format!("\"{}\": \"{}\"", key, value));
    }

    json_str.push('}');
    json_str
}

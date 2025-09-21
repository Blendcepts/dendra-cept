// Removed unused 'use std::fs;'

use std::process::Command;
// 'Read' is only needed when you read the body, but the compiler warns if you don't.
// Let's keep it for now as the next step will use it.
use std::io::Read; 
use tiny_http::{Server, Response, Header, StatusCode};

// Embed the static HTML file into the binary
const HTML_CONTENT: &[u8] = include_bytes!("../static_ui.html");

fn main() {
    let server_address = "127.0.0.1:8000";
    let server = Server::http(server_address).expect("Failed to start server");
    println!("✅ Server started at http://{}", server_address);

    // Open the browser to the server address
    // This logic attempts to open the browser on Linux, macOS, and Windows.
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/C").arg("start").arg(format!("http://{}", server_address)).output();
    } else if cfg!(target_os = "macos") {
        let _ = Command::new("open").arg(format!("http://{}", server_address)).output();
    } else {
        let _ = Command::new("xdg-open").arg(format!("http://{}", server_address)).output();
    }

    for mut request in server.incoming_requests() { // 'request' must be mutable to read
        let url = request.url().to_string(); // Use to_string() for clean matching
        
        match (request.method(), url.as_str()) {
            // Serve the main HTML page
            (tiny_http::Method::Get, "/") => {
                let response = Response::from_data(HTML_CONTENT)
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
                let _ = request.respond(response);
            }
            // A placeholder endpoint to handle the processing request from the UI
            (tiny_http::Method::Post, "/process") => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).unwrap();
                println!("Received content: {}", content);
                
                // This is where you will implement the LLM processing and web scraping logic
                // For now, it just sends back a placeholder response.
                
                let mock_response = serde_json::json!({
                    "output": "# First Principles Analysis\n\nThis is a placeholder. The LLM will generate real content here once the logic is implemented.\n\n```mermaid\nflowchart TD\n    A[Input from UI] --> B{Process with Rust Backend}\n    B --> C[Call Local LLM]\n    C --> D[Generate Mermaid Diagram]\n    D --> E[Send back to UI]\n```"
                }).to_string();

                let response = Response::from_string(mock_response)
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                    .with_status_code(StatusCode(200));
                
                let _ = request.respond(response);
            }
            // Handle any other request with a 404
            _ => {
                // CORRECTED LINE: Use Response::empty() for a simple 404 response.
                let response = Response::empty(StatusCode(404));
                let _ = request.respond(response);
            }
        }
    }
}
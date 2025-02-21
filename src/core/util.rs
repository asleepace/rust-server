use crate::core::http;
use crate::core::mime::get_mime_type;
use crate::core::Request;
use std::f32::consts::PI;
use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, Write};

static HTTP_CRLF: &[u8] = b"\r\n";
static HTTP_VERSION: &[u8] = b"HTTP/1.1";
static HTML_NOT_FOUND: &str = "src/public/404.html";
static PUBLIC_DIR: &str = "src/public";

static INVALID_CHARS: [&str; 4] = ["..", "~", "\\", " "];

pub fn sanitize_path(path: &str) -> String {
    let trimmed = path.trim();
    INVALID_CHARS
        .iter()
        .fold(trimmed.to_string(), |acc, &c| acc.replace(c, ""))
        .replace("//", "/")
        .to_string()
}

/// Find Static File
/// Attempts to find a static file in the public directory.
/// Will sanitize the path and return a 404 if the file is not found.
pub fn find_static_file(uri: &str) -> String {
    let path = sanitize_path(uri);
    let file_path = match path.as_str() {
        "" | "/" => format!("{}/index.html", PUBLIC_DIR),
        path if !path.starts_with(PUBLIC_DIR) => format!("{}{}", PUBLIC_DIR, path),
        _ => path.to_string(),
    };
    fs::exists(&file_path)
        .map(|exists| if exists { &file_path } else { HTML_NOT_FOUND })
        .unwrap_or(HTML_NOT_FOUND)
        .to_string()
}

/// Copy Static File
/// Attempts to copy a static file from the public directory to the response.
///
pub fn copy_static_file(request: &mut Request, path: String) -> http::Response {
    println!("[main] static route: {}", path);
    let mut reader = File::open(&path)?;
    let mut bytes_sent = 0;
    let mut writer = request.stream();

    // write status code
    bytes_sent += writer.write(b"HTTP/1.1 200 OK")?;
    bytes_sent += writer.write(HTTP_CRLF)?;

    let mime = get_mime_type(&path);
    let size = reader.metadata()?.len();

    bytes_sent += writer.write(format!("Content-Type: {}", mime).as_bytes())?;
    bytes_sent += writer.write(HTTP_CRLF)?;
    bytes_sent += writer.write(format!("Content-Length: {}", size).as_bytes())?;

    // end headers
    bytes_sent += writer.write(HTTP_CRLF)?;
    bytes_sent += writer.write(HTTP_CRLF)?;

    if bytes_sent == 0 {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "Failed to write response",
        ));
    }

    // attempt to copy the file
    io::copy(&mut reader, &mut writer)?;

    // response
    Ok(200)
}

use std::io::{self, Read, Write};

use crate::core::get_mime_type;

pub type Bytes = Vec<u8>;
pub type Headers = Vec<(String, String)>;
pub type Response = Result<u8, io::Error>;

static HTTP_CRLF: &[u8] = b"\r\n";
static HTTP_VERSION: &[u8] = b"HTTP/1.1";

pub struct HttpResponse {
    status: u16,
    status_text: String,
    headers: Headers,
    body: Bytes,
}

// impl HttpResponse {
//     pub fn new(status: u16, status_text: &str, headers: Headers, body: Bytes) -> Self {
//         HttpResponse {
//             status,
//             status_text: status_text.to_string(),
//             headers,
//             body,
//         }
//     }
// }

/// Create a response from a file
pub fn static_file(file_path: &str) -> HttpResponse {
    let public_file_path = format!("src/public{}", file_path);

    let file = match std::fs::File::open(&public_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[http] {} error: {}", public_file_path, e);
            return HttpResponse {
                status: 404,
                status_text: "Not Found".to_string(),
                headers: vec![("Content-Type".to_string(), "text/html".to_string())],
                body: format!("File not found: {}", e).as_bytes().to_vec(),
            };
        }
    };

    let mime = get_mime_type(&public_file_path);
    let metadata = file.metadata().unwrap();
    let bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();

    HttpResponse {
        status: 200,
        status_text: "OK".to_string(),
        headers: vec![
            ("Content-Type".to_string(), mime),
            ("Content-Length".to_string(), metadata.len().to_string()),
        ],
        body: bytes,
    }
}

/// A trait for encoding HTTP responses.
/// Converts an HttpResponse into a byte buffer.
/// This is used to send the response over a TcpStream.
pub trait HttpCodec {
    fn encode_to(&self, buffer: &mut impl Write) -> io::Result<usize>;
    // fn encode(&self, buffer: &mut Bytes);
}

impl HttpCodec for HttpResponse {
    /// Encode the response into a writer.
    fn encode_to(&self, writer: &mut impl Write) -> io::Result<usize> {
        let mut written = 0;
        written += writer.write(HTTP_VERSION)?;

        // small stack frame for status code
        let status_str = self.status.to_string();
        let status_bytes = status_str.as_bytes();

        // write status code
        written += writer.write(status_bytes)?;
        written += writer.write(b" ")?;

        // write status text
        written += writer.write(self.status_text.as_bytes())?;
        written += writer.write(HTTP_CRLF)?;

        // write headers
        for (key, value) in &self.headers {
            written += writer.write(key.as_bytes())?;
            written += writer.write(b": ")?;
            written += writer.write(value.as_bytes())?;
            written += writer.write(HTTP_CRLF)?;
        }

        // write body
        written += writer.write(HTTP_CRLF)?;
        written += writer.write(&self.body)?;

        Ok(written)
    }

    // /// Encode the response into a byte buffer.
    // fn encode(&self, buffer: &mut Bytes) {
    //     // Pre-calculate capacity to avoid reallocations
    //     let headers_size = self
    //         .headers
    //         .iter()
    //         .map(|(k, v)| k.len() + v.len() + 4) // 4 for ": " and CRLF
    //         .sum::<usize>();

    //     let total_size = HTTP_VERSION.len() +
    //             4 + // status code + space
    //             self.status_text.len() +
    //             2 + // CRLF
    //             headers_size +
    //             2 + // Headers terminator CRLF
    //             self.body.len();

    //     buffer.reserve(total_size);

    //     // Now do the actual writing
    //     buffer.extend_from_slice(HTTP_VERSION);
    //     buffer.extend_from_slice(self.status.to_string().as_bytes());
    //     buffer.extend_from_slice(b" ");
    //     buffer.extend_from_slice(self.status_text.as_bytes());
    //     buffer.extend_from_slice(HTTP_CRLF);

    //     for (key, value) in &self.headers {
    //         buffer.extend_from_slice(key.as_bytes());
    //         buffer.extend_from_slice(b": ");
    //         buffer.extend_from_slice(value.as_bytes());
    //         buffer.extend_from_slice(HTTP_CRLF);
    //     }

    //     buffer.extend_from_slice(HTTP_CRLF);
    //     buffer.extend_from_slice(&self.body);
    // }
}

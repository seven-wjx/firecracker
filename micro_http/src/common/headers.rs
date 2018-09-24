use std::collections::HashMap;
use std::io::{Error as WriteError, Write};

use ascii::{COLON, CR, LF, SP};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Header {
    ContentLength,
    ContentType,
}

impl Header {
    fn raw(&self) -> &'static [u8] {
        match self {
            Header::ContentLength => b"Content-Length",
            Header::ContentType => b"Content-Type",
        }
    }
}

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<Header, String>,
}

impl Headers {
    pub fn default() -> Headers {
        return Headers {
            headers: HashMap::new(),
        };
    }

    pub fn add(&mut self, header: Header, value: String) {
        self.headers.insert(header, value);
    }

    pub fn write_all<T: Write>(&self, buf: &mut T) -> Result<(), WriteError> {
        for (key, val) in &self.headers {
            buf.write_all(key.raw())?;
            buf.write_all(&[COLON, SP])?;
            buf.write_all(&val.as_bytes())?;
            buf.write_all(&[CR, LF])?;
        }

        // The header section ends with a CRLF.
        buf.write_all(&[CR, LF])?;

        Ok(())
    }
}

pub enum MediaType {
    PlainText,
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::PlainText => "text/plain",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert!(Headers::default().headers == HashMap::new());
    }

    #[test]
    fn test_add_headers() {
        let mut headers = Headers::default();

        headers.add(Header::ContentType, "text/plain".to_string());
        headers.add(Header::ContentLength, "120".to_string());

        assert!(headers.headers.contains_key(&Header::ContentType));
        assert_eq!(
            headers.headers.get(&Header::ContentType).unwrap(),
            &"text/plain".to_string()
        );
        assert!(headers.headers.contains_key(&Header::ContentLength));
        assert_eq!(
            headers.headers.get(&Header::ContentLength).unwrap(),
            &"120".to_string()
        );

        // Test that adding a Header with the same key, updates the value.
        headers.add(Header::ContentLength, "130".to_string());
        assert_eq!(
            headers.headers.get(&Header::ContentLength).unwrap(),
            &"130".to_string()
        );
    }

    #[test]
    fn test_write_headers() {
        // Test write empty headers object
        {
            let headers = Headers::default();
            let mut response_buf: [u8; 2] = [0_u8; 2];

            assert!(headers.write_all(&mut response_buf.as_mut()).is_ok());
            assert_eq!(response_buf, [CR, LF]);
        }

        // Test write with one header
        {
            let mut headers = Headers::default();
            headers.add(Header::ContentLength, "10".to_string());
            let expected: &'static [u8] = b"Content-Length: 10\r\n\r\n";
            let mut response_buf = [0_u8; 22];

            assert!(headers.write_all(&mut response_buf.as_mut()).is_ok());
            assert_eq!(expected, response_buf.as_ref());
        }
    }
}
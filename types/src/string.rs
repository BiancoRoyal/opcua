//! Contains the implementation of `UAString`.

use std::{
    fmt, io::{Read, Write},
};

use crate::{
    encoding::{BinaryEncoder, DecodingLimits, EncodingResult, process_decode_io_result, process_encode_io_result, write_i32},
    status_codes::StatusCode,
};

/// To avoid naming conflict hell, the OPC UA String type is typed `UAString` so it does not collide
/// with the Rust `String`.
///
/// A string contains UTF-8 encoded characters or a null value. A null value is distinct from
/// being an empty string so internally, the code maintains that distinction by holding the value
/// as an `Option<String>`.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct UAString {
    value: Option<String>,
}

impl fmt::Display for UAString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref value) = self.value {
            write!(f, "{}", value)
        } else {
            write!(f, "[null]")
        }
    }
}

impl BinaryEncoder<UAString> for UAString {
    fn byte_len(&self) -> usize {
        // Length plus the actual string length in bytes for a non-null string.
        4 + if self.value.is_none() { 0 } else { self.value.as_ref().unwrap().len() }
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        // Strings are encoded as UTF8 chars preceded by an Int32 length. A -1 indicates a null string
        if self.value.is_none() {
            write_i32(stream, -1)
        } else {
            let value = self.value.as_ref().unwrap();
            let mut size: usize = 0;
            size += write_i32(stream, value.len() as i32)?;
            let buf = value.as_bytes();
            size += process_encode_io_result(stream.write(&buf))?;
            assert_eq!(size, self.byte_len());
            Ok(size)
        }
    }

    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let len = i32::decode(stream, decoding_limits)?;
        // Null string?
        if len == -1 {
            Ok(UAString::null())
        } else if len < -1 {
            error!("String buf length is a negative number {}", len);
            Err(StatusCode::BadDecodingError)
        } else if len as usize > decoding_limits.max_string_length {
            error!("String buf length {} exceeds decoding limit {}", len, decoding_limits.max_string_length);
            Err(StatusCode::BadDecodingError)
        } else {
            // Create a buffer filled with zeroes and read the string over the top
            let mut buf = vec![0u8; len as usize];
            process_decode_io_result(stream.read_exact(&mut buf))?;
            let value = String::from_utf8(buf)
                .map_err(|err| {
                    trace!("Decoded string was not valid UTF-8 - {}", err.to_string());
                    StatusCode::BadDecodingError
                })?;
            Ok(UAString::from(value))
        }
    }
}

impl From<UAString> for String {
    fn from(value: UAString) -> Self {
        value.as_ref().to_string()
    }
}

impl AsRef<str> for UAString {
    fn as_ref(&self) -> &str {
        if self.is_null() { "" } else { self.value.as_ref().unwrap() }
    }
}

impl<'a> From<&'a str> for UAString {
    fn from(value: &'a str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<&String> for UAString {
    fn from(value: &String) -> Self {
        UAString { value: Some(value.clone()) }
    }
}

impl From<String> for UAString {
    fn from(value: String) -> Self {
        UAString { value: Some(value) }
    }
}

impl Default for UAString {
    fn default() -> Self {
        UAString::null()
    }
}

impl UAString {
    pub fn value(&self) -> &Option<String> {
        &self.value
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }

    /// Returns true if the string is null or empty, false otherwise
    pub fn is_empty(&self) -> bool {
        if self.value.is_none() { true } else { self.value.as_ref().unwrap().is_empty() }
    }

    /// Returns the length of the string in bytes or -1 for null.
    pub fn len(&self) -> isize {
        if self.value.is_none() { -1 } else { self.value.as_ref().unwrap().len() as isize }
    }

    /// Create a null string (not the same as an empty string).
    pub fn null() -> UAString {
        UAString { value: None }
    }

    /// Test if the string is null.
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
}

/// An XML element.
pub type XmlElement = UAString;
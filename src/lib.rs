use std::ffi::CString;
use std::io::{self, Error, ErrorKind};
use std::ptr::NonNull;

#[allow(non_upper_case_globals, non_camel_case_types, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug)]
pub struct Connection {
    // store the connection pointer as NonNull to indicate it's never null
    conn: NonNull<bindings::cups_connection>,
}

// implement Drop to automatically free resources when the connection is dropped
impl Drop for Connection {
    fn drop(&mut self) {
        // we know the pointer is valid because of NonNull
        unsafe {
            bindings::cups_free(self.conn.as_ptr());
        }
    }
}


impl Connection {
    /// connect to a printer at the specified URL.
    pub fn connect(url: &str) -> io::Result<Self> {
        let c_url = CString::new(url)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "URL contains null bytes"))?;
        
        let mut status = 0;
        
        // we are passing a valid c string pointer and a valid status pointer
        let conn_ptr = unsafe {
            bindings::cups_connect(c_url.as_ptr(), &mut status)
        };
        
        // status code matching
        if conn_ptr.is_null() {
            return match status {
                -1 => Err(Error::new(ErrorKind::InvalidInput, "Invalid URL")),
                0 => Err(Error::new(ErrorKind::Other, "Unknown connection error")),
                _ => Err(Error::from_raw_os_error(status)),
            };
        }
        
        // we have already checked that conn_ptr is not null
        let conn = unsafe { NonNull::new_unchecked(conn_ptr) };
        
        Ok(Self { conn })
    }
    
    /// print a document.
    pub fn print(&self, data: &str) -> io::Result<()> {
        let c_data = CString::new(data)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Document data contains null bytes"))?;
        
        // we know the connection pointer is valid, and we are passing a valid C string
        let result = unsafe {
            bindings::cups_print(self.conn.as_ptr(), c_data.as_ptr())
        };
        
        // handle result
        if result == 0 {
            Ok(())
        } else {
            Err(Error::from_raw_os_error(result))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_connect_valid_url() {
        let result = Connection::connect("http://localhost:631");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_connect_invalid_url() {
        let result = Connection::connect("ftp://localhost");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }
    
    #[test]
    fn test_connect_bad_printer() {
        let result = Connection::connect("http://bad-printer/");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_print() {
        if let Ok(conn) = Connection::connect("http://localhost:631") {
            // so we just test that calling it doesn't panic
            let _ = conn.print("Hello, world!");
        }
    }
}
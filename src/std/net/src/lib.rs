use std::{
    ffi::CStr,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr, slice,
};

/// Represents a TCP server.
type NSTDTCPServer = *mut c_void;

/// Represents a TCP stream.
type NSTDTCPStream = *mut c_void;

/// Creates a TCP server bound to the given address. Call `nstd_std_net_tcp_server_close` to free
/// memory allocated by this function and close the server.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDTCPServer server` - The TCP server, null on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_server_bind(addr: *const c_char) -> NSTDTCPServer {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match TcpListener::bind(addr) {
            Ok(server) => Box::into_raw(Box::new(server)) as NSTDTCPServer,
            _ => ptr::null_mut(),
        },
        _ => ptr::null_mut(),
    }
}

/// Accepts a connection on the TCP server. Call `nstd_std_net_tcp_stream_close` to free memory
/// allocated by this function and close the connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
/// Returns: `NSTDTCPStream client` - The server<=>client stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_server_accept(server: NSTDTCPServer) -> NSTDTCPStream {
    let server = &*(server as *mut TcpListener);
    match server.accept() {
        Ok(c) => Box::into_raw(Box::new(c.0)) as NSTDTCPStream,
        _ => ptr::null_mut(),
    }
}

/// Closes a TCP server and frees memory allocated by `nstd_std_net_tcp_server_bind`.
/// Parameters:
///     `NSTDTCPServer *server` - Pointer to the server.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_server_close(server: *mut NSTDTCPServer) {
    Box::from_raw(*server as *mut TcpListener);
    *server = ptr::null_mut();
}

/// Connects a TCP stream to a server.
/// Parameters:
///     `const char *const addr` - The address to connect to.
/// Returns: `NSTDTCPStream client` - The TCP stream connected to the server.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_stream_connect(addr: *const c_char) -> NSTDTCPStream {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match TcpStream::connect(addr) {
            Ok(client) => Box::into_raw(Box::new(client)) as NSTDTCPStream,
            _ => ptr::null_mut(),
        },
        _ => ptr::null_mut(),
    }
}

/// Reads data from a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `NSTDSize *size` - Returns as the number of bytes read.
/// Returns: `NSTDByte *bytes` - The bytes read from the stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_stream_read(
    stream: NSTDTCPStream,
    size: *mut usize,
) -> *mut c_uchar {
    let mut stream = BufReader::new(&mut *(stream as *mut TcpStream));
    match stream.fill_buf() {
        Ok(bytes) => {
            *size = bytes.len();
            let raw = Box::into_raw(bytes.to_vec().into_boxed_slice()) as *mut c_uchar;
            stream.consume(*size);
            raw
        }
        _ => ptr::null_mut(),
    }
}

/// Writes data to a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `const NSTDByte *const bytes` - The bytes to write.
///     `const NSTDSize size` - Number of bytes to write.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_stream_write(
    stream: NSTDTCPStream,
    bytes: *const c_uchar,
    size: usize,
) -> c_int {
    let stream = &mut *(stream as *mut TcpStream);
    match stream.write_all(slice::from_raw_parts(bytes, size)) {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Closes and frees memory of a TCP stream.
/// Parameters:
///     `NSTDTCPStream *stream` - Pointer to the TCP stream.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_tcp_stream_close(stream: *mut NSTDTCPStream) {
    Box::from_raw(*stream as *mut TcpStream);
    *stream = ptr::null_mut();
}

/// Frees bytes allocated by any of the `nstd_std_net_*` functions.
/// parameters:
///     `NSTDByte **bytes` - Pointer to the bytes to free.
///     `const NSTDSize size` - Number of bytes.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_free_bytes(bytes: *mut *mut c_uchar, size: usize) {
    Box::from_raw(slice::from_raw_parts_mut(*bytes, size) as *mut [u8]);
    *bytes = ptr::null_mut();
}

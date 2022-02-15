use crate::{
    collections::vec::NSTDVec,
    core::{def::NSTDErrorCode, str::NSTDStr},
};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    ptr::addr_of_mut,
};

/// Represents a TCP server.
pub type NSTDTCPServer = *mut TcpListener;

/// Represents a TCP stream.
pub type NSTDTCPStream = *mut TcpStream;

/// Creates a TCP server bound to the given address. Call `nstd_net_tcp_server_close` to free
/// memory allocated by this function and close the server.
/// Parameters:
///     `const NSTDStr *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDTCPServer server` - The TCP server, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_server_bind(addr: &NSTDStr) -> NSTDTCPServer {
    match std::str::from_utf8(addr.bytes.as_byte_slice()) {
        Ok(addr) => match TcpListener::bind(addr) {
            Ok(server) => Box::into_raw(Box::new(server)),
            _ => std::ptr::null_mut(),
        },
        _ => std::ptr::null_mut(),
    }
}

/// Accepts a connection on the TCP server. Call `nstd_net_tcp_stream_close` to free memory
/// allocated by this function and close the connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
/// Returns: `NSTDTCPStream client` - The server<=>client stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_server_accept(server: NSTDTCPServer) -> NSTDTCPStream {
    match (*server).accept() {
        Ok(c) => Box::into_raw(Box::new(c.0)),
        _ => std::ptr::null_mut(),
    }
}

/// Accepts all incoming connect requests, calling `callback` for each connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
///     `void(*callback)(NSTDTCPStream)` - The callback function when a connection is made.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_server_accept_all(
    server: NSTDTCPServer,
    callback: extern "C" fn(NSTDTCPStream),
) {
    for client in (*server).incoming() {
        match client {
            Ok(mut client) => callback(addr_of_mut!(client)),
            _ => (),
        }
    }
}

/// Closes a TCP server and frees memory allocated by `nstd_net_tcp_server_bind`.
/// Parameters:
///     `NSTDTCPServer *const server` - Pointer to the server.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_server_close(server: *mut NSTDTCPServer) {
    Box::from_raw(*server);
    *server = std::ptr::null_mut();
}

/// Connects a TCP stream to a server.
/// Parameters:
///     `const NSTDStr *const addr` - The address to connect to.
/// Returns: `NSTDTCPStream client` - The TCP stream connected to the server.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_stream_connect(addr: &NSTDStr) -> NSTDTCPStream {
    match std::str::from_utf8(addr.bytes.as_byte_slice()) {
        Ok(addr) => match TcpStream::connect(addr) {
            Ok(client) => Box::into_raw(Box::new(client)),
            _ => std::ptr::null_mut(),
        },
        _ => std::ptr::null_mut(),
    }
}

/// Reads data from a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
/// Returns: `NSTDVec bytes` - The bytes read from the stream.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_stream_read(stream: NSTDTCPStream) -> NSTDVec {
    let mut stream = BufReader::new(&mut *stream);
    match stream.fill_buf() {
        Ok(bytes) => NSTDVec::from(bytes),
        _ => {
            let null_slice = crate::core::slice::nstd_core_slice_new(0, 0, std::ptr::null_mut());
            crate::collections::vec::nstd_collections_vec_from_existing(0, &null_slice)
        }
    }
}

/// Writes data to a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `const NSTDByte *const bytes` - The bytes to write.
///     `const NSTDUSize size` - Number of bytes to write.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_stream_write(
    stream: NSTDTCPStream,
    bytes: *const u8,
    size: usize,
) -> NSTDErrorCode {
    match (*stream).write_all(std::slice::from_raw_parts(bytes, size)) {
        Ok(_) => 0,
        _ => 1,
    }
}

/// Closes and frees memory of a TCP stream.
/// Parameters:
///     `NSTDTCPStream *const stream` - Pointer to the TCP stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_tcp_stream_close(stream: *mut NSTDTCPStream) {
    Box::from_raw(*stream);
    *stream = std::ptr::null_mut();
}
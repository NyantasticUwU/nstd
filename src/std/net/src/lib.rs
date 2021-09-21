use std::{
    ffi::{CStr, CString},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr, slice,
};

/// Represents a TCP server.
type NSTDTCPServer = *mut c_void;

/// Represents a TCP stream.
type NSTDTCPStream = *mut c_void;

/// Represents a UDP socket.
type NSTDUDPSocket = *mut c_void;

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

/// Creates a UDP socket bound to the given address. Call `nstd_std_net_udp_socket_close` to free
/// memory allocated by this function and close the socket.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDUDPSocket socket` - The UDP socket, null on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_bind(addr: *const c_char) -> NSTDUDPSocket {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match UdpSocket::bind(addr) {
            Ok(socket) => Box::into_raw(Box::new(socket)) as NSTDUDPSocket,
            _ => ptr::null_mut(),
        },
        _ => ptr::null_mut(),
    }
}

/// Connects a UDP socket to a remote address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to connect.
///     `const char *const addr` - The remote address to connect to.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_connect(
    socket: NSTDUDPSocket,
    addr: *const c_char,
) -> c_int {
    let socket = &*(socket as *mut UdpSocket);
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match socket.connect(addr) {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Receives bytes sent from the connected address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes on.
///     `const NSTDSize num` - Number of bytes to receive.
///     `NSTDSize *size` - Returns as actual number of bytes received.
/// Returns: `NSTDByte *bytes` - The bytes received.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_receive(
    socket: NSTDUDPSocket,
    num: usize,
    size: *mut usize,
) -> *mut c_uchar {
    let socket = &*(socket as *mut UdpSocket);
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match socket.recv(&mut buf) {
        Ok(recv_size) => {
            *size = recv_size;
            Box::into_raw(buf.into_boxed_slice()) as *mut c_uchar
        }
        _ => ptr::null_mut(),
    }
}

/// Receives bytes sent to a UDP socket.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes from.
///     `const NSTDSize num` - Number of bytes to receive.
///     `NSTDSize *size` - Returns as actual number of bytes received.
///     `char **ip` - Returns as the socket IP address the bytes came from.
/// Returns: `NSTDByte *bytes` - The bytes received.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_receive_from(
    socket: NSTDUDPSocket,
    num: usize,
    size: *mut usize,
    ip: *mut *mut c_char,
) -> *mut c_uchar {
    let socket = &*(socket as *mut UdpSocket);
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match socket.recv_from(&mut buf) {
        Ok((recv_size, recv_ip)) => {
            *size = recv_size;
            let mut ipv = recv_ip.to_string().into_bytes();
            ipv.push(0);
            *ip = CString::from_vec_unchecked(ipv).into_raw();
            Box::into_raw(buf.into_boxed_slice()) as *mut c_uchar
        }
        _ => ptr::null_mut(),
    }
}

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDByte *const bytes` - The bytes to send.
///     `const NSTDSize num` - Number of bytes to send.
///     `NSTDSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_send(
    socket: NSTDUDPSocket,
    bytes: *const c_uchar,
    num: usize,
    size: *mut usize,
) -> c_int {
    let socket = &*(socket as *mut UdpSocket);
    match socket.send(slice::from_raw_parts(bytes, num)) {
        Ok(sent) => {
            *size = sent;
            0
        }
        _ => 1,
    }
}

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `NSTDUDPSocket socket` - The UDP socket.
///     `const char *const addr` - The address to send the bytes to.
///     `const NSTDByte *const bytes` - The bytes to send.
///     `const NSTDSize num` - Number of bytes to send.
///     `NSTDSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_send_to(
    socket: NSTDUDPSocket,
    addr: *const c_char,
    bytes: *const c_uchar,
    num: usize,
    size: *mut usize,
) -> c_int {
    let socket = &*(socket as *mut UdpSocket);
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match socket.send_to(slice::from_raw_parts(bytes, num), addr) {
            Ok(sent) => {
                *size = sent;
                0
            }
            _ => 1,
        },
        _ => 1,
    }
}

/// Closes and frees memory of a UDP socket.
/// Parameters:
///     `NSTDUDPSocket *socket` - Pointer to the UDP socket.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_udp_socket_close(socket: *mut NSTDUDPSocket) {
    Box::from_raw(*socket as *mut UdpSocket);
    *socket = ptr::null_mut();
}

/// Deallocates memory where an IP address string is allocated.
/// Parameters:
///     `char **ip` - The IP address.
#[no_mangle]
pub unsafe extern "C" fn nstd_std_net_free_ip(ip: *mut *mut c_char) {
    CString::from_raw(*ip);
    *ip = ptr::null_mut();
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

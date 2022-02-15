use std::{
    ffi::{CStr, CString},
    net::UdpSocket,
    os::raw::{c_char, c_int, c_uchar},
};

/// Represents a UDP socket.
pub type NSTDUDPSocket = *mut UdpSocket;

/// Creates a UDP socket bound to the given address. Call `nstd_net_udp_socket_close` to free
/// memory allocated by this function and close the socket.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDUDPSocket socket` - The UDP socket, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_bind(addr: *const c_char) -> NSTDUDPSocket {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match UdpSocket::bind(addr) {
            Ok(socket) => Box::into_raw(Box::new(socket)),
            _ => std::ptr::null_mut(),
        },
        _ => std::ptr::null_mut(),
    }
}

/// Connects a UDP socket to a remote address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to connect.
///     `const char *const addr` - The remote address to connect to.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_connect(
    socket: NSTDUDPSocket,
    addr: *const c_char,
) -> c_int {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match (*socket).connect(addr) {
            Ok(_) => 0,
            _ => 1,
        },
        _ => 1,
    }
}

/// Receives bytes sent from the connected address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes on.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDUSize *size` - Returns as actual number of bytes received.
/// Returns: `NSTDByte *bytes` - The bytes received.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_receive(
    socket: NSTDUDPSocket,
    num: usize,
    size: *mut usize,
) -> *mut c_uchar {
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match (*socket).recv(&mut buf) {
        Ok(recv_size) => {
            *size = recv_size;
            Box::into_raw(buf.into_boxed_slice()) as *mut c_uchar
        }
        _ => std::ptr::null_mut(),
    }
}

/// Receives bytes sent to a UDP socket.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes from.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDUSize *size` - Returns as actual number of bytes received.
///     `char **ip` - Returns as the socket IP address the bytes came from.
/// Returns: `NSTDByte *bytes` - The bytes received.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_receive_from(
    socket: NSTDUDPSocket,
    num: usize,
    size: *mut usize,
    ip: *mut *mut c_char,
) -> *mut c_uchar {
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match (*socket).recv_from(&mut buf) {
        Ok((recv_size, recv_ip)) => {
            *size = recv_size;
            let mut ipv = recv_ip.to_string().into_bytes();
            ipv.push(0);
            *ip = CString::from_vec_unchecked(ipv).into_raw();
            Box::into_raw(buf.into_boxed_slice()) as *mut c_uchar
        }
        _ => std::ptr::null_mut(),
    }
}

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDByte *const bytes` - The bytes to send.
///     `const NSTDUSize num` - Number of bytes to send.
///     `NSTDUSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_send(
    socket: NSTDUDPSocket,
    bytes: *const c_uchar,
    num: usize,
    size: *mut usize,
) -> c_int {
    match (*socket).send(std::slice::from_raw_parts(bytes, num)) {
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
///     `const NSTDUSize num` - Number of bytes to send.
///     `NSTDUSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_send_to(
    socket: NSTDUDPSocket,
    addr: *const c_char,
    bytes: *const c_uchar,
    num: usize,
    size: *mut usize,
) -> c_int {
    match CStr::from_ptr(addr).to_str() {
        Ok(addr) => match (*socket).send_to(std::slice::from_raw_parts(bytes, num), addr) {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_close(socket: *mut NSTDUDPSocket) {
    Box::from_raw(*socket);
    *socket = std::ptr::null_mut();
}

/// Deallocates memory where an IP address string is allocated.
/// Parameters:
///     `char **ip` - The IP address.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_free_ip(ip: *mut *mut c_char) {
    drop(CString::from_raw(*ip));
    *ip = std::ptr::null_mut();
}

/// Frees bytes allocated by any of the `nstd_net_*` functions.
/// parameters:
///     `NSTDByte **bytes` - Pointer to the bytes to free.
///     `const NSTDUSize size` - Number of bytes.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_free_bytes(bytes: *mut *mut c_uchar, size: usize) {
    Box::from_raw(std::slice::from_raw_parts_mut(*bytes, size) as *mut [u8]);
    *bytes = std::ptr::null_mut();
}

use crate::{
    core::{def::NSTDErrorCode, slice::NSTDSlice, str::NSTDStr},
    string::NSTDString,
    vec::NSTDVec,
};
use std::net::UdpSocket;

/// Represents a UDP socket.
pub type NSTDUDPSocket = *mut UdpSocket;

/// Creates a UDP socket bound to the given address. Call `nstd_net_udp_socket_close` to free
/// memory allocated by this function and close the socket.
/// Parameters:
///     `const NSTDStr *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDUDPSocket socket` - The UDP socket, null on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_bind(addr: &NSTDStr) -> NSTDUDPSocket {
    if let Ok(addr) = std::str::from_utf8(addr.bytes.as_byte_slice()) {
        if let Ok(socket) = UdpSocket::bind(addr) {
            return Box::into_raw(Box::new(socket));
        }
    }
    std::ptr::null_mut()
}

/// Connects a UDP socket to a remote address.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to connect.
///     `const NSTDStr *const addr` - The remote address to connect to.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_connect(
    socket: NSTDUDPSocket,
    addr: &NSTDStr,
) -> NSTDErrorCode {
    if let Ok(addr) = std::str::from_utf8(addr.bytes.as_byte_slice()) {
        if (*socket).connect(addr).is_ok() {
            return 0;
        }
    }
    1
}

/// Receives bytes sent from the connected address.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to receive bytes on.
///     `const NSTDUSize num` - Number of bytes to receive.
/// Returns: `NSTDVec bytes` - The bytes received.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_receive(socket: NSTDUDPSocket, num: usize) -> NSTDVec {
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match (*socket).recv(&mut buf) {
        Ok(_) => NSTDVec::from(buf.as_slice()),
        _ => null_vec(),
    }
}

/// Receives bytes sent to a UDP socket.
/// NOTE: This creates a new `NSTDString` so make sure `ip` is freed before using this function.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to receive bytes from.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDString *const ip` - Returns as the socket IP address the bytes came from.
/// Returns: `NSTDVec bytes` - The bytes received.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_receive_from(
    socket: NSTDUDPSocket,
    num: usize,
    ip: &mut NSTDString,
) -> NSTDVec {
    let mut buf = Vec::new();
    buf.resize(num, 0);
    match (*socket).recv_from(&mut buf) {
        Ok((_, recv_ip)) => {
            *ip = NSTDString::from(recv_ip.to_string().as_bytes());
            NSTDVec::from(buf.as_slice())
        }
        _ => null_vec(),
    }
}

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `const NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDSlice *const bytes` - The bytes to send.
///     `NSTDUSize *const size` - Returns as number of bytes actually sent.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_send(
    socket: NSTDUDPSocket,
    bytes: &NSTDSlice,
    size: *mut usize,
) -> NSTDErrorCode {
    if let Ok(sent) = (*socket).send(bytes.as_byte_slice()) {
        *size = sent;
        return 0;
    }
    1
}

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `const NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDStr *const addr` - The address to send the bytes to.
///     `const NSTDSlice *const bytes` - The bytes to send.
///     `NSTDUSize *const size` - Returns as number of bytes actually sent.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_send_to(
    socket: NSTDUDPSocket,
    addr: &NSTDStr,
    bytes: &NSTDSlice,
    size: *mut usize,
) -> NSTDErrorCode {
    if let Ok(addr) = std::str::from_utf8(addr.bytes.as_byte_slice()) {
        if let Ok(sent) = (*socket).send_to(bytes.as_byte_slice(), addr) {
            *size = sent;
            return 0;
        }
    }
    1
}

/// Closes and frees memory of a UDP socket.
/// Parameters:
///     `NSTDUDPSocket *const socket` - Pointer to the UDP socket.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_net_udp_socket_close(socket: *mut NSTDUDPSocket) {
    Box::from_raw(*socket);
    *socket = std::ptr::null_mut();
}

/// Creates a null vector.
#[inline]
unsafe fn null_vec() -> NSTDVec {
    let null_slice = crate::core::slice::nstd_core_slice_new(0, 0, std::ptr::null_mut());
    crate::vec::nstd_vec_from_existing(0, &null_slice)
}

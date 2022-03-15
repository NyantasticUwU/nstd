#ifndef NSTD_NET_UDP_H_INCLUDED
#define NSTD_NET_UDP_H_INCLUDED
#include "../core/def.h"
#include "../core/slice.h"
#include "../core/str.h"
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"
NSTDCPPSTART

/// Represents a UDP socket.
typedef NSTDAny NSTDUDPSocket;

/// Creates a UDP socket bound to the given address. Call `nstd_net_udp_socket_close` to free
/// memory allocated by this function and close the socket.
/// Parameters:
///     `const NSTDStr *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDUDPSocket socket` - The UDP socket, null on error.
NSTDAPI NSTDUDPSocket nstd_net_udp_socket_bind(const NSTDStr *const addr);

/// Connects a UDP socket to a remote address.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to connect.
///     `const NSTDStr *const addr` - The remote address to connect to.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_net_udp_socket_connect(
    const NSTDUDPSocket socket,
    const NSTDStr *const addr);

/// Receives bytes sent from the connected address.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to receive bytes on.
///     `const NSTDUSize num` - Number of bytes to receive.
/// Returns: `NSTDVec bytes` - The bytes received.
NSTDAPI NSTDVec nstd_net_udp_socket_receive(const NSTDUDPSocket socket, const NSTDUSize num);

/// Receives bytes sent to a UDP socket.
/// NOTE: This creates a new `NSTDString` so make sure `ip` is freed before using this function.
/// Parameters:
///     `const NSTDUDPSocket socket` - The socket to receive bytes from.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDString *const ip` - Returns as the socket IP address the bytes came from.
/// Returns: `NSTDVec bytes` - The bytes received.
NSTDAPI NSTDVec nstd_net_udp_socket_receive_from(
    const NSTDUDPSocket socket,
    const NSTDUSize num,
    NSTDString *const ip);

/// Sends bytes from a UDP socket to it's connected address.
/// Parameters:
///     `const NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDSlice *const bytes` - The bytes to send.
///     `NSTDUSize *const size` - Returns as number of bytes actually sent.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_net_udp_socket_send(
    const NSTDUDPSocket socket,
    const NSTDSlice *const bytes,
    NSTDUSize *const size);

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `const NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDStr *const addr` - The address to send the bytes to.
///     `const NSTDSlice *const bytes` - The bytes to send.
///     `NSTDUSize *const size` - Returns as number of bytes actually sent.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_net_udp_socket_send_to(
    const NSTDUDPSocket socket,
    const NSTDStr *const addr,
    const NSTDSlice *const bytes,
    NSTDUSize *const size);

/// Closes and frees memory of a UDP socket.
/// Parameters:
///     `NSTDUDPSocket *const socket` - Pointer to the UDP socket.
NSTDAPI void nstd_net_udp_socket_close(NSTDUDPSocket *const socket);

NSTDCPPEND
#endif

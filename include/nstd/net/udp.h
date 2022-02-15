#ifndef NSTD_NET_UDP_H_INCLUDED
#define NSTD_NET_UDP_H_INCLUDED
#include "../core/def.h"
#include "../core/str.h"
#include "../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

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
///     `NSTDUDPSocket socket` - The socket to connect.
///     `const NSTDStr *const addr` - The remote address to connect to.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_net_udp_socket_connect(NSTDUDPSocket socket, const NSTDStr *const addr);

/// Receives bytes sent from the connected address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes on.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDUSize *size` - Returns as actual number of bytes received.
/// Returns: `NSTDByte *bytes` - The bytes received.
NSTDAPI NSTDByte *nstd_net_udp_socket_receive(
    NSTDUDPSocket socket,
    const NSTDUSize num,
    NSTDUSize *size);

/// Receives bytes sent to a UDP socket.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to receive bytes from.
///     `const NSTDUSize num` - Number of bytes to receive.
///     `NSTDUSize *size` - Returns as actual number of bytes received.
///     `char **ip` - Returns as the socket IP address the bytes came from.
/// Returns: `NSTDByte *bytes` - The bytes received.
NSTDAPI NSTDByte *nstd_net_udp_socket_receive_from(
    NSTDUDPSocket socket,
    const NSTDUSize num,
    NSTDUSize *size,
    char **ip);

/// Sends bytes from a UDP socket to it's connected address.
/// Parameters:
///     `NSTDUDPSocket socket` - The UDP socket.
///     `const NSTDByte *const bytes` - The bytes to send.
///     `const NSTDUSize num` - Number of bytes to send.
///     `NSTDUSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_net_udp_socket_send(
    NSTDUDPSocket socket,
    const NSTDByte *const bytes,
    const NSTDUSize num,
    NSTDUSize *size);

/// Sends bytes from a UDP socket to another.
/// Parameters:
///     `NSTDUDPSocket socket` - The UDP socket.
///     `const char *const addr` - The address to send the bytes to.
///     `const NSTDByte *const bytes` - The bytes to send.
///     `const NSTDUSize num` - Number of bytes to send.
///     `NSTDUSize *size` - Returns as number of bytes actually sent.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_net_udp_socket_send_to(
    NSTDUDPSocket socket,
    const char *const addr,
    const NSTDByte *const bytes,
    const NSTDUSize num,
    NSTDUSize *size);

/// Closes and frees memory of a UDP socket.
/// Parameters:
///     `NSTDUDPSocket *socket` - Pointer to the UDP socket.
NSTDAPI void nstd_net_udp_socket_close(NSTDUDPSocket *socket);

/// Deallocates memory where an IP address string is allocated.
/// Parameters:
///     `char **ip` - The IP address.
NSTDAPI void nstd_net_udp_free_ip(char **ip);

/// Frees bytes allocated by any of the `nstd_net_*` functions.
/// parameters:
///     `NSTDByte **bytes` - Pointer to the bytes to free.
///     `const NSTDUSize size` - Number of bytes.
NSTDAPI void nstd_net_udp_free_bytes(NSTDByte **bytes, const NSTDUSize size);

#ifdef __cplusplus
}
#endif
#endif

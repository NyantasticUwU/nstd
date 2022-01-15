#ifndef NSTD_NET_H_INCLUDED
#define NSTD_NET_H_INCLUDED
#include "core/def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a TCP server.
typedef void *NSTDTCPServer;

/// Represents a TCP stream.
typedef void *NSTDTCPStream;

/// Represents a UDP socket.
typedef void *NSTDUDPSocket;

/// Creates a TCP server bound to the given address. Call `nstd_net_tcp_server_close` to free
/// memory allocated by this function and close the server.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDTCPServer server` - The TCP server, null on error.
NSTDAPI NSTDTCPServer nstd_net_tcp_server_bind(const char *const addr);

/// Accepts a connection on the TCP server. Call `nstd_net_tcp_stream_close` to free memory
/// allocated by this function and close the connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
/// Returns: `NSTDTCPStream client` - The server<=>client stream.
NSTDAPI NSTDTCPStream nstd_net_tcp_server_accept(NSTDTCPServer server);

/// Accepts all incoming connect requests, calling `callback` for each connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
///     `void(*callback)(NSTDTCPStream)` - The callback function when a connection is made.
NSTDAPI void nstd_net_tcp_server_accept_all(NSTDTCPServer server, void(*callback)(NSTDTCPStream));

/// Closes a TCP server and frees memory allocated by `nstd_net_tcp_server_bind`.
/// Parameters:
///     `NSTDTCPServer *server` - Pointer to the server.
NSTDAPI void nstd_net_tcp_server_close(NSTDTCPServer *server);

/// Connects a TCP stream to a server.
/// Parameters:
///     `const char *const addr` - The address to connect to.
/// Returns: `NSTDTCPStream client` - The TCP stream connected to the server.
NSTDAPI NSTDTCPStream nstd_net_tcp_stream_connect(const char *const addr);

/// Reads data from a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `NSTDUSize *size` - Returns as the number of bytes read.
/// Returns: `NSTDByte *bytes` - The bytes read from the stream.
NSTDAPI NSTDByte *nstd_net_tcp_stream_read(NSTDTCPStream stream, NSTDUSize *size);

/// Writes data to a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `const NSTDByte *const bytes` - The bytes to write.
///     `const NSTDUSize size` - Number of bytes to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_net_tcp_stream_write(
    NSTDTCPStream stream,
    const NSTDByte *const bytes,
    const NSTDUSize size);

/// Closes and frees memory of a TCP stream.
/// Parameters:
///     `NSTDTCPStream *stream` - Pointer to the TCP stream.
NSTDAPI void nstd_net_tcp_stream_close(NSTDTCPStream *stream);

/// Creates a UDP socket bound to the given address. Call `nstd_net_udp_socket_close` to free
/// memory allocated by this function and close the socket.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDUDPSocket socket` - The UDP socket, null on error.
NSTDAPI NSTDUDPSocket nstd_net_udp_socket_bind(const char *const addr);

/// Connects a UDP socket to a remote address.
/// Parameters:
///     `NSTDUDPSocket socket` - The socket to connect.
///     `const char *const addr` - The remote address to connect to.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_net_udp_socket_connect(NSTDUDPSocket socket, const char *const addr);

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
NSTDAPI void nstd_net_free_ip(char **ip);

/// Frees bytes allocated by any of the `nstd_net_*` functions.
/// parameters:
///     `NSTDByte **bytes` - Pointer to the bytes to free.
///     `const NSTDUSize size` - Number of bytes.
NSTDAPI void nstd_net_free_bytes(NSTDByte **bytes, const NSTDUSize size);

#ifdef __cplusplus
}
#endif
#endif

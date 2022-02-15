#ifndef NSTD_NET_TCP_H_INCLUDED
#define NSTD_NET_TCP_H_INCLUDED
#include "collections/vec.h"
#include "../core/def.h"
#include "../core/str.h"
#include "../nstd.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a TCP server.
typedef NSTDAny NSTDTCPServer;

/// Represents a TCP stream.
typedef NSTDAny NSTDTCPStream;

/// Creates a TCP server bound to the given address. Call `nstd_net_tcp_server_close` to free
/// memory allocated by this function and close the server.
/// Parameters:
///     `const NSTDStr *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDTCPServer server` - The TCP server, null on error.
NSTDAPI NSTDTCPServer nstd_net_tcp_server_bind(const NSTDStr *const addr);

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
///     `NSTDTCPServer *const server` - Pointer to the server.
NSTDAPI void nstd_net_tcp_server_close(NSTDTCPServer *const server);

/// Connects a TCP stream to a server.
/// Parameters:
///     `const NSTDStr *const addr` - The address to connect to.
/// Returns: `NSTDTCPStream client` - The TCP stream connected to the server.
NSTDAPI NSTDTCPStream nstd_net_tcp_stream_connect(const NSTDStr *const addr);

/// Reads data from a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
/// Returns: `NSTDVec bytes` - The bytes read from the stream.
NSTDAPI NSTDVec nstd_net_tcp_stream_read(NSTDTCPStream stream);

/// Writes data to a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `const NSTDByte *const bytes` - The bytes to write.
///     `const NSTDUSize size` - Number of bytes to write.
/// Returns: `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_net_tcp_stream_write(
    NSTDTCPStream stream,
    const NSTDByte *const bytes,
    const NSTDUSize size);

/// Closes and frees memory of a TCP stream.
/// Parameters:
///     `NSTDTCPStream *const stream` - Pointer to the TCP stream.
NSTDAPI void nstd_net_tcp_stream_close(NSTDTCPStream *const stream);

#ifdef __cplusplus
}
#endif
#endif

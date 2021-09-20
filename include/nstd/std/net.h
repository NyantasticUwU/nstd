#ifndef NSTD_STD_NET_H_INCLUDED
#define NSTD_STD_NET_H_INCLUDED
#include "../core/def.h"
#include "def.h"
#ifdef __cplusplus
extern "C"
{
#endif

/// Represents a TCP server.
typedef void *NSTDTCPServer;

/// Represents a TCP stream.
typedef void *NSTDTCPStream;

/// Creates a TCP server bound to the given address. Call `nstd_std_net_tcp_server_close` to free
/// memory allocated by this function and close the server.
/// Parameters:
///     `const char *const addr` - The address to listen on, formatted as "IP:Port".
/// Returns: `NSTDTCPServer server` - The TCP server, null on error.
NSTDAPI NSTDTCPServer nstd_std_net_tcp_server_bind(const char *const addr);

/// Accepts a connection on the TCP server. Call `nstd_std_net_tcp_stream_close` to free memory
/// allocated by this function and close the connection.
/// Parameters:
///     `NSTDTCPServer server` - The TCP server.
/// Returns: `NSTDTCPStream client` - The server<=>client stream.
NSTDAPI NSTDTCPStream nstd_std_net_tcp_server_accept(NSTDTCPServer server);

/// Closes a TCP server and frees memory allocated by `nstd_std_net_tcp_server_bind`.
/// Parameters:
///     `NSTDTCPServer *server` - Pointer to the server.
NSTDAPI void nstd_std_net_tcp_server_close(NSTDTCPServer *server);

/// Connects a TCP stream to a server.
/// Parameters:
///     `const char *const addr` - The address to connect to.
/// Returns: `NSTDTCPStream client` - The TCP stream connected to the server.
NSTDAPI NSTDTCPStream nstd_std_net_tcp_stream_connect(const char *const addr);

/// Reads data from a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `NSTDSize *size` - Returns as the number of bytes read.
/// Returns: `NSTDByte *bytes` - The bytes read from the stream.
NSTDAPI NSTDByte *nstd_std_net_tcp_stream_read(NSTDTCPStream stream, NSTDSize *size);

/// Writes data to a TCP stream.
/// Parameters:
///     `NSTDTCPStream stream` - The TCP stream.
///     `const NSTDByte *const bytes` - The bytes to write.
///     `const NSTDSize size` - Number of bytes to write.
/// Returns: `int errc` - Nonzero on error.
NSTDAPI int nstd_std_net_tcp_stream_write(
    NSTDTCPStream stream,
    const NSTDByte *const bytes,
    const NSTDSize size);

/// Closes and frees memory of a TCP stream.
/// Parameters:
///     `NSTDTCPStream *stream` - Pointer to the TCP stream.
NSTDAPI void nstd_std_net_tcp_stream_close(NSTDTCPStream *stream);

/// Frees bytes allocated by any of the `nstd_std_net_*` functions.
/// parameters:
///     `NSTDByte **bytes` - Pointer to the bytes to free.
///     `const NSTDSize size` - Number of bytes.
NSTDAPI void nstd_std_net_free_bytes(NSTDByte **bytes, const NSTDSize size);

#ifdef __cplusplus
}
#endif
#endif

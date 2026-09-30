#include <cstddef>
#include <cstdint>
// Inspect only the fixed GGUF prefix. This does not validate tensor payloads.
extern "C" int localtrust_gguf_version(const unsigned char* data, std::size_t size) {
    if (!data || size < 24 || data[0] != 'G' || data[1] != 'G' ||
        data[2] != 'U' || data[3] != 'F') return -1;
    std::uint32_t v = std::uint32_t(data[4]) | (std::uint32_t(data[5]) << 8) |
        (std::uint32_t(data[6]) << 16) | (std::uint32_t(data[7]) << 24);
    return (v == 2 || v == 3) ? static_cast<int>(v) : -1;
}

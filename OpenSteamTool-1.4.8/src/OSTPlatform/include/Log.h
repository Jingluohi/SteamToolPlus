#pragma once

// Small logging facade for OSTPlatform. The host installs a sink after its
// own logger is ready; Release builds compile OSTP_LOG_* to no-ops.

#include <cstdint>
#include <string_view>

#ifdef OPENSTEAMTOOL_LOGGING_ENABLED
#include <format>
#endif

namespace OSTPlatform::Log {

enum class Level : uint8_t { Trace, Debug, Info, Warn, Error };

struct Source {
    const char* file = nullptr;
    int line = 0;
    const char* function = nullptr;
};

// Host-injected receiver. `msg` is already formatted.
using Sink = void (*)(Level level, const Source& src, std::string_view msg);

void SetSink(Sink sink) noexcept;
void Dispatch(Level level, const Source& src, std::string_view msg) noexcept;

} // namespace OSTPlatform::Log

#ifdef OPENSTEAMTOOL_LOGGING_ENABLED
#define OSTP_LOG(lvl, ...)                                            \
    ::OSTPlatform::Log::Dispatch(                                     \
        (lvl),                                                        \
        ::OSTPlatform::Log::Source{__FILE__, __LINE__, __func__},     \
        ::std::format(__VA_ARGS__))
#else
#define OSTP_LOG(lvl, ...) ((void)0)
#endif

#define OSTP_LOG_TRACE(...) OSTP_LOG(::OSTPlatform::Log::Level::Trace, __VA_ARGS__)
#define OSTP_LOG_DEBUG(...) OSTP_LOG(::OSTPlatform::Log::Level::Debug, __VA_ARGS__)
#define OSTP_LOG_INFO(...)  OSTP_LOG(::OSTPlatform::Log::Level::Info,  __VA_ARGS__)
#define OSTP_LOG_WARN(...)  OSTP_LOG(::OSTPlatform::Log::Level::Warn,  __VA_ARGS__)
#define OSTP_LOG_ERROR(...) OSTP_LOG(::OSTPlatform::Log::Level::Error, __VA_ARGS__)

#include "include/Log.h"

#include <atomic>

namespace OSTPlatform::Log {
namespace {
std::atomic<Sink> g_sink{nullptr};
} // namespace

void SetSink(Sink sink) noexcept {
    g_sink.store(sink, std::memory_order_release);
}

void Dispatch(Level level, const Source& src, std::string_view msg) noexcept {
    if (Sink sink = g_sink.load(std::memory_order_acquire)) {
        sink(level, src, msg);
    }
}

} // namespace OSTPlatform::Log

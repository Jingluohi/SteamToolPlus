#pragma once

#include <cstdint>
#include <functional>

namespace OSTPlatform::Thread {

    using NativeThreadHandle = void*;

    NativeThreadHandle CurrentNativeThreadHandle();
    bool StartDetached(std::function<uint32_t()> entry);

} // namespace OSTPlatform::Thread

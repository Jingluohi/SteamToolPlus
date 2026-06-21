#include "include/Thread.h"

#include "include/Log.h"

#include <windows.h>

#include <memory>

namespace OSTPlatform::Thread {

NativeThreadHandle CurrentNativeThreadHandle() {
    return GetCurrentThread();
}

bool StartDetached(std::function<uint32_t()> entry) {
    if (!entry) {
        OSTP_LOG_WARN("StartDetached: entry is empty");
        return false;
    }

    auto* heapEntry = new std::function<uint32_t()>(std::move(entry));
    HANDLE thread = CreateThread(
        nullptr,
        0,
        [](void* param) -> DWORD {
            std::unique_ptr<std::function<uint32_t()>> fn(static_cast<std::function<uint32_t()>*>(param));
            return static_cast<DWORD>((*fn)());
        },
        heapEntry,
        0,
        nullptr);
    if (!thread) {
        OSTP_LOG_WARN("CreateThread failed (error={})", GetLastError());
        delete heapEntry;
        return false;
    }
    CloseHandle(thread);
    return true;
}

} // namespace OSTPlatform::Thread

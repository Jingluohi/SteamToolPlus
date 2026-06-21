#include "include/Detour.h"

#include "include/Log.h"
#include "include/Thread.h"

#include <windows.h>
#include <detours.h>

namespace OSTPlatform::Detour {

namespace {

bool DetourSucceeded(LONG result, const char* operation) {
    if (result == NO_ERROR) return true;
    OSTP_LOG_WARN("{} failed (detours_error={})", operation, result);
    return false;
}

} // namespace

bool BeginTransaction() {
    const LONG begin = DetourTransactionBegin();
    if (!DetourSucceeded(begin, "DetourTransactionBegin")) return false;
    return DetourSucceeded(
        DetourUpdateThread(Thread::CurrentNativeThreadHandle()),
        "DetourUpdateThread");
}

bool CommitTransaction() {
    return DetourSucceeded(DetourTransactionCommit(), "DetourTransactionCommit");
}

bool Attach(void** target, void* detour) {
    return DetourSucceeded(
        DetourAttach(reinterpret_cast<PVOID*>(target), reinterpret_cast<PVOID>(detour)),
        "DetourAttach");
}

bool Detach(void** target, void* detour) {
    return DetourSucceeded(
        DetourDetach(reinterpret_cast<PVOID*>(target), reinterpret_cast<PVOID>(detour)),
        "DetourDetach");
}

} // namespace OSTPlatform::Detour

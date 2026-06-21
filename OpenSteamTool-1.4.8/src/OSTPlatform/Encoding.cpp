#include "include/Encoding.h"

#include "include/Log.h"

#include <windows.h>

#include <limits>

namespace OSTPlatform::Encoding {
namespace {

bool FitsWindowsInt(size_t size, const char* operation) {
    if (size <= static_cast<size_t>((std::numeric_limits<int>::max)())) return true;
    OSTP_LOG_DEBUG("{}: input is too large ({} bytes/chars)", operation, size);
    return false;
}

} // namespace

std::string WideToUtf8(std::wstring_view value) {
    if (value.empty()) return {};
    if (!FitsWindowsInt(value.size(), "WideToUtf8")) return {};

    const int size = WideCharToMultiByte(CP_UTF8, 0, value.data(), static_cast<int>(value.size()),
                                         nullptr, 0, nullptr, nullptr);
    if (size <= 0) {
        OSTP_LOG_DEBUG("WideCharToMultiByte(size query) failed (error={})", GetLastError());
        return {};
    }

    std::string result(static_cast<size_t>(size), '\0');
    const int written = WideCharToMultiByte(CP_UTF8, 0, value.data(), static_cast<int>(value.size()),
                                            result.data(), size, nullptr, nullptr);
    if (written != size) {
        OSTP_LOG_DEBUG("WideCharToMultiByte(convert) failed (written={}, expected={}, error={})",
                       written, size, GetLastError());
        return {};
    }
    return result;
}

std::wstring Utf8ToWide(std::string_view value) {
    if (value.empty()) return {};
    if (!FitsWindowsInt(value.size(), "Utf8ToWide")) return {};

    const int size = MultiByteToWideChar(CP_UTF8, 0, value.data(), static_cast<int>(value.size()), nullptr, 0);
    if (size <= 0) {
        OSTP_LOG_DEBUG("MultiByteToWideChar(size query) failed (error={})", GetLastError());
        return {};
    }

    std::wstring result(static_cast<size_t>(size), L'\0');
    const int written = MultiByteToWideChar(CP_UTF8, 0, value.data(), static_cast<int>(value.size()), result.data(), size);
    if (written != size) {
        OSTP_LOG_DEBUG("MultiByteToWideChar(convert) failed (written={}, expected={}, error={})",
                       written, size, GetLastError());
        return {};
    }
    return result;
}

} // namespace OSTPlatform::Encoding

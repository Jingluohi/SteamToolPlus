#pragma once

#include <cstdint>
#include <filesystem>

namespace OSTPlatform::RemoteProcess {

    enum class Architecture {
        Unknown,
        X86,
        X64,
    };

    enum class InjectStatus {
        Ok,
        OpenProcessFailed,
        UnknownArchitecture,
        AllocFailed,
        WriteFailed,
        ResolveLoadLibraryFailed,
        CreateThreadFailed,
        WaitFailed,
        RemoteLoadFailed,
    };

    const char* ToString(Architecture architecture);
    const char* ToString(InjectStatus status);
    Architecture GetArchitecture(uint32_t pid);
    InjectStatus InjectLibrary(uint32_t pid, const std::filesystem::path& libraryPath);

} // namespace OSTPlatform::RemoteProcess

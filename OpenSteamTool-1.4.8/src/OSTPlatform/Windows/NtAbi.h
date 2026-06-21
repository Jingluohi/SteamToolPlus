#pragma once

#include <windows.h>

#include <cstddef>
#include <cstdint>

namespace OSTPlatform::Windows::NtAbi {

    enum class ProcessInfoClass : ULONG {
        BasicInformation = 0,
        Wow64Information = 26,
    };

    enum class MemoryInfoClass : ULONG {
        BasicInformation = 0,
    };

    struct ProcessBasicInformation {
        NTSTATUS exitStatus;
        PVOID pebBaseAddress;
        ULONG_PTR affinityMask;
        LONG basePriority;
        ULONG_PTR uniqueProcessId;
        ULONG_PTR inheritedFromUniqueProcessId;
    };

    using NtQueryInformationProcessFn = NTSTATUS (NTAPI*)(
        HANDLE processHandle,
        ProcessInfoClass processInformationClass,
        PVOID processInformation,
        ULONG processInformationLength,
        PULONG returnLength);

    using NtQueryVirtualMemoryFn = NTSTATUS (NTAPI*)(
        HANDLE processHandle,
        PVOID baseAddress,
        MemoryInfoClass memoryInformationClass,
        PVOID memoryInformation,
        SIZE_T memoryInformationLength,
        PSIZE_T returnLength);

    constexpr bool NtSuccess(NTSTATUS status) {
        return status >= 0;
    }

    struct Peb {
        BYTE reserved0[0x20];
        PVOID processParameters;
    };

    struct RtlUserProcessParameters {
        BYTE reserved0[0x80];
        PVOID environment;
    };

    struct Peb32 {
        BYTE reserved0[0x10];
        uint32_t processParameters;
    };

    struct RtlUserProcessParameters32 {
        BYTE reserved0[0x48];
        uint32_t environment;
    };

    static_assert(offsetof(Peb, processParameters) == 0x20);
    static_assert(offsetof(RtlUserProcessParameters, environment) == 0x80);
    static_assert(offsetof(Peb32, processParameters) == 0x10);
    static_assert(offsetof(RtlUserProcessParameters32, environment) == 0x48);

} // namespace OSTPlatform::Windows::NtAbi

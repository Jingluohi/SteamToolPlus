#pragma once

#include "include/DynamicLibrary.h"

#include <cstddef>
#include <cstdint>
#include <optional>

namespace OSTPlatform::Memory {

    struct ModuleImage {
        uint8_t* base = nullptr;
        size_t size = 0;
    };

    std::optional<ModuleImage> GetModuleImage(DynamicLibrary::ModuleHandle module);
    bool WriteExecutableByte(void* target, uint8_t value);

} // namespace OSTPlatform::Memory

#pragma once
#include "IPCMessages.gen.h"
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

namespace IPCLoader {

    struct Method {
        EIPCInterface interfaceID = static_cast<EIPCInterface>(0);
        std::string   name;
        uint32_t      funcHash  = 0;
        uint32_t      fencepost = 0;
        uint32_t      argc      = 0;
    };

    struct Interface {
        EIPCInterface       id = static_cast<EIPCInterface>(0);
        std::string         name;
        uint32_t            vtableRva = 0;
        std::vector<Method> methods;
    };

    // Fetch and parse metadata before installing IPC hooks.
    bool Load(const std::string& steamclientPath);

    // Return nullptr when metadata is unavailable for a method.
    const Method* Find(EIPCInterface interfaceID, uint32_t funcHash);
    const Method* Find(std::string_view ifaceName, std::string_view methodName);

    size_t InterfaceCount();
    size_t MethodCount();

} // namespace IPCLoader

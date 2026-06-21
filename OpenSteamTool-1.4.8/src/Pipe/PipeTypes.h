#pragma once

// Shared identities passed between Pipe features. Keep feature-owned state in
// each feature module so this layer stays only about "who is calling".

#include "Steam/Structs.h"
#include "Steam/Types.h"
#include "OSTPlatform/include/Process.h"

#include <cstddef>
#include <cstdint>
#include <format>
#include <string>

namespace PipeManager {

    // PID plus creation time lets caches survive PID reuse safely.
    struct ProcessKey {
        PID_t pid = 0;
        uint64 creationTime = 0;

        bool operator==(const ProcessKey&) const = default;

        bool IsValid() const {
            return pid != 0 && creationTime != 0;
        }

        std::string DebugString() const {
            return std::format("pid={} creation={} filetime={}",pid,OSTPlatform::Process::FormatCreationTime(creationTime),creationTime);
        }
    };

    // Container companion for unordered caches; ProcessKey owns the meaning.
    struct ProcessKeyHash {
        std::size_t operator()(const ProcessKey& key) const noexcept {
            return (static_cast<std::size_t>(key.pid) << 1) ^
                   static_cast<std::size_t>(key.creationTime ^ (key.creationTime >> 32));
        }
    };

    // A process may open several Steam IPC pipes.
    struct PipeKey {
        PID_t pid = 0;
        HSteamPipe pipe = 0;

        bool operator==(const PipeKey&) const = default;

        bool IsValid() const {
            return pid != 0 && pipe != 0;
        }

        std::string DebugString() const {
            return std::format("pid={} pipe=0x{:08X}", pid, pipe);
        }
    };

    // Container companion for unordered pipe maps; PipeKey owns the meaning.
    struct PipeKeyHash {
        std::size_t operator()(const PipeKey& key) const noexcept {
            return (static_cast<std::size_t>(key.pid) << 1) ^ static_cast<std::size_t>(key.pipe);
        }
    };

    inline PipeKey MakePipeKey(const CPipeClient* pipe) {
        if (!pipe) return {};
        return PipeKey{pipe->m_clientPID, pipe->m_hSteamPipe};
    }

    struct PipeContext {
        CPipeClient* pipe = nullptr;
        ProcessKey process;
        AppId_t appId = k_uAppIdInvalid;
        // These flags are resolved once in PipeManager and consumed by features.
        bool gameProcess = false;
        bool trackedApp = false;
        bool owned = false;
    };

} // namespace PipeManager

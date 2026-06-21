#pragma once
#include "Steam/Types.h"
#include "Steam/Structs.h"
#include "IPCMessages.gen.h"
#include <span>

using IPCHandlerFn = void(*)(CPipeClient* pipe,CUtlBuffer* pRead, CUtlBuffer* pWrite);

struct IPCHandlerEntry {
    const char*  interfaceName;
    const char*  methodName;
    IPCHandlerFn pre;
    IPCHandlerFn post;

    std::string DebugString() const {
        return std::format("{}::{} pre={} post={}", interfaceName, methodName,
                            pre ? "yes" : "no", post ? "yes" : "no");
    }
};

#define ADD_IPC_PRE_HANDLER(iface, method) \
    { #iface, #method, HandlerPre_##iface##_##method, nullptr }

#define ADD_IPC_POST_HANDLER(iface, method) \
    { #iface, #method, nullptr, HandlerPost_##iface##_##method }

#define ADD_IPC_BOTH_HANDLER(iface, method) \
    { #iface, #method, HandlerPre_##iface##_##method, HandlerPost_##iface##_##method }


namespace Hooks_IPC {
    // IPC hooks: intercepts IPC messages between game and steam
    void Install();
    void Uninstall();
    void RegisterHandlers(std::span<const IPCHandlerEntry> entries);
}
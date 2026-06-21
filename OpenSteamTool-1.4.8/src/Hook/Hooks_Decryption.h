#pragma once
#include "dllmain.h"

namespace Hooks_Decryption {
    // LoadDepotDecryptionKey hook: serves user-provided decryption keys for
    // depots configured via Lua.
    void Install();
    void Uninstall();

    std::vector<uint8_t> GetCacheAppOwnershipTicket(AppId_t appId);
}

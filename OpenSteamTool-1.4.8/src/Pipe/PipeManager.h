#pragma once

#include "Steam/Structs.h"
#include "Steam/Types.h"

namespace PipeManager {

    // Called after steamclient processes a pipe handshake. Resolves the caller
    // once, caches the process snapshot, then lets each Pipe feature react.
    void OnHandshake(CPipeClient* pipe);

} // namespace PipeManager

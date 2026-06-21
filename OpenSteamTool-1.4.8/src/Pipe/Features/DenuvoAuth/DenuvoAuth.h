#pragma once

#include "Pipe/PipeTypes.h"

namespace PipeManager::DenuvoAuth {

    // Per-handshake entry point: runs the one-time Denuvo detection (cached per
    // process) and advances the authorization state machine.
    void Apply(const PipeContext& ctx);

    // True only while the pipe is the selected authorization pipe and Denuvo has
    // not reached the end-authorization handshake.
    bool IsAuthorizedPipe(const CPipeClient* pipe);

} // namespace PipeManager::DenuvoAuth

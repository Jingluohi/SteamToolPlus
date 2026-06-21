#pragma once

namespace OSTPlatform::Detour {

    bool BeginTransaction();
    bool CommitTransaction();
    bool Attach(void** target, void* detour);
    bool Detach(void** target, void* detour);

} // namespace OSTPlatform::Detour

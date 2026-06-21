#pragma once

#include <string>

namespace SteamDiagnostics {

    // Capture the Steam build id and DLL hashes used in support popups.
    void Initialize(const std::string& steamclientPath,
                    const std::string& steamUIPath);

    // Reuse captured hashes for known Steam DLLs and hash other files on demand.
    std::string Sha256Of(const std::string& path);

    // Display a warning asynchronously with the captured Steam diagnostics.
    void ShowWarning(std::string title, std::string message);

} // namespace SteamDiagnostics

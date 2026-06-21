#pragma once

#include <filesystem>
#include <string>

namespace OSTPlatform::Hash {

    std::string Sha256OfFile(const std::filesystem::path& path);

} // namespace OSTPlatform::Hash

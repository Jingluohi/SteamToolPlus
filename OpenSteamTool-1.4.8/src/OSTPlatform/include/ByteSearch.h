#pragma once

#include <cstddef>
#include <cstdint>
#include <filesystem>
#include <optional>
#include <span>

namespace OSTPlatform::ByteSearch {

std::optional<size_t> Find(std::span<const uint8_t> bytes, std::span<const uint8_t> pattern);
std::optional<uint64_t> FindInFile(const std::filesystem::path& path,std::span<const uint8_t> pattern,size_t chunkBytes = 8 * 1024 * 1024);
std::optional<uint64_t> FindInFileRange(
    const std::filesystem::path& path,
    uint64_t offset,
    uint64_t size,
    std::span<const uint8_t> pattern,
    size_t chunkBytes = 8 * 1024 * 1024);

} // namespace OSTPlatform::ByteSearch

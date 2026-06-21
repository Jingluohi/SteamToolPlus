#include "Pipe/Features/Injection/Injection.h"

#include "OSTPlatform/include/RemoteProcess.h"
#include "OSTPlatform/include/Encoding.h"
#include "Utils/Config/Config.h"
#include "Utils/Logging/Log.h"

#include "dllmain.h"

#include <filesystem>
#include <mutex>
#include <string>
#include <unordered_set>

namespace PipeManager::Injection {
namespace {

    std::mutex g_mutex;
    std::unordered_set<ProcessKey, ProcessKeyHash> g_injected;

    bool WasInjected(const ProcessKey& key) {
        std::scoped_lock lock(g_mutex);
        return g_injected.contains(key);
    }

    void MarkInjected(const ProcessKey& key) {
        std::scoped_lock lock(g_mutex);
        g_injected.insert(key);
    }

    std::filesystem::path ResolveLibraryPath(const std::string& configured) {
        std::filesystem::path path(OSTPlatform::Encoding::Utf8ToWide(configured));
        if (path.is_absolute()) return path;

        std::filesystem::path base(OSTPlatform::Encoding::Utf8ToWide(SteamInstallPath));
        return base / path;
    }

    const std::string* ConfiguredLibraryFor(const Config::InjectionSettings& settings,
                                            OSTPlatform::RemoteProcess::Architecture architecture) {
        // Unknown architecture means we cannot choose a safe library path.
        switch (architecture) {
        case OSTPlatform::RemoteProcess::Architecture::X64:
            return settings.libraryX64.empty() ? nullptr : &settings.libraryX64;
        case OSTPlatform::RemoteProcess::Architecture::X86:
            return settings.libraryX86.empty() ? nullptr : &settings.libraryX86;
        case OSTPlatform::RemoteProcess::Architecture::Unknown:
            return nullptr;
        }
        return nullptr;
    }

} // namespace

void Apply(const PipeContext& ctx) {
    const Config::InjectionSettings settings = Config::GetInjectionSettings();
    if (!settings.enabled) return;
    if (!ctx.gameProcess) return;

    const auto architecture = OSTPlatform::RemoteProcess::GetArchitecture(ctx.process.pid);
    const std::string* configuredLibrary = ConfiguredLibraryFor(settings, architecture);
    if (!configuredLibrary) return;
    if (WasInjected(ctx.process)) return;

    const std::filesystem::path libraryPath = ResolveLibraryPath(*configuredLibrary);
    const auto status = OSTPlatform::RemoteProcess::InjectLibrary(ctx.process.pid, libraryPath);
    if (status == OSTPlatform::RemoteProcess::InjectStatus::Ok) {
        MarkInjected(ctx.process);
        LOG_PIPE_INFO("Injection: injected {} library into pid={} path={}",
                      OSTPlatform::RemoteProcess::ToString(architecture), ctx.process.pid, libraryPath.string());
    } else {
        LOG_PIPE_WARN("Injection: failed pid={} arch={} status={} path={}",
                      ctx.process.pid,
                      OSTPlatform::RemoteProcess::ToString(architecture),
                      OSTPlatform::RemoteProcess::ToString(status),
                      libraryPath.string());
    }
}

} // namespace PipeManager::Injection

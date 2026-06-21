#include "Hooks_IPC.h"
#include "Hooks_IPC_ISteamUser.h"
#include "PendingAPICalls.h"
#include "Utils/Tickets/AppTicket.h"
#include "Pipe/PipeManager.h"
#include "Pipe/Features/DenuvoAuth/DenuvoAuth.h"
#include "Utils/Logging/Log.h"
#include "Hooks_Misc.h"

namespace {
    using namespace IPCMessages::IClientUser;

    // [Post-Handler]: IClientUser::GetSteamID
    void HandlerPost_IClientUser_GetSteamID(CPipeClient* pipe,CUtlBuffer* pRead, CUtlBuffer* pWrite)
    {
        AppId_t appId = Hooks_Misc::ResolveAppId();
        GetSteamIDResp resp{pWrite};
        if (!resp.ok()) return;

        if (!PipeManager::DenuvoAuth::IsAuthorizedPipe(pipe)) {
            LOG_IPC_TRACE("IClientUser::GetSteamID: AppId={} not in authorization window, skip spoofing", appId);
            return;
        }

        const uint64 spoofed = AppTicket::GetSpoofSteamID(appId);
        if (!spoofed) {
            LOG_IPC_WARN("IClientUser::GetSteamID: AppId={} no valid steamid - cannot spoof", appId);
            return;
        }

        LOG_IPC_DEBUG("IClientUser::GetSteamID: AppId={} Original: {} -> Spoofed: 0x{:X}({})", 
                        appId,resp.DebugString(),spoofed, spoofed);
        resp.set_returnValue(spoofed);
    }

    // [Post-Handler]: IClientUser::GetAppOwnershipTicketExtendedData
    void HandlerPost_IClientUser_GetAppOwnershipTicketExtendedData(CPipeClient* pipe, CUtlBuffer* pRead, CUtlBuffer* pWrite)
    {
        GetAppOwnershipTicketExtendedDataReq req{pRead};
        if (!req.ok()) return;

        LOG_IPC_DEBUG("IClientUser::GetAppOwnershipTicketExtendedData:{}", req.DebugString());
        if (req.cbMaxTicket() < 0) return;

        AppTicket::AppOwnershipTicket ticket{};
        AppId_t appId = req.unAppID() == kOnlineFixAppId ? Hooks_Misc::ResolveAppId() : req.unAppID();
        
        AppTicket::AppTicketSource ticketSource;
        if (PipeManager::DenuvoAuth::IsAuthorizedPipe(pipe)) {
            ticketSource = AppTicket::AppTicketSource::CredentialStoreOnly;
        } else {
            LOG_IPC_DEBUG("IClientUser::GetAppOwnershipTicketExtendedData: AppId={} not in authorization window, only forge available", appId);
            ticketSource = AppTicket::AppTicketSource::ForgeOnly;
        }        
        if (!AppTicket::GetAppOwnershipTicket(appId, ticket, ticketSource)) return;

        if (ticket.data.size() > static_cast<size_t>(req.cbMaxTicket())) {
            LOG_IPC_WARN("IClientUser::GetAppOwnershipTicketExtendedData: AppId={} ticket too large ({} bytes) for buffer ({} bytes)",
                         appId, ticket.data.size(), req.cbMaxTicket());
            return;
        }

        GetAppOwnershipTicketExtendedDataResp resp{pWrite, static_cast<size_t>(req.cbMaxTicket())};
        if (!resp.ok()) return;

        resp.set_returnValue(ticket.totalSize);
        if (!resp.set_pTicket(ticket.data)) return;
        resp.set_piAppId(ticket.appIdOffset);
        resp.set_piSteamId(ticket.steamIdOffset);
        resp.set_piSignature(ticket.signatureOffset);
        resp.set_pcbSignature(ticket.signatureSize);

        LOG_IPC_DEBUG("IClientUser::GetAppOwnershipTicketExtendedData: AppId={} {}", 
                        appId,resp.DebugString());
    }

    // [Post-Handler]: IClientUser::RequestEncryptedAppTicket
    // Reads the hAsyncCall steamclient already wrote into the response,
    // so we know which AppId to mint an eticket for in GetAPICallResult.
    void HandlerPost_IClientUser_RequestEncryptedAppTicket(CPipeClient* pipe, CUtlBuffer* pRead, CUtlBuffer* pWrite)
    {
        RequestEncryptedAppTicketResp resp{pWrite};
        if (!resp.ok()) return;

        AppId_t appId = Hooks_Misc::ResolveAppId();
        std::vector<uint8_t> ticket = AppTicket::GetEncryptedTicketFromCredentialStore(appId);
        if (ticket.empty()) {
            LOG_IPC_DEBUG("RequestEncryptedAppTicket: AppId={} - no cached eticket, skip", appId);
            return;
        }

        const SteamAPICall_t hAsyncCall = resp.returnValue();
        PendingAPICalls::RecordEncryptedTicket(hAsyncCall, appId);
        LOG_IPC_DEBUG("RequestEncryptedAppTicket: AppId={} hAsyncCall=0x{:X} - recorded",
                      appId, hAsyncCall);
    }

    // [Post-Handler]: IClientUser::GetEncryptedAppTicket
    void HandlerPost_IClientUser_GetEncryptedAppTicket(CPipeClient* pipe, CUtlBuffer* pRead, CUtlBuffer* pWrite)
    {
        AppId_t appId = Hooks_Misc::ResolveAppId();
        std::vector<uint8_t> ticket = AppTicket::GetEncryptedTicketFromCredentialStore(appId);
        if (ticket.empty()) {
            LOG_IPC_DEBUG("GetEncryptedAppTicket: AppId={} - no cached eticket, skip", appId);
            return;
        }

        uint32 ticketSize = static_cast<uint32>(ticket.size());
        uint32 newCapacity = pWrite->Capacity() + ticketSize;
        if (!Hooks_Misc::EnsureBufferCapacity(pWrite, newCapacity,true)) {
            LOG_IPC_DEBUG("GetEncryptedAppTicket: AppId={} - failed to ensure buffer size", appId);
            return;
        }

        GetEncryptedAppTicketResp resp{pWrite};
        if (!resp.ok()) return;

        resp.set_returnValue(true);
        resp.set_pcbTicket(ticketSize);
        if (!resp.set_pTicket(ticket)) return;

        LOG_IPC_DEBUG("GetEncryptedAppTicket: AppId={} {}", appId, resp.DebugString());
    }

} // namespace

namespace Hooks_IPC_ISteamUser {
    void Register() {
        IPCHandlerEntry UserEntries[] = {
            ADD_IPC_POST_HANDLER(IClientUser, GetSteamID),
            ADD_IPC_POST_HANDLER(IClientUser, GetAppOwnershipTicketExtendedData),
            ADD_IPC_POST_HANDLER(IClientUser, RequestEncryptedAppTicket),
            ADD_IPC_POST_HANDLER(IClientUser, GetEncryptedAppTicket),
        };
        Hooks_IPC::RegisterHandlers(UserEntries);
    }
}

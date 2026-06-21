#pragma once

#include <cstdint>

namespace OSTPlatform::Trap {

    enum class ExceptionKind {
        Breakpoint,
        SingleStep,
        Other,
    };

    class Context {
    public:
        explicit Context(void* nativeContext) : nativeContext_(nativeContext) {}

        uint64_t InstructionPointer() const;

        // Nth integer/pointer argument (1-based) under the x64 calling
        // convention. Args 1-4 come from RCX/RDX/R8/R9; args 5+ are read from
        // the stack and are ONLY valid when the trap fired at the function's
        // first instruction (RSP still at the return address, before the
        // prologue moves it). Returns 0 on a bad index or unreadable stack slot.
        uint64_t Argument(int index) const;

        void EnableSingleStep();

    private:
        void* nativeContext_ = nullptr;
    };

    using Handler = bool (*)(ExceptionKind kind, Context& context);
    using HandlerHandle = void*;

    HandlerHandle AddVectoredHandler(Handler handler);
    void RemoveVectoredHandler(HandlerHandle handle);

} // namespace OSTPlatform::Trap

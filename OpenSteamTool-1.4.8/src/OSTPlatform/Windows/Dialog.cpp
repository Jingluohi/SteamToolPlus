#include "include/Dialog.h"

#include <windows.h>

namespace OSTPlatform::Dialog {

void ShowWarning(std::string title, std::string message) {
    MessageBoxA(nullptr, message.c_str(), title.c_str(), MB_OK | MB_ICONWARNING | MB_TOPMOST);
}

} // namespace OSTPlatform::Dialog

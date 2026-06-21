import os
import sys
from typing import Optional
import Millennium
import PluginUtils

logger = PluginUtils.Logger()

if sys.platform.startswith('win'):
    try:
        import winreg
    except Exception:
        winreg = None

_steam_install_path: Optional[str] = None

def detect_steam_install_path() -> str:
    global _steam_install_path
    if _steam_install_path:
        return _steam_install_path

    path = None
    if sys.platform.startswith('win') and winreg is not None:
        try:
            with winreg.OpenKey(winreg.HKEY_CURRENT_USER, r"Software\Valve\Steam") as key:
                path, _ = winreg.QueryValueEx(key, 'SteamPath')
        except Exception as e:
            logger.log(f'kernelua (steam_utils): Registry lookup failed: {e}')
            path = None

    if not path:
        try:
            path = Millennium.steam_path()
        except Exception as e:
            logger.error(f'kernelua (steam_utils): Millennium steam_path() failed: {e}')
            path = None

    _steam_install_path = path
    return _steam_install_path or ''

def get_steam_config_path() -> str:
    steam_path = detect_steam_install_path()
    if not steam_path:
        raise RuntimeError("Steam installation path not found")
    return os.path.join(steam_path, 'config')

def get_stplug_in_path() -> str:
    config_path = get_steam_config_path()
    stplug_path = os.path.join(config_path, 'stplug-in')
    os.makedirs(stplug_path, exist_ok=True)
    return stplug_path

def has_lua_for_app(appid: int) -> bool:
    try:
        base_path = detect_steam_install_path()
        if not base_path:
            return False
        stplug_path = os.path.join(base_path, 'config', 'stplug-in')
        lua_file = os.path.join(stplug_path, f'{appid}.lua')
        disabled_file = os.path.join(stplug_path, f'{appid}.lua.disabled')
        return os.path.exists(lua_file) or os.path.exists(disabled_file)
    except Exception as e:
        logger.error(f'kernelua (steam_utils): Error checking Lua scripts for app {appid}: {e}')
        return False

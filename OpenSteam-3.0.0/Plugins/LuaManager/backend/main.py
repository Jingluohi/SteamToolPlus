import Millennium
import PluginUtils
import json
import os

from http_client import close_global_client
from api_manager import APIManager
from LuaManager import LuaManager
from steam_utils import (
    detect_steam_install_path,
    has_lua_for_app
)

logger = PluginUtils.Logger()

def GetPluginDir():
    current_file = os.path.realpath(__file__)
    if current_file.endswith('/main.py/main.py') or current_file.endswith('\\main.py\\main.py'):
        current_file = current_file[:-8]
    elif current_file.endswith('/main.py') or current_file.endswith('\\main.py'):
        current_file = current_file[:-8]
    backend_dir = os.path.dirname(current_file) if current_file.endswith('main.py') else current_file
    return os.path.dirname(backend_dir)

class Logger:
    @staticmethod
    def log(message: str) -> None: logger.log(message)
    @staticmethod
    def warn(message: str) -> None: logger.warn(message)
    @staticmethod
    def error(message: str) -> None: logger.error(message)

class Plugin:
    def __init__(self):
        self.plugin_dir = GetPluginDir()
        self.backend_path = os.path.join(self.plugin_dir, 'backend')
        self.api_manager = APIManager(self.backend_path)
        self.lua_manager = LuaManager(self.backend_path)
        self._api_key = None
        self._injected = False

    def _inject_webkit_files(self):
        if self._injected: return
        try:
            js_file_path = os.path.join(self.plugin_dir, 'inject.js')
            if os.path.exists(js_file_path):
                Millennium.add_browser_js(js_file_path)
                self._injected = True
                logger.log("LuaManager: Injected")
            else:
                logger.error(f"LuaManager: inject.js not found at {js_file_path}")
        except Exception as e:
            logger.error(f'LuaManager: Failed to inject: {e}')

    def _load(self):
        try:
            detect_steam_install_path()
        except Exception as e:
            logger.log(f'LuaManager: Steam path detection failed: {e}')
        self._inject_webkit_files()
        Millennium.ready()

    def _unload(self):
        logger.log("Unloading LuaManager plugin")
        close_global_client()

_plugin_instance = None
def get_plugin():
    global _plugin_instance
    if _plugin_instance is None:
        _plugin_instance = Plugin()
        _plugin_instance._load()
    return _plugin_instance

plugin = get_plugin()

def hasLuaForApp(appid: int) -> str:
    try:
        exists = has_lua_for_app(appid)
        return json.dumps({'success': True, 'exists': exists})
    except Exception as e:
        logger.error(f'hasLuaForApp failed for {appid}: {e}')
        return json.dumps({'success': False, 'error': str(e)})

def addViaLuaManager(appid: int) -> str:
    try:
        result = plugin.lua_manager.add_via_lua(appid)
        return json.dumps(result)
    except Exception as e:
        logger.error(f'addViaLuaManager failed for {appid}: {e}')
        return json.dumps({'success': False, 'error': str(e)})

def GetStatus(appid: int) -> str:
    try:
        result = plugin.lua_manager.get_download_status(appid)
        return json.dumps(result)
    except Exception as e:
        logger.error(f'GetStatus failed for {appid}: {e}')
        return json.dumps({'success': False, 'error': str(e)})

def RemoveViaLuaManager(appid: int) -> str:
    try:
        result = plugin.lua_manager.remove_via_lua(appid)
        return json.dumps(result)
    except Exception as e:
        logger.error(f'RemoveViaLuaManager failed for {appid}: {e}')
        return json.dumps({'success': False, 'error': str(e)})
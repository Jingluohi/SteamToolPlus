import os
import traceback
import json
import time
import re
import zipfile
import threading
from io import BytesIO
from datetime import datetime
from typing import Dict, Any, List, Optional

import PluginUtils
from http_client import get_global_client
from steam_utils import detect_steam_install_path, get_stplug_in_path

logger = PluginUtils.Logger()


def _get_depotcache_path() -> str:
    """Returns the depotcache folder inside the Steam config directory."""
    steam_path = detect_steam_install_path()
    if not steam_path:
        raise RuntimeError("Steam installation path not found")
    depot_path = os.path.join(steam_path, 'depotcache')
    os.makedirs(depot_path, exist_ok=True)
    return depot_path


class LuaManager:
    def __init__(self, backend_path: str):
        self.backend_path = backend_path
        self._download_state: Dict[int, Dict[str, Any]] = {}
        self._download_lock = threading.Lock()

    # ------------------------------------------------------------------ state

    def _set_download_state(self, appid: int, update: Dict[str, Any]) -> None:
        with self._download_lock:
            state = self._download_state.get(appid, {})
            state.update(update)
            self._download_state[appid] = state

    def _get_download_state(self, appid: int) -> Dict[str, Any]:
        with self._download_lock:
            return self._download_state.get(appid, {}).copy()

    def get_download_status(self, appid: int) -> Dict[str, Any]:
        return {'success': True, 'state': self._get_download_state(appid)}

    # ----------------------------------------------------------- core download

    def _download_backend(self, appid: int) -> None:
        """
        Genera stplug-in/{appid}.lua directamente desde la API de depots
        y añade al final los setManifestid disponibles desde la API.
        """
        self._set_download_state(appid, {
            'status': 'checking',
            'currentApi': 'SteamProof',
            'bytesRead': 0,
            'totalBytes': 0,
            'endpoint': 'apps/depots'
        })

        api_base = "https://api.steamproof.net"
        depot_keys_url = "https://gitlab.com/steamautocracks/manifesthub/-/raw/main/depotkeys.json"
        cache_days = 7

        client = get_global_client()
        if not client:
            self._set_download_state(appid, {'status': 'failed', 'error': 'Failed to get HTTP client'})
            return

        try:
            self._set_download_state(appid, {'status': 'downloading', 'endpoint': 'SteamProof Lua'})
            logger.log(f'LuaManager: Requesting SteamProof API for appid {appid}')

            st_info, _, body_info = client.raw_get(f"{api_base}/apps/depots?ids={appid}")
            logger.log(f'LuaManager: SteamProof response HTTP {st_info}')

            if st_info != 200:
                self._set_download_state(appid, {
                    'status': 'failed',
                    'error': f'SteamProof API error (HTTP {st_info})'
                })
                return

            info_json = json.loads(body_info)
            apps = info_json.get('apps') or []
            if not apps:
                self._set_download_state(appid, {
                    'status': 'failed',
                    'error': 'SteamProof API returned no apps'
                })
                return

            app_data = apps[0]
            depots = app_data.get('depots') or []

            # Caché de depotkeys.json
            cache_dir = get_stplug_in_path()
            cache_file = os.path.join(cache_dir, 'depotkeys.json')

            depot_keys = {}
            should_refresh = True

            if os.path.exists(cache_file):
                mtime = os.path.getmtime(cache_file)
                file_age_days = (time.time() - mtime) / (24 * 60 * 60)
                if file_age_days < cache_days:
                    try:
                        with open(cache_file, 'r', encoding='utf-8') as f:
                            depot_keys = json.load(f)
                        should_refresh = False
                        logger.log(f'Using cached depotkeys.json ({file_age_days:.1f} days old)')
                    except Exception as cache_error:
                        logger.warn(f'Cache read failed: {cache_error}, refreshing')

            if should_refresh:
                logger.log('Downloading fresh depotkeys.json')
                keys_info, _, keys_body = client.raw_get(depot_keys_url)
                if keys_info == 200:
                    depot_keys = json.loads(keys_body)
                    parent = os.path.dirname(cache_file)
                    if parent:
                        os.makedirs(parent, exist_ok=True)
                    with open(cache_file, 'w', encoding='utf-8') as f:
                        json.dump(depot_keys, f)
                else:
                    logger.warn('Failed to refresh depotkeys.json, using empty dict')

            lua_dir = get_stplug_in_path()
            os.makedirs(lua_dir, exist_ok=True)
            dst_lua = os.path.join(lua_dir, f'{appid}.lua')

            lines = [
                '-- Open Steam Lua Generator',
                f'addappid({appid})'
            ]

            manifest_lines = []

            for depot in depots:
                did = depot.get('depotId')
                if did is None:
                    continue

                depot_key_str = str(did)
                if depot_key_str in depot_keys and depot_keys[depot_key_str]:
                    lines.append(f'addappid({did},1,"{depot_keys[depot_key_str]}")')
                else:
                    lines.append(f'addappid({did},0,"")')

                manifests = depot.get('manifests') or {}
                manifest_id = None

                public_manifest = manifests.get('public')
                if isinstance(public_manifest, dict):
                    manifest_id = public_manifest.get('manifestId')

                if not manifest_id:
                    for _, branch_data in manifests.items():
                        if isinstance(branch_data, dict) and branch_data.get('manifestId'):
                            manifest_id = branch_data.get('manifestId')
                            break

                if manifest_id:
                    max_size = depot.get('maxSize')
                    if max_size:
                        manifest_lines.append(f'setManifestid({did}, "{manifest_id}", {max_size})')
                    else:
                        manifest_lines.append(f'setManifestid({did}, "{manifest_id}")')

            final_lua_content = "\n".join(lines)

            if manifest_lines:
                timestamp = datetime.utcnow().strftime('%Y-%m-%d %H:%M UTC')
                final_lua_content += f"\n\n-- SteamProof Manifests (updated {timestamp})\n"
                final_lua_content += "\n".join(manifest_lines)

            final_lua_content += "\n"

            with open(dst_lua, 'w', encoding='utf-8', newline='\n') as f:
                f.write(final_lua_content)

            self._set_download_state(appid, {
                'status': 'done',
                'success': True,
                'api': 'SteamProof',
                'installedFiles': [dst_lua],
                'installedPath': dst_lua
            })

        except Exception as e:
            tb = traceback.format_exc()
            logger.error(f'LuaManager: Lua generation failed for {appid}: {e}\n{tb}')
            self._set_download_state(appid, {'status': 'failed', 'error': str(e)})

    # ---------------------------------------------------------- public API

    def add_via_lua(self, appid: int, endpoints: Optional[List[str]] = None) -> Dict[str, Any]:
        try:
            appid = int(appid)
        except (ValueError, TypeError):
            return {'success': False, 'error': 'Invalid appid'}

        self._set_download_state(appid, {'status': 'queued', 'bytesRead': 0, 'totalBytes': 0})

        def run():
            try:
                self._download_backend(appid)
            except Exception as e:
                logger.error(f'LuaManager: unhandled error for {appid}: {e}')
                self._set_download_state(appid, {'status': 'failed', 'error': f'Crash: {str(e)}'})

        threading.Thread(target=run, daemon=True).start()
        return {'success': True}

    def remove_via_lua(self, appid: int) -> Dict[str, Any]:
        try:
            appid = int(appid)
        except (ValueError, TypeError):
            return {'success': False, 'error': 'Invalid appid'}

        try:
            stplug = get_stplug_in_path()
            removed = []

            lua_file = os.path.join(stplug, f'{appid}.lua')
            if os.path.exists(lua_file):
                os.remove(lua_file)
                removed.append(f'{appid}.lua')

            disabled = os.path.join(stplug, f'{appid}.lua.disabled')
            if os.path.exists(disabled):
                os.remove(disabled)
                removed.append(f'{appid}.lua.disabled')

            try:
                depot = _get_depotcache_path()
                for name in os.listdir(depot):
                    if name.startswith(f'{appid}_') and name.endswith('.manifest'):
                        os.remove(os.path.join(depot, name))
                        removed.append(name)
            except Exception as e:
                logger.log(f'LuaManager: Could not remove manifests for {appid}: {e}')

            if removed:
                logger.log(f'LuaManager: Removed {removed}')
                return {'success': True, 'message': f'Removed {len(removed)} files', 'removed_files': removed}
            return {'success': False, 'error': f'No files found for app {appid}'}
        except Exception as e:
            logger.error(f'LuaManager: remove error for {appid}: {e}')
            return {'success': False, 'error': str(e)}

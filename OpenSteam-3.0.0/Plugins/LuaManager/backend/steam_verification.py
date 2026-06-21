import os
import hashlib
import time
import random
import Millennium
import PluginUtils
from typing import Dict, Optional, Any

try:
    import psutil
    PSUTIL_AVAILABLE = True
except ImportError:
    psutil = None
    PSUTIL_AVAILABLE = False

logger = PluginUtils.Logger()

class SteamVerification:
    def __init__(self):
        self.steam_pid = None
        self.steam_process = None
        self.millennium_version = None
        self.plugin_checksum = None
        self._discover_steam_process()
        self._calculate_plugin_checksum()

    def _discover_steam_process(self):
        try:
            if not PSUTIL_AVAILABLE:
                logger.warn("kernelua (steam_verification): psutil not available, using fallback PID")
                self.steam_pid = random.randint(1000, 65535)
                return

            if psutil is not None:
                for proc in psutil.process_iter(['pid', 'name', 'exe']):
                    try:
                        info = proc.info
                        if info['name'] and 'steam' in info['name'].lower():
                            if info['exe'] and 'steam.exe' in info['exe'].lower():
                                self.steam_pid = info['pid']
                                self.steam_process = proc
                                break
                    except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                        continue

            if not self.steam_pid:
                logger.warn("kernelua (steam_verification): Steam process not found, using fallback PID")
                self.steam_pid = random.randint(1000, 65535)

            try:
                self.millennium_version = Millennium.version()
            except Exception as e:
                logger.warn(f"kernelua (steam_verification): Could not get Millennium version: {e}")
                self.millennium_version = "1.0.0"

        except Exception as e:
            logger.error(f"kernelua (steam_verification): Error discovering Steam process: {e}")
            self.steam_pid = random.randint(1000, 65535)
            self.millennium_version = "1.0.0"

    def _calculate_plugin_checksum(self):
        try:
            hasher = hashlib.sha256()
            if os.path.exists(__file__):
                with open(__file__, 'rb') as f:
                    hasher.update(f.read())
            if self.steam_process:
                try:
                    steam_exe = self.steam_process.exe()
                    if steam_exe and os.path.exists(steam_exe):
                        with open(steam_exe, 'rb') as f:
                            hasher.update(f.read(1024))
                except Exception:
                    pass
            import platform
            machine_info = f"{platform.node()}-{platform.processor()}-{os.environ.get('USERNAME', 'unknown')}"
            hasher.update(machine_info.encode())
            self.plugin_checksum = hasher.hexdigest()
        except Exception as e:
            logger.error(f"kernelua (steam_verification): checksum error: {e}")
            fallback = f"{time.time()}-{os.environ.get('USERNAME', 'unknown')}-{self.steam_pid}"
            self.plugin_checksum = hashlib.sha256(fallback.encode()).hexdigest()

    def _get_process_hash(self) -> str:
        try:
            if self.steam_process:
                mi = self.steam_process.memory_info()
                cpu = self.steam_process.cpu_percent()
                ct = self.steam_process.create_time()
                return hashlib.sha256(f"{mi.rss}-{mi.vms}-{cpu}-{ct}".encode()).hexdigest()[:32]
        except Exception:
            pass
        return hashlib.sha256(f"{time.time()}-{self.steam_pid}".encode()).hexdigest()[:32]

    def _get_memory_proof(self) -> str:
        try:
            if self.steam_process:
                threads = len(self.steam_process.threads())
                maps = len(self.steam_process.memory_maps()) if hasattr(self.steam_process, 'memory_maps') else 0
                return hashlib.sha256(f"{threads}-{maps}-{self.steam_pid}".encode()).hexdigest()[:32]
        except Exception:
            pass
        return hashlib.sha256(f"memory-{self.steam_pid}-{time.time()}".encode()).hexdigest()[:32]

    def get_verification_headers(self) -> Dict[str, str]:
        ts = str(int(time.time() * 1000))
        return {
            'X-Steam-PID': str(self.steam_pid),
            'X-Millennium-Version': self.millennium_version or "1.0.0",
            'X-Plugin-Checksum': self.plugin_checksum or "",
            'X-Process-Hash': self._get_process_hash(),
            'X-Memory-Proof': self._get_memory_proof(),
            'X-Plugin-Timestamp': ts,
            'User-Agent': f'kernelua-plugin/{self.millennium_version or "1.0.0"} (Millennium)',
        }

    def refresh_verification(self):
        try:
            if self.steam_process and not self.steam_process.is_running():
                logger.log("kernelua (steam_verification): Steam process changed, refreshing...")
                self._discover_steam_process()
            import random
            if random.random() < 0.1:
                self._calculate_plugin_checksum()
        except Exception as e:
            logger.error(f"kernelua (steam_verification): refresh error: {e}")

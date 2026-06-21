import PluginUtils

logger = PluginUtils.Logger()

MANIFESTHUB_BASE = 'https://codeload.github.com/SteamAutoCracks/ManifestHub/zip/refs/heads/'

class APIManager:
    def __init__(self, backend_path: str):
        self.backend_path = backend_path

    def get_signed_download_url(self, appid: int) -> dict:
        """
        Builds the ManifestHub ZIP download URL for the given appid.
        No external API call is needed — the URL is deterministic.
        """
        try:
            url = MANIFESTHUB_BASE + str(appid)
            return {'success': True, 'url': url}
        except Exception as e:
            logger.error(f'APIManager: get_signed_download_url error for {appid}: {e}')
            return {'success': False, 'error': str(e)}

    def check_availability(self, appid: int, endpoint: str = '') -> dict:
        """
        Always reports ManifestHub as available since there is no pre-check endpoint.
        Actual availability is determined at download time.
        """
        return {'success': True, 'available': True, 'endpoint': 'manifesthub'}

    def fetch_available_endpoints(self) -> dict:
        return {'success': True, 'endpoints': ['manifesthub']}

    def get_download_endpoints(self) -> list:
        return ['manifesthub']

    def get_user_id(self) -> dict:
        return {'success': False, 'error': 'Not supported in kernelua'}

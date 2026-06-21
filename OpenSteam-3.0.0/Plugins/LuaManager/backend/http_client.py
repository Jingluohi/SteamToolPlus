from typing import Optional, Dict, Any, Tuple
import PluginUtils

try:
    import httpx
    from httpx import HTTPStatusError, RequestError, Response
    HTTPX_AVAILABLE = True
except ImportError:
    httpx = None
    HTTPStatusError = None
    RequestError = None
    Response = None
    HTTPX_AVAILABLE = False

logger = PluginUtils.Logger()

PLUGIN_UA = 'kernelua-plugin/1.0.0 (Millennium)'
WAF_HEADER_NAME = 'X-KernelUA'
WAF_HEADER_VALUE = 'kernelua-plugin-1'

BASE_HEADERS = {
    'Accept': 'application/json',
    'X-Requested-With': 'kernelua-Plugin',
    'Origin': 'https://store.steampowered.com',
    'Sec-Fetch-Dest': 'empty',
    'Sec-Fetch-Mode': 'cors',
    'Sec-Fetch-Site': 'cross-site',
}

class HTTPClient:
    def __init__(self, timeout: int = 30):
        self._client = None
        self._timeout = timeout

    def _ensure_client(self):
        if not HTTPX_AVAILABLE:
            raise Exception("httpx is not available. pip install httpx")
        if self._client is None:
            self._client = httpx.Client(timeout=self._timeout, follow_redirects=True)
        return self._client

    def _build_headers(self, extra: Optional[Dict[str, str]] = None, auth_token: Optional[str] = None) -> Dict[str, str]:
        headers = BASE_HEADERS.copy()
        headers['User-Agent'] = PLUGIN_UA
        headers[WAF_HEADER_NAME] = WAF_HEADER_VALUE
        if auth_token:
            headers['Authorization'] = f'Bearer {auth_token}'
        if extra:
            headers.update(extra)
        return headers

    def _success_json(self, response: Response) -> Dict[str, Any]:
        try:
            data = response.json()
        except Exception:
            data = response.text
        return {'success': True, 'data': data, 'status_code': response.status_code}

    def _error_dict(self, url: str, e: Exception) -> Dict[str, Any]:
        if HTTPX_AVAILABLE and HTTPStatusError is not None and isinstance(e, HTTPStatusError):
            error_msg = f"HTTP {e.response.status_code}: {e.response.text if e.response else 'No response'}"
            logger.error(f'HTTPClient: {url} -> {error_msg}')
            return {'success': False, 'error': error_msg, 'status_code': e.response.status_code if e.response else None}
        elif HTTPX_AVAILABLE and RequestError is not None and isinstance(e, RequestError):
            error_msg = f"Request error: {str(e)}"
            logger.error(f'HTTPClient: {url} -> {error_msg}')
            return {'success': False, 'error': error_msg}
        else:
            error_msg = f"Unexpected error: {str(e)}"
            logger.error(f'HTTPClient: {url} -> {error_msg}')
            return {'success': False, 'error': error_msg}

    def get(self, url: str, params: Optional[Dict[str, Any]] = None, auth_token: Optional[str] = None) -> Dict[str, Any]:
        try:
            client = self._ensure_client()
            headers = self._build_headers(auth_token=auth_token)
            resp = client.get(url, params=params or {}, headers=headers)
            resp.raise_for_status()
            return self._success_json(resp)
        except Exception as e:
            return self._error_dict(url, e)

    def get_binary(self, url: str, params: Optional[Dict[str, Any]] = None, auth_token: Optional[str] = None) -> Dict[str, Any]:
        try:
            client = self._ensure_client()
            headers = self._build_headers(auth_token=auth_token)
            resp = client.get(url, params=params or {}, headers=headers)
            resp.raise_for_status()
            return {'success': True, 'data': resp.content, 'status_code': resp.status_code}
        except Exception as e:
            return self._error_dict(url, e)

    def raw_get(self, url: str, params: Optional[Dict[str, Any]] = None, auth_token: Optional[str] = None,
                extra_headers: Optional[Dict[str, str]] = None) -> Tuple[int, Dict[str, str], bytes]:
        client = self._ensure_client()
        headers = self._build_headers(extra=extra_headers, auth_token=auth_token)
        resp = client.get(url, params=params or {}, headers=headers)
        return resp.status_code, {k: v for k, v in resp.headers.items()}, resp.content

    def get_with_headers(self, url: str, params: Optional[Dict[str, Any]] = None,
                         auth_token: Optional[str] = None,
                         extra_headers: Optional[Dict[str, str]] = None) -> Tuple[bytes, Dict[str, str], int]:
        status, headers, content = self.raw_get(url, params=params, auth_token=auth_token, extra_headers=extra_headers)
        return content, headers, status

    def stream_get(self, url: str, **kwargs):
        client = self._ensure_client()
        headers = self._build_headers()
        return client.stream('GET', url, headers=headers, **kwargs)

    def close(self) -> None:
        if self._client is not None:
            try:
                self._client.close()
            except Exception as e:
                logger.warn(f'HTTPClient: error while closing client: {e}')
            finally:
                self._client = None

    def post(self, url: str, data: Optional[Dict[str, Any]] = None, auth_token: Optional[str] = None) -> Dict[str, Any]:
        try:
            client = self._ensure_client()
            headers = self._build_headers(auth_token=auth_token)
            if data:
                headers['Content-Type'] = 'application/json'
            resp = client.post(url, json=data or {}, headers=headers)
            resp.raise_for_status()
            return {'success': True, 'data': resp.text, 'status_code': resp.status_code}
        except Exception as e:
            return self._error_dict(url, e)

_global_client: Optional[HTTPClient] = None

def get_global_client() -> HTTPClient:
    global _global_client
    if _global_client is None:
        _global_client = HTTPClient()
    return _global_client

def close_global_client() -> None:
    global _global_client
    if _global_client is not None:
        _global_client.close()
        _global_client = None

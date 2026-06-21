(function () {
  'use strict';

  function backendLog(message) {
    try {
      if (typeof Millennium !== 'undefined' && typeof Millennium.callServerMethod === 'function') {
        Millennium.callServerMethod('luamanager', 'Logger.log', { message: String(message) });
      }
    } catch (err) { if (console && console.warn) console.warn('[luamanager] backendLog failed', err); }
  }

  class AppState {
    constructor() {
      this.logs = { missingOnce: false, existsOnce: false };
      this.run = { inProgress: false, appid: null };
      this.cache = new Map();
    }
    setRunState(inProgress, appid = null) { this.run = { inProgress, appid }; }
    cacheResult(key, value, ttl = 30000) { this.cache.set(key, { value, expires: Date.now() + ttl }); }
    getCached(key) { const c = this.cache.get(key); if (c && Date.now() < c.expires) return c.value; this.cache.delete(key); return null; }
  }

  const state = new AppState();

  function ensureStyles() {
    if (!document.getElementById('luamanager-styles')) {
      const style = document.createElement('style');
      style.id = 'luamanager-styles';
      style.textContent = `
        :root {
          --steam-bg-modal: linear-gradient(135deg, #23262E 0%, #191B20 100%);
          --steam-bg-header: linear-gradient(135deg, #2D3139 0%, #1E2024 100%);
          --steam-bg-progress: linear-gradient(135deg, #3D4450 0%, #2A2D35 100%);
          --steam-btn-primary: linear-gradient(135deg, #1976D2 0%, #1565C0 100%);
          --steam-btn-primary-hover: linear-gradient(135deg, #1E88E5 0%, #1976D2 100%);
          --steam-btn-primary-active: linear-gradient(135deg, #1565C0 0%, #0D47A1 100%);
          --steam-btn-secondary: linear-gradient(135deg, #1e2329 0%, #1a1e22 100%);
          --steam-btn-secondary-hover: linear-gradient(135deg, #3c4043 0%, #2a2d31 100%);
          --steam-border: #3D4450; --steam-border-btn: #1976D2; --steam-border-light: #495a6b;
          --steam-text-primary: #C6D4DF; --steam-text-secondary: #8F98A0; --steam-text-muted: #b8bcbf;
          --steam-font: "Motiva Sans", Arial, sans-serif; --steam-shadow: 0 0 20px rgba(0,0,0,.8);
          --steam-shadow-btn: 0 2px 4px rgba(0,0,0,.3); --steam-shadow-hover: 0 4px 8px rgba(0,0,0,.4);
        }
        .luamanager-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.85); backdrop-filter: blur(2px); z-index: 99999; display:flex; align-items:center; justify-content:center; animation: fadeIn .3s ease-out; }
        .luamanager-modal { background: var(--steam-bg-modal); border:1px solid var(--steam-border); box-shadow: var(--steam-shadow); font-family: var(--steam-font); animation: modalSlideIn .4s ease-out; width:min(90vw,500px); max-width:600px; max-height:80vh; overflow-y:auto; position:relative; }
        .luamanager-header { background: var(--steam-bg-header); border-bottom:1px solid var(--steam-border); padding:12px 16px; display:flex; align-items:center; justify-content:space-between; }
        .luamanager-title { color: var(--steam-text-primary); font-size:14px; font-weight:normal; }
        .luamanager-close { color: var(--steam-text-secondary); cursor:pointer; font-size:16px; padding:4px; user-select:none; transition: color .2s; }
        .luamanager-close:hover { color:#fff; }
        .luamanager-content { padding:16px 20px 20px 20px; }
        .luamanager-status { font-size:13px; line-height:1.4; margin-bottom:12px; color:var(--steam-text-primary); min-height:18px; }
        .luamanager-progress { background: var(--steam-bg-progress); border:1px solid #4A5462; height:16px; border-radius:4px; overflow:hidden; margin-bottom:12px; position:relative; display:none; }
        .luamanager-progress-bar { height:100%; width:0%; background: var(--steam-btn-primary); transition: width .3s; position:relative; overflow:hidden; }
        .luamanager-progress-bar::after { content:''; position:absolute; inset:0; background: linear-gradient(45deg, transparent 25%, rgba(255,255,255,.1) 25%, rgba(255,255,255,.1) 50%, transparent 50%, transparent 75%, rgba(255,255,255,.1) 75%); background-size: 20px 20px; animation: progressStripes 1s linear infinite; }
        .luamanager-percent { display:none; font-size:12px; color:var(--steam-text-muted); }
        .luamanager-btn { border-radius:3px; cursor:pointer; font-size:12px; font-family:var(--steam-font); font-weight:500; padding:8px 16px; transition: all .2s; text-align:center; outline:none; display:inline-block; margin:4px; text-decoration:none; min-width:80px; }
        .luamanager-btn-primary { background: var(--steam-btn-primary); border:1px solid var(--steam-border-btn); color:#fff; box-shadow: var(--steam-shadow-btn); }
        .luamanager-btn-primary:hover { background: var(--steam-btn-primary-hover); transform: translateY(-1px); box-shadow: var(--steam-shadow-hover); }
        .luamanager-btn-secondary { background: var(--steam-btn-secondary); border:1px solid var(--steam-border); color:var(--steam-text-secondary); box-shadow: var(--steam-shadow-btn); }
        .luamanager-endpoints { margin-top:12px; display:flex; gap:8px; flex-wrap:wrap; }
        @keyframes progressStripes { 0% { background-position: 0 0; } 100% { background-position: 20px 0; } }
        @keyframes fadeIn { from { opacity:0; } to { opacity:1; } }
        @keyframes modalSlideIn { from { opacity:0; transform: scale(.9) translateY(-20px); } to { opacity:1; transform: scale(1) translateY(0); } }
        .luamanager-button-container { }
        #luamanager-in-library-banner { }
      `;
      document.head.appendChild(style);
    }
  }

  function createModal(opts = {}) {
    const overlay = document.createElement('div');
    overlay.className = `luamanager-overlay ${opts.overlayClass || ''}`;

    const modal = document.createElement('div');
    modal.className = 'luamanager-modal';
    modal.style.width = opts.width || '400px';

    const header = document.createElement('div');
    header.className = 'luamanager-header';

    const title = document.createElement('div');
    title.className = 'luamanager-title';
    title.textContent = opts.title || 'LuaManager';

    const closeBtn = document.createElement('div');
    closeBtn.className = 'luamanager-close'; closeBtn.innerHTML = '×';
    closeBtn.onclick = () => overlay.remove();

    const content = document.createElement('div');
    content.className = 'luamanager-content';

    header.append(title, closeBtn); modal.append(header, content); overlay.appendChild(modal);
    overlay.addEventListener('click', (e) => { if (e.target === overlay) overlay.remove(); });
    return { overlay, modal, header, title, closeBtn, content };
  }

  function createButton(text, className = 'luamanager-btn-primary', onClick = null) {
    const btn = document.createElement('button');
    btn.className = `luamanager-btn ${className}`;
    btn.textContent = text;
    if (onClick) btn.onclick = onClick;
    return btn;
  }

  function createProgressBar() {
    const wrap = document.createElement('div'); wrap.className = 'luamanager-progress';
    const bar = document.createElement('div'); bar.className = 'luamanager-progress-bar'; wrap.appendChild(bar);
    const percent = document.createElement('div'); percent.className = 'luamanager-percent'; percent.textContent = '0%';
    return { wrap, bar, percent };
  }

  class Api {
    static async call(method, params = {}) {
      const res = await Millennium.callServerMethod('luamanager', method, params);
      return JSON.parse(res);
    }
    static async hasLua(appId) {
      const key = `appExists_${appId}`; const cached = state.getCached(key);
      if (cached !== null) return cached;
      try {
        const r = await this.call('hasLuaForApp', { appid: appId });
        const exists = r.success && r.exists; state.cacheResult(key, exists, 120000);
        return exists;
      } catch {
        return false;
      }
    }
  }

  function setSteamTooltip(el, text) {
    el.setAttribute('data-tooltip-text', text); el.title = text; el.setAttribute('data-panel-tooltip', text);
  }

  function createInLibraryBanner(gameName) {
    const banner = document.createElement('div');
    banner.className = 'game_area_already_owned page_content';
    banner.id = 'luamanager-in-library-banner';
    const ctn = document.createElement('div'); ctn.className = 'game_area_already_owned_ctn';
    const flag = document.createElement('div'); flag.className = 'ds_owned_flag ds_flag'; flag.innerHTML = 'IN LIBRARY&nbsp;&nbsp;';
    const msg = document.createElement('div'); msg.className = 'already_in_library'; msg.textContent = `${gameName} is already in your Steam library`;
    ctn.append(flag, msg); banner.appendChild(ctn); return banner;
  }
  function addInLibraryFlag(section) {
    if (section && !section.querySelector('.package_in_library_flag')) {
      const flag = document.createElement('div'); flag.className = 'package_in_library_flag in_own_library';
      flag.innerHTML = '<span class="icon">☰</span> <span>In library</span>'; section.insertBefore(flag, section.firstChild);
    }
  }

  function showDownloadModal() {
    if (document.querySelector('.luamanager-overlay')) return;
    const { overlay, title, content } = createModal({ title: 'LuaManager', overlayClass: 'luamanager-overlay' });
    const status = document.createElement('div'); status.className = 'luamanager-status'; status.textContent = 'Working…';
    const { wrap, bar, percent } = createProgressBar();
    content.append(status, wrap, percent); document.body.appendChild(overlay);
    return { overlay, title, status, wrap, bar, percent };
  }

  function createAndInjectButton(appId) {
    const container =
      document.querySelector('.game_area_purchase_game_wrapper .game_purchase_action_bg') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .game_purchase_action_bg') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .game_purchase_action') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .btn_addtocart')?.parentElement ||
      document.querySelector('.game_area_purchase_game_wrapper') ||
      document.querySelector('.game_purchase_action_bg') ||
      document.querySelector('.game_purchase_action') ||
      document.querySelector('.btn_addtocart')?.parentElement ||
      document.querySelector('[class*="purchase"]');

    if (!container) { backendLog('No suitable container for luamanager button'); return; }

    const btnContainer = document.createElement('div');
    btnContainer.className = 'btn_addtocart btn_packageinfo luamanager-button-container';

    const button = document.createElement('span');
    button.setAttribute('data-panel', '{"focusable":true,"clickOnActivate":true}');
    button.setAttribute('role', 'button');
    button.className = 'btn_blue_steamui btn_medium';
    button.style.marginLeft = '2px';

    const buttonSpan = document.createElement('span'); buttonSpan.textContent = 'Add Game';
    button.appendChild(buttonSpan); btnContainer.appendChild(button);

    setSteamTooltip(button, 'Download and install game files');

    button.onclick = () => {
      if (state.run.inProgress) return;
      state.setRunState(true, appId);
      button.style.pointerEvents = 'none'; buttonSpan.textContent = 'Loading...'; button.style.opacity = '0.7';
      const modal = showDownloadModal(); if (modal) modal.status.textContent = 'Starting...';
      Millennium.callServerMethod('luamanager', 'addViaLuaManager', { appid: appId })
        .then((raw) => {
          const r = typeof raw === 'string' ? JSON.parse(raw) : raw;
          if (!r.success && modal) modal.status.textContent = `Error: ${r.error || 'Unknown error'}`;
        })
        .catch((e) => { if (modal) modal.status.textContent = `Error: ${e?.message || e || 'Unknown error'}`; })
        .finally(() => { startProgressMonitoring(appId); });
    };

    container.appendChild(btnContainer);
  }

  function createRemoveButton(appId) {
    const container =
      document.querySelector('.game_area_purchase_game_wrapper .game_purchase_action_bg') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .game_purchase_action_bg') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .game_purchase_action') ||
      document.querySelector('.game_area_purchase_game:not(.demo_above_purchase) .btn_addtocart')?.parentElement ||
      document.querySelector('.game_area_purchase_game_wrapper') ||
      document.querySelector('.game_purchase_action_bg') ||
      document.querySelector('.game_purchase_action') ||
      document.querySelector('.btn_addtocart')?.parentElement ||
      document.querySelector('[class*="purchase"]');

    if (!container) { backendLog('No suitable container for luamanager remove'); return; }

    const btnContainer = document.createElement('div'); btnContainer.className = 'btn_addtocart btn_packageinfo luamanager-button-container';
    const button = document.createElement('span');
    button.setAttribute('data-panel', '{"focusable":true,"clickOnActivate":true}');
    button.setAttribute('role', 'button');
    button.className = 'btn_blue_steamui btn_medium'; button.style.marginLeft = '2px';

    const buttonSpan = document.createElement('span'); buttonSpan.textContent = 'Remove Game';
    button.appendChild(buttonSpan); btnContainer.appendChild(button);
    setSteamTooltip(button, 'Remove game from library');

    button.onclick = () => {
      if (state.run.inProgress) return;
      state.setRunState(true, appId);
      button.style.pointerEvents = 'none'; buttonSpan.textContent = 'Removing...'; button.style.opacity = '0.7';
      Api.call('RemoveViaLuaManager', { appid: appId })
        .then((r) => {
          if (r.success) {
            state.cache.delete(`appExists_${appId}`);
            document.querySelector('#luamanager-in-library-banner')?.remove();
            document.querySelectorAll('.package_in_library_flag').forEach(f => f.remove());
            document.querySelector('.luamanager-button-container')?.remove();
            setTimeout(addLuaManagerButton, 100);
          }
        })
        .catch(() => { })
        .finally(() => { state.setRunState(false); });
    };

    container.appendChild(btnContainer);
  }

  function getCurrentAppId() {
    const m = window.location.href.match(/\/app\/(\d+)/);
    if (m) return parseInt(m[1]);
    const d = document.querySelector('[data-appid]');
    if (d) return parseInt(d.getAttribute('data-appid'));
    return null;
  }

  function getGameName() {
    const el = document.querySelector('.apphub_AppName') ||
      document.querySelector('.pageheader .breadcrumbs h1') ||
      document.querySelector('h1') || document.querySelector('title');
    if (!el) return 'This game';
    let name = el.textContent || el.innerText || '';
    return (name.replace(/\s+on\s+Steam$/i, '').trim()) || 'This game';
  }

  function showLibraryBanners() {
    if (document.querySelector('#luamanager-in-library-banner')) return;
    const gameName = getGameName();
    const queue = document.querySelector('#queueActionsCtn');
    if (queue) queue.insertAdjacentElement('afterend', createInLibraryBanner(gameName));
    const btn = document.querySelector('.luamanager-button-container');
    if (btn) {
      const sec = btn.closest('.game_area_purchase_game');
      if (sec && !sec.classList.contains('demo_above_purchase')) addInLibraryFlag(sec);
    }
  }

  function addLuaManagerButton() {
    try {
      const appId = getCurrentAppId();
      if (!appId) { if (!state.logs.missingOnce) state.logs.missingOnce = true; return; }
      if (!state.logs.existsOnce) state.logs.existsOnce = true;

      Api.hasLua(appId).then(exists => {
        if (exists) {
          if (!document.querySelector('.luamanager-button-container')) createRemoveButton(appId);
          showLibraryBanners();
        } else {
          if (!document.querySelector('.luamanager-button-container')) createAndInjectButton(appId);
        }
      }).catch(() => {
        if (!document.querySelector('.luamanager-button-container')) createAndInjectButton(appId);
      });
    } catch (e) { backendLog(`addLuaManagerButton error: ${e}`); }
  }

  function startProgressMonitoring(appid) {
    let done = false;

    const timer = setInterval(async () => {
      if (done) { clearInterval(timer); return; }
      try {
        const response = await Millennium.callServerMethod('luamanager', 'GetStatus', { appid });
        const payload = JSON.parse(response);
        const st = payload?.state || {};

        // Re-query elements each tick so we always have live references
        const overlay = document.querySelector('.luamanager-overlay');
        const title   = overlay?.querySelector('.luamanager-title');
        const status  = overlay?.querySelector('.luamanager-status');
        const wrap    = overlay?.querySelector('.luamanager-progress');
        const bar     = overlay?.querySelector('.luamanager-progress-bar');
        const percent = overlay?.querySelector('.luamanager-percent');

        if (title) title.textContent = st.currentApi ? `LuaManager - ${st.currentApi}` : 'LuaManager';
        const map = {
          'checking': st.currentApi ? `Checking ${st.currentApi}…` : 'Checking availability…',
          'checking_availability': 'Checking endpoints…',
          'queued': 'Initializing download…',
          'downloading': st.endpoint ? `Downloading from ${st.endpoint}…` : 'Downloading package…',
          'processing': 'Processing package…',
          'extracting': 'Extracting LUA…',
          'installing': st.installedFiles ? `Installing (${st.installedFiles.length} files)…` : 'Installing…',
          'done': 'Installation complete!',
          'failed': st.error || 'Download failed'
        };
        const text = map[st.status] || st.status || 'Processing…';
        if (status) status.textContent = text;

        if (wrap && bar && percent && ['downloading', 'processing', 'extracting', 'installing'].includes(st.status)) {
          wrap.style.display = 'block'; percent.style.display = 'block';
          let pct = 0;
          if (st.status === 'downloading') {
            const t = st.totalBytes || 0, r = st.bytesRead || 0;
            pct = t > 0 ? Math.floor((r / t) * 100) : (r ? 1 : 0);
          } else if (st.status === 'processing') pct = 25;
          else if (st.status === 'extracting') pct = 60;
          else if (st.status === 'installing') pct = 90;
          bar.style.width = `${Math.min(100, Math.max(0, pct))}%`;
          percent.textContent = `${Math.min(100, Math.max(0, pct))}%`;
        }

        if (st.status === 'done') {
          done = true;
          clearInterval(timer);
          if (bar && percent && status) { bar.style.width = '100%'; percent.textContent = '100%'; status.textContent = 'Game Added!'; }
          state.setRunState(false); state.cache.delete(`appExists_${appid}`);
          setTimeout(() => {
            document.querySelector('.luamanager-overlay')?.remove();
            document.querySelector('.luamanager-button-container')?.remove();
            setTimeout(() => { addLuaManagerButton(); }, 500);
          }, 1200);
        }

        if (st.status === 'failed') {
          done = true;
          clearInterval(timer);
          if (status) status.textContent = `Failed: ${st.error || 'Unknown error'}`;
          if (wrap) wrap.style.display = 'none';
          if (percent) percent.style.display = 'none';
          state.setRunState(false);
        }
      } catch (e) { backendLog(`progress error: ${e}`); }
    }, 300);
  }


  function ensureCompatCss() {
    if (document.getElementById('luamanager-compat-css')) return;
    const style = document.createElement('style');
    style.id = 'luamanager-compat-css';
    style.textContent = `
      .luamanager-compat-badge { display:inline-flex; align-items:center; gap:8px; font-size:13px; line-height:18px; padding:4px 10px; border-radius:12px; background:rgba(0,0,0,.3); border:1px solid rgba(255,255,255,.12); user-select:none; backdrop-filter: blur(2px); }
      .luamanager-compat-dot { width:9px; height:9px; border-radius:50%; flex:0 0 9px; background:#888; }
      .luamanager-compat-wrap { margin-left:0; display:block; vertical-align:middle; margin-top:6px; }
      .luamanager-compat-badge[title] { cursor:help; }
    `;
    document.head.appendChild(style);
  }

  function norm(s) {
    try { return (s || '').toString().normalize('NFD').replace(/\p{Diacritic}/gu, '').toLowerCase(); } catch { return (s || '').toString().toLowerCase(); }
  }

  function uniqueNormList(list) {
    const out = [];
    const seen = new Set();
    for (const x of list) {
      const n = norm(x).trim();
      if (!n) continue;
      if (!seen.has(n)) { seen.add(n); out.push(n); }
    }
    return out;
  }

  function collectStructured() {
    const tagNodes = document.querySelectorAll('.glance_tags .app_tag, .popular_tags .app_tag, #category_block a, #category_block .label');
    const specNodes = document.querySelectorAll('.game_area_details_specs a.name, .game_area_details_specs li, .game_area_features_list li');
    const noticeNodes = document.querySelectorAll('.DRM_notice, .game_meta_data, .glance_ctn, .game_area_purchase');

    const tags = uniqueNormList(Array.from(tagNodes).map(n => n.textContent || n.innerText || ''));
    const specs = uniqueNormList(Array.from(specNodes).map(n => n.textContent || n.innerText || ''));
    const noticesText = norm(Array.from(noticeNodes).map(n => n.innerText || n.textContent || '').join(' \n '));

    return { tags, specs, noticesText };
  }

  function analyzeCompat() {
    const { tags, specs, noticesText } = collectStructured();

    const ONLINE_TERMS = [
      'online pvp', 'online co-op', 'co-op online', 'cooperativo en linea', 'cooperativo en linea', 'multijugador en linea',
      'multiplayer online', 'massively multiplayer', 'mmo', 'mmorpg', 'cross-platform multiplayer', 'crossplay', 'cross-play',
      'pvp en linea', 'jcj en linea', 'pve en linea', 'requires internet connection', 'requiere conexion', 'requiere conexion a internet',
      'always online', 'live service', 'games as a service', 'servicio en linea'
    ];
    const SINGLE_PLAYER_TERMS = ['single-player', 'un jugador', 'single player'];
    const DRM_TERMS = ['requires 3rd-party drm', 'third-party drm', 'drm de terceros', 'denuvo', 'secucrom', 'securom', 'arxan', 'vmprotect', 'dmm drm', 'xadrs drm', 'rockstar launcher drm', 'proteccion denuvo'];
    const ACCOUNT_TERMS = ['requires 3rd-party account', '3rd-party account', 'cuenta de terceros', 'requiere cuenta', 'ea account', 'ea app', 'ea play', 'ubisoft connect', 'uplay', 'rockstar social club', 'battle.net', 'bethesda.net', '2k account', 'epic account', 'riot account', 'bnet'];

    const inList = (list, terms) => list.some(x => terms.some(t => x.includes(t)));
    const hasOnline = inList(tags, ONLINE_TERMS) || inList(specs, ONLINE_TERMS);
    const hasSingle = inList(tags, SINGLE_PLAYER_TERMS) || inList(specs, SINGLE_PLAYER_TERMS);

    let level = 'ok';
    const reasons = [];

    if (DRM_TERMS.some(t => noticesText.includes(t))) {
      level = 'bad';
      reasons.push('Third-party DRM detected (bypass required).');
    }

    if (ACCOUNT_TERMS.some(t => noticesText.includes(t)) || inList(tags, ACCOUNT_TERMS) || inList(specs, ACCOUNT_TERMS)) {
      if (level !== 'bad') level = 'warn';
      reasons.push('Requires a third-party account (license verification may fail; bypass may be required).');
    }

    if (hasOnline) {
      if (level !== 'bad') level = 'warn';
      reasons.push('Online/multiplayer content (may not work).');
    }

    if (level === 'ok' && hasSingle && !hasOnline) {
    }

    const labels = { ok: 'Works', warn: 'May not work', bad: "Doesn't work without bypass" };
    const colors = { ok: '#5c7e10', warn: '#a0790b', bad: '#a0352c' };

    return { level, label: labels[level], color: colors[level], reasons };
  }

  function makeBadge(info) {
    const wrap = document.createElement('span');
    wrap.className = 'luamanager-compat-wrap';
    const badge = document.createElement('span');
    badge.className = 'luamanager-compat-badge';
    badge.title = (info.reasons.length ? info.reasons.join(' • ') : info.label);
    const dot = document.createElement('span');
    dot.className = 'luamanager-compat-dot';
    dot.style.background = info.color;
    const text = document.createElement('span');
    text.textContent = `Compatibility: ${info.label}`;
    badge.appendChild(dot);
    badge.appendChild(text);
    wrap.appendChild(badge);
    const titleEl = document.querySelector('#appHubAppName, .apphub_AppName');
    if (titleEl && titleEl.parentElement) {
      titleEl.parentElement.style.display = 'flex';
      titleEl.parentElement.style.flexDirection = 'column';
      titleEl.parentElement.insertBefore(wrap, titleEl.nextSibling);
      return wrap;
    }
    const buy = document.querySelector('.game_area_purchase_game');
    if (buy) {
      const holder = document.createElement('div');
      holder.style.margin = '6px 0 2px 0';
      holder.appendChild(wrap);
      buy.prepend(holder);
      return wrap;
    }
    return wrap;
  }

  function renderCompatibilityBadge() {
    ensureCompatCss();
    const existing = document.querySelector('.luamanager-compat-wrap');
    if (existing) return;
    const info = analyzeCompat();
    makeBadge(info);
  }

  ensureStyles();
  addLuaManagerButton();
  renderCompatibilityBadge();
  setTimeout(addLuaManagerButton, 1000);
  setTimeout(addLuaManagerButton, 3000);
  setTimeout(renderCompatibilityBadge, 500);
  setTimeout(renderCompatibilityBadge, 2000);
  if (typeof MutationObserver !== 'undefined') {
    new MutationObserver(() => { addLuaManagerButton(); renderCompatibilityBadge(); }).observe(document.body, { childList: true, subtree: true });
  }
})();

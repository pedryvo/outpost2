/**
 * Outpost 2: Divided Destiny — Service Worker & Offline Cache Subsystem
 *
 * Implements a robust Cache-First offline strategy for:
 * - WebAssembly engine binary (.wasm)
 * - JavaScript glue and runtime loader (.js)
 * - HTML5 retro shell and UI layout (index.html)
 * - PWA manifest and vector/raster icons
 * - Authentic legacy audio sound bank (.wav assets)
 *
 * Ensures full offline playability on Desktop Chrome/Firefox/Edge/Safari, iOS Safari, and Android Chrome.
 */

const CACHE_VERSION = 'outpost2-classic-interface-v2';
const CACHE_NAME = `outpost2-cache-${CACHE_VERSION}`;

// Pre-cached core assets
const PRECACHE_ASSETS = [
  './',
  './index.html',
  './manifest.json',
  './icons/icon-192.png',
  './icons/icon-512.png',
  './assets/terrain.png',
  './assets/terrain_colors.bin',
  './assets/op2_art.prt',
  './assets/OP2_ART.BMP',
  // Audio Sound Bank WAV assets (96 legacy samples)
  'assets/audio/acid2.wav',
  'assets/audio/ag_sel.wav',
  'assets/audio/beep2.wav',
  'assets/audio/beep5.wav',
  'assets/audio/beep6.wav',
  'assets/audio/beep8.wav',
  'assets/audio/beep9.wav',
  'assets/audio/bigexp1.wav',
  'assets/audio/bigexp2.wav',
  'assets/audio/bigexp3.wav',
  'assets/audio/bld_exp.wav',
  'assets/audio/bld_make.wav',
  'assets/audio/bld_not.wav',
  'assets/audio/combhev.wav',
  'assets/audio/comblit.wav',
  'assets/audio/combmed.wav',
  'assets/audio/consvhc.wav',
  'assets/audio/dirt.wav',
  'assets/audio/dozr_sel.wav',
  'assets/audio/dump.wav',
  'assets/audio/evac_trn.wav',
  'assets/audio/fac_sel.wav',
  'assets/audio/foam.wav',
  'assets/audio/fuse.wav',
  'assets/audio/gar_sel.wav',
  'assets/audio/geo_sel.wav',
  'assets/audio/geyser.wav',
  'assets/audio/guardsel.wav',
  'assets/audio/laser.wav',
  'assets/audio/medexp1.wav',
  'assets/audio/medexp2.wav',
  'assets/audio/message2.wav',
  'assets/audio/micfire.wav',
  'assets/audio/microbe.wav',
  'assets/audio/microwve.wav',
  'assets/audio/mis_fire.wav',
  'assets/audio/quake.wav',
  'assets/audio/railfir.wav',
  'assets/audio/railtar.wav',
  'assets/audio/repvhc.wav',
  'assets/audio/robmine.wav',
  'assets/audio/robosel.wav',
  'assets/audio/robsurv.wav',
  'assets/audio/savant14.wav',
  'assets/audio/savant17.wav',
  'assets/audio/savant18.wav',
  'assets/audio/savant19.wav',
  'assets/audio/savant2.wav',
  'assets/audio/savant20.wav',
  'assets/audio/savant21.wav',
  'assets/audio/savant23.wav',
  'assets/audio/savant24.wav',
  'assets/audio/savant25.wav',
  'assets/audio/savant26.wav',
  'assets/audio/savant27.wav',
  'assets/audio/savant28.wav',
  'assets/audio/savant29.wav',
  'assets/audio/savant3.wav',
  'assets/audio/savant30.wav',
  'assets/audio/savant4.wav',
  'assets/audio/savant65.wav',
  'assets/audio/savant66.wav',
  'assets/audio/savant67.wav',
  'assets/audio/savant68.wav',
  'assets/audio/savant70.wav',
  'assets/audio/savant71.wav',
  'assets/audio/savant74.wav',
  'assets/audio/savant75.wav',
  'assets/audio/savant76.wav',
  'assets/audio/savant77.wav',
  'assets/audio/savant79.wav',
  'assets/audio/savant80.wav',
  'assets/audio/savnt101.wav',
  'assets/audio/savnt205.wav',
  'assets/audio/savnt206.wav',
  'assets/audio/savnt210.wav',
  'assets/audio/savnt211.wav',
  'assets/audio/savnt215.wav',
  'assets/audio/savnt223.wav',
  'assets/audio/savnt226.wav',
  'assets/audio/savnt227.wav',
  'assets/audio/savnt267.wav',
  'assets/audio/savnt273.wav',
  'assets/audio/savnt274.wav',
  'assets/audio/savnt276.wav',
  'assets/audio/scout.wav',
  'assets/audio/smalxp1.wav',
  'assets/audio/smalxp2.wav',
  'assets/audio/smalxp3.wav',
  'assets/audio/spam1.wav',
  'assets/audio/spidsel.wav',
  'assets/audio/thor.wav',
  'assets/audio/thunder.wav',
  'assets/audio/tornado2.wav',
  'assets/audio/truk_sel.wav',
  'assets/audio/uni_grad.wav'
];

// Install: Pre-cache shell assets with fault tolerance
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then(async (cache) => {
      console.log('[SW] Pre-caching core Outpost 2 application shell and audio bank...');
      // Use individual fetches with Promise.allSettled to ensure non-fatal failures don't block install
      const cachePromises = PRECACHE_ASSETS.map(async (url) => {
        try {
          const response = await fetch(url, { cache: 'no-cache' });
          if (response.ok) {
            await cache.put(url, response);
          } else {
            console.warn(`[SW] Warning: Optional asset failed to pre-cache (${response.status}): ${url}`);
          }
        } catch (err) {
          console.warn(`[SW] Pre-cache fetch bypassed for ${url}:`, err);
        }
      });
      await Promise.allSettled(cachePromises);
      console.log('[SW] Pre-caching finished successfully.');
    }).then(() => self.skipWaiting())
  );
});

// Activate: Purge obsolete cache generations and claim control
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((existingCache) => {
          if (existingCache !== CACHE_NAME) {
            console.log('[SW] Purging outdated cache:', existingCache);
            return caches.delete(existingCache);
          }
        })
      );
    }).then(() => {
      console.log('[SW] Claiming clients for immediate offline capability');
      return self.clients.claim();
    })
  );
});

// Fetch: Cache-First strategy with runtime dynamic caching for WASM and JS
self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET') {
    return;
  }

  const url = new URL(event.request.url);

  // Ignore browser extensions or non-http protocols
  if (!url.protocol.startsWith('http')) {
    return;
  }

  if (event.request.mode === 'navigate') {
    event.respondWith(
      fetch(event.request).then((response) => {
        if (response.ok) {
          const copy = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, copy));
        }
        return response;
      }).catch(async () =>
        (await caches.match(event.request)) ||
        (await caches.match('./index.html')) ||
        caches.match('./')
      )
    );
    return;
  }

  event.respondWith(
    caches.match(event.request, { ignoreSearch: false }).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      // If not in cache, fetch from network and store copy in cache
      return fetch(event.request).then((networkResponse) => {
        if (!networkResponse || networkResponse.status !== 200) {
          return networkResponse;
        }

        // Cache successful responses for WASM, JS, CSS, fonts, audio, maps
        const shouldCache =
          event.request.destination === 'script' ||
          event.request.destination === 'style' ||
          event.request.destination === 'image' ||
          event.request.destination === 'font' ||
          url.pathname.endsWith('.wasm') ||
          url.pathname.endsWith('.js') ||
          url.pathname.endsWith('.wav') ||
          url.pathname.endsWith('.map') ||
          url.pathname.endsWith('.prt') ||
          url.pathname.toLowerCase().endsWith('.bmp') ||
          url.pathname.endsWith('.json') ||
          event.request.mode === 'navigate';

        if (shouldCache) {
          const responseToCache = networkResponse.clone();
          caches.open(CACHE_NAME).then((cache) => {
            cache.put(event.request, responseToCache);
          });
        }

        return networkResponse;
      }).catch((fetchErr) => {
        // If network failed and navigating, return cached root/index.html
        if (event.request.mode === 'navigate') {
          return caches.match('./index.html').then((indexFallback) => {
            return indexFallback || caches.match('./');
          });
        }
        throw fetchErr;
      });
    })
  );
});

// Message handler for manual skipWaiting from web UI
self.addEventListener('message', (event) => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
});

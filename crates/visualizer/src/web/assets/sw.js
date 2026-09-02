var cacheName = 'dip-visualizer-pwa';
var filesToCache = [
  './',
  './index.html',
  './manifest.json',
  './assets/favicon-192x192.png',
  './assets/favicon-512x512.png',
  './assets/apple-touch-icon.png',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener('activate', function (e) {
  e.waitUntil(
    caches.keys().then(function (cacheNames) {
      return Promise.all(
        cacheNames
          .filter(function (name) {
            return name !== cacheName;
          })
          .map(function (name) {
            return caches.delete(name);
          })
      );
    })
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  if (e.request.method !== 'GET') {
    return;
  }

  e.respondWith(
    caches.match(e.request).then(function (response) {
      if (response) {
        return response;
      }

      return fetch(e.request).then(function (networkResponse) {
        var responseToCache = networkResponse.clone();
        caches.open(cacheName).then(function (cache) {
          cache.put(e.request, responseToCache);
        });
        return networkResponse;
      });
    })
  );
});

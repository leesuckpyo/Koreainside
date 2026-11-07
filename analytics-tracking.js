/* Korea Inside cookieless Matomo pageview tracker. Respect opt-out and browser privacy signals. */
(function (window, document) {
  'use strict';

  if (window.__kiaMatomoPageviewCanary) return;
  window.__kiaMatomoPageviewCanary = true;

  var location = window.location;
  var allowedHost = location.hostname === 'getkoreainside.com' ||
    location.hostname === 'www.getkoreainside.com';
  var publicPath = location.pathname.replace(/\/index\.html$/, '/');
  var allowedPath = /^\/(?:(?:es|es-mx|fr|de|id|ja|th|zh-tw)\/)?(?:[a-z0-9-]+\.html)?$/.test(publicPath);
  var canonical;
  try {
    canonical = new URL(document.querySelector('link[rel="canonical"]').href, location.href);
  } catch (_error) {
    return;
  }
  if (location.protocol !== 'https:' || !allowedHost || !allowedPath ||
      canonical.origin !== 'https://www.getkoreainside.com' ||
      canonical.pathname !== publicPath || canonical.search || canonical.hash) return;

  var dnt = String(window.navigator.doNotTrack || window.doNotTrack || '').toLowerCase();
  if (window.navigator.globalPrivacyControl === true || dnt === '1' || dnt === 'yes') return;

  var endpoint = 'https://analytics.getkoreainside.com';
  var queue = window._paq = window._paq || [];
  var optedOut = /(?:^|;\s*)mtm_consent_removed=/.test(document.cookie);
  var ready = false;
  var loading = false;
  var pageviewSent = false;

  // Privacy settings precede the automatic pageview.
  queue.push(['disableCookies']);
  queue.push(['setRequestMethod', 'POST']);
  queue.push(['setTrackerUrl', endpoint + '/matomo.php']);
  queue.push(['setSiteId', '1']);
  queue.push(['setCustomUrl', canonical.origin + canonical.pathname]);
  queue.push(['setDocumentTitle', String(document.title || '').replace(/[\u0000-\u001f\u007f]/g, '').slice(0, 200)]);

  if (document.referrer) {
    try {
      var referrer = new URL(document.referrer);
      if (referrer.protocol === 'https:' || referrer.protocol === 'http:') {
        queue.push(['setReferrerUrl', referrer.origin]);
      }
    } catch (_error) {
      // An invalid referrer is omitted rather than passed to Matomo.
    }
  }

  function recordPageview() {
    if (optedOut || !ready || pageviewSent) return;
    window._paq.push(['disableCookies']);
    window._paq.push(['trackPageView']);
    pageviewSent = true;
  }

  function loadLibrary() {
    if (loading || ready) return;
    loading = true;
    var script = document.createElement('script');
    script.async = true;
    script.referrerPolicy = 'no-referrer';
    script.src = endpoint + '/matomo.js';
    script.onload = function () {
      loading = false;
      ready = true;
      recordPageview();
    };
    script.onerror = function () {
      loading = false;
      script.remove();
    };
    document.head.appendChild(script);
  }

  function revoke() {
    optedOut = true;
    window._paq.push(['optUserOut']);
  }

  function enable() {
    if (!optedOut) return;
    window._paq.push(['forgetUserOptOut']);
    optedOut = false;
    if (ready) recordPageview();
    else loadLibrary();
  }

  window.KIAMatomoPageview = Object.freeze({
    revoke: revoke,
    enable: enable,
    isOptedOut: function () { return optedOut; }
  });
  document.addEventListener('kia:analytics-consent-revoked', revoke);
  if (!optedOut) loadLibrary();
}(window, document));

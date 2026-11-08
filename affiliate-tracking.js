(function () {
  "use strict";

  var initializedFlag = "__koreaInsideAffiliateTrackingInitialized";
  if (window[initializedFlag]) {
    return;
  }
  window[initializedFlag] = true;

  window.va = window.va || function () {
    (window.vaq = window.vaq || []).push(arguments);
  };

  function safeString(value) {
    return String(value || "").slice(0, 255);
  }

  function safeEventIdentifier(value) {
    var text = String(value || "").trim();
    return /^[A-Za-z0-9][A-Za-z0-9_-]{0,127}$/.test(text) ? text : "";
  }

  function trackMatomoAffiliateClick(link) {
    var hotel = link.closest("[data-hotel-id]");
    var hotelId = hotel ? safeEventIdentifier(hotel.dataset.hotelId) : "";
    var click = {
      provider: safeString(link.dataset.affiliateBrand).trim(),
      action: hotelId ? "hotel_cta_click" : "affiliate_outlink_click",
      hotelId: hotelId,
      placement: safeEventIdentifier(link.dataset.placement),
    };
    var api = window.KIAMatomoPageview;
    if (api && typeof api.trackAffiliateClick === "function") {
      api.trackAffiliateClick(click);
      return;
    }
    var queueName = "__kiaMatomoAffiliateQueue";
    var pending = Array.isArray(window[queueName]) ? window[queueName] : [];
    window[queueName] = pending;
    if (pending.length < 50) {
      pending.push(click);
    }
  }

  document.addEventListener("click", function (event) {
    var target = event.target;
    if (!(target instanceof Element)) {
      return;
    }

    var link = target.closest('a[data-affiliate-track="true"]');
    if (!link) {
      return;
    }

    try {
      var destinationHost = new URL(link.href, window.location.href).hostname;

      window.va("event", {
        name: "affiliate_click",
        data: {
          brand: safeString(link.dataset.affiliateBrand),
          page_path: safeString(window.location.pathname),
          page_category: safeString(link.dataset.pageCategory),
          content_topic: safeString(link.dataset.contentTopic),
          placement: safeString(link.dataset.placement),
          link_stage: safeString(link.dataset.linkStage),
          destination_host: safeString(destinationHost),
        },
      });
    } catch (_error) {
      // Analytics failures must never interrupt the link's default navigation.
    }

    try {
      trackMatomoAffiliateClick(link);
    } catch (_error) {
      // Matomo failures must never interrupt Vercel Analytics or navigation.
    }
  });
})();

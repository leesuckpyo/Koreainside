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
  });
})();

(function () {
  "use strict";

  var mapRoot = document.getElementById("budget-map");

  if (!mapRoot) {
    return;
  }

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yjf5zan";
  var SOURCE_LINKS = {
    coordinates: "https://www.arcgis.com/home/item.html?id=a3ca58b3ef864e61aab932c5c592e729",
    accessibility: "https://english.seoul.go.kr/service/movement/public-transportation/subway-accessibility-facilities/",
    arex: "https://www.arex.or.kr/"
  };

  var areas = {
    hongdae: {
      name: "Hongdae",
      center: { lat: 37.556748, lng: 126.923643 },
      zoom: 15,
      radius: 850,
      verdict: {
        title: "Hongdae budget verdict",
        bestFor: "Solo travelers, airport-rail users and travelers who want many accommodation choices.",
        savings: "Direct AREX access can reduce airport-transfer friction and simplify arrival.",
        watchOut: "Weekend prices, nightlife noise and the walking distance from the AREX platform to the hotel.",
        beforeBooking: "Confirm the exact exit, elevator, crossings and final outdoor walking route."
      }
    },
    gongdeok: {
      name: "Mapo / Gongdeok",
      center: { lat: 37.543592, lng: 126.951664 },
      zoom: 15,
      radius: 900,
      verdict: {
        title: "Mapo / Gongdeok budget verdict",
        bestFor: "Travelers with large luggage, light sleepers and visitors who prefer a calmer western base.",
        savings: "Gongdeok combines AREX with useful city connections while Mapo keeps central access practical.",
        watchOut: "Hotel choice is narrower than Hongdae, and a multi-line station still requires an exact exit check.",
        beforeBooking: "Confirm the AREX-to-lobby route, street elevator, taxi approach and luggage-storage rules."
      }
    },
    sinchon: {
      name: "Sinchon",
      center: { lat: 37.555153, lng: 126.93689 },
      zoom: 15,
      radius: 800,
      verdict: {
        title: "Sinchon budget verdict",
        bestFor: "Longer stays that benefit from student-priced meals and a repeatable neighborhood routine.",
        savings: "Daily food and practical services can keep recurring costs lower over a longer visit.",
        watchOut: "Airport travel is less direct, and property quality, room setup and luggage access vary.",
        beforeBooking: "Check which Sinchon station is meant, then verify laundry, stairs, storage and the final walk."
      }
    },
    euljiro: {
      name: "Euljiro / Myeongdong",
      center: { lat: 37.566292, lng: 126.990873 },
      zoom: 15,
      radius: 950,
      verdict: {
        title: "Euljiro / Myeongdong budget verdict",
        bestFor: "Short first trips, central sightseeing and shopping-led itineraries.",
        savings: "A central base can reduce repeated subway trips, long returns and shopping detours.",
        watchOut: "Nightly rates may be higher, while underground passages, crowds and the wrong exit complicate luggage.",
        beforeBooking: "Verify the station exit, elevator, airport-bus stop and final street route to the lobby."
      }
    }
  };

  var markers = [
    {
      id: "hongdae-arex",
      area: "hongdae",
      category: "airport",
      name: "Hongik University Station AREX",
      lat: 37.55774103,
      lng: 126.9264939,
      assessment: "Official airport-rail station point with direct all-stop train access.",
      travelerImpact: "Check the indoor walking distance from the AREX platform to the exit serving the hotel.",
      sourceType: "KRIC station coordinates and AREX official station network",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.arex
    },
    {
      id: "hongdae-line2",
      area: "hongdae",
      category: "subway",
      name: "Hongik University Station, Subway Line 2",
      lat: 37.556748,
      lng: 126.923643,
      assessment: "Line 2 station point for daily city travel from Hongdae.",
      travelerImpact: "The station is large, so compare the useful exit rather than measuring from the station label.",
      sourceType: "KRIC official-source station dataset",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.coordinates
    },
    {
      id: "hapjeong-elevator",
      area: "hongdae",
      category: "elevator",
      name: "Hapjeong Station",
      lat: 37.550025,
      lng: 126.914557,
      assessment: "Official Seoul accessibility data lists elevator facilities at this station.",
      travelerImpact: "Confirm which street elevator matches the hotel route before relying on it with luggage.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "mangwon-elevator",
      area: "hongdae",
      category: "elevator",
      name: "Mangwon Station",
      lat: 37.556031,
      lng: 126.910129,
      assessment: "Official Seoul accessibility data lists elevator facilities at this station.",
      travelerImpact: "Mangwon can extend the practical search area, but verify the final walk and street-level elevator.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "gongdeok-arex",
      area: "gongdeok",
      category: "airport",
      name: "Gongdeok Station AREX",
      lat: 37.54284055,
      lng: 126.9513301,
      assessment: "Official airport-rail station point between Incheon Airport and central Seoul.",
      travelerImpact: "A direct rail stop can simplify luggage days, but the interchange route remains important.",
      sourceType: "KRIC station coordinates and AREX official station network",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.arex
    },
    {
      id: "gongdeok-subway",
      area: "gongdeok",
      category: "subway",
      name: "Gongdeok Station, Subway network",
      lat: 37.543592,
      lng: 126.951664,
      assessment: "Multi-line station point supporting western and central Seoul movement.",
      travelerImpact: "Check the exact line, concourse and exit because transfer routes differ inside the station.",
      sourceType: "KRIC official-source station dataset",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.coordinates
    },
    {
      id: "mapo-elevator",
      area: "gongdeok",
      category: "elevator",
      name: "Mapo Station",
      lat: 37.539718,
      lng: 126.946043,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 5 station.",
      travelerImpact: "Verify the street elevator and hotel-side exit before choosing Mapo for easier luggage movement.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "aeogae-elevator",
      area: "gongdeok",
      category: "elevator",
      name: "Aeogae Station",
      lat: 37.553592,
      lng: 126.956733,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 5 station.",
      travelerImpact: "This expands the quieter search area, but compare the final walk with central Gongdeok.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "sinchon-line2",
      area: "sinchon",
      category: "subway",
      name: "Sinchon Station, Subway Line 2",
      lat: 37.555153,
      lng: 126.93689,
      assessment: "The main Line 2 station point serving the Sinchon commercial area.",
      travelerImpact: "Do not confuse it with the separate Gyeongui-Jungang Line station when checking a hotel route.",
      sourceType: "KRIC official-source station dataset",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.coordinates
    },
    {
      id: "ewha-elevator",
      area: "sinchon",
      category: "elevator",
      name: "Ewha Womans University Station",
      lat: 37.556734,
      lng: 126.945897,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 2 station.",
      travelerImpact: "The area can support a longer routine, but verify hills and the exact street-level route.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "sogang-university",
      area: "sinchon",
      category: "subway",
      name: "Sogang University Station",
      lat: 37.552132,
      lng: 126.935389,
      assessment: "Gyeongui-Jungang Line station point on the southern side of the Sinchon area.",
      travelerImpact: "Check which rail line and entrance the hotel description refers to before planning transfers.",
      sourceType: "KRIC official-source station dataset",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.coordinates
    },
    {
      id: "daeheung-elevator",
      area: "sinchon",
      category: "elevator",
      name: "Daeheung Station",
      lat: 37.547732,
      lng: 126.942214,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 6 station.",
      travelerImpact: "Daeheung can suit a quieter edge-of-area stay, but compare the walk with central Sinchon.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "euljiro1-elevator",
      area: "euljiro",
      category: "elevator",
      name: "Euljiro 1-ga Station",
      lat: 37.565998,
      lng: 126.982569,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 2 station.",
      travelerImpact: "Confirm the hotel-side street elevator because underground passages span several blocks.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "euljiro3-subway",
      area: "euljiro",
      category: "subway",
      name: "Euljiro 3-ga Station",
      lat: 37.566292,
      lng: 126.990873,
      assessment: "Line 2 station point at a central transfer area with block-by-block hotel conditions.",
      travelerImpact: "Check whether the useful exit reduces walking or adds an underground and street-level detour.",
      sourceType: "KRIC official-source station dataset",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.coordinates
    },
    {
      id: "myeongdong-elevator",
      area: "euljiro",
      category: "elevator",
      name: "Myeongdong Station",
      lat: 37.560955,
      lng: 126.986271,
      assessment: "Official Seoul accessibility data lists elevator facilities at this Line 4 station.",
      travelerImpact: "Crowded pavements and the wrong exit can still make a short map distance difficult with bags.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    },
    {
      id: "chungmuro-elevator",
      area: "euljiro",
      category: "elevator",
      name: "Chungmuro Station",
      lat: 37.561352,
      lng: 126.994573,
      assessment: "Official Seoul accessibility data lists elevator facilities at this central transfer station.",
      travelerImpact: "Use the exact entrance and line interchange route when comparing the eastern edge of this area.",
      sourceType: "KRIC coordinates and Seoul Metropolitan Government accessibility data",
      lastChecked: "2026-07",
      link: SOURCE_LINKS.accessibility
    }
  ];

  var categoryLabels = {
    airport: "Airport access",
    subway: "Subway and exits",
    elevator: "Elevator access"
  };

  var categorySymbols = {
    airport: "A",
    subway: "S",
    elevator: "E"
  };

  var categoryClasses = {
    airport: "budget-map-marker--airport",
    subway: "budget-map-marker--subway",
    elevator: "budget-map-marker--elevator"
  };

  var tabs = Array.prototype.slice.call(document.querySelectorAll("[data-map-area]"));
  var filterButtons = Array.prototype.slice.call(document.querySelectorAll("[data-map-category]"));
  var loadButton = document.getElementById("budget-map-load");
  var statusNode = document.getElementById("budget-map-status");
  var errorNode = document.getElementById("budget-map-error");
  var detailsNode = document.getElementById("budget-marker-details");
  var verdictPanel = document.getElementById("budget-map-verdict");
  var verdictTitle = document.getElementById("budget-map-verdict-title");
  var tabPanel = document.getElementById("budget-map-panel");
  var verdictFields = {
    bestFor: verdictPanel.querySelector('[data-verdict-field="bestFor"]'),
    savings: verdictPanel.querySelector('[data-verdict-field="savings"]'),
    watchOut: verdictPanel.querySelector('[data-verdict-field="watchOut"]'),
    beforeBooking: verdictPanel.querySelector('[data-verdict-field="beforeBooking"]')
  };

  var activeArea = "hongdae";
  var activeCategories = new Set(["airport", "subway", "elevator"]);
  var sdkPromise = null;
  var map = null;
  var mapMarkers = [];
  var areaCircle = null;
  var infoWindow = null;
  var resizeTimer = null;

  function getAreaMarkers(areaKey) {
    return markers.filter(function (marker) {
      return marker.area === areaKey;
    });
  }

  function updateFilterAvailability(areaKey) {
    var availableCategories = new Set(
      getAreaMarkers(areaKey).map(function (marker) {
        return marker.category;
      })
    );

    activeCategories = new Set();

    filterButtons.forEach(function (button) {
      var category = button.getAttribute("data-map-category");
      var available = availableCategories.has(category);

      button.hidden = !available;
      button.setAttribute("aria-pressed", available ? "true" : "false");

      if (available) {
        activeCategories.add(category);
      }
    });
  }

  function updateVerdict(areaKey) {
    var verdict = areas[areaKey].verdict;

    verdictTitle.textContent = verdict.title;
    verdictFields.bestFor.textContent = verdict.bestFor;
    verdictFields.savings.textContent = verdict.savings;
    verdictFields.watchOut.textContent = verdict.watchOut;
    verdictFields.beforeBooking.textContent = verdict.beforeBooking;
  }

  function resetMarkerDetails(areaKey) {
    var heading = document.createElement("h3");
    var copy = document.createElement("p");

    heading.textContent = areas[areaKey].name + " verified points";
    copy.textContent = "Select a marker to read the assessment, traveler impact, source type and last-checked date.";
    detailsNode.replaceChildren(heading, copy);
  }

  function showMarkerDetails(marker) {
    var heading = document.createElement("h3");
    var assessment = document.createElement("p");
    var impact = document.createElement("p");
    var metadata = document.createElement("p");
    var sourceLink = document.createElement("a");

    heading.textContent = marker.name;
    assessment.textContent = marker.assessment;
    impact.innerHTML = "<strong>Traveler impact:</strong> ";
    impact.appendChild(document.createTextNode(marker.travelerImpact));
    metadata.className = "budget-marker-details__meta";
    metadata.textContent = categoryLabels[marker.category] + " · " + marker.sourceType + " · Last checked " + marker.lastChecked;
    sourceLink.href = marker.link;
    sourceLink.target = "_blank";
    sourceLink.rel = "noopener noreferrer";
    sourceLink.textContent = "Open source";
    detailsNode.replaceChildren(heading, assessment, impact, metadata, sourceLink);
  }

  function markerIcon(marker) {
    return {
      content:
        '<button type="button" class="budget-map-marker ' +
        categoryClasses[marker.category] +
        '" aria-label="' +
        marker.name +
        ', ' +
        categoryLabels[marker.category] +
        '">' +
        '<span aria-hidden="true">' +
        categorySymbols[marker.category] +
        "</span></button>",
      anchor: new window.naver.maps.Point(22, 22)
    };
  }

  function clearMapObjects() {
    mapMarkers.forEach(function (marker) {
      marker.setMap(null);
    });

    mapMarkers = [];

    if (areaCircle) {
      areaCircle.setMap(null);
      areaCircle = null;
    }

    if (infoWindow) {
      infoWindow.close();
    }
  }

  function renderMapArea() {
    if (!map || !window.naver || !window.naver.maps) {
      return;
    }

    var area = areas[activeArea];
    var areaData = getAreaMarkers(activeArea).filter(function (marker) {
      return activeCategories.has(marker.category);
    });

    clearMapObjects();
    map.setCenter(new window.naver.maps.LatLng(area.center.lat, area.center.lng));
    map.setZoom(area.zoom);

    areaCircle = new window.naver.maps.Circle({
      map: map,
      center: new window.naver.maps.LatLng(area.center.lat, area.center.lng),
      radius: area.radius,
      strokeColor: "#3F5D52",
      strokeOpacity: 0.72,
      strokeWeight: 2,
      fillColor: "#3F5D52",
      fillOpacity: 0.18,
      clickable: false
    });

    areaData.forEach(function (markerData) {
      var marker = new window.naver.maps.Marker({
        map: map,
        position: new window.naver.maps.LatLng(markerData.lat, markerData.lng),
        title: markerData.name,
        icon: markerIcon(markerData)
      });

      window.naver.maps.Event.addListener(marker, "click", function () {
        showMarkerDetails(markerData);
      });

      mapMarkers.push(marker);
    });

    statusNode.textContent =
      areas[activeArea].name +
      " map ready. " +
      areaData.length +
      (areaData.length === 1 ? " verified point is shown." : " verified points are shown.");
  }

  function failMap() {
    map = null;
    statusNode.textContent = "The written area comparison remains available below.";
    errorNode.hidden = false;
    mapRoot.classList.add("budget-map-canvas--unavailable");
    mapRoot.replaceChildren();
  }

  function loadSdk() {
    if (window.naver && window.naver.maps) {
      return Promise.resolve();
    }

    if (sdkPromise) {
      return sdkPromise;
    }

    sdkPromise = new Promise(function (resolve, reject) {
      var existingScript = document.querySelector("script[data-budget-map-sdk]");
      var script = existingScript || document.createElement("script");
      var settled = false;
      var timeoutId = window.setTimeout(function () {
        if (!settled) {
          settled = true;
          reject(new Error("Map SDK timeout"));
        }
      }, 12000);

      function complete() {
        if (settled) {
          return;
        }

        settled = true;
        window.clearTimeout(timeoutId);

        if (window.naver && window.naver.maps) {
          resolve();
        } else {
          reject(new Error("Map SDK unavailable"));
        }
      }

      function fail() {
        if (settled) {
          return;
        }

        settled = true;
        window.clearTimeout(timeoutId);
        reject(new Error("Map SDK failed"));
      }

      script.addEventListener("load", complete, { once: true });
      script.addEventListener("error", fail, { once: true });

      if (!existingScript) {
        script.async = true;
        script.src = SDK_URL;
        script.setAttribute("data-budget-map-sdk", "");
        document.head.appendChild(script);
      }
    });

    return sdkPromise;
  }

  function initializeMap() {
    if (map) {
      return Promise.resolve();
    }

    loadButton.disabled = true;
    statusNode.textContent = "Loading the interactive map…";
    errorNode.hidden = true;

    return loadSdk()
      .then(function () {
        mapRoot.replaceChildren();
        map = new window.naver.maps.Map(mapRoot, {
          center: new window.naver.maps.LatLng(areas[activeArea].center.lat, areas[activeArea].center.lng),
          zoom: areas[activeArea].zoom,
          minZoom: 12,
          maxZoom: 19,
          zoomControl: true,
          zoomControlOptions: {
            position: window.naver.maps.Position.TOP_RIGHT
          },
          scaleControl: true,
          mapDataControl: false
        });
        infoWindow = new window.naver.maps.InfoWindow();
        mapRoot.classList.add("budget-map-canvas--ready");
        renderMapArea();
      })
      .catch(function () {
        failMap();
      });
  }

  function activateArea(areaKey, focusTab) {
    var nextTab = tabs.find(function (tab) {
      return tab.getAttribute("data-map-area") === areaKey;
    });

    if (!areas[areaKey] || !nextTab) {
      return;
    }

    activeArea = areaKey;

    tabs.forEach(function (tab) {
      var selected = tab === nextTab;
      tab.setAttribute("aria-selected", selected ? "true" : "false");
      tab.setAttribute("tabindex", selected ? "0" : "-1");
    });

    tabPanel.setAttribute("aria-labelledby", nextTab.id);
    updateFilterAvailability(areaKey);
    updateVerdict(areaKey);
    resetMarkerDetails(areaKey);

    if (map) {
      renderMapArea();
    } else {
      statusNode.textContent = areas[areaKey].name + " selected. Load the map to show verified points.";
    }

    if (focusTab) {
      nextTab.focus();
    }
  }

  tabs.forEach(function (tab, tabIndex) {
    tab.addEventListener("click", function () {
      activateArea(tab.getAttribute("data-map-area"), false);
    });

    tab.addEventListener("keydown", function (event) {
      var nextIndex = tabIndex;

      if (event.key === "ArrowRight") {
        nextIndex = (tabIndex + 1) % tabs.length;
      } else if (event.key === "ArrowLeft") {
        nextIndex = (tabIndex - 1 + tabs.length) % tabs.length;
      } else if (event.key === "Home") {
        nextIndex = 0;
      } else if (event.key === "End") {
        nextIndex = tabs.length - 1;
      } else {
        return;
      }

      event.preventDefault();
      activateArea(tabs[nextIndex].getAttribute("data-map-area"), true);
    });
  });

  filterButtons.forEach(function (button) {
    button.addEventListener("click", function () {
      var category = button.getAttribute("data-map-category");
      var isActive = button.getAttribute("aria-pressed") === "true";

      button.setAttribute("aria-pressed", isActive ? "false" : "true");

      if (isActive) {
        activeCategories.delete(category);
      } else {
        activeCategories.add(category);
      }

      if (map) {
        renderMapArea();
      }
    });
  });

  loadButton.addEventListener("click", function () {
    initializeMap();
  });

  window.addEventListener("resize", function () {
    if (!map || !window.naver || !window.naver.maps) {
      return;
    }

    window.clearTimeout(resizeTimer);
    resizeTimer = window.setTimeout(function () {
      window.naver.maps.Event.trigger(map, "resize");
      map.setCenter(new window.naver.maps.LatLng(areas[activeArea].center.lat, areas[activeArea].center.lng));
    }, 160);
  });

  updateFilterAvailability(activeArea);
  updateVerdict(activeArea);
  resetMarkerDetails(activeArea);
})();

(function () {
  "use strict";

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yif5zan&language=en";
  // Property points checked on 2026-11-07; not surveyed room entrances.
  // NAVER Place lookup remains NOT VERIFIED (tool access unavailable).
  // Trip.com coordinates below are precise hotelPositionInfo points, not map viewports.
  // 1. Marriott address: 366 Hangang-daero, Yongsan-gu.
  // https://www.marriott.com/en-us/hotels/selfp-four-points-josun-seoul-station/overview/
  // https://www.trip.com/hotels/seoul-hotel-detail-1919822/four-points-by-sheraton-josun-seoul-station/
  // 2. Hotelette booking property: 11 Huam-ro 60-gil, Jung-gu.
  // https://www.trip.com/hotels/seoul-hotel-detail-686683/hotelette-seoul-station/
  // 3. ZIYOLK official address: 17 Huam-ro 60-gil (Namdaemunno 5-ga 542), Jung-gu.
  // https://ziyolk.com/ZIYOLKSeoulStation
  // https://www.trip.com/hotels/seoul-hotel-detail-1171947/ziyolk-seoul-station/
  // 4. Hotel Manu: 19 Toegye-ro, Jung-gu; Seoul Tourism listing matches booking property.
  // https://english.visitseoul.net/hotels/view/3618306
  // https://www.trip.com/hotels/seoul-hotel-detail-988781/hotel-manu-seoul-station/
  // 5. Official hotel Google Maps link !3d/!4d property point, not @ viewport.
  // 12 Sejong-daero 12-gil, Jung-gu.
  // https://en.gracery.com/seoul/access/ -> https://maps.app.goo.gl/FXoBdj5kGDNRQLKH9
  // 6. The Seoul branch, 211 Mallijae-ro, Jung-gu; not Seoul Square.
  // Official branch and booked property match; room-specific entrances not verified.
  // https://web.stationbyuhc.com/kr/branch/9
  // https://www.trip.com/hotels/seoul-hotel-detail-81300866/uh-suite-the-seoul/
  // 7. Official hotel's named Google Maps !3d/!4d POI, not @ viewport.
  // 27 Chilpae-ro, Jung-gu.
  // https://www.seanhotelgroup.com/hotels/ramada-namdaemun/en/location
  // 8. Exact card booking property, 202 Mallijae-ro, Yongsan-gu; apartment room entrance not verified.
  // https://www.agoda.com/travel-house/hotel/seoul-kr.html
  // https://www.trip.com/hotels/seoul-hotel-detail-14968995/lv-xing-zhi-jia/
  var hotels = [
    {
      "id": "four-points-seoul-station",
      "lat": 37.55098,
      "lng": 126.972751,
      "number": 1,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "hotelette-seoul-station",
      "lat": 37.554676,
      "lng": 126.973828,
      "number": 2,
      "labelSide": "left",
      "labelY": 24
    },
    {
      "id": "ziyolk-seoul-station",
      "lat": 37.554815,
      "lng": 126.974228,
      "number": 3,
      "labelSide": "right",
      "labelY": -24
    },
    {
      "id": "hotel-manu-seoul",
      "lat": 37.557096,
      "lng": 126.974842,
      "number": 4,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "hotel-gracery-seoul",
      "lat": 37.5616151,
      "lng": 126.9768612,
      "number": 5,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "uh-suite-the-seoul",
      "lat": 37.556253,
      "lng": 126.968242,
      "number": 6,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "ramada-namdaemun",
      "lat": 37.5604462,
      "lng": 126.9718322,
      "number": 7,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "travel-house",
      "lat": 37.554953,
      "lng": 126.968377,
      "number": 8,
      "labelSide": "left",
      "labelY": 0
    }
  ];

  var section = document.getElementById("seoul-station-hotel-map");
  var canvas = document.getElementById("seoul-station-hotel-map-canvas");
  var status = document.getElementById("seoul-station-hotel-map-status");

  if (!section || !canvas || !status) {
    return;
  }

  var hotelById = {};
  var markers = {};
  var map = null;
  var selectedId = null;
  var sdkPromise = null;
  var authFailed = false;
  var assetPrefix = section.getAttribute("data-map-asset-prefix") || "";
  var pinPaths = {
    red: assetPrefix + "images/Accommodation/hongdae-hotel-pin-red.svg",
    blue: assetPrefix + "images/Accommodation/hongdae-hotel-pin-blue.svg"
  };
  var defaultCopy = {
    "default": "All 8 stay locations are shown. Select a pin or stay name.",
    "selected": "Selected: {name}. Pin {number} remains red; the other pins are blue.",
    "unavailable_selected": "Selected: {name}. NAVER map is unavailable, but the in-page stay link remains active.",
    "pending_selected": "Selected: {name}. The map will apply this selection when it is ready.",
    "fallback": "The interactive NAVER map could not load. Use the stay list to review all 8 options, then try again later.",
    "fallback_status": "NAVER map unavailable. Stay names and in-page links remain available.",
    "marker_aria": "Select {number}. {name}"
  };

  var cardSelector = '.seoul-station-page article.seoul-station-hotel[data-hotel-id]';
  var cardsMatch = document.querySelectorAll(cardSelector).length === hotels.length && hotels.every(function (hotel) {
    var cards = document.querySelectorAll(cardSelector + '[data-hotel-id="' + hotel.id + '"]');
    var heading = cards.length === 1 && cards[0].querySelector('h3');
    var controls = section.querySelectorAll('button[data-map-hotel="' + hotel.id + '"]');
    return heading && heading.textContent.trim() && controls.length === 1;
  });
  if (!cardsMatch) {
    showMapFailure();
    return;
  }

  hotels.forEach(function (hotel) {
    var heading = document.querySelector(cardSelector + '[data-hotel-id="' + hotel.id + '"] h3');
    hotel.name = heading.textContent.trim();
    section.querySelector('button[data-map-hotel="' + hotel.id + '"] > span:last-child').textContent = hotel.name;
    hotelById[hotel.id] = hotel;
  });

  function mapText(key, hotel) {
    var template = section.getAttribute("data-map-copy-" + key.replace(/_/g, "-")) || defaultCopy[key];
    if (!hotel) {
      return template;
    }
    return template.replace(/\{(name|number)\}/g, function (match, field) {
      return hotel[field];
    });
  }

  function escapeHtml(value) {
    return String(value)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function markerIcon(hotel) {
    var isSelected = selectedId === hotel.id;
    var color = selectedId === null || isSelected ? "red" : "blue";
    var sideClass = hotel.labelSide === "left" ? " seoul-station-hotel-map-marker--left" : "";
    var content = "";

    content += '<button type="button" class="seoul-station-hotel-map-marker' + sideClass + '"';
    content += ' data-map-marker="' + escapeHtml(hotel.id) + '"';
    content += ' aria-label="' + escapeHtml(mapText("marker_aria", hotel)) + '"';
    content += ' aria-pressed="' + (isSelected ? "true" : "false") + '"';
    content += ' style="--label-y:' + hotel.labelY + 'px">';
    content += '<img class="seoul-station-hotel-map-marker__pin" src="' + pinPaths[color] + '" alt="">';
    content += '<span class="seoul-station-hotel-map-marker__number" aria-hidden="true">' + hotel.number + "</span>";
    content += '<span class="seoul-station-hotel-map-marker__label" aria-hidden="true">' + escapeHtml(hotel.name) + "</span>";
    content += "</button>";

    return {
      content: content,
      size: new window.naver.maps.Size(36, 46),
      anchor: new window.naver.maps.Point(18, 39)
    };
  }


  // Reflow only the labels in screen pixels; marker coordinates and anchors stay fixed.
  function layoutLabels() {
    if (!map || !map.getProjection) {
      return;
    }
    var projection = map.getProjection();
    var width = canvas.clientWidth;
    var height = canvas.clientHeight;
    if (!projection || !width || !height) {
      return;
    }
    var entries = hotels.map(function (hotel) {
      var button = canvas.querySelector('[data-map-marker="' + hotel.id + '"]');
      return {
        hotel: hotel,
        button: button,
        label: button && button.querySelector('.seoul-station-hotel-map-marker__label'),
        point: projection.fromCoordToOffset(new window.naver.maps.LatLng(hotel.lat, hotel.lng))
      };
    });
    // Reserve pin hit areas and the top-right zoom controls before placing names.
    var occupied = entries.map(function (entry) {
      return { x: entry.point.x - 20, y: entry.point.y - 41, w: 40, h: 48 };
    });
    occupied.push({ x: width - 52, y: 0, w: 52, h: 180 });
    entries.sort(function (a, b) {
      return Number(b.hotel.id === selectedId) - Number(a.hotel.id === selectedId) || a.hotel.number - b.hotel.number;
    });
    entries.forEach(function (entry) {
      if (!entry.button || !entry.label) {
        return;
      }
      entry.button.removeAttribute('data-label-hidden');
      var labelWidth = entry.label.offsetWidth;
      var labelHeight = entry.label.offsetHeight;
      if (!labelWidth || !labelHeight) {
        return; // Mobile keeps the full names in the visible hotel directory.
      }
      var sides = entry.hotel.labelSide === 'left' ? ['left', 'right'] : ['right', 'left'];
      var offsets = [entry.hotel.labelY, 0, -48, 48, -96, 96, -144, 144];
      var chosen = null;
      for (var i = 0; i < offsets.length && !chosen; i += 1) {
        for (var j = 0; j < sides.length && !chosen; j += 1) {
          var box = {
            x: sides[j] === 'left' ? entry.point.x - 26 - labelWidth : entry.point.x + 26,
            y: entry.point.y - 19 - labelHeight / 2 + offsets[i],
            w: labelWidth,
            h: labelHeight
          };
          var inside = box.x >= 8 && box.y >= 8 && box.x + box.w <= width - 8 && box.y + box.h <= height - 28;
          var overlaps = occupied.some(function (other) {
            return box.x < other.x + other.w + 6 && box.x + box.w + 6 > other.x &&
              box.y < other.y + other.h + 6 && box.y + box.h + 6 > other.y;
          });
          if (inside && !overlaps) {
            chosen = { box: box, side: sides[j], offset: offsets[i] };
          }
        }
      }
      if (chosen) {
        entry.button.classList.toggle('seoul-station-hotel-map-marker--left', chosen.side === 'left');
        entry.button.style.setProperty('--label-y', chosen.offset + 'px');
        occupied.push(chosen.box);
      } else {
        // At a crowded zoom, keep the numbered pin and accessible name; retry on idle.
        entry.button.setAttribute('data-label-hidden', 'true');
      }
    });
  }

  var labelFrame = null;
  function scheduleLabels() {
    if (labelFrame !== null) {
      window.cancelAnimationFrame(labelFrame);
    }
    labelFrame = window.requestAnimationFrame(function () {
      labelFrame = null;
      layoutLabels();
    });
  }

  function updateSelectionControls() {
    section.querySelectorAll("[data-map-hotel]").forEach(function (control) {
      var active = control.getAttribute("data-map-hotel") === selectedId;
      control.setAttribute("aria-pressed", active ? "true" : "false");
    });

    document.querySelectorAll(cardSelector).forEach(function (card) {
      var active = card.getAttribute("data-hotel-id") === selectedId;
      card.toggleAttribute("data-map-selected", active);
    });
  }

  function updateMarkers() {
    if (!map || !window.naver || !window.naver.maps) {
      return;
    }

    var focusedMarker = document.activeElement && document.activeElement.getAttribute("data-map-marker");
    hotels.forEach(function (hotel) {
      var marker = markers[hotel.id];
      if (!marker) {
        return;
      }
      marker.setIcon(markerIcon(hotel));
      marker.setZIndex(selectedId === hotel.id ? 200 : hotel.number);
    });
    if (focusedMarker) {
      var nextFocus = canvas.querySelector('[data-map-marker="' + focusedMarker + '"]');
      if (nextFocus) {
        nextFocus.focus({ preventScroll: true });
      }
    }
    scheduleLabels();
  }

  function selectHotel(id, options) {
    var hotel = hotelById[id];
    var settings = options || {};

    if (!hotel) {
      return;
    }

    selectedId = hotel.id;
    updateSelectionControls();

    if (map && window.naver && window.naver.maps) {
      try {
        updateMarkers();

        if (settings.pan !== false) {
          map.panTo(new window.naver.maps.LatLng(hotel.lat, hotel.lng), {
            duration: 350,
            easing: "easeOutCubic"
          });
        }
      } catch (error) {
        showMapFailure();
      }
    }

    if (map) {
      status.textContent = mapText("selected", hotel);
    } else if (authFailed) {
      status.textContent = mapText("unavailable_selected", hotel);
    } else {
      status.textContent = mapText("pending_selected", hotel);
    }

    if (settings.focusList) {
      window.setTimeout(function () {
        var selectedControl = section.querySelector('[data-map-hotel="' + hotel.id + '"]');
        if (selectedControl) {
          selectedControl.focus({ preventScroll: true });
        }
      }, 450);
    }
  }

  function bindControls() {
    document.addEventListener("click", function (event) {
      var markerControl = event.target.closest("[data-map-marker]");
      if (markerControl) {
        event.preventDefault();
        selectHotel(markerControl.getAttribute("data-map-marker"));
        return;
      }

      var mapControl = event.target.closest("[data-map-hotel]");
      if (!mapControl) {
        return;
      }

      var isCardMapLink = mapControl.matches('a.seoul-station-hotel-map-link[href="#seoul-station-hotel-map"]');
      if (!isCardMapLink) {
        event.preventDefault();
      }

      selectHotel(mapControl.getAttribute("data-map-hotel"), {
        focusList: isCardMapLink
      });
    });
  }

  function showMapFailure() {
    authFailed = true;
    map = null;
    markers = {};
    canvas.innerHTML = '<p class="seoul-station-hotel-map-fallback">' + escapeHtml(mapText("fallback")) + "</p>";
    status.textContent = mapText("fallback_status");
  }

  function loadSdk() {
    if (window.naver && window.naver.maps && window.naver.maps.Map) {
      return Promise.resolve();
    }

    if (sdkPromise) {
      return sdkPromise;
    }

    sdkPromise = new Promise(function (resolve, reject) {
      var script = document.createElement("script");
      script.src = SDK_URL;
      script.async = true;
      script.dataset.seoulStationNaverMapSdk = "true";
      script.addEventListener("load", function () {
        if (window.naver && window.naver.maps && window.naver.maps.Map) {
          resolve();
          return;
        }
        reject(new Error("NAVER Maps SDK loaded without the map API."));
      });
      script.addEventListener("error", function () {
        reject(new Error("NAVER Maps SDK request failed."));
      });
      document.head.appendChild(script);
    });

    return sdkPromise;
  }

  function initializeMap() {
    if (authFailed || !window.naver || !window.naver.maps || !window.naver.maps.Map) {
      showMapFailure();
      return;
    }

    var bounds = new window.naver.maps.LatLngBounds();
    hotels.forEach(function (hotel) {
      bounds.extend(new window.naver.maps.LatLng(hotel.lat, hotel.lng));
    });

    map = new window.naver.maps.Map(canvas, {
      center: bounds.getCenter(),
      maxZoom: 19,
      zoomControl: true,
      zoomControlOptions: {
        position: window.naver.maps.Position.TOP_RIGHT
      }
    });

    hotels.forEach(function (hotel) {
      var position = new window.naver.maps.LatLng(hotel.lat, hotel.lng);
      var marker = new window.naver.maps.Marker({
        map: map,
        position: position,
        icon: markerIcon(hotel),
        title: hotel.name,
        zIndex: hotel.number
      });

      window.naver.maps.Event.addListener(marker, "click", function () {
        selectHotel(hotel.id);
      });

      markers[hotel.id] = marker;
    });

    function fitHotelBounds() {
      var padding = canvas.clientWidth <= 620 ? 48 : 120;
      map.fitBounds(bounds, {
        top: 72,
        right: padding,
        bottom: 48,
        left: padding,
        maxZoom: 16
      });
    }

    window.naver.maps.Event.addListener(map, "idle", scheduleLabels);
    fitHotelBounds();
    scheduleLabels();

    if (selectedId) {
      selectHotel(selectedId);
    } else {
      status.textContent = mapText("default");
    }

    window.addEventListener("resize", function () {
      if (map) {
        map.autoResize();
        if (!selectedId) {
          fitHotelBounds();
        }
        scheduleLabels();
      }
    });
  }

  bindControls();
  window.navermap_authFailure = function () {
    authFailed = true;
    showMapFailure();
  };
  loadSdk().then(initializeMap).catch(showMapFailure);
})();

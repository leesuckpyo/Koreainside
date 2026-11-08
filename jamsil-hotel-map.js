(function () {
  "use strict";

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yif5zan&language=en";
  // Building/property locations checked on 2026-11-08; individual entrances NOT VERIFIED.
  // NAVER POI direct inspection remains NOT VERIFIED (access limited).
  // Current Production Insadong runtime retains capture-phase clicks and focus restoration.
  // 1. Lotte Hotel World - 240 Olympic-ro, Songpa-gu, Seoul, 05554, South Korea
  // Coordinate source: https://maps.app.goo.gl/QRfDkfB8ZpQkRs3H7
  // Official Google hotel POI 3d/4d, exactly matches Trip.com; rejected complex-center candidate.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-988393/lotte-hotel-world/
  // 2. Sofitel Ambassador Seoul Hotel & Serviced Residences - 209 Jamsil-ro, Songpa-gu, Seoul, 05552, South Korea
  // Coordinate source: https://www.sofitel-seoul.com/en/location/
  // Official hotel-specific _hotel marker, preferred to differing generic Accor GPS; booking address identical.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-73711899/sofitel-ambassador-seoul-hotel-and-serviced-residences/
  // 3. Rosana Hotel - 98 Samhaksa-ro, Songpa, Songpa-gu, Seoul, 05612, South Korea
  // Coordinate source: https://www.trip.com/hotels/seoul-hotel-detail-1715441/rosana-business-and-boutique-hotel/
  // Exact matching Trip.com property hotelPositionInfo mapType gg; official address and embedded Rosana POI matched. Embedded viewport center not used.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-1715441/rosana-business-and-boutique-hotel/
  // 4. Delight Hotel Jamsil - 21-9 Ogeum-ro 11-gil, Bangi-dong, Songpa-gu, Seoul, 05543, South Korea
  // Coordinate source: http://www.hoteldelight.co.kr/view/viewLink.do?page=homepage/KOR/company/location
  // Official Kakao markerPosition rounded to six decimals; Google marker and matching OTA within a few metres.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-8024377/delight-hotel-jamsil
  // 5. Seoul Sangju Hotel - 19 Olympic-ro 8-gil, Songpa-gu, Seoul, 05561, South Korea
  // Coordinate source: https://sangjuhotel.jalib.site/content.html?home_no=10&menu_no=312
  // Official home_y/home_x hotel coordinates; same address and OTA property.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-17324371/sangju-hotel/
  // 6. Hotel Lake - 216 Seokchonhosu-ro, Songpa, Songpa-gu, Seoul, 05610, South Korea
  // Coordinate source: https://www.songpa.go.kr/culture/detailInfo.do?key=3845&rcpp=6&resrceCd=TR0230-1002889&sc1=TR0230
  // Songpa district resource location (resrceLa/resrceLo), rounded to six decimals; official address and OTA agree.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-2104894/lake-tourist-hotel/
  // 7. SIGNIEL Seoul - 300 Olympic-ro, Songpa-gu, Seoul, 05551, South Korea
  // Coordinate source: https://maps.app.goo.gl/syQxCD2m9K7B5RfT6
  // Official Google hotel POI 3d/4d inside Lotte World Tower, matches OTA to rounding.
  // Matching booking property: https://www.trip.com/hotels/seoul-hotel-detail-19624480/signiel-seoul/
  var hotels = [
    {
      "id": "lotte-hotel-world",
      "lat": 37.511534,
      "lng": 127.099981,
      "number": 1,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "sofitel",
      "lat": 37.51377,
      "lng": 127.10638,
      "number": 2,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "rosana",
      "lat": 37.505988,
      "lng": 127.097548,
      "number": 3,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "delight",
      "lat": 37.515388,
      "lng": 127.109312,
      "number": 4,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "seoul-sangju",
      "lat": 37.51007,
      "lng": 127.08055,
      "number": 5,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "hotel-lake",
      "lat": 37.50687,
      "lng": 127.100993,
      "number": 6,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "signiel",
      "lat": 37.5126039,
      "lng": 127.1025242,
      "number": 7,
      "labelSide": "left",
      "labelY": 0
    }
  ];

  var section = document.getElementById("jamsil-hotel-map");
  var canvas = document.getElementById("jamsil-hotel-map-canvas");
  var status = document.getElementById("jamsil-hotel-map-status");

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
    "default": "All 7 hotel locations are shown. Select a pin or hotel name.",
    "selected": "Selected: {name}. Pin {number} remains red; the other pins are blue.",
    "unavailable_selected": "Selected: {name}. NAVER map is unavailable, but the in-page hotel link remains active.",
    "pending_selected": "Selected: {name}. The map will apply this selection when it is ready.",
    "fallback": "The interactive NAVER map could not load. Use the hotel list to review all 7 options, then try again later.",
    "fallback_status": "NAVER map unavailable. Hotel names and in-page links remain available.",
    "marker_aria": "Select {number}. {name}"
  };

  var cardSelector = '.jamsil-page article.hm-hotel[data-hotel-id]';
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
    var sideClass = hotel.labelSide === "left" ? " jamsil-hotel-map-marker--left" : "";
    var content = "";

    content += '<button type="button" class="jamsil-hotel-map-marker' + sideClass + '"';
    content += ' data-map-marker="' + escapeHtml(hotel.id) + '"';
    content += ' aria-label="' + escapeHtml(mapText("marker_aria", hotel)) + '"';
    content += ' aria-pressed="' + (isSelected ? "true" : "false") + '"';
    content += ' style="--label-y:' + hotel.labelY + 'px">';
    content += '<img class="jamsil-hotel-map-marker__pin" src="' + pinPaths[color] + '" alt="">';
    content += '<span class="jamsil-hotel-map-marker__number" aria-hidden="true">' + hotel.number + "</span>";
    content += '<span class="jamsil-hotel-map-marker__label" aria-hidden="true">' + escapeHtml(hotel.name) + "</span>";
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
        label: button && button.querySelector('.jamsil-hotel-map-marker__label'),
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
        entry.button.classList.toggle('jamsil-hotel-map-marker--left', chosen.side === 'left');
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
    section.querySelectorAll("button[data-map-hotel]").forEach(function (control) {
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
    // Native button activation has no mousedown coordinates for the SDK click handler.
    // Own marker clicks before the SDK receives them, including Enter and Space clicks.
    canvas.addEventListener("click", function (event) {
      var markerControl = event.target.closest("[data-map-marker]");
      if (markerControl) {
        event.preventDefault();
        event.stopPropagation();
        selectHotel(markerControl.getAttribute("data-map-marker"));
      }
    }, true);

    document.addEventListener("click", function (event) {
      var mapControl = event.target.closest("[data-map-hotel]");
      if (!mapControl) {
        return;
      }

      var isCardMapLink = mapControl.matches('a.jamsil-hotel-map-link[href="#jamsil-hotel-map"]');
      var isStayLink = mapControl.matches("a.jamsil-hotel-map-stay-link");
      if (!isCardMapLink && !isStayLink) {
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
    canvas.innerHTML = '<p class="jamsil-hotel-map-fallback">' + escapeHtml(mapText("fallback")) + "</p>";
    status.textContent = mapText("fallback_status");
    section.querySelectorAll("button[data-map-hotel]").forEach(function (button) {
      var link = document.createElement("a");
      var id = button.getAttribute("data-map-hotel");
      link.className = "jamsil-hotel-map-stay-link";
      link.href = "#" + id;
      link.setAttribute("data-map-hotel", id);
      link.innerHTML = button.innerHTML;
      button.replaceWith(link);
    });
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
      script.dataset.jamsilNaverMapSdk = "true";
      var timeout = window.setTimeout(function () {
        reject(new Error("NAVER Maps SDK request timed out."));
      }, 15000);
      script.addEventListener("load", function () {
        window.clearTimeout(timeout);
        if (window.naver && window.naver.maps && window.naver.maps.Map) {
          resolve();
          return;
        }
        reject(new Error("NAVER Maps SDK loaded without the map API."));
      });
      script.addEventListener("error", function () {
        window.clearTimeout(timeout);
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

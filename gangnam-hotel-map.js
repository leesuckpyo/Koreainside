(function () {
  "use strict";

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yif5zan&language=en";
  // Property points checked on 2026-11-07. These are not surveyed entrances.
  // NAVER place-detail and live rendering remain NOT VERIFIED (access unavailable).
  // 1. https://all.accor.com/hotel/1633/index.en.shtml
  // Official GPS and hotel GeoCoordinates.
  // 2. https://dormy-hotels.com/dormyinn/hotels/gangnam/access/
  // Official embedded Google Maps named hotel POI, not its viewport.
  // 3. http://www.ocloudhotel.com/
  // Official embedded Google Maps named hotel POI; KTO independently places the same Seocho-gu hotel within 10m. HTTPS certificate invalid; official HTTP site linked by tourism listing read without disabling TLS.
  // 4. https://hotelcappuccino.co.kr/about/how-to-get-us/
  // Official embedded Google Maps named hotel POI, not iframe viewport.
  // 5. https://sodowehotels.com/ABOUTUS
  // Hotel operator official NAVER Marker position and matching hotel name/address; Marriott map point is about 12m away within the property.
  // 6. https://seoul.intercontinental.com/en/about-us/customer-service/directions
  // Official hotel directions JS marker-GIC overlay at position N(), not surrounding landmark markers.
  // 7. https://www.shillahotels.com/en/shillastay/samsung/introduction/location.do
  // Official Google Maps short link resolves named Samsung hotel !3d/!4d POI. Generic location text is incorrect; Samsung-specific about/footer address is 506 Yeongdong-daero.
  // 8. https://www.peytohotel.com/Samseong/m/view/mapDirection.do
  // Current official embedded Google Maps named Samseong hotel POI. Disused commented map point differs by 7m and was not used.
  // 9. https://access.visitkorea.or.kr/acm/detail.do?cotId=3432c60e-b604-4a9a-9015-c9c5966de21c
  // Korea Tourism Organization named property directions coordinates; Hyatt address independently agrees.
  // 10. https://medicaltour.gangnam.go.kr/2021/content/hotelDetail.do?hotel_seq=261
  // Gangnam-gu official hotel record data-dblat/data-dblng, not its generic FALLBACK center; IHG address agrees.
  var hotels = [
    {
      "id": "novotel",
      "name": "Novotel Ambassador Seoul Gangnam",
      "lat": 37.505258,
      "lng": 127.028829,
      "number": 1,
      "labelSide": "left",
      "labelY": 24
    },
    {
      "id": "dormy-inn",
      "name": "Dormy Inn Seoul Gangnam",
      "lat": 37.505624,
      "lng": 127.0294587,
      "number": 2,
      "labelSide": "right",
      "labelY": -24
    },
    {
      "id": "ocloud",
      "name": "Ocloud Hotel Gangnam",
      "lat": 37.5030438,
      "lng": 127.0238097,
      "number": 3,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "hotel-cappuccino",
      "name": "Hotel Cappuccino",
      "lat": 37.5067961,
      "lng": 127.0315248,
      "number": 4,
      "labelSide": "right",
      "labelY": -12
    },
    {
      "id": "ac-hotel",
      "name": "AC Hotel Seoul Gangnam",
      "lat": 37.5013002,
      "lng": 127.0354031,
      "number": 5,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "grand-intercontinental",
      "name": "Grand InterContinental Seoul Parnas",
      "lat": 37.508897,
      "lng": 127.060816,
      "number": 6,
      "labelSide": "left",
      "labelY": 24
    },
    {
      "id": "shilla-stay-samsung",
      "name": "Shilla Stay Samsung",
      "lat": 37.5099527,
      "lng": 127.0631277,
      "number": 7,
      "labelSide": "left",
      "labelY": -24
    },
    {
      "id": "hotel-peyto-samseong",
      "name": "Hotel Peyto Samseong",
      "lat": 37.508408,
      "lng": 127.058676,
      "number": 8,
      "labelSide": "left",
      "labelY": 0
    },
    {
      "id": "andaz",
      "name": "Andaz Seoul Gangnam",
      "lat": 37.52532163054886,
      "lng": 127.02886508625117,
      "number": 9,
      "labelSide": "right",
      "labelY": 0
    },
    {
      "id": "voco",
      "name": "voco Seoul Gangnam",
      "lat": 37.5177632,
      "lng": 127.0246113,
      "number": 10,
      "labelSide": "right",
      "labelY": 0
    }
  ];

  var section = document.getElementById("gangnam-hotel-map");
  var canvas = document.getElementById("gangnam-hotel-map-canvas");
  var status = document.getElementById("gangnam-hotel-map-status");

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
    default: "All 10 hotel locations are shown. Select a pin or hotel name.",
    selected: "Selected: {name}. Pin {number} remains red; the other hotel pins are blue.",
    unavailable_selected: "Selected: {name}. NAVER map is unavailable, but the in-page hotel link remains active.",
    pending_selected: "Selected: {name}. The map will apply this selection when it is ready.",
    fallback: "The interactive NAVER map could not load. Use the hotel list to review all 10 stays, then try again later.",
    fallback_status: "NAVER map unavailable. Hotel names and in-page links remain available.",
    marker_aria: "Select {number}. {name}"
  };

  hotels.forEach(function (hotel) {
    var heading = document.querySelector('[data-hotel-id="' + hotel.id + '"] h3');
    if (heading) {
      hotel.name = heading.textContent.trim();
    }
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
    var sideClass = hotel.labelSide === "left" ? " gangnam-map-marker--left" : "";
    var content = "";

    content += '<button type="button" class="gangnam-map-marker' + sideClass + '"';
    content += ' data-map-marker="' + escapeHtml(hotel.id) + '"';
    content += ' aria-label="' + escapeHtml(mapText("marker_aria", hotel)) + '"';
    content += ' aria-pressed="' + (isSelected ? "true" : "false") + '"';
    content += ' style="--label-y:' + hotel.labelY + 'px">';
    content += '<img class="gangnam-map-marker__pin" src="' + pinPaths[color] + '" alt="">';
    content += '<span class="gangnam-map-marker__number" aria-hidden="true">' + hotel.number + "</span>";
    content += '<span class="gangnam-map-marker__label" aria-hidden="true">' + escapeHtml(hotel.name) + "</span>";
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
        label: button && button.querySelector('.gangnam-map-marker__label'),
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
        entry.button.classList.toggle('gangnam-map-marker--left', chosen.side === 'left');
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

    document.querySelectorAll("[data-hotel-id]").forEach(function (card) {
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

      var isCardMapLink = mapControl.matches('a.gangnam-map-link[href="#gangnam-hotel-map"]');
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
    canvas.innerHTML = '<p class="gangnam-hotel-map-fallback">' + escapeHtml(mapText("fallback")) + "</p>";
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
      script.dataset.gangnamNaverMapSdk = "true";
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

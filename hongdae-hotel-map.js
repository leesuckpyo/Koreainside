(function () {
  "use strict";

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yif5zan&language=en";
  var hotels = [
    { id: "jsm-studio-hongdae", name: "JSM Studio Hongdae", lat: 37.556666, lng: 126.922948, number: 1, labelSide: "right", labelY: -34 },
    { id: "stay-here-again", name: "Stay Here, Again", lat: 37.556357772, lng: 126.921932074, number: 2, labelSide: "left", labelY: -18 },
    { id: "l7-hongdae", name: "L7 HONGDAE by LOTTE HOTELS", lat: 37.555753, lng: 126.921669, number: 3, labelSide: "left", labelY: 18 },
    { id: "ryse-autograph-collection", name: "RYSE, Autograph Collection", lat: 37.5541, lng: 126.921, number: 4, labelSide: "left", labelY: 38 },
    { id: "holiday-inn-express-seoul-hongdae", name: "Holiday Inn Express Seoul Hongdae", lat: 37.55757, lng: 126.92665, number: 5, labelSide: "right", labelY: -8 },
    { id: "mercure-ambassador-seoul-hongdae", name: "Mercure Ambassador Seoul Hongdae", lat: 37.555135, lng: 126.922511, number: 6, labelSide: "right", labelY: 34 },
    { id: "9-brick-hotel", name: "9 Brick Hotel", lat: 37.55378, lng: 126.9205, number: 7, labelSide: "right", labelY: 12 },
    { id: "junibino-hotel-hongdae", name: "Junibino Hotel Hongdae", lat: 37.5477285, lng: 126.9108814, number: 8, labelSide: "right", labelY: 0 },
    { id: "amanti-hotel-seoul", name: "Amanti Hotel Seoul", lat: 37.557178, lng: 126.918603, number: 9, labelSide: "left", labelY: 0 },
    { id: "localstitch-creator-town-seogyo", name: "Localstitch Creator Town Seogyo", lat: 37.5563, lng: 126.9168, number: 10, labelSide: "left", labelY: 24 },
    { id: "batwo-stay", name: "Batwo Stay - For foreigners only", lat: 37.558992733, lng: 126.920039919, number: 11, labelSide: "left", labelY: -34 },
    { id: "baroato-2nd", name: "Hotel Baroato 2nd", lat: 37.558684768, lng: 126.921659685, number: 12, labelSide: "right", labelY: -42 },
    { id: "hongdae-style-guesthouse", name: "Hongdae Style Guesthouse", lat: 37.557495681, lng: 126.920774789, number: 13, labelSide: "right", labelY: 12 },
    { id: "twotwo-house", name: "TwoTwo House", lat: 37.561507663, lng: 126.924881038, number: 14, labelSide: "right", labelY: 0 }
  ];

  var section = document.getElementById("hongdae-hotel-map");
  var canvas = document.getElementById("hongdae-hotel-map-canvas");
  var status = document.getElementById("hongdae-hotel-map-status");

  if (!section || !canvas || !status) {
    return;
  }

  var hotelById = {};
  var markers = {};
  var map = null;
  var selectedId = null;
  var sdkPromise = null;
  var authFailed = false;
  var pinPaths = {
    red: "images/Accommodation/hongdae-hotel-pin-red.svg",
    blue: "images/Accommodation/hongdae-hotel-pin-blue.svg"
  };

  hotels.forEach(function (hotel) {
    hotelById[hotel.id] = hotel;
  });

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
    var sideClass = hotel.labelSide === "left" ? " hongdae-map-marker--left" : "";
    var content = "";

    content += '<button type="button" class="hongdae-map-marker' + sideClass + '"';
    content += ' data-map-marker="' + escapeHtml(hotel.id) + '"';
    content += ' aria-label="Select ' + hotel.number + '. ' + escapeHtml(hotel.name) + '"';
    content += ' aria-pressed="' + (isSelected ? "true" : "false") + '"';
    content += ' style="--label-y:' + hotel.labelY + 'px">';
    content += '<img class="hongdae-map-marker__pin" src="' + pinPaths[color] + '" alt="">';
    content += '<span class="hongdae-map-marker__number" aria-hidden="true">' + hotel.number + "</span>";
    content += '<span class="hongdae-map-marker__label" aria-hidden="true">' + escapeHtml(hotel.name) + "</span>";
    content += "</button>";

    return {
      content: content,
      size: new window.naver.maps.Size(36, 46),
      anchor: new window.naver.maps.Point(18, 44)
    };
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

    hotels.forEach(function (hotel) {
      var marker = markers[hotel.id];
      if (!marker) {
        return;
      }
      marker.setIcon(markerIcon(hotel));
      marker.setZIndex(selectedId === hotel.id ? 200 : hotel.number);
    });
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
      status.textContent = "Selected: " + hotel.name + ". Pin " + hotel.number + " remains red; the other hotel pins are blue.";
    } else if (authFailed) {
      status.textContent = "Selected: " + hotel.name + ". NAVER map is unavailable, but the in-page hotel link remains active.";
    } else {
      status.textContent = "Selected: " + hotel.name + ". The map will apply this selection when it is ready.";
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

      var isCardMapLink = mapControl.matches('a.hongdae-map-link[href="#hongdae-hotel-map"]');
      if (!isCardMapLink) {
        event.preventDefault();
      }

      selectHotel(mapControl.getAttribute("data-map-hotel"), {
        focusList: isCardMapLink
      });
    });
  }

  function showMapFailure() {
    map = null;
    markers = {};
    canvas.innerHTML = '<p class="hongdae-hotel-map-fallback">The interactive NAVER map could not load. Use the hotel list to review all 14 stays, then try again later.</p>';
    status.textContent = "NAVER map unavailable. Hotel names and in-page links remain available.";
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
      script.dataset.hongdaeNaverMapSdk = "true";
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

    map = new window.naver.maps.Map(canvas, {
      center: new window.naver.maps.LatLng(37.5557, 126.9205),
      zoom: 14,
      minZoom: 13,
      maxZoom: 19,
      zoomControl: true,
      zoomControlOptions: {
        position: window.naver.maps.Position.TOP_RIGHT
      }
    });

    var bounds = new window.naver.maps.LatLngBounds();

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
      bounds.extend(position);
    });

    map.fitBounds(bounds, {
      top: 72,
      right: 72,
      bottom: 72,
      left: 72,
      maxZoom: 15
    });

    if (selectedId) {
      selectHotel(selectedId);
    } else {
      status.textContent = "All 14 hotel locations are shown. Select a pin or hotel name.";
    }

    window.addEventListener("resize", function () {
      if (map) {
        map.autoResize();
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

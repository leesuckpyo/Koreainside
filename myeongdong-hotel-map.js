(function () {
  "use strict";

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yif5zan&language=en";
  // Coordinates verified against official hotel addresses and linked maps on 2026-11-06.
  // 1. 137 Toegye-ro | https://app.lottehotel.com/myeongdong-l7/en/about/location
  // Official Google Maps link: https://maps.app.goo.gl/feQ1SUHQJYJkaa2H7 ; !3d/!4d hotel point, not viewport center.
  // 2. 139 Toegye-ro | https://m.skyparkhotel.com/html/hotel/3_md3/md3_about.asp
  // Official Google Maps link !3d37.5612896!4d126.9872401.
  // 3. 38 Myeongdong 8na-gil | https://www.marriott.com/en-us/hotels/selmm-le-meridien-seoul-myeongdong/overview/
  // Official VIEW MAP query coordinates. Shared building point for hotels 3 and 10; not separate entrances.
  // 4. 13 Myeongdong 7-gil | https://www.hotel28.co.kr/view/viewLink.do?page=homepage/ENG/hotel/location
  // Official initMap myLatlng marker.
  // 5. 61 Myeongdong-gil | https://www.royal.co.kr/en/bbs/page.php?hid=m09_01
  // Official embedded Google Maps resolved Royal Hotel Seoul named POI point, not initial iframe viewport.
  // 6. 30 Eulji-ro | https://app.lottehotel.com/seoul-grand/en/about/location
  // Official Google Maps link: https://maps.app.goo.gl/BxyNxeREVcfQPhP4A ; !3d/!4d hotel point.
  // 7. 28 Mareunnae-ro | https://www.ninetreehotels.com/en/hub/hotel-find
  // Official hotel-find page JS myeongdong2 pointer coordinates.
  // 8. 48 Myeongdong-gil | https://sotetsu-hotels.com/en/fresa-inn/myeong-dong/access/
  // Official Google Maps link https://goo.gl/maps/UbUR9AgU5DQ2dW4C7 ; !3d/!4d hotel point.
  // 9. 78 Namdaemun-ro | https://all.accor.com/hotel/6317/index.en.shtml
  // Official GPS and static map marker.
  // 10. 38 Myeongdong 8na-gil | https://www.marriott.com/en-us/hotels/selmx-moxy-seoul-myeongdong/overview/
  // Same building as Le Meridien. Moxy official point is ~7m away within same building; use hotel 3 official building point identically, as requested. Pixel offset only.
  // 11. 49 Myeongdong 8ga-gil | https://uhsuite.co.kr/main
  // Official: 47 annex / 49 main / 51 new. Card Trip.com resolves property 68087991, address49, coordinates37.561944/126.987786: https://www.trip.com/hotels/seoul-hotel-detail-68087991/uh-suite-myeongdong-avenue-49/ . Representative main building, not confirmed entrance or room assignment.
  var hotels = [
    {"id":"l7-myeongdong","name":"L7 MYEONGDONG by LOTTE HOTELS","lat":37.561246,"lng":126.986967,"number":1,"labelSide":"left","labelY":32,"pinOffsetX":0},
    {"id":"skypark-myeongdong-iii","name":"Hotel Skypark Myeongdong Ⅲ","lat":37.5612896,"lng":126.9872401,"number":2,"labelSide":"right","labelY":32,"pinOffsetX":0},
    {"id":"le-meridien-myeongdong","name":"Le Méridien Seoul, Myeongdong","lat":37.56185866448736,"lng":126.98268302570197,"number":3,"labelSide":"right","labelY":-24,"pinOffsetX":-24},
    {"id":"hotel28-myeongdong","name":"Hotel28 Myeongdong","lat":37.564349,"lng":126.983943,"number":4,"labelSide":"right","labelY":12,"pinOffsetX":0},
    {"id":"royal-hotel-seoul","name":"Royal Hotel Seoul","lat":37.5643146,"lng":126.9856446,"number":5,"labelSide":"right","labelY":-30,"pinOffsetX":0},
    {"id":"grand-lotte-seoul","name":"THE GRAND LOTTE SEOUL","lat":37.5653,"lng":126.980979,"number":6,"labelSide":"right","labelY":-26,"pinOffsetX":0},
    {"id":"nine-tree-myeongdong-ii","name":"NINE TREE BY PARNAS SEOUL MYEONGDONG II","lat":37.564505,"lng":126.990815,"number":7,"labelSide":"left","labelY":8,"pinOffsetX":0},
    {"id":"sotetsu-fresa-myeongdong","name":"Sotetsu Fresa Inn Seoul Myeong-dong","lat":37.563643,"lng":126.9849755,"number":8,"labelSide":"right","labelY":28,"pinOffsetX":0},
    {"id":"ibis-ambassador-myeongdong","name":"ibis Ambassador Seoul Myeongdong","lat":37.564316,"lng":126.982635,"number":9,"labelSide":"right","labelY":-34,"pinOffsetX":0},
    {"id":"moxy-myeongdong","name":"Moxy Seoul Myeongdong","lat":37.56185866448736,"lng":126.98268302570197,"number":10,"labelSide":"right","labelY":24,"pinOffsetX":24},
    {"id":"uh-suite-myeongdong","name":"UH Suite The Myeongdong","lat":37.561944,"lng":126.987786,"number":11,"labelSide":"right","labelY":-24,"pinOffsetX":0}
  ];

  var section = document.getElementById("myeongdong-hotel-map");
  var canvas = document.getElementById("myeongdong-hotel-map-canvas");
  var status = document.getElementById("myeongdong-hotel-map-status");

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
    default: "All 11 hotel locations are shown. Select a pin or hotel name.",
    selected: "Selected: {name}. Pin {number} remains red; the other hotel pins are blue.",
    unavailable_selected: "Selected: {name}. NAVER map is unavailable, but the in-page hotel link remains active.",
    pending_selected: "Selected: {name}. The map will apply this selection when it is ready.",
    fallback: "The interactive NAVER map could not load. Use the hotel list to review all 11 stays, then try again later.",
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
    var sideClass = hotel.labelSide === "left" ? " myeongdong-map-marker--left" : "";
    var sharedClass = hotel.pinOffsetX < 0 ? " myeongdong-map-marker--shared-left" : hotel.pinOffsetX > 0 ? " myeongdong-map-marker--shared-right" : "";
    var content = "";

    content += '<button type="button" class="myeongdong-map-marker' + sideClass + sharedClass + '"';
    content += ' data-map-marker="' + escapeHtml(hotel.id) + '"';
    content += ' aria-label="' + escapeHtml(mapText("marker_aria", hotel)) + '"';
    if (hotel.id === "uh-suite-myeongdong") {
      content += ' aria-describedby="myeongdong-hotel-map-uh-note"';
    }
    content += ' aria-pressed="' + (isSelected ? "true" : "false") + '"';
    content += ' style="--label-y:' + hotel.labelY + 'px">';
    if (hotel.pinOffsetX) {
      content += '<span class="myeongdong-map-marker__leader" aria-hidden="true"></span>';
    }
    content += '<img class="myeongdong-map-marker__pin" src="' + pinPaths[color] + '" alt="">';
    content += '<span class="myeongdong-map-marker__number" aria-hidden="true">' + hotel.number + "</span>";
    content += '<span class="myeongdong-map-marker__label" aria-hidden="true">' + escapeHtml(hotel.name) + "</span>";
    content += "</button>";

    return {
      content: content,
      size: new window.naver.maps.Size(36, 46),
      anchor: new window.naver.maps.Point(18 - hotel.pinOffsetX, 39)
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

      var isCardMapLink = mapControl.matches('a.myeongdong-map-link[href="#myeongdong-hotel-map"]');
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
    canvas.innerHTML = '<p class="myeongdong-hotel-map-fallback">' + escapeHtml(mapText("fallback")) + "</p>";
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
      script.dataset.myeongdongNaverMapSdk = "true";
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
      center: new window.naver.maps.LatLng(37.564349, 126.983943),
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

    function fitHotelBounds() {
      var padding = canvas.clientWidth <= 620 ? 24 : 72;
      map.fitBounds(bounds, {
        top: 72,
        right: padding,
        bottom: 48,
        left: padding,
        maxZoom: 16
      });
    }

    fitHotelBounds();

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

(function () {
  "use strict";

  var mapRoot = document.getElementById("budget-map");

  if (!mapRoot) {
    return;
  }

  var SDK_URL = "https://oapi.map.naver.com/openapi/v3/maps.js?ncpKeyId=r63yjif5zan&language=en";
  var SOURCE_LINKS = {
    coordinates: "https://www.arcgis.com/home/item.html?id=a3ca58b3ef864e61aab932c5c592e729",
    accessibility: "https://english.seoul.go.kr/service/movement/public-transportation/subway-accessibility-facilities/",
    arex: "https://www.arex.or.kr/"
  };

  var areas = {
    hongdae: {
      name: "Hongdae",
      searchArea: "hongdae",
      center: { lat: 37.556748, lng: 126.923643 },
      zoom: 15,
      radius: 850
    },
    gongdeok: {
      name: "Mapo / Gongdeok",
      searchArea: "gongdeok",
      center: { lat: 37.543592, lng: 126.951664 },
      zoom: 15,
      radius: 900
    },
    sinchon: {
      name: "Sinchon",
      searchArea: "sinchon",
      center: { lat: 37.555153, lng: 126.93689 },
      zoom: 15,
      radius: 800
    },
    euljiro: {
      name: "Euljiro / Myeongdong",
      searchArea: "euljiro-myeongdong",
      center: { lat: 37.566292, lng: 126.990873 },
      zoom: 15,
      radius: 950
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
    elevator: "Elevator access",
    convenience: "Convenience store",
    pharmacy: "Pharmacy",
    oliveyoung: "Beauty and personal-care store",
    daiso: "Discount variety store"
  };

  var categorySymbols = {
    airport: "A",
    subway: "S",
    elevator: "E",
    convenience: "C",
    pharmacy: "P",
    oliveyoung: "O",
    daiso: "D"
  };

  var categoryClasses = {
    airport: "budget-map-marker--airport",
    subway: "budget-map-marker--subway",
    elevator: "budget-map-marker--elevator",
    convenience: "budget-map-marker--convenience",
    pharmacy: "budget-map-marker--pharmacy",
    oliveyoung: "budget-map-marker--oliveyoung",
    daiso: "budget-map-marker--daiso"
  };

  var facilityCategoryOrder = ["convenience", "pharmacy", "oliveyoung", "daiso"];
  var facilityCategories = new Set(facilityCategoryOrder);
  var facilityAriaLabels = {
    convenience: "Convenience store",
    pharmacy: "Pharmacy",
    oliveyoung: "Olive Young",
    daiso: "Daiso"
  };
  var facilityPluralLabels = {
    convenience: "convenience stores",
    pharmacy: "pharmacies",
    oliveyoung: "Olive Young stores",
    daiso: "Daiso stores"
  };
  var facilityGuidance = {
    convenience:
      "Useful for drinks, snacks, transit cards and basic travel supplies. Check the current listing before visiting.",
    pharmacy:
      "Useful for basic medicines and travel health needs. Opening hours can change, so confirm the current listing before visiting.",
    oliveyoung:
      "Useful for skincare, toiletries and personal-care items. Product availability and opening hours can change, so confirm the current listing before visiting.",
    daiso:
      "Useful for low-cost travel supplies, storage items and everyday necessities. Stock and opening hours can change, so confirm the current listing before visiting."
  };
  var markerOverlapOffsets = [
    { x: 11, y: 0 },
    { x: -11, y: 0 },
    { x: 0, y: 11 },
    { x: 0, y: -11 },
    { x: 8, y: 8 },
    { x: -8, y: 8 },
    { x: 8, y: -8 },
    { x: -8, y: -8 }
  ];

  var tabs = Array.prototype.slice.call(document.querySelectorAll("[data-map-area]"));
  var filterButtons = Array.prototype.slice.call(document.querySelectorAll("[data-map-category]"));
  var loadButton = document.getElementById("budget-map-load");
  var statusNode = document.getElementById("budget-map-status");
  var errorNode = document.getElementById("budget-map-error");
  var placeErrorNode = document.getElementById("budget-place-error");
  var detailsNode = document.getElementById("budget-marker-details");
  var placeListNode = document.getElementById("budget-place-list");
  var placeListTitle = document.getElementById("budget-place-list-title");
  var placeListItems = placeListNode.querySelector("ul");
  var legendNode = document.querySelector(".budget-map-legend");
  var tabPanel = document.getElementById("budget-map-panel");

  var activeArea = "hongdae";
  var activeCategories = new Set(["airport", "subway", "elevator"]);
  var sdkPromise = null;
  var map = null;
  var mapMarkers = [];
  var areaCircle = null;
  var infoWindow = null;
  var resizeTimer = null;
  var placeCache = new Map();
  var placeRequests = new Map();
  var placeErrors = new Set();
  var loadingPlaceKeys = new Set();

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

    filterButtons.forEach(function (button) {
      var category = button.getAttribute("data-map-category");
      var available = facilityCategories.has(category) || availableCategories.has(category);

      button.hidden = !available;
      button.setAttribute("aria-pressed", available && activeCategories.has(category) ? "true" : "false");
    });
  }

  function resetMarkerDetails() {
    var guidance = document.createElement("p");
    guidance.textContent = "Select a map marker or nearby place name to view details.";
    detailsNode.replaceChildren(guidance);
  }

  function appendLabeledText(node, label, value) {
    var strong = document.createElement("strong");
    strong.textContent = label;
    node.appendChild(strong);
    node.appendChild(document.createTextNode(value));
  }

  function toRadians(value) {
    return (value * Math.PI) / 180;
  }

  function distanceFromAreaCenter(marker) {
    var center = areas[marker.area].center;
    var earthRadius = 6371000;
    var latDelta = toRadians(marker.lat - center.lat);
    var lngDelta = toRadians(marker.lng - center.lng);
    var centerLat = toRadians(center.lat);
    var markerLat = toRadians(marker.lat);
    var a =
      Math.sin(latDelta / 2) * Math.sin(latDelta / 2) +
      Math.cos(centerLat) * Math.cos(markerLat) * Math.sin(lngDelta / 2) * Math.sin(lngDelta / 2);
    var c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return Math.round(earthRadius * c);
  }

  function formatLastChecked(value) {
    var checkedDate = new Date(value);
    return Number.isNaN(checkedDate.getTime()) ? "Not available" : checkedDate.toISOString().slice(0, 10);
  }

  function showMarkerDetails(marker) {
    var heading = document.createElement("h3");
    var type = document.createElement("p");
    var address = document.createElement("p");
    var distance = document.createElement("p");
    var impact = document.createElement("p");
    var lastChecked = document.createElement("p");
    var sourceLink = document.createElement("a");

    heading.textContent = marker.name;
    appendLabeledText(type, "Type: ", categoryLabels[marker.category]);
    appendLabeledText(address, "Address: ", "Not provided in the verified source.");
    appendLabeledText(
      distance,
      "Approximate distance: ",
      distanceFromAreaCenter(marker) + " m from the area center"
    );
    appendLabeledText(impact, "Traveler impact: ", marker.travelerImpact);
    lastChecked.className = "budget-marker-details__meta";
    appendLabeledText(lastChecked, "Last checked: ", marker.lastChecked);
    sourceLink.href = marker.link;
    sourceLink.target = "_blank";
    sourceLink.rel = "noopener noreferrer";
    sourceLink.textContent = "Open source";
    detailsNode.replaceChildren(heading, type, address, distance, impact, lastChecked, sourceLink);
  }

  function showPlaceDetails(place) {
    var heading = document.createElement("h3");
    var type = document.createElement("p");
    var address = document.createElement("p");
    var distance = document.createElement("p");
    var impact = document.createElement("p");
    var lastChecked = document.createElement("p");
    var children = [heading, type, address, distance, impact, lastChecked];

    heading.textContent = place.name;
    appendLabeledText(type, "Type: ", place.type);
    appendLabeledText(address, "Road address: ", place.roadAddress || place.address || "Address unavailable");
    appendLabeledText(distance, "Approximate distance: ", place.distanceMeters + " m from the area center");
    appendLabeledText(impact, "Traveler impact: ", facilityGuidance[place.category]);
    lastChecked.className = "budget-marker-details__meta";
    appendLabeledText(lastChecked, "Last checked: ", formatLastChecked(place.lastChecked));

    if (place.link) {
      var placeLink = document.createElement("a");
      placeLink.href = place.link;
      placeLink.target = "_blank";
      placeLink.rel = "noopener noreferrer";
      placeLink.textContent = "Open place link";
      children.push(placeLink);
    }

    detailsNode.replaceChildren.apply(detailsNode, children);
  }

  function distanceBetweenMarkers(firstMarker, secondMarker) {
    var earthRadius = 6371000;
    var firstLat = toRadians(firstMarker.lat);
    var secondLat = toRadians(secondMarker.lat);
    var latDelta = toRadians(secondMarker.lat - firstMarker.lat);
    var lngDelta = toRadians(secondMarker.lng - firstMarker.lng);
    var calculation =
      Math.sin(latDelta / 2) * Math.sin(latDelta / 2) +
      Math.cos(firstLat) *
        Math.cos(secondLat) *
        Math.sin(lngDelta / 2) *
        Math.sin(lngDelta / 2);

    return earthRadius * 2 * Math.atan2(Math.sqrt(calculation), Math.sqrt(1 - calculation));
  }

  function markerDisplayOffset(marker, markerIndex, markerData) {
    var overlapCount = 0;

    for (var index = 0; index < markerIndex; index += 1) {
      if (distanceBetweenMarkers(marker, markerData[index]) < 55) {
        overlapCount += 1;
      }
    }

    if (overlapCount === 0) {
      return { x: 0, y: 0 };
    }

    return markerOverlapOffsets[(overlapCount - 1) % markerOverlapOffsets.length];
  }

  function markerZIndex(marker) {
    if (marker.category === "airport") {
      return 320;
    }

    if (marker.category === "subway") {
      return 300;
    }

    if (marker.category === "elevator") {
      return 280;
    }

    return 100;
  }

  function markerIcon(marker, displayOffset) {
    var ariaLabel = facilityCategories.has(marker.category)
      ? facilityAriaLabels[marker.category]
      : marker.name + ", " + categoryLabels[marker.category];

    return {
      content:
        '<button type="button" class="budget-map-marker ' +
        categoryClasses[marker.category] +
        '" aria-label="' +
        ariaLabel +
        '">' +
        '<span aria-hidden="true">' +
        categorySymbols[marker.category] +
        "</span></button>",
      anchor: new window.naver.maps.Point(17 - displayOffset.x, 17 - displayOffset.y)
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

  function getPlaceCacheKey(areaKey, category) {
    return areas[areaKey].searchArea + "|" + category;
  }

  function getActiveFacilityCategories() {
    return facilityCategoryOrder.filter(function (category) {
      return activeCategories.has(category);
    });
  }

  function getCachedPlaces(areaKey, category) {
    var cached = placeCache.get(getPlaceCacheKey(areaKey, category));

    if (!cached) {
      return [];
    }

    return cached.items.map(function (place) {
      return Object.assign({}, place, { lastChecked: cached.updatedAt });
    });
  }

  function getVisiblePlaces() {
    return getActiveFacilityCategories().reduce(function (places, category) {
      return places.concat(
        getCachedPlaces(activeArea, category).map(function (place) {
          return Object.assign({}, place, { category: category });
        })
      );
    }, []);
  }

  function normalizePlaceLink(value) {
    if (value === null) {
      return null;
    }

    if (typeof value !== "string") {
      throw new Error("Invalid place link");
    }

    try {
      var parsed = new URL(value);
      return parsed.protocol === "http:" || parsed.protocol === "https:" ? parsed.href : null;
    } catch (error) {
      return null;
    }
  }

  function normalizePlaceItem(item, category) {
    if (
      !item ||
      typeof item.id !== "string" ||
      typeof item.name !== "string" ||
      typeof item.type !== "string" ||
      typeof item.address !== "string" ||
      typeof item.roadAddress !== "string" ||
      item.type !== categoryLabels[category] ||
      !Number.isFinite(item.lat) ||
      !Number.isFinite(item.lng) ||
      !Number.isFinite(item.distanceMeters) ||
      item.distanceMeters < 0 ||
      item.lat < 37.3 ||
      item.lat > 37.75 ||
      item.lng < 126.7 ||
      item.lng > 127.3
    ) {
      throw new Error("Invalid place item");
    }

    var name = item.name.trim();

    if (!name) {
      throw new Error("Invalid place name");
    }

    return {
      id: item.id,
      name: name,
      type: item.type,
      address: item.address.trim(),
      roadAddress: item.roadAddress.trim(),
      lat: item.lat,
      lng: item.lng,
      link: normalizePlaceLink(item.link),
      distanceMeters: Math.round(item.distanceMeters)
    };
  }

  function validatePlaceResponse(payload, areaKey, category) {
    if (
      !payload ||
      payload.area !== areas[areaKey].searchArea ||
      payload.category !== category ||
      typeof payload.updatedAt !== "string" ||
      Number.isNaN(Date.parse(payload.updatedAt)) ||
      !Array.isArray(payload.items) ||
      payload.items.length > 3
    ) {
      throw new Error("Invalid place response");
    }

    var ids = new Set();
    var items = payload.items.map(function (item) {
      var normalized = normalizePlaceItem(item, category);

      if (ids.has(normalized.id)) {
        throw new Error("Duplicate place id");
      }

      ids.add(normalized.id);
      return normalized;
    });

    return {
      updatedAt: payload.updatedAt,
      items: items
    };
  }

  function formatHumanList(values) {
    if (values.length === 0) {
      return "";
    }

    if (values.length === 1) {
      return values[0];
    }

    if (values.length === 2) {
      return values[0] + " and " + values[1];
    }

    return values.slice(0, -1).join(", ") + " and " + values[values.length - 1];
  }

  function numberWord(value) {
    var words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    return words[value] || String(value);
  }

  function updatePlaceErrorVisibility() {
    var hasActiveError = getActiveFacilityCategories().some(function (category) {
      return placeErrors.has(getPlaceCacheKey(activeArea, category));
    });

    placeErrorNode.hidden = !hasActiveError;
  }

  function updateMapStatus() {
    var areaName = areas[activeArea].name;
    var activeFacility = getActiveFacilityCategories();
    var loadingFacility = activeFacility.filter(function (category) {
      return loadingPlaceKeys.has(getPlaceCacheKey(activeArea, category));
    });

    if (loadingFacility.length > 0) {
      statusNode.textContent =
        "Loading nearby " +
        formatHumanList(
          loadingFacility.map(function (category) {
            return facilityPluralLabels[category];
          })
        ) +
        " around " +
        areaName +
        "…";
      return;
    }

    if (activeFacility.length > 0) {
      var placeCount = getVisiblePlaces().length;
      var resultText = placeCount + (placeCount === 1 ? " result shown. " : " results shown. ");

      if (activeFacility.length === 1) {
        var category = activeFacility[0];
        var singleMessages = {
          convenience:
            "Showing nearby convenience stores around " +
            areaName +
            ". " +
            resultText +
            "Business details may change, so confirm the current listing before visiting.",
          pharmacy:
            "Showing nearby pharmacies around " +
            areaName +
            ". " +
            resultText +
            "Opening hours can change, so confirm the current listing before visiting.",
          oliveyoung:
            "Showing nearby Olive Young stores around " +
            areaName +
            ". " +
            resultText +
            "Store details may change, so confirm the current listing before visiting.",
          daiso:
            "Showing nearby Daiso stores around " +
            areaName +
            ". " +
            resultText +
            "Stock and opening hours may change, so confirm the current listing before visiting."
        };
        statusNode.textContent = singleMessages[category];
        return;
      }

      statusNode.textContent =
        "Showing nearby " +
        formatHumanList(
          activeFacility.map(function (category) {
            return facilityPluralLabels[category];
          })
        ) +
        " around " +
        areaName +
        ". " +
        resultText +
        "Store details and opening hours may change, so confirm the current listing before visiting.";
      return;
    }

    if (map) {
      var verifiedCount = getAreaMarkers(activeArea).filter(function (marker) {
        return activeCategories.has(marker.category);
      }).length;
      statusNode.textContent =
        areaName +
        " shows " +
        numberWord(verifiedCount) +
        " verified transport and access " +
        (verifiedCount === 1 ? "point." : "points.");
      return;
    }

    statusNode.textContent = areaName + " selected. Load the map to show verified transport and access points.";
  }

  function renderPlaceList() {
    var activeFacility = getActiveFacilityCategories();
    var places = getVisiblePlaces();
    var hasResolvedCategory = activeFacility.some(function (category) {
      return placeCache.has(getPlaceCacheKey(activeArea, category));
    });

    placeListItems.replaceChildren();

    if (activeFacility.length === 0 || (!hasResolvedCategory && places.length === 0)) {
      placeListNode.hidden = true;
      return;
    }

    placeListTitle.textContent = "Nearby places (" + places.length + ")";

    if (places.length === 0) {
      var emptyItem = document.createElement("li");
      var emptyText = document.createElement("p");
      emptyText.textContent = "No nearby results were returned for the selected filters.";
      emptyItem.appendChild(emptyText);
      placeListItems.appendChild(emptyItem);
      placeListNode.hidden = false;
      return;
    }

    places.forEach(function (place) {
      var item = document.createElement("li");
      var button = document.createElement("button");
      var metadata = document.createElement("p");

      button.type = "button";
      button.textContent = place.name;
      button.addEventListener("click", function () {
        showPlaceDetails(place);

        if (map && window.naver && window.naver.maps) {
          map.panTo(new window.naver.maps.LatLng(place.lat, place.lng));
        }
      });
      metadata.textContent =
        place.type +
        " · " +
        (place.roadAddress || place.address || "Address unavailable") +
        " · " +
        place.distanceMeters +
        " m from the area center";
      item.append(button, metadata);
      placeListItems.appendChild(item);
    });

    placeListNode.hidden = false;
  }

  function refreshMapInterface() {
    updatePlaceErrorVisibility();

    if (map) {
      renderMapArea();
      return;
    }

    resetMarkerDetails();
    renderPlaceList();
    updateMapStatus();
  }

  function loadPlaceCategory(areaKey, category) {
    var cacheKey = getPlaceCacheKey(areaKey, category);

    if (placeCache.has(cacheKey)) {
      return Promise.resolve(placeCache.get(cacheKey));
    }

    if (placeRequests.has(cacheKey)) {
      return placeRequests.get(cacheKey);
    }

    loadingPlaceKeys.add(cacheKey);
    placeErrors.delete(cacheKey);

    if (activeArea === areaKey) {
      refreshMapInterface();
    }

    var requestPromise = fetch(
      "/api/budget-stay-places?area=" +
        encodeURIComponent(areas[areaKey].searchArea) +
        "&category=" +
        encodeURIComponent(category),
      {
        method: "GET",
        headers: {
          Accept: "application/json"
        }
      }
    )
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Place request failed");
        }

        return response.json();
      })
      .then(function (payload) {
        var validated = validatePlaceResponse(payload, areaKey, category);
        placeCache.set(cacheKey, validated);
        return validated;
      })
      .catch(function (error) {
        placeErrors.add(cacheKey);
        throw error;
      })
      .finally(function () {
        loadingPlaceKeys.delete(cacheKey);
        placeRequests.delete(cacheKey);

        if (activeArea === areaKey) {
          refreshMapInterface();
        }
      });

    placeRequests.set(cacheKey, requestPromise);
    return requestPromise;
  }

  function loadActivePlaceCategories(areaKey) {
    getActiveFacilityCategories().forEach(function (category) {
      loadPlaceCategory(areaKey, category).catch(function () {
        return undefined;
      });
    });
  }

  function renderMapArea() {
    if (!map || !window.naver || !window.naver.maps) {
      return;
    }

    var area = areas[activeArea];
    var verifiedData = getAreaMarkers(activeArea).filter(function (marker) {
      return activeCategories.has(marker.category);
    });
    var placeData = getVisiblePlaces();
    var areaData = verifiedData.concat(placeData);

    clearMapObjects();
    resetMarkerDetails();
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

    areaData.forEach(function (markerData, markerIndex) {
      var displayOffset = markerDisplayOffset(markerData, markerIndex, areaData);
      var marker = new window.naver.maps.Marker({
        map: map,
        position: new window.naver.maps.LatLng(markerData.lat, markerData.lng),
        title: facilityCategories.has(markerData.category)
          ? facilityAriaLabels[markerData.category]
          : markerData.name,
        icon: markerIcon(markerData, displayOffset),
        zIndex: markerZIndex(markerData)
      });

      window.naver.maps.Event.addListener(marker, "click", function () {
        if (facilityCategories.has(markerData.category)) {
          showPlaceDetails(markerData);
        } else {
          showMarkerDetails(markerData);
        }
      });

      mapMarkers.push(marker);
    });

    renderPlaceList();
    updateMapStatus();
  }

  function failMap() {
    map = null;
    statusNode.textContent = "The written area comparison remains available below.";
    errorNode.hidden = false;
    mapRoot.classList.remove("budget-map-canvas--ready");
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
        mapRoot.classList.add("budget-map-canvas--ready");
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
        window.naver.maps.Event.addListener(map, "click", resetMarkerDetails);
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
    resetMarkerDetails();
    loadActivePlaceCategories(areaKey);
    refreshMapInterface();

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

      resetMarkerDetails();

      if (!isActive && facilityCategories.has(category)) {
        loadPlaceCategory(activeArea, category).catch(function () {
          return undefined;
        });
      } else {
        refreshMapInterface();
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

  function configureLegendDisclosure() {
    if (!legendNode || typeof window.matchMedia !== "function") {
      return;
    }

    var mobileLegend = window.matchMedia("(max-width: 1024px)");

    function syncLegend(event) {
      if (event.matches) {
        legendNode.removeAttribute("open");
      } else {
        legendNode.setAttribute("open", "");
      }
    }

    syncLegend(mobileLegend);

    if (typeof mobileLegend.addEventListener === "function") {
      mobileLegend.addEventListener("change", syncLegend);
    } else if (typeof mobileLegend.addListener === "function") {
      mobileLegend.addListener(syncLegend);
    }
  }

  configureLegendDisclosure();
  updateFilterAvailability(activeArea);
  resetMarkerDetails();
  renderPlaceList();
})();

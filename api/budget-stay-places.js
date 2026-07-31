"use strict";

const NAVER_LOCAL_SEARCH_ENDPOINT = "https://naverapihub.apigw.ntruss.com/search/v1/local";
var CACHE_CONTROL = "public, s-maxage=21600, stale-while-revalidate=86400";
var REQUEST_TIMEOUT_MS = 6000;
var MAX_ITEMS_PER_CATEGORY = 3;

var AREA_CENTERS = {
  hongdae: { lat: 37.556748, lng: 126.923643 },
  gongdeok: { lat: 37.543592, lng: 126.951664 },
  sinchon: { lat: 37.555153, lng: 126.93689 },
  "euljiro-myeongdong": { lat: 37.566292, lng: 126.990873 }
};

var SEARCH_QUERIES = {
  hongdae: {
    convenience: ["홍대입구역 편의점"],
    pharmacy: ["홍대입구역 약국"],
    oliveyoung: ["홍대입구역 올리브영"],
    daiso: ["홍대입구역 다이소"]
  },
  gongdeok: {
    convenience: ["공덕역 편의점"],
    pharmacy: ["공덕역 약국"],
    oliveyoung: ["공덕역 올리브영"],
    daiso: ["공덕역 다이소"]
  },
  sinchon: {
    convenience: ["신촌역 편의점"],
    pharmacy: ["신촌역 약국"],
    oliveyoung: ["신촌역 올리브영"],
    daiso: ["신촌역 다이소"]
  },
  "euljiro-myeongdong": {
    convenience: ["명동역 편의점", "을지로입구역 편의점"],
    pharmacy: ["명동역 약국", "을지로입구역 약국"],
    oliveyoung: ["명동역 올리브영", "을지로입구역 올리브영"],
    daiso: ["명동역 다이소", "을지로입구역 다이소"]
  }
};

var CATEGORY_TYPES = {
  convenience: "Convenience store",
  pharmacy: "Pharmacy",
  oliveyoung: "Beauty and personal-care store",
  daiso: "Discount variety store"
};

function sendJson(response, status, payload) {
  return response.status(status).json(payload);
}

function cleanText(value) {
  return String(value == null ? "" : value)
    .replace(/<[^>]*>/g, "")
    .replace(/&nbsp;/gi, " ")
    .replace(/&amp;/gi, "&")
    .replace(/&quot;/gi, '"')
    .replace(/&#39;/gi, "'")
    .replace(/\s+/g, " ")
    .trim();
}

function normalizeLink(value) {
  var link = cleanText(value);

  if (!link) {
    return null;
  }

  try {
    var parsed = new URL(link);
    return parsed.protocol === "http:" || parsed.protocol === "https:" ? parsed.href : null;
  } catch (error) {
    return null;
  }
}

function normalizeCoordinate(value) {
  var coordinate = Number(value);

  if (!Number.isFinite(coordinate)) {
    return null;
  }

  if (Math.abs(coordinate) > 180) {
    coordinate = coordinate / 10000000;
  }

  return coordinate;
}

function isInSeoul(lat, lng) {
  return lat >= 37.3 && lat <= 37.75 && lng >= 126.7 && lng <= 127.3;
}

function toRadians(value) {
  return (value * Math.PI) / 180;
}

function distanceMeters(origin, destination) {
  var earthRadius = 6371000;
  var latDelta = toRadians(destination.lat - origin.lat);
  var lngDelta = toRadians(destination.lng - origin.lng);
  var originLat = toRadians(origin.lat);
  var destinationLat = toRadians(destination.lat);
  var a =
    Math.sin(latDelta / 2) * Math.sin(latDelta / 2) +
    Math.cos(originLat) *
      Math.cos(destinationLat) *
      Math.sin(lngDelta / 2) *
      Math.sin(lngDelta / 2);
  var c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

  return Math.round(earthRadius * c);
}

function hashString(value) {
  var hash = 2166136261;

  for (var index = 0; index < value.length; index += 1) {
    hash ^= value.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }

  return (hash >>> 0).toString(36);
}

function normalizeItems(rawItems, area, category) {
  var center = AREA_CENTERS[area];
  var seen = new Set();
  var normalized = [];

  rawItems.forEach(function (item) {
    var name = cleanText(item && item.title);
    var lat = normalizeCoordinate(item && item.mapy);
    var lng = normalizeCoordinate(item && item.mapx);

    if (!name || lat === null || lng === null || !isInSeoul(lat, lng)) {
      return;
    }

    var duplicateKey = name.toLowerCase() + "|" + lat.toFixed(6) + "|" + lng.toFixed(6);

    if (seen.has(duplicateKey)) {
      return;
    }

    seen.add(duplicateKey);

    normalized.push({
      id: "place-" + area + "-" + category + "-" + hashString(duplicateKey),
      name: name,
      type: CATEGORY_TYPES[category],
      address: cleanText(item.address),
      roadAddress: cleanText(item.roadAddress),
      lat: Number(lat.toFixed(7)),
      lng: Number(lng.toFixed(7)),
      link: normalizeLink(item.link),
      distanceMeters: distanceMeters(center, { lat: lat, lng: lng })
    });
  });

  normalized.sort(function (left, right) {
    return left.distanceMeters - right.distanceMeters;
  });

  return normalized.slice(0, MAX_ITEMS_PER_CATEGORY);
}

function safeDiagnosticValue(value, depth) {
  if (value == null) {
    return null;
  }

  if (typeof value === "string") {
    return value.trim().slice(0, 500);
  }

  if (typeof value === "number" || typeof value === "boolean") {
    return value;
  }

  if (depth >= 2) {
    return "[omitted]";
  }

  if (Array.isArray(value)) {
    return value.slice(0, 5).map(function (item) {
      return safeDiagnosticValue(item, depth + 1);
    });
  }

  if (typeof value === "object") {
    var safeObject = {};

    Object.keys(value)
      .slice(0, 10)
      .forEach(function (key) {
        if (!/(client|secret|authorization|api.?key|token|header|credential|env)/i.test(key)) {
          safeObject[key] = safeDiagnosticValue(value[key], depth + 1);
        }
      });

    return safeObject;
  }

  return null;
}

function logUpstreamError(response, url, responseText, area, category) {
  var parsed = null;

  try {
    parsed = JSON.parse(responseText);
  } catch (error) {
    parsed = null;
  }

  console.error("NAVER_LOCAL_SEARCH_UPSTREAM_ERROR", {
    upstreamStatus: response.status,
    contentType: response.headers.get("content-type"),
    pathname: url.pathname,
    area: area,
    category: category,
    errorCode: safeDiagnosticValue(parsed && parsed.errorCode, 0),
    errorMessage: safeDiagnosticValue(parsed && parsed.errorMessage, 0),
    error: {
      errorCode: safeDiagnosticValue(parsed && parsed.error && parsed.error.errorCode, 0),
      message: safeDiagnosticValue(parsed && parsed.error && parsed.error.message, 0),
      details: safeDiagnosticValue(parsed && parsed.error && parsed.error.details, 0)
    }
  });
}

async function fetchNaverItems(query, area, category, clientId, clientSecret) {
  var controller = new AbortController();
  var timeoutId = setTimeout(function () {
    controller.abort();
  }, REQUEST_TIMEOUT_MS);
  var url = new URL(NAVER_LOCAL_SEARCH_ENDPOINT);

  url.searchParams.set("query", query);
  url.searchParams.set("display", "5");
  url.searchParams.set("start", "1");
  url.searchParams.set("sort", "random");
  url.searchParams.set("format", "json");

  try {
    var upstreamResponse = await fetch(url.toString(), {
      method: "GET",
      headers: {
        "X-NCP-APIGW-API-KEY-ID": clientId,
        "X-NCP-APIGW-API-KEY": clientSecret,
        Accept: "application/json"
      },
      signal: controller.signal
    });
    var responseText = await upstreamResponse.text();

    if (!upstreamResponse.ok) {
      logUpstreamError(upstreamResponse, url, responseText, area, category);
      throw new Error("NAVER_LOCAL_SEARCH_FAILED");
    }

    var payload = JSON.parse(responseText);

    if (!payload || !Array.isArray(payload.items)) {
      throw new Error("NAVER_LOCAL_SEARCH_INVALID_RESPONSE");
    }

    return payload.items;
  } finally {
    clearTimeout(timeoutId);
  }
}

module.exports = async function budgetStayPlaces(request, response) {
  if (request.method !== "GET") {
    response.setHeader("Allow", "GET");
    return sendJson(response, 405, { error: "Method not allowed." });
  }

  var area = request.query && request.query.area;
  var category = request.query && request.query.category;

  if (
    typeof area !== "string" ||
    typeof category !== "string" ||
    !Object.prototype.hasOwnProperty.call(SEARCH_QUERIES, area) ||
    !Object.prototype.hasOwnProperty.call(CATEGORY_TYPES, category)
  ) {
    return sendJson(response, 400, { error: "Invalid area or category." });
  }

  var clientId = process.env.NAVER_LOCAL_SEARCH_CLIENT_ID;
  var clientSecret = process.env.NAVER_LOCAL_SEARCH_CLIENT_SECRET;

  if (!clientId || !clientSecret) {
    return sendJson(response, 503, { error: "Place information is temporarily unavailable." });
  }

  clientId = clientId.trim();
  clientSecret = clientSecret.trim();

  if (!clientId || !clientSecret) {
    return sendJson(response, 503, { error: "Place information is temporarily unavailable." });
  }

  var rawItems;

  try {
    var resultSets = await Promise.all(
      SEARCH_QUERIES[area][category].map(function (query) {
        return fetchNaverItems(query, area, category, clientId, clientSecret);
      })
    );
    rawItems = resultSets.reduce(function (combined, resultSet) {
      return combined.concat(resultSet);
    }, []);
  } catch (error) {
    return sendJson(response, 502, { error: "Place information is temporarily unavailable." });
  }

  try {
    var items = normalizeItems(rawItems, area, category);
    response.setHeader("Cache-Control", CACHE_CONTROL);
    return sendJson(response, 200, {
      area: area,
      category: category,
      updatedAt: new Date().toISOString(),
      items: items
    });
  } catch (error) {
    return sendJson(response, 500, { error: "Place information is temporarily unavailable." });
  }
};

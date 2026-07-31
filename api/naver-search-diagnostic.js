"use strict";

const NAVER_LOCAL_SEARCH_ENDPOINT = "https://naverapihub.apigw.ntruss.com/search/v1/local";
const REQUEST_TIMEOUT_MS = 6000;

function sendJson(response, status, payload) {
  response.setHeader("Cache-Control", "no-store");
  return response.status(status).json(payload);
}

function safeString(value) {
  if (typeof value === "number" && Number.isFinite(value)) {
    return String(value);
  }

  if (typeof value !== "string") {
    return null;
  }

  var normalized = value.trim();
  return normalized ? normalized.slice(0, 500) : null;
}

function firstSafeString(values) {
  for (var index = 0; index < values.length; index += 1) {
    var candidate = safeString(values[index]);

    if (candidate) {
      return candidate;
    }
  }

  return null;
}

module.exports = async function naverSearchDiagnostic(request, response) {
  if (request.method !== "GET") {
    response.setHeader("Allow", "GET");
    return sendJson(response, 405, {
      environmentVariablesPresent: false,
      upstreamStatus: null,
      errorCode: null,
      errorMessage: "Method not allowed.",
      itemCount: 0
    });
  }

  var clientId = process.env.NAVER_LOCAL_SEARCH_CLIENT_ID;
  var clientSecret = process.env.NAVER_LOCAL_SEARCH_CLIENT_SECRET;
  var environmentVariablesPresent = Boolean(clientId && clientSecret);

  if (!environmentVariablesPresent) {
    return sendJson(response, 503, {
      environmentVariablesPresent: false,
      upstreamStatus: null,
      errorCode: null,
      errorMessage: null,
      itemCount: 0
    });
  }

  clientId = clientId.trim();
  clientSecret = clientSecret.trim();
  environmentVariablesPresent = Boolean(clientId && clientSecret);

  if (!environmentVariablesPresent) {
    return sendJson(response, 503, {
      environmentVariablesPresent: false,
      upstreamStatus: null,
      errorCode: null,
      errorMessage: null,
      itemCount: 0
    });
  }

  var url = new URL(NAVER_LOCAL_SEARCH_ENDPOINT);
  url.searchParams.set("query", "올리브영 홍대");
  url.searchParams.set("display", "2");
  url.searchParams.set("start", "1");
  url.searchParams.set("sort", "random");
  url.searchParams.set("format", "json");

  var controller = new AbortController();
  var timeoutId = setTimeout(function () {
    controller.abort();
  }, REQUEST_TIMEOUT_MS);

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
    var contentType = upstreamResponse.headers.get("content-type");
    var responseText = await upstreamResponse.text();
    var parsed = null;

    try {
      parsed = JSON.parse(responseText);
    } catch (error) {
      parsed = null;
    }

    var diagnostic = {
      environmentVariablesPresent: true,
      upstreamStatus: upstreamResponse.status,
      errorCode: firstSafeString([
        parsed && parsed.errorCode,
        parsed && parsed.code,
        parsed && parsed.error && parsed.error.errorCode,
        parsed && parsed.error && parsed.error.code
      ]),
      errorMessage: firstSafeString([
        parsed && parsed.errorMessage,
        parsed && parsed.message,
        parsed && parsed.error && parsed.error.message
      ]),
      itemCount: parsed && Array.isArray(parsed.items) ? parsed.items.length : 0
    };

    if (!parsed) {
      diagnostic.contentType = safeString(contentType);
    }

    return sendJson(response, 200, diagnostic);
  } catch (error) {
    return sendJson(response, 200, {
      environmentVariablesPresent: true,
      upstreamStatus: null,
      errorCode: error && error.name === "AbortError" ? "TIMEOUT" : "REQUEST_FAILED",
      errorMessage: null,
      itemCount: 0
    });
  } finally {
    clearTimeout(timeoutId);
  }
};

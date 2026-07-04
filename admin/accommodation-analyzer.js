(function () {
  "use strict";

  const WALKING_METERS_PER_MINUTE = 80;
  const EARTH_RADIUS_METERS = 6371000;

  const FIELD_DEFS = [
    { key: "hotelName", label: "Hotel Name", aliases: ["hotel name", "property name", "accommodation name", "name", "hotel"] },
    { key: "type", label: "Type", aliases: ["type", "property type", "accommodation type", "category"] },
    { key: "address", label: "Address", aliases: ["address", "location", "street address"] },
    { key: "latitude", label: "Latitude", aliases: ["latitude", "lat", "geo lat", "y"] },
    { key: "longitude", label: "Longitude", aliases: ["longitude", "lng", "lon", "geo lng", "geo lon", "x"] },
    { key: "bookingUrl", label: "Booking URL", aliases: ["booking url", "url", "link", "property url", "booking.com url"] },
    { key: "rating", label: "Rating", aliases: ["rating", "score", "review score", "guest rating"] },
    { key: "reviewCount", label: "Review Count", aliases: ["review count", "reviews", "number of reviews", "review total"] },
    { key: "price", label: "Price", aliases: ["price", "rate", "nightly price", "total price"] }
  ];

  const CRITERIA_DEFS = [
    { key: "luggageConvenience", label: "Luggage Convenience" },
    { key: "hillRisk", label: "Hill Risk" },
    { key: "stationAccess", label: "Station Access" },
    { key: "airportAccess", label: "Airport Access" },
    { key: "noiseRisk", label: "Noise Risk" },
    { key: "familyFriendly", label: "Family Friendly" },
    { key: "nightlifeAccess", label: "Nightlife Access" },
    { key: "shoppingAccess", label: "Shopping Access" }
  ];

  const TYPE_LABELS = ["Hotel", "Guesthouse", "Hostel", "Residence", "Apartment", "Villa", "Hanok", "Other"];
  const ZONE_LABELS = ["Core", "Walkable", "Extended", "Nearby", "Needs Coordinates"];
  const REVIEW_STATUSES = ["Needs Review", "Reviewed", "Approved", "Excluded"];

  const state = {
    fileName: "",
    headers: [],
    rawRows: [],
    mapping: {},
    analyzedRows: [],
    selectedRowId: null,
    sortKey: "distanceMeters",
    sortDirection: "asc",
    filters: {
      search: "",
      zone: "all",
      type: "all"
    }
  };

  const els = {
    tabButtons: Array.from(document.querySelectorAll("[data-tab-target]")),
    tabPanels: Array.from(document.querySelectorAll("[data-tab-panel]")),
    tabJumps: Array.from(document.querySelectorAll("[data-tab-jump]")),
    dropZone: document.getElementById("dropZone"),
    csvInput: document.getElementById("csvInput"),
    chooseFileBtn: document.getElementById("chooseFileBtn"),
    statusMessage: document.getElementById("statusMessage"),
    workspace: document.getElementById("workspace"),
    mappingFields: document.getElementById("mappingFields"),
    applyMappingBtn: document.getElementById("applyMappingBtn"),
    recalculateHrpBtn: document.getElementById("recalculateHrpBtn"),
    analyzeBtn: document.getElementById("analyzeBtn"),
    pointALat: document.getElementById("pointALat"),
    pointALng: document.getElementById("pointALng"),
    pointBLat: document.getElementById("pointBLat"),
    pointBLng: document.getElementById("pointBLng"),
    hrpLat: document.getElementById("hrpLat"),
    hrpLng: document.getElementById("hrpLng"),
    coreMax: document.getElementById("coreMax"),
    walkableMax: document.getElementById("walkableMax"),
    extendedMax: document.getElementById("extendedMax"),
    statTotal: document.getElementById("statTotal"),
    statCore: document.getElementById("statCore"),
    statWalkable: document.getElementById("statWalkable"),
    statExtended: document.getElementById("statExtended"),
    statNearby: document.getElementById("statNearby"),
    statNeedsReview: document.getElementById("statNeedsReview"),
    typeStats: document.getElementById("typeStats"),
    zoneStats: document.getElementById("zoneStats"),
    tableSearch: document.getElementById("tableSearch"),
    zoneFilter: document.getElementById("zoneFilter"),
    typeFilter: document.getElementById("typeFilter"),
    clearFiltersBtn: document.getElementById("clearFiltersBtn"),
    resultBody: document.getElementById("resultBody"),
    tableCount: document.getElementById("tableCount"),
    detailHint: document.getElementById("detailHint"),
    detailEmpty: document.getElementById("detailEmpty"),
    detailForm: document.getElementById("detailForm"),
    selectedAccommodationTitle: document.getElementById("selectedAccommodationTitle"),
    basicInfoList: document.getElementById("basicInfoList"),
    calculationInfoList: document.getElementById("calculationInfoList"),
    criteriaFields: document.getElementById("criteriaFields"),
    adminNote: document.getElementById("adminNote"),
    reviewStatus: document.getElementById("reviewStatus"),
    saveReviewBtn: document.getElementById("saveReviewBtn"),
    clearSelectionBtn: document.getElementById("clearSelectionBtn"),
    clearSelectionBtnSecondary: document.getElementById("clearSelectionBtnSecondary"),
    exportJsonBtn: document.getElementById("exportJsonBtn"),
    exportCsvBtn: document.getElementById("exportCsvBtn"),
    exportStatus: document.getElementById("exportStatus")
  };

  initialize();

  function initialize() {
    calculateDefaultHrp();
    bindEvents();
    renderDashboard();
    renderAccommodationList();
    renderDetail();
  }

  function bindEvents() {
    els.tabButtons.forEach(function (button) {
      button.addEventListener("click", function () {
        showTab(button.dataset.tabTarget);
      });
    });

    els.tabJumps.forEach(function (button) {
      button.addEventListener("click", function () {
        showTab(button.dataset.tabJump);
      });
    });

    els.chooseFileBtn.addEventListener("click", function () {
      els.csvInput.click();
    });

    els.csvInput.addEventListener("change", function (event) {
      const file = event.target.files && event.target.files[0];
      if (file) {
        loadFile(file);
      }
    });

    els.dropZone.addEventListener("click", function (event) {
      if (event.target.closest("button") || event.target.closest("input")) {
        return;
      }
      els.csvInput.click();
    });

    els.dropZone.addEventListener("keydown", function (event) {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        els.csvInput.click();
      }
    });

    ["dragenter", "dragover"].forEach(function (eventName) {
      els.dropZone.addEventListener(eventName, function (event) {
        event.preventDefault();
        els.dropZone.classList.add("is-dragging");
      });
    });

    ["dragleave", "drop"].forEach(function (eventName) {
      els.dropZone.addEventListener(eventName, function (event) {
        event.preventDefault();
        els.dropZone.classList.remove("is-dragging");
      });
    });

    els.dropZone.addEventListener("drop", function (event) {
      const file = event.dataTransfer.files && event.dataTransfer.files[0];
      if (file) {
        loadFile(file);
      }
    });

    els.applyMappingBtn.addEventListener("click", function () {
      applyMappingFromSelectors();
      analyzeRows();
    });

    els.recalculateHrpBtn.addEventListener("click", function () {
      calculateDefaultHrp();
      analyzeRows();
    });

    els.analyzeBtn.addEventListener("click", analyzeRows);

    [els.hrpLat, els.hrpLng, els.coreMax, els.walkableMax, els.extendedMax].forEach(function (input) {
      input.addEventListener("change", analyzeRows);
    });

    els.tableSearch.addEventListener("input", function () {
      state.filters.search = els.tableSearch.value.trim().toLowerCase();
      renderAccommodationList();
    });

    els.zoneFilter.addEventListener("change", function () {
      state.filters.zone = els.zoneFilter.value;
      renderAccommodationList();
    });

    els.typeFilter.addEventListener("change", function () {
      state.filters.type = els.typeFilter.value;
      renderAccommodationList();
    });

    els.clearFiltersBtn.addEventListener("click", function () {
      state.filters.search = "";
      state.filters.zone = "all";
      state.filters.type = "all";
      els.tableSearch.value = "";
      els.zoneFilter.value = "all";
      els.typeFilter.value = "all";
      renderAccommodationList();
    });

    document.querySelectorAll("[data-sort]").forEach(function (button) {
      button.addEventListener("click", function () {
        updateSort(button.dataset.sort);
      });
    });

    els.resultBody.addEventListener("click", function (event) {
      const button = event.target.closest("[data-review-row-id]");
      if (!button) {
        return;
      }
      selectRow(Number(button.dataset.reviewRowId));
    });

    els.saveReviewBtn.addEventListener("click", saveSelectedReview);
    els.clearSelectionBtn.addEventListener("click", clearSelection);
    els.clearSelectionBtnSecondary.addEventListener("click", clearSelection);
    els.exportJsonBtn.addEventListener("click", exportJson);
    els.exportCsvBtn.addEventListener("click", exportCsv);
  }

  function showTab(target) {
    els.tabButtons.forEach(function (button) {
      button.classList.toggle("tab-button--active", button.dataset.tabTarget === target);
    });

    els.tabPanels.forEach(function (panel) {
      const isActive = panel.dataset.tabPanel === target;
      panel.hidden = !isActive;
      panel.classList.toggle("tab-panel--active", isActive);
    });
  }

  function loadFile(file) {
    if (!file.name.toLowerCase().endsWith(".csv") && file.type !== "text/csv") {
      setStatus("Please upload a CSV file.", "error");
      return;
    }

    const reader = new FileReader();
    reader.onload = function () {
      try {
        const parsed = parseCsv(String(reader.result || ""));
        if (!parsed.headers.length || !parsed.rows.length) {
          setStatus("The CSV appears to be empty or missing rows.", "error");
          return;
        }

        state.fileName = file.name;
        state.headers = parsed.headers;
        state.rawRows = parsed.rows;
        state.mapping = autoMapColumns(parsed.headers);
        state.analyzedRows = [];
        state.selectedRowId = null;
        renderMappingFields();
        renderDetail();
        els.workspace.hidden = false;
        setStatus(`Loaded ${parsed.rows.length} rows from ${file.name}. Review mapping, then analyze.`, "ok");
        analyzeRows();
      } catch (error) {
        setStatus(`CSV parsing failed: ${error.message}`, "error");
      }
    };
    reader.onerror = function () {
      setStatus("The CSV file could not be read.", "error");
    };
    reader.readAsText(file);
  }

  function parseCsv(text) {
    const matrix = csvToMatrix(text.replace(/^\uFEFF/, ""));
    if (!matrix.length) {
      return { headers: [], rows: [] };
    }

    const headers = matrix[0].map(function (header, index) {
      const value = String(header || "").trim();
      return value || `Column ${index + 1}`;
    });

    const rows = matrix.slice(1)
      .filter(function (row) {
        return row.some(function (cell) {
          return String(cell || "").trim() !== "";
        });
      })
      .map(function (row) {
        const item = {};
        headers.forEach(function (header, index) {
          item[header] = row[index] == null ? "" : String(row[index]).trim();
        });
        return item;
      });

    return { headers, rows };
  }

  function csvToMatrix(text) {
    const rows = [];
    let row = [];
    let cell = "";
    let insideQuotes = false;

    for (let index = 0; index < text.length; index += 1) {
      const char = text[index];
      const next = text[index + 1];

      if (char === '"') {
        if (insideQuotes && next === '"') {
          cell += '"';
          index += 1;
        } else {
          insideQuotes = !insideQuotes;
        }
      } else if (char === "," && !insideQuotes) {
        row.push(cell);
        cell = "";
      } else if ((char === "\n" || char === "\r") && !insideQuotes) {
        if (char === "\r" && next === "\n") {
          index += 1;
        }
        row.push(cell);
        rows.push(row);
        row = [];
        cell = "";
      } else {
        cell += char;
      }
    }

    row.push(cell);
    rows.push(row);
    return rows;
  }

  function autoMapColumns(headers) {
    const usedHeaders = new Set();
    const mapping = {};

    FIELD_DEFS.forEach(function (field) {
      let bestHeader = "";
      let bestScore = 0;

      headers.forEach(function (header) {
        if (usedHeaders.has(header)) {
          return;
        }

        const score = scoreHeader(field, header);
        if (score > bestScore) {
          bestScore = score;
          bestHeader = header;
        }
      });

      if (bestHeader && bestScore >= 55) {
        mapping[field.key] = bestHeader;
        usedHeaders.add(bestHeader);
      } else {
        mapping[field.key] = "";
      }
    });

    return mapping;
  }

  function scoreHeader(field, header) {
    const normalizedHeader = normalizeText(header);
    const compactHeader = compactText(header);
    let bestScore = 0;

    field.aliases.forEach(function (alias) {
      const normalizedAlias = normalizeText(alias);
      const compactAlias = compactText(alias);
      const headerTokens = normalizedHeader.split(" ").filter(Boolean);
      const aliasTokens = normalizedAlias.split(" ").filter(Boolean);
      const overlap = aliasTokens.filter(function (token) {
        return headerTokens.includes(token);
      }).length;

      if (normalizedHeader === normalizedAlias) {
        bestScore = Math.max(bestScore, 100);
      } else if (compactHeader === compactAlias) {
        bestScore = Math.max(bestScore, 96);
      } else if (normalizedHeader.includes(normalizedAlias)) {
        bestScore = Math.max(bestScore, 82);
      } else if (normalizedAlias.includes(normalizedHeader) && normalizedHeader.length > 2) {
        bestScore = Math.max(bestScore, 65);
      } else if (overlap > 0) {
        bestScore = Math.max(bestScore, Math.round((overlap / aliasTokens.length) * 62));
      }
    });

    return bestScore;
  }

  function renderMappingFields() {
    els.mappingFields.textContent = "";

    FIELD_DEFS.forEach(function (field) {
      const label = document.createElement("label");
      label.textContent = field.label;

      const select = document.createElement("select");
      select.dataset.fieldKey = field.key;

      const emptyOption = document.createElement("option");
      emptyOption.value = "";
      emptyOption.textContent = "Not mapped";
      select.appendChild(emptyOption);

      state.headers.forEach(function (header) {
        const option = document.createElement("option");
        option.value = header;
        option.textContent = header;
        option.selected = state.mapping[field.key] === header;
        select.appendChild(option);
      });

      label.appendChild(select);
      els.mappingFields.appendChild(label);
    });
  }

  function applyMappingFromSelectors() {
    document.querySelectorAll("[data-field-key]").forEach(function (select) {
      state.mapping[select.dataset.fieldKey] = select.value;
    });
  }

  function calculateDefaultHrp() {
    const pointALat = parseNumber(els.pointALat.value);
    const pointALng = parseNumber(els.pointALng.value);
    const pointBLat = parseNumber(els.pointBLat.value);
    const pointBLng = parseNumber(els.pointBLng.value);

    if (![pointALat, pointALng, pointBLat, pointBLng].every(Number.isFinite)) {
      setStatus("Reference point coordinates must be valid numbers.", "error");
      return;
    }

    els.hrpLat.value = ((pointALat + pointBLat) / 2).toFixed(6);
    els.hrpLng.value = ((pointALng + pointBLng) / 2).toFixed(6);
  }

  function analyzeRows() {
    if (!state.rawRows.length) {
      return;
    }

    applyMappingFromSelectors();

    const missing = ["hotelName", "latitude", "longitude"].filter(function (key) {
      return !state.mapping[key];
    });

    if (missing.length) {
      setStatus(`Required mapping missing: ${missing.map(labelForField).join(", ")}.`, "error");
      return;
    }

    const hrp = getHrp();
    const thresholds = getThresholds();

    if (!hrp || !thresholds) {
      return;
    }

    const previousRows = new Map(state.analyzedRows.map(function (row) {
      return [row.identityKey, row];
    }));

    state.analyzedRows = state.rawRows.map(function (row, index) {
      const lat = parseNumber(getMappedValue(row, "latitude"));
      const lng = parseNumber(getMappedValue(row, "longitude"));
      const hasCoordinates = Number.isFinite(lat) && Number.isFinite(lng);
      const distanceMeters = hasCoordinates ? Math.round(haversineMeters(lat, lng, hrp.lat, hrp.lng)) : null;
      const walkingMinutes = hasCoordinates ? Math.ceil(distanceMeters / WALKING_METERS_PER_MINUTE) : null;
      const rawType = getMappedValue(row, "type");
      const normalizedType = normalizeType(rawType);
      const baseRow = {
        id: index + 1,
        hotelName: getMappedValue(row, "hotelName") || `Unnamed accommodation ${index + 1}`,
        type: rawType || "Unspecified",
        normalizedType,
        address: getMappedValue(row, "address"),
        latitude: hasCoordinates ? lat : null,
        longitude: hasCoordinates ? lng : null,
        bookingUrl: getMappedValue(row, "bookingUrl"),
        rating: parseNumber(getMappedValue(row, "rating")),
        reviewCount: parseNumber(getMappedValue(row, "reviewCount")),
        price: getMappedValue(row, "price"),
        distanceMeters,
        walkingMinutes,
        zone: hasCoordinates ? classifyZone(walkingMinutes, thresholds) : "Needs Coordinates",
        rawImportedData: copyRawImportedData(row)
      };
      baseRow.identityKey = createIdentityKey(baseRow);
      baseRow.autoSuggestions = createAutoSuggestions(baseRow);

      const previous = previousRows.get(baseRow.identityKey);
      return Object.assign(baseRow, {
        finalCriteria: previous ? cloneCriteria(previous.finalCriteria || previous.criteria) : cloneCriteria(baseRow.autoSuggestions),
        adminNote: previous ? previous.adminNote || "" : createAdminNoteDraft(baseRow),
        reviewStatus: previous ? previous.reviewStatus || "Needs Review" : "Needs Review"
      });
    });

    if (state.selectedRowId && !state.analyzedRows.some(function (row) { return row.id === state.selectedRowId; })) {
      state.selectedRowId = null;
    }

    updateFilters();
    renderDashboard();
    renderAccommodationList();
    renderDetail();
    updateExportState();
    setStatus(`Analyzed ${state.analyzedRows.length} accommodations using HRP ${hrp.lat.toFixed(6)}, ${hrp.lng.toFixed(6)}.`, "ok");
    showTab("list");
  }

  function getMappedValue(row, fieldKey) {
    const header = state.mapping[fieldKey];
    if (!header || !Object.prototype.hasOwnProperty.call(row, header)) {
      return "";
    }
    return row[header];
  }

  function getHrp() {
    const lat = parseNumber(els.hrpLat.value);
    const lng = parseNumber(els.hrpLng.value);
    if (!Number.isFinite(lat) || !Number.isFinite(lng)) {
      setStatus("Final HRP latitude and longitude must be valid numbers.", "error");
      return null;
    }
    return { lat, lng };
  }

  function getThresholds() {
    const core = parseNumber(els.coreMax.value);
    const walkable = parseNumber(els.walkableMax.value);
    const extended = parseNumber(els.extendedMax.value);

    if (![core, walkable, extended].every(Number.isFinite) || core <= 0 || walkable <= core || extended <= walkable) {
      setStatus("Zone thresholds must increase in this order: Core, Walkable, Extended.", "error");
      return null;
    }

    return { core, walkable, extended };
  }

  function classifyZone(minutes, thresholds) {
    if (minutes <= thresholds.core) {
      return "Core";
    }
    if (minutes <= thresholds.walkable) {
      return "Walkable";
    }
    if (minutes <= thresholds.extended) {
      return "Extended";
    }
    return "Nearby";
  }

  function haversineMeters(lat1, lng1, lat2, lng2) {
    const dLat = toRadians(lat2 - lat1);
    const dLng = toRadians(lng2 - lng1);
    const a = Math.sin(dLat / 2) * Math.sin(dLat / 2)
      + Math.cos(toRadians(lat1)) * Math.cos(toRadians(lat2))
      * Math.sin(dLng / 2) * Math.sin(dLng / 2);
    return EARTH_RADIUS_METERS * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  function copyRawImportedData(row) {
    return Object.keys(row).reduce(function (copy, key) {
      copy[key] = row[key];
      return copy;
    }, {});
  }

  function createAutoSuggestions(row) {
    return {
      luggageConvenience: suggestByWalkingTime(row.walkingMinutes),
      hillRisk: "",
      stationAccess: suggestByWalkingTime(row.walkingMinutes),
      airportAccess: suggestByZone(row.zone, {
        Core: "4",
        Walkable: "4",
        Extended: "3",
        Nearby: "2"
      }),
      noiseRisk: suggestByZone(row.zone, {
        Core: "4",
        Walkable: "3",
        Extended: "2",
        Nearby: "2"
      }),
      familyFriendly: suggestByZone(row.zone, {
        Core: "3",
        Walkable: "4",
        Extended: "4",
        Nearby: "3"
      }),
      nightlifeAccess: suggestByZone(row.zone, {
        Core: "5",
        Walkable: "4",
        Extended: "3",
        Nearby: "2"
      }),
      shoppingAccess: suggestByZone(row.zone, {
        Core: "5",
        Walkable: "4",
        Extended: "3",
        Nearby: "2"
      })
    };
  }

  function suggestByWalkingTime(minutes) {
    if (!Number.isFinite(minutes)) {
      return "";
    }
    if (minutes <= 5) {
      return "5";
    }
    if (minutes <= 10) {
      return "4";
    }
    if (minutes <= 15) {
      return "3";
    }
    return "2";
  }

  function suggestByZone(zone, values) {
    return values[zone] || "";
  }

  function createAdminNoteDraft(row) {
    const walkingText = Number.isFinite(row.walkingMinutes)
      ? `approximately ${row.walkingMinutes} minutes from HRP`
      : "walking time could not be calculated from HRP";

    return [
      `${row.hotelName} is classified as ${row.zone}, ${walkingText}.`,
      "Manual checks required: hill condition, actual luggage route, noise environment.",
      "Manual check required: hill/slope condition."
    ].join("\n");
  }

  function renderDashboard() {
    const zoneCounts = countBy(state.analyzedRows, "zone");
    const typeCounts = countBy(state.analyzedRows, "normalizedType");
    const reviewCounts = countBy(state.analyzedRows, "reviewStatus");

    els.statTotal.textContent = String(state.analyzedRows.length);
    els.statCore.textContent = String(zoneCounts.Core || 0);
    els.statWalkable.textContent = String(zoneCounts.Walkable || 0);
    els.statExtended.textContent = String(zoneCounts.Extended || 0);
    els.statNearby.textContent = String(zoneCounts.Nearby || 0);
    els.statNeedsReview.textContent = String(reviewCounts["Needs Review"] || 0);

    renderCountList(els.typeStats, typeCounts, TYPE_LABELS);
    renderCountList(els.zoneStats, zoneCounts, ZONE_LABELS);
  }

  function renderCountList(container, counts, labels) {
    container.textContent = "";
    labels.forEach(function (label) {
      const item = document.createElement("div");
      item.className = "stats-list__item";
      const name = document.createElement("span");
      name.textContent = label;
      const count = document.createElement("strong");
      count.textContent = String(counts[label] || 0);
      item.append(name, count);
      container.appendChild(item);
    });
  }

  function countBy(rows, key) {
    return rows.reduce(function (counts, row) {
      counts[row[key]] = (counts[row[key]] || 0) + 1;
      return counts;
    }, {});
  }

  function updateFilters() {
    rebuildFilter(els.zoneFilter, ZONE_LABELS, "All zones");
    const typeLabels = TYPE_LABELS.filter(function (type) {
      return state.analyzedRows.some(function (row) {
        return row.normalizedType === type;
      });
    });
    rebuildFilter(els.typeFilter, typeLabels, "All types");
  }

  function rebuildFilter(select, labels, defaultLabel) {
    const currentValue = select.value || "all";
    select.textContent = "";

    const allOption = document.createElement("option");
    allOption.value = "all";
    allOption.textContent = defaultLabel;
    select.appendChild(allOption);

    labels.forEach(function (label) {
      const option = document.createElement("option");
      option.value = label;
      option.textContent = label;
      select.appendChild(option);
    });

    select.value = labels.includes(currentValue) ? currentValue : "all";
  }

  function renderAccommodationList() {
    const rows = getFilteredRows().sort(compareRows);
    const fragment = document.createDocumentFragment();
    els.resultBody.textContent = "";

    if (!state.analyzedRows.length) {
      const tr = document.createElement("tr");
      const td = document.createElement("td");
      td.colSpan = 9;
      td.textContent = "No accommodation rows imported yet. Use CSV Import to begin.";
      tr.appendChild(td);
      els.resultBody.appendChild(tr);
      els.tableCount.textContent = "No accommodation rows imported yet.";
      return;
    }

    rows.forEach(function (row) {
      const tr = document.createElement("tr");
      appendCell(tr, row.hotelName);
      appendZoneCell(tr, row.zone);
      appendCell(tr, row.distanceMeters === null ? "N/A" : formatDistance(row.distanceMeters));
      appendCell(tr, row.walkingMinutes === null ? "N/A" : `${row.walkingMinutes} min`);
      appendCell(tr, row.type);
      appendCell(tr, Number.isFinite(row.rating) ? row.rating.toFixed(1) : "N/A");
      appendCell(tr, Number.isFinite(row.reviewCount) ? formatInteger(row.reviewCount) : "N/A");
      appendStatusCell(tr, row.reviewStatus);
      appendReviewButtonCell(tr, row.id);
      fragment.appendChild(tr);
    });

    els.resultBody.appendChild(fragment);
    els.tableCount.textContent = `${rows.length} of ${state.analyzedRows.length} rows shown.`;
  }

  function getFilteredRows() {
    return state.analyzedRows.filter(function (row) {
      const searchText = [
        row.hotelName,
        row.type,
        row.address,
        row.zone,
        row.reviewStatus,
        row.rating,
        row.reviewCount
      ].join(" ").toLowerCase();

      const matchesSearch = !state.filters.search || searchText.includes(state.filters.search);
      const matchesZone = state.filters.zone === "all" || row.zone === state.filters.zone;
      const matchesType = state.filters.type === "all" || row.normalizedType === state.filters.type;

      return matchesSearch && matchesZone && matchesType;
    });
  }

  function compareRows(a, b) {
    const direction = state.sortDirection === "asc" ? 1 : -1;
    const aValue = a[state.sortKey];
    const bValue = b[state.sortKey];

    if (aValue === null || aValue === undefined || aValue === "") {
      return 1;
    }
    if (bValue === null || bValue === undefined || bValue === "") {
      return -1;
    }

    if (typeof aValue === "number" && typeof bValue === "number") {
      return (aValue - bValue) * direction;
    }

    return String(aValue).localeCompare(String(bValue), undefined, { numeric: true, sensitivity: "base" }) * direction;
  }

  function updateSort(key) {
    if (state.sortKey === key) {
      state.sortDirection = state.sortDirection === "asc" ? "desc" : "asc";
    } else {
      state.sortKey = key;
      state.sortDirection = "asc";
    }
    renderAccommodationList();
  }

  function appendCell(row, value) {
    const td = document.createElement("td");
    td.textContent = value == null || value === "" ? "N/A" : String(value);
    row.appendChild(td);
  }

  function appendZoneCell(row, zone) {
    const td = document.createElement("td");
    const pill = document.createElement("span");
    pill.className = `zone-pill zone-pill--${normalizeText(zone).replace(/\s+/g, "-")}`;
    pill.textContent = zone;
    td.appendChild(pill);
    row.appendChild(td);
  }

  function appendStatusCell(row, status) {
    const td = document.createElement("td");
    const pill = document.createElement("span");
    pill.className = `status-pill status-pill--${normalizeText(status).replace(/\s+/g, "-")}`;
    pill.textContent = status;
    td.appendChild(pill);
    row.appendChild(td);
  }

  function appendReviewButtonCell(row, id) {
    const td = document.createElement("td");
    const button = document.createElement("button");
    button.className = "button button--secondary";
    button.type = "button";
    button.dataset.reviewRowId = String(id);
    button.textContent = "Review";
    td.appendChild(button);
    row.appendChild(td);
  }

  function selectRow(id) {
    state.selectedRowId = id;
    renderDetail();
    showTab("detail");
  }

  function renderDetail() {
    const row = getSelectedRow();
    if (!row) {
      els.detailEmpty.hidden = false;
      els.detailForm.hidden = true;
      els.detailHint.textContent = "Select a row from Accommodation List to review imported data and Korea Inside suggestions.";
      return;
    }

    els.detailEmpty.hidden = true;
    els.detailForm.hidden = false;
    els.selectedAccommodationTitle.textContent = row.hotelName;
    els.detailHint.textContent = `Editing ${row.hotelName}. Changes are stored in browser memory until export.`;

    renderDefinitionList(els.basicInfoList, [
      ["Hotel Name", row.hotelName],
      ["Type", row.type],
      ["Address", row.address],
      ["Latitude", row.latitude],
      ["Longitude", row.longitude],
      ["Booking URL", row.bookingUrl],
      ["Rating", Number.isFinite(row.rating) ? row.rating.toFixed(1) : "N/A"],
      ["Review Count", Number.isFinite(row.reviewCount) ? formatInteger(row.reviewCount) : "N/A"],
      ["Price", row.price]
    ]);

    renderDefinitionList(els.calculationInfoList, [
      ["HRP Distance", row.distanceMeters === null ? "N/A" : formatDistance(row.distanceMeters)],
      ["Walking Time", row.walkingMinutes === null ? "N/A" : `${row.walkingMinutes} min`],
      ["Korea Inside Zone", row.zone]
    ]);

    document.querySelectorAll("[data-suggestion-key]").forEach(function (element) {
      const key = element.dataset.suggestionKey;
      element.textContent = formatSuggestionValue(key, row.autoSuggestions[key]);
    });

    document.querySelectorAll("[data-criteria-key]").forEach(function (select) {
      select.value = row.finalCriteria[select.dataset.criteriaKey] || "";
    });

    els.adminNote.value = row.adminNote || "";
    els.reviewStatus.value = REVIEW_STATUSES.includes(row.reviewStatus) ? row.reviewStatus : "Needs Review";
  }

  function renderDefinitionList(container, pairs) {
    container.textContent = "";
    pairs.forEach(function (pair) {
      const dt = document.createElement("dt");
      const dd = document.createElement("dd");
      dt.textContent = pair[0];
      dd.textContent = pair[1] == null || pair[1] === "" ? "N/A" : String(pair[1]);
      container.append(dt, dd);
    });
  }

  function formatSuggestionValue(key, value) {
    if (key === "hillRisk") {
      return "Manual check";
    }
    return value || "N/A";
  }

  function saveSelectedReview() {
    const row = getSelectedRow();
    if (!row) {
      setStatus("Select an accommodation before saving review details.", "error");
      showTab("detail");
      return;
    }

    document.querySelectorAll("[data-criteria-key]").forEach(function (select) {
      row.finalCriteria[select.dataset.criteriaKey] = select.value;
    });
    row.adminNote = els.adminNote.value.trim();
    row.reviewStatus = els.reviewStatus.value;

    renderDashboard();
    renderAccommodationList();
    renderDetail();
    updateExportState();
    setStatus(`Saved review for ${row.hotelName}.`, "ok");
  }

  function clearSelection() {
    state.selectedRowId = null;
    renderDetail();
  }

  function getSelectedRow() {
    return state.analyzedRows.find(function (row) {
      return row.id === state.selectedRowId;
    });
  }

  function updateExportState() {
    const hasRows = state.analyzedRows.length > 0;
    els.exportJsonBtn.disabled = !hasRows;
    els.exportCsvBtn.disabled = !hasRows;
    els.exportStatus.textContent = hasRows
      ? `${state.analyzedRows.length} accommodation rows ready for export.`
      : "Import and analyze a CSV before exporting.";
  }

  function exportJson() {
    const hrp = getHrp();
    const thresholds = getThresholds();
    if (!hrp || !thresholds || !state.analyzedRows.length) {
      return;
    }

    const payload = {
      tool: "Korea Inside Accommodation Admin",
      version: "1.2",
      generatedAt: new Date().toISOString(),
      sourceFile: state.fileName,
      hrp,
      zoneThresholdsMinutes: thresholds,
      walkingMetersPerMinute: WALKING_METERS_PER_MINUTE,
      rows: state.analyzedRows.map(toExportRow)
    };

    downloadBlob(JSON.stringify(payload, null, 2), "application/json", `${exportBaseName()}.json`);
  }

  function exportCsv() {
    if (!state.analyzedRows.length) {
      return;
    }

    const headers = [
      "Hotel Name",
      "Type",
      "Normalized Type",
      "Address",
      "Latitude",
      "Longitude",
      "Distance (m)",
      "Walking Time (min)",
      "Zone",
      "Rating",
      "Review Count",
      "Price",
      "Booking URL",
      "Raw Imported Data",
      "Suggested Luggage Convenience",
      "Suggested Hill Risk",
      "Suggested Station Access",
      "Suggested Airport Access",
      "Suggested Noise Risk",
      "Suggested Family Friendly",
      "Suggested Nightlife Access",
      "Suggested Shopping Access",
      "Final Luggage Convenience",
      "Final Hill Risk",
      "Final Station Access",
      "Final Airport Access",
      "Final Noise Risk",
      "Final Family Friendly",
      "Final Nightlife Access",
      "Final Shopping Access",
      "Admin Note",
      "Review Status"
    ];

    const lines = [headers.map(escapeCsvValue).join(",")];
    state.analyzedRows.forEach(function (row) {
      lines.push([
        row.hotelName,
        row.type,
        row.normalizedType,
        row.address,
        row.latitude,
        row.longitude,
        row.distanceMeters,
        row.walkingMinutes,
        row.zone,
        Number.isFinite(row.rating) ? row.rating : "",
        Number.isFinite(row.reviewCount) ? row.reviewCount : "",
        row.price,
        row.bookingUrl,
        JSON.stringify(row.rawImportedData),
        row.autoSuggestions.luggageConvenience,
        row.autoSuggestions.hillRisk,
        row.autoSuggestions.stationAccess,
        row.autoSuggestions.airportAccess,
        row.autoSuggestions.noiseRisk,
        row.autoSuggestions.familyFriendly,
        row.autoSuggestions.nightlifeAccess,
        row.autoSuggestions.shoppingAccess,
        row.finalCriteria.luggageConvenience,
        row.finalCriteria.hillRisk,
        row.finalCriteria.stationAccess,
        row.finalCriteria.airportAccess,
        row.finalCriteria.noiseRisk,
        row.finalCriteria.familyFriendly,
        row.finalCriteria.nightlifeAccess,
        row.finalCriteria.shoppingAccess,
        row.adminNote,
        row.reviewStatus
      ].map(escapeCsvValue).join(","));
    });

    downloadBlob(lines.join("\n"), "text/csv;charset=utf-8", `${exportBaseName()}.csv`);
  }

  function toExportRow(row) {
    return {
      id: row.id,
      hotelName: row.hotelName,
      type: row.type,
      normalizedType: row.normalizedType,
      address: row.address,
      latitude: row.latitude,
      longitude: row.longitude,
      bookingUrl: row.bookingUrl,
      rating: Number.isFinite(row.rating) ? row.rating : null,
      reviewCount: Number.isFinite(row.reviewCount) ? row.reviewCount : null,
      price: row.price,
      distanceMeters: row.distanceMeters,
      walkingMinutes: row.walkingMinutes,
      zone: row.zone,
      rawImportedData: copyRawImportedData(row.rawImportedData),
      autoSuggestions: cloneCriteria(row.autoSuggestions),
      finalCriteria: cloneCriteria(row.finalCriteria),
      adminNote: row.adminNote,
      reviewStatus: row.reviewStatus
    };
  }

  function downloadBlob(content, type, fileName) {
    const blob = new Blob([content], { type });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    link.remove();
    window.setTimeout(function () {
      URL.revokeObjectURL(url);
    }, 1000);
  }

  function exportBaseName() {
    const source = state.fileName.replace(/\.csv$/i, "") || "accommodation-admin";
    return `${source}-korea-inside-v1-2`;
  }

  function escapeCsvValue(value) {
    const text = value == null ? "" : String(value);
    if (/[",\n\r]/.test(text)) {
      return `"${text.replace(/"/g, '""')}"`;
    }
    return text;
  }

  function normalizeType(value) {
    const text = normalizeText(value);
    if (!text) return "Other";
    if (text.includes("hanok")) return "Hanok";
    if (text.includes("hostel")) return "Hostel";
    if (text.includes("guesthouse") || text.includes("guest house")) return "Guesthouse";
    if (text.includes("residence") || text.includes("serviced apartment") || text.includes("aparthotel") || text.includes("condo hotel")) return "Residence";
    if (text.includes("apartment") || text.includes("flat")) return "Apartment";
    if (text.includes("villa")) return "Villa";
    if (text.includes("hotel") || text.includes("motel") || text.includes("resort")) return "Hotel";
    return "Other";
  }

  function createEmptyCriteria() {
    return CRITERIA_DEFS.reduce(function (criteria, field) {
      criteria[field.key] = "";
      return criteria;
    }, {});
  }

  function cloneCriteria(criteria) {
    const source = criteria || {};
    return CRITERIA_DEFS.reduce(function (copy, field) {
      copy[field.key] = source[field.key] || "";
      return copy;
    }, {});
  }

  function createIdentityKey(row) {
    return [row.hotelName, row.bookingUrl, row.address].map(function (value) {
      return normalizeText(value);
    }).join("|");
  }

  function parseNumber(value) {
    const text = String(value == null ? "" : value).replace(/,/g, "").trim();
    const match = text.match(/-?\d+(\.\d+)?/);
    return match ? Number(match[0]) : NaN;
  }

  function normalizeText(value) {
    return String(value == null ? "" : value)
      .toLowerCase()
      .replace(/[_-]+/g, " ")
      .replace(/[^a-z0-9 ]+/g, " ")
      .replace(/\s+/g, " ")
      .trim();
  }

  function compactText(value) {
    return normalizeText(value).replace(/\s+/g, "");
  }

  function labelForField(key) {
    const field = FIELD_DEFS.find(function (item) {
      return item.key === key;
    });
    return field ? field.label : key;
  }

  function formatDistance(value) {
    if (!Number.isFinite(value)) {
      return "N/A";
    }
    if (value >= 1000) {
      return `${(value / 1000).toFixed(1)} km`;
    }
    return `${Math.round(value)} m`;
  }

  function formatInteger(value) {
    return Number(value).toLocaleString("en-US", { maximumFractionDigits: 0 });
  }

  function toRadians(value) {
    return value * Math.PI / 180;
  }

  function setStatus(message, type) {
    els.statusMessage.textContent = message;
    els.statusMessage.classList.remove("status-ok", "status-error");
    if (type) {
      els.statusMessage.classList.add(`status-${type}`);
    }
  }
}());

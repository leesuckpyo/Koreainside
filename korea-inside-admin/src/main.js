window.addEventListener("DOMContentLoaded", () => {
  const viewTitle = document.querySelector("#view-title");
  const initializationState = document.querySelector("#initialization-state");
  const navigationItems = document.querySelectorAll(".nav-item[data-view]");
  const viewPanels = document.querySelectorAll("[data-panel]");
  const selectRepositoryButton = document.querySelector("#select-repository");
  const disconnectRepositoryButton = document.querySelector("#disconnect-repository");
  const connectionBadge = document.querySelector("#connection-badge");
  const readOnlyBadge = document.querySelector("#read-only-badge");
  const repositoryLoading = document.querySelector("#repository-loading");
  const repositoryError = document.querySelector("#repository-error");
  const repositoryEmpty = document.querySelector("#repository-empty");
  const repositoryConnected = document.querySelector("#repository-connected");
  const repositoryPath = document.querySelector("#repository-path");
  const repositoryCounts = document.querySelector("#repository-counts");
  const repositoryWarning = document.querySelector("#repository-warning");
  const repositoryTree = document.querySelector("#repository-tree");
  const dashboardRepositoryStatus = document.querySelector("#dashboard-repository-status");
  const dashboardAccessMode = document.querySelector("#dashboard-access-mode");
  const dashboardAnalyticsStatus = document.querySelector("#dashboard-analytics-status");
  const analyticsConnectionBadge = document.querySelector("#analytics-connection-badge");
  const analyticsCredentialStatus = document.querySelector("#analytics-credential-status");
  const analyticsTestStatus = document.querySelector("#analytics-test-status");
  const analyticsLastChecked = document.querySelector("#analytics-last-checked");
  const vercelTokenForm = document.querySelector("#vercel-token-form");
  const vercelAccessToken = document.querySelector("#vercel-access-token");
  const saveVercelTokenButton = document.querySelector("#save-vercel-token");
  const testVercelConnectionButton = document.querySelector("#test-vercel-connection");
  const deleteVercelTokenButton = document.querySelector("#delete-vercel-token");
  const analyticsMessage = document.querySelector("#analytics-message");
  const analyticsSummary = document.querySelector("#analytics-summary");
  const analyticsPeriodInputs = document.querySelectorAll('input[name="analytics-period"]');
  const refreshAnalyticsSummaryButton = document.querySelector("#refresh-analytics-summary");
  const analyticsVisitors = document.querySelector("#analytics-visitors");
  const analyticsPageviews = document.querySelector("#analytics-pageviews");
  const analyticsRange = document.querySelector("#analytics-range");
  const analyticsFetchedAt = document.querySelector("#analytics-fetched-at");
  const analyticsCacheStatus = document.querySelector("#analytics-cache-status");
  const analyticsSummaryMessage = document.querySelector("#analytics-summary-message");
  const exportFormatInputs = document.querySelectorAll('input[name="export-format"]');
  const previewExportButton = document.querySelector("#preview-export");
  const saveExportButton = document.querySelector("#save-export");
  const exportStatus = document.querySelector("#export-status");
  const exportPreview = document.querySelector("#export-preview");
  const exportHistory = document.querySelector("#export-history");
  const exportHistoryEmpty = document.querySelector("#export-history-empty");
  const runSiteStatusAuditButton = document.querySelector("#run-site-status-audit");
  const siteStatusBadge = document.querySelector("#site-status-badge");
  const siteStatusRepositoryPath = document.querySelector("#site-status-repository-path");
  const siteStatusRepositoryEmpty = document.querySelector("#site-status-repository-empty");
  const siteStatusLastChecked = document.querySelector("#site-status-last-checked");
  const siteStatusMessage = document.querySelector("#site-status-message");
  const siteStatusHtmlCount = document.querySelector("#site-status-html-count");
  const siteStatusOkCount = document.querySelector("#site-status-ok-count");
  const siteStatusWarningCount = document.querySelector("#site-status-warning-count");
  const siteStatusErrorCount = document.querySelector("#site-status-error-count");
  const siteStatusSitemapExists = document.querySelector("#site-status-sitemap-exists");
  const siteStatusRobotsExists = document.querySelector("#site-status-robots-exists");
  const siteStatusVercelJsonExists = document.querySelector("#site-status-vercel-json-exists");
  const siteStatusVercelignoreExists = document.querySelector("#site-status-vercelignore-exists");
  const siteStatusSitemapUrlCount = document.querySelector("#site-status-sitemap-url-count");
  const siteStatusSitemapDuplicates = document.querySelector("#site-status-sitemap-duplicates");
  const siteStatusRobotsSitemap = document.querySelector("#site-status-robots-sitemap");
  const siteStatusPublicMissing = document.querySelector("#site-status-public-missing");
  const siteStatusLocalMissing = document.querySelector("#site-status-local-missing");
  const siteStatusExclusionEvidence = document.querySelector("#site-status-exclusion-evidence");
  const siteStatusReviewRequired = document.querySelector("#site-status-review-required");
  const siteStatusPagesEmpty = document.querySelector("#site-status-pages-empty");
  const siteStatusPagesBody = document.querySelector("#site-status-pages-body");
  const viewTitles = {
    dashboard: "운영 대시보드",
    analytics: "Vercel Analytics",
    "site-status": "사이트 상태점검",
    explorer: "프로젝트 탐색기",
  };
  let connectedRepository = null;
  let repositorySelectionLoading = false;
  let vercelCredentialStored = false;
  let analyticsConnectionLoading = false;
  let analyticsSummaryLoading = false;
  let siteStatusLoading = false;
  let analyticsSummaryState = {
    period: "7d",
    status: "idle",
    result: null,
    errorCode: null,
    message: null,
    retryAt: null,
  };
  const allowedAnalyticsPeriods = new Set(["24h", "7d", "30d"]);
  const analyticsErrorMessages = {
    not_configured: "먼저 Vercel Access Token을 저장해 주십시오.",
    credential_changed: "자격증명이 변경되었습니다. 잠시 후 다시 조회해 주십시오.",
    credential_read_failed: "Windows 자격 증명 관리자에서 연결 정보를 확인할 수 없습니다.",
    request_in_progress: "같은 기간의 조회가 이미 진행 중입니다.",
    rate_limited: "Vercel 요청 제한 상태입니다.",
    unauthorized: "저장된 Vercel 자격증명이 유효하지 않습니다.",
    forbidden: "Vercel Analytics를 읽을 권한이 없습니다.",
    plan_or_billing_required: "Vercel 플랜 또는 결제 상태를 확인해 주십시오.",
    invalid_request: "Analytics 조회 요청을 처리할 수 없습니다.",
    not_found: "Vercel 프로젝트 또는 Analytics 데이터를 확인할 수 없습니다.",
    service_unavailable: "Vercel 서비스가 일시적으로 요청을 처리하지 못했습니다.",
    timeout: "Vercel 응답 시간이 초과되었습니다.",
    network_error: "Vercel API에 연결할 수 없습니다.",
    response_too_large: "Analytics 응답 크기가 허용 범위를 초과했습니다.",
    invalid_response: "Vercel Analytics 응답 형식을 확인할 수 없습니다.",
  };
  const analyticsDateFormatter = new Intl.DateTimeFormat("ko-KR", {
    timeZone: "Asia/Seoul",
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });

  navigationItems.forEach((item) => {
    item.addEventListener("click", () => {
      const selectedView = item.dataset.view;

      navigationItems.forEach((navigationItem) => {
        const isSelected = navigationItem === item;
        navigationItem.classList.toggle("is-active", isSelected);
        if (isSelected) {
          navigationItem.setAttribute("aria-current", "page");
        } else {
          navigationItem.removeAttribute("aria-current");
        }
      });

      viewPanels.forEach((panel) => {
        panel.hidden = panel.dataset.panel !== selectedView;
      });

      viewTitle.textContent = viewTitles[selectedView];
    });
  });

  selectRepositoryButton.addEventListener("click", async () => {
    setLoading(true);
    clearError();

    try {
      const result = await window.__TAURI__.core.invoke("select_repository");

      if (result.status === "cancelled") {
        return;
      }

      if (result.status === "error") {
        showError(result.message || "저장소를 연결할 수 없습니다.");
        return;
      }

      if (result.status !== "connected") {
        showError("저장소 연결 결과를 확인할 수 없습니다.");
        return;
      }

      connectedRepository = result;
      renderConnectedRepository();
    } catch (error) {
      showError("관리자 앱과 통신할 수 없습니다.");
    } finally {
      setLoading(false);
    }
  });

  disconnectRepositoryButton.addEventListener("click", async () => {
    clearError();
    setLoading(true);
    try {
      await window.__TAURI__.core.invoke("disconnect_repository");
      connectedRepository = null;
      renderDisconnectedRepository();
    } catch (error) {
      showError("저장소 연결을 해제할 수 없습니다.");
    } finally {
      setLoading(false);
    }
  });

  exportFormatInputs.forEach((input) => {
    input.addEventListener("change", resetExportPreview);
  });

  if (runSiteStatusAuditButton) {
    runSiteStatusAuditButton.addEventListener("click", runSiteStatusAudit);
  }

  previewExportButton.addEventListener("click", async () => {
    setExportLoading(true, "내보내기 정보를 확인하고 있습니다.");
    try {
      const result = await window.__TAURI__.core.invoke("preview_repository_export", {
        format: selectedExportFormat(),
      });
      if (result.status !== "ready") {
        setExportStatus(result.message || "내보내기 미리보기를 확인할 수 없습니다.", "error");
        exportPreview.hidden = true;
        return;
      }
      renderExportPreview(result);
      setExportStatus("내보내기 미리보기가 준비되었습니다.");
    } catch (error) {
      setExportStatus("관리자 앱과 통신할 수 없습니다.", "error");
    } finally {
      setExportLoading(false);
    }
  });

  saveExportButton.addEventListener("click", async () => {
    const format = selectedExportFormat();
    setExportLoading(true, "저장 위치를 선택하십시오.");
    try {
      const result = await window.__TAURI__.core.invoke("export_repository_inventory", { format });
      if (result.status === "cancelled") {
        setExportStatus("파일 저장을 취소했습니다.");
        addExportHistory(format, "취소", null, null);
        return;
      }
      if (result.status !== "saved") {
        setExportStatus(result.message || "파일을 저장할 수 없습니다.", "error");
        addExportHistory(format, "실패", null, null);
        return;
      }
      setExportStatus(`${result.fileName} 파일을 저장했습니다.`, "success");
      addExportHistory(format, "성공", result.fileName, result.sizeBytes);
    } catch (error) {
      setExportStatus("관리자 앱과 통신할 수 없습니다.", "error");
      addExportHistory(format, "실패", null, null);
    } finally {
      setExportLoading(false);
    }
  });

  analyticsPeriodInputs.forEach((input) => {
    input.addEventListener("change", () => {
      if (!input.checked || !allowedAnalyticsPeriods.has(input.value)) {
        return;
      }
      resetAnalyticsSummary(input.value);
    });
  });

  refreshAnalyticsSummaryButton.addEventListener("click", async () => {
    if (analyticsSummaryLoading || analyticsConnectionLoading) {
      return;
    }

    const selectedPeriod = selectedAnalyticsPeriod();
    if (!allowedAnalyticsPeriods.has(selectedPeriod)) {
      renderAnalyticsSummaryError("invalid_response", null);
      return;
    }

    setAnalyticsSummaryLoading(true);
    setAnalyticsSummaryMessage("Vercel Analytics 요약을 조회하고 있습니다.");
    try {
      const result = await window.__TAURI__.core.invoke("get_vercel_analytics_summary", {
        period: selectedPeriod,
      });
      if (result && result.status === "error") {
        const errorCode = typeof result.errorCode === "string" ? result.errorCode : "invalid_response";
        const retryAt = typeof result.retryAt === "string" ? result.retryAt : null;
        renderAnalyticsSummaryError(errorCode, retryAt);
        return;
      }
      if (!isValidAnalyticsSummary(result, selectedPeriod)) {
        renderAnalyticsSummaryError("invalid_response", null);
        return;
      }
      renderAnalyticsSummarySuccess(result);
    } catch (error) {
      renderAnalyticsSummaryError("network_error", null);
    } finally {
      setAnalyticsSummaryLoading(false);
    }
  });

  vercelTokenForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    let token = vercelAccessToken.value;
    vercelAccessToken.value = "";
    resetAnalyticsSummary(selectedAnalyticsPeriod());
    setAnalyticsLoading(true, "Access Token을 안전하게 저장하고 있습니다.");

    try {
      const result = await window.__TAURI__.core.invoke("save_vercel_access_token", { token });
      renderAnalyticsStatus(result);
      if (result.status === "credential_stored") {
        setAnalyticsMessage("Access Token을 Windows 자격 증명 관리자에 저장했습니다.", "success");
      } else {
        setAnalyticsMessage(result.message || "Access Token을 저장할 수 없습니다.", "error");
      }
    } catch (error) {
      renderAnalyticsError("관리자 앱과 통신할 수 없습니다.");
    } finally {
      token = "";
      vercelAccessToken.value = "";
      setAnalyticsLoading(false);
    }
  });

  testVercelConnectionButton.addEventListener("click", async () => {
    setAnalyticsLoading(true, "Vercel Analytics 연결을 확인하고 있습니다.");
    try {
      const result = await window.__TAURI__.core.invoke("test_vercel_analytics_connection");
      renderAnalyticsStatus(result);
      if (result.status === "connected") {
        setAnalyticsMessage("Vercel Analytics 읽기 전용 연결을 확인했습니다.", "success");
      } else {
        setAnalyticsMessage(result.message || "Vercel Analytics 연결에 실패했습니다.", "error");
      }
    } catch (error) {
      renderAnalyticsError("관리자 앱과 통신할 수 없습니다.");
    } finally {
      setAnalyticsLoading(false);
    }
  });

  deleteVercelTokenButton.addEventListener("click", async () => {
    resetAnalyticsSummary(selectedAnalyticsPeriod());
    setAnalyticsLoading(true, "저장된 자격 증명을 삭제하고 있습니다.");
    try {
      const result = await window.__TAURI__.core.invoke("delete_vercel_access_token");
      renderAnalyticsStatus(result);
      if (result.status === "not_configured") {
        setAnalyticsMessage("저장된 Vercel 자격 증명을 삭제했습니다.", "success");
      } else {
        setAnalyticsMessage(result.message || "자격 증명을 삭제할 수 없습니다.", "error");
      }
    } catch (error) {
      renderAnalyticsError("관리자 앱과 통신할 수 없습니다.");
    } finally {
      setAnalyticsLoading(false);
    }
  });

  initializationState.textContent = "관리자 앱 기반 구성이 완료되었습니다.";
  refreshVercelConnectionStatus();
  resetSiteStatusReport();

  function setLoading(isLoading) {
    repositorySelectionLoading = isLoading;
    selectRepositoryButton.disabled = isLoading;
    disconnectRepositoryButton.disabled = isLoading;
    repositoryLoading.hidden = !isLoading;
    repositoryConnected.setAttribute("aria-busy", String(isLoading));
    updateSiteStatusControls();
  }

  function showError(message) {
    repositoryError.textContent = message;
    repositoryError.hidden = false;
  }

  function clearError() {
    repositoryError.textContent = "";
    repositoryError.hidden = true;
  }

  function renderDisconnectedRepository() {
    connectionBadge.textContent = "연결되지 않음";
    connectionBadge.classList.remove("is-connected");
    readOnlyBadge.hidden = true;
    disconnectRepositoryButton.hidden = true;
    repositoryEmpty.hidden = false;
    repositoryConnected.hidden = true;
    repositoryPath.textContent = "";
    repositoryCounts.textContent = "";
    repositoryWarning.textContent = "";
    repositoryWarning.hidden = true;
    repositoryTree.replaceChildren();
    dashboardRepositoryStatus.textContent = "선택되지 않음";
    dashboardAccessMode.textContent = "읽기 전용 예정";
    resetExportPreview();
    resetSiteStatusReport();
  }

  function renderConnectedRepository() {
    connectionBadge.textContent = "연결됨";
    connectionBadge.classList.add("is-connected");
    readOnlyBadge.hidden = false;
    disconnectRepositoryButton.hidden = false;
    repositoryEmpty.hidden = true;
    repositoryConnected.hidden = false;
    repositoryPath.textContent = connectedRepository.rootPath;
    repositoryCounts.textContent = [
      `표시 항목 ${connectedRepository.totalItems.toLocaleString("ko-KR")}개`,
      `제외 ${connectedRepository.excludedCount.toLocaleString("ko-KR")}개`,
      `건너뜀 ${connectedRepository.skippedCount.toLocaleString("ko-KR")}개`,
    ].join(" · ");

    renderWarnings();
    renderTree();
    dashboardRepositoryStatus.textContent = "연결됨";
    dashboardAccessMode.textContent = "읽기 전용";
    resetExportPreview();
    resetSiteStatusReport();
  }

  function selectedExportFormat() {
    return document.querySelector('input[name="export-format"]:checked').value;
  }

  function setExportLoading(isLoading, message = "") {
    previewExportButton.disabled = isLoading || !connectedRepository;
    saveExportButton.disabled = isLoading || !connectedRepository;
    exportFormatInputs.forEach((input) => {
      input.disabled = isLoading || !connectedRepository;
    });
    if (isLoading) {
      setExportStatus(message);
    }
  }

  function setExportStatus(message, kind = "default") {
    exportStatus.textContent = message;
    exportStatus.classList.toggle("is-error", kind === "error");
    exportStatus.classList.toggle("is-success", kind === "success");
  }

  function resetExportPreview() {
    exportPreview.replaceChildren();
    exportPreview.hidden = true;
    setExportStatus(
      connectedRepository
        ? "내보내기 형식을 선택하고 미리보기를 확인하십시오."
        : "저장소를 연결하면 내보내기를 사용할 수 있습니다.",
    );
    setExportLoading(false);
  }

  function renderExportPreview(result) {
    const details = document.createElement("dl");
    const values = [
      ["형식", result.format.toUpperCase()],
      ["항목 수", `${result.totalItems.toLocaleString("ko-KR")}개`],
      ["예상 크기", formatBytes(result.estimatedBytes)],
      ["데이터 상태", result.partial || result.truncated ? "부분 결과" : "전체 결과"],
    ];
    values.forEach(([label, value]) => {
      const term = document.createElement("dt");
      term.textContent = label;
      const description = document.createElement("dd");
      description.textContent = value;
      const item = document.createElement("div");
      item.append(term, description);
      details.append(item);
    });
    exportPreview.replaceChildren(details);
    exportPreview.hidden = false;
  }

  function addExportHistory(format, status, fileName, sizeBytes) {
    const item = document.createElement("li");
    const summary = document.createElement("span");
    summary.textContent = fileName || `${format.toUpperCase()} 저장`;
    const details = document.createElement("span");
    const parts = [new Date().toLocaleTimeString("ko-KR"), status];
    if (Number.isFinite(sizeBytes)) {
      parts.push(formatBytes(sizeBytes));
    }
    details.textContent = parts.join(" · ");
    item.append(summary, details);
    exportHistory.prepend(item);
    exportHistoryEmpty.hidden = true;
  }

  function formatBytes(bytes) {
    if (bytes < 1024) {
      return `${bytes.toLocaleString("ko-KR")} B`;
    }
    return `${(bytes / 1024).toLocaleString("ko-KR", { maximumFractionDigits: 1 })} KB`;
  }

  async function runSiteStatusAudit() {
    if (siteStatusLoading) {
      return;
    }

    const repositoryRoot = connectedRepository && connectedRepository.rootPath;
    if (!repositoryRoot) {
      setSiteStatusMessage("저장소를 먼저 연결하십시오.", "error");
      updateSiteStatusControls();
      return;
    }

    setSiteStatusLoading(true);
    setSiteStatusBadge("점검 중", "loading");
    setSiteStatusMessage("연결된 저장소를 읽기 전용으로 점검하고 있습니다.");
    setText(siteStatusLastChecked, new Date().toLocaleString("ko-KR"));

    try {
      const result = await window.__TAURI__.core.invoke("get_site_status_report", {
        repositoryRoot,
      });

      if (!isValidSiteStatusReport(result)) {
        renderSiteStatusError("사이트 상태 점검 응답 형식을 확인할 수 없습니다.");
        return;
      }

      if (!connectedRepository || connectedRepository.rootPath !== repositoryRoot) {
        renderSiteStatusError("저장소 연결 상태가 변경되었습니다. 다시 점검하십시오.");
        return;
      }

      if (result.status === "error") {
        renderSiteStatusError(result.message || "사이트 상태를 점검할 수 없습니다.", result.errorCode);
        return;
      }

      renderSiteStatusReport(result);
    } catch (error) {
      renderSiteStatusError("사이트 상태 점검 command를 실행할 수 없습니다.", "invoke_failed");
    } finally {
      setSiteStatusLoading(false);
    }
  }

  function resetSiteStatusReport() {
    updateSiteStatusRepositoryState();
    setSiteStatusBadge("점검 전", "idle");
    setSiteStatusMessage("연결된 저장소를 읽기 전용으로 점검합니다. 자동 수정이나 파일 저장은 수행하지 않습니다.");
    setText(siteStatusLastChecked, "마지막 점검 없음");
    setText(siteStatusHtmlCount, "—");
    setText(siteStatusOkCount, "—");
    setText(siteStatusWarningCount, "—");
    setText(siteStatusErrorCount, "—");
    setText(siteStatusSitemapExists, "—");
    setText(siteStatusRobotsExists, "—");
    setText(siteStatusVercelJsonExists, "—");
    setText(siteStatusVercelignoreExists, "—");
    setText(siteStatusSitemapUrlCount, "—");
    setText(siteStatusSitemapDuplicates, "—");
    setText(siteStatusRobotsSitemap, "—");
    renderSiteStatusList(siteStatusPublicMissing, ["점검 전"]);
    renderSiteStatusList(siteStatusLocalMissing, ["점검 전"]);
    renderSiteStatusList(siteStatusExclusionEvidence, ["점검 전"]);
    renderSiteStatusList(siteStatusReviewRequired, ["점검 전"]);
    if (siteStatusPagesBody) {
      siteStatusPagesBody.replaceChildren();
    }
    setText(siteStatusPagesEmpty, "아직 점검 결과가 없습니다.");
    updateSiteStatusControls();
  }

  function updateSiteStatusRepositoryState() {
    const repositoryRoot = connectedRepository && connectedRepository.rootPath;
    setText(siteStatusRepositoryPath, repositoryRoot || "연결되지 않음");
    if (siteStatusRepositoryEmpty) {
      siteStatusRepositoryEmpty.hidden = Boolean(repositoryRoot);
    }
    updateSiteStatusControls();
  }

  function updateSiteStatusControls() {
    if (!runSiteStatusAuditButton) {
      return;
    }
    runSiteStatusAuditButton.disabled =
      siteStatusLoading || repositorySelectionLoading || !connectedRepository || !connectedRepository.rootPath;
    runSiteStatusAuditButton.textContent = siteStatusLoading ? "점검 중" : "사이트 상태 점검";
  }

  function setSiteStatusLoading(isLoading) {
    siteStatusLoading = isLoading;
    updateSiteStatusControls();
  }

  function renderSiteStatusReport(report) {
    const summary = report.summary;
    const global = report.global;
    const pages = Array.isArray(report.pages) ? report.pages : [];
    const reviewRequiredPages = pages.filter((page) => page.pageClassification === "review_required");
    const checkedHtmlCount = safeCount(summary.checkedHtmlCount);
    const okCount = safeCount(summary.okCount);
    const warningCount = safeCount(summary.warningCount);
    const errorCount = safeCount(summary.errorCount);
    const sitemapUrlCount = safeCount(global.sitemapUrlCount);
    const statusKind = errorCount > 0 ? "error" : warningCount > 0 ? "warning" : "ok";
    const statusText = statusKind === "error" ? "오류" : statusKind === "warning" ? "경고" : "정상";

    setSiteStatusBadge(statusText, statusKind);
    setSiteStatusMessage(
      `점검 완료: HTML ${checkedHtmlCount.toLocaleString("ko-KR")}개, 경고 ${warningCount.toLocaleString("ko-KR")}개, 오류 ${errorCount.toLocaleString("ko-KR")}개`,
      statusKind === "ok" ? "success" : statusKind,
    );
    setText(siteStatusLastChecked, new Date().toLocaleString("ko-KR"));
    setText(siteStatusHtmlCount, checkedHtmlCount.toLocaleString("ko-KR"));
    setText(siteStatusOkCount, okCount.toLocaleString("ko-KR"));
    setText(siteStatusWarningCount, warningCount.toLocaleString("ko-KR"));
    setText(siteStatusErrorCount, errorCount.toLocaleString("ko-KR"));
    setText(siteStatusSitemapExists, formatExists(summary.sitemapExists));
    setText(siteStatusRobotsExists, formatExists(summary.robotsTxtExists));
    setText(siteStatusVercelJsonExists, formatExists(summary.vercelJsonExists));
    setText(siteStatusVercelignoreExists, formatExists(summary.vercelignoreExists));
    setText(siteStatusSitemapUrlCount, sitemapUrlCount.toLocaleString("ko-KR"));
    setText(siteStatusSitemapDuplicates, formatDuplicateUrls(global));
    setText(siteStatusRobotsSitemap, formatBoolean(global.robotsHasSitemap));
    renderSiteStatusList(siteStatusPublicMissing, normalArray(global.publicHtmlNotInSitemap), "없음");
    renderSiteStatusList(siteStatusLocalMissing, normalArray(global.sitemapUrlsWithoutLocalFile), "없음");
    renderSiteStatusList(siteStatusExclusionEvidence, formatDeploymentEvidence(global.deploymentExclusionEvidence), "없음");
    renderSiteStatusList(siteStatusReviewRequired, formatReviewRequiredPages(reviewRequiredPages), "없음");
    renderSiteStatusPages(pages);
  }

  function renderSiteStatusError(message, errorCode = null) {
    const detail = errorCode ? ` (${errorCode})` : "";
    setSiteStatusBadge("오류", "error");
    setSiteStatusMessage(`${message}${detail}`, "error");
  }

  function renderSiteStatusPages(pages) {
    if (!siteStatusPagesBody) {
      return;
    }

    siteStatusPagesBody.replaceChildren();
    setText(
      siteStatusPagesEmpty,
      pages.length === 0 ? "페이지 결과가 없습니다." : `${pages.length.toLocaleString("ko-KR")}개 페이지 결과`,
    );

    pages.forEach((page) => {
      const row = document.createElement("tr");
      row.append(
        createTableCell(page.relativePath || "경로 없음"),
        createTableCell(formatClassification(page.pageClassification)),
        createTableCell(formatPresence(page.titleExists)),
        createTableCell(formatPresence(page.metaDescriptionExists)),
        createTableCell(formatPresence(page.canonicalExists)),
        createTableCell(Number.isSafeInteger(page.h1Count) ? page.h1Count.toLocaleString("ko-KR") : "확인 필요"),
        createTableCell(formatBoolean(page.robotsNoindex)),
        createTableCell(formatBoolean(page.sitemapIncluded)),
        createIssuesCell(page.issues),
      );
      siteStatusPagesBody.append(row);
    });
  }

  function renderSiteStatusList(listElement, items, emptyText = "없음") {
    if (!listElement) {
      return;
    }

    listElement.replaceChildren();
    const values = normalArray(items);
    if (values.length === 0) {
      const emptyItem = document.createElement("li");
      emptyItem.textContent = emptyText;
      listElement.append(emptyItem);
      return;
    }

    values.forEach((value) => {
      const item = document.createElement("li");
      item.textContent = String(value);
      listElement.append(item);
    });
  }

  function formatDeploymentEvidence(evidenceItems) {
    return normalArray(evidenceItems).map((item) => {
      const evidence = normalArray(item.evidence).join(", ");
      return `${item.relativePath || "경로 없음"} · ${formatClassification(item.pageClassification)} · ${evidence || "근거 없음"}`;
    });
  }

  function formatReviewRequiredPages(pages) {
    return pages.map((page) => {
      const issues = normalArray(page.issues).join(", ");
      return `${page.relativePath || "경로 없음"} · ${issues || "검토 필요"}`;
    });
  }

  function createTableCell(value) {
    const cell = document.createElement("td");
    cell.textContent = value;
    return cell;
  }

  function createIssuesCell(issues) {
    const cell = document.createElement("td");
    const values = normalArray(issues);

    if (values.length === 0) {
      cell.textContent = "0개";
      return cell;
    }

    const details = document.createElement("details");
    const summary = document.createElement("summary");
    const list = document.createElement("ul");
    summary.textContent = `${values.length.toLocaleString("ko-KR")}개`;
    values.forEach((issue) => {
      const item = document.createElement("li");
      item.textContent = issue;
      list.append(item);
    });
    details.append(summary, list);
    cell.append(details);
    return cell;
  }

  function isValidSiteStatusReport(result) {
    return (
      result !== null &&
      typeof result === "object" &&
      (result.status === "ok" || result.status === "error") &&
      result.summary !== null &&
      typeof result.summary === "object" &&
      result.global !== null &&
      typeof result.global === "object" &&
      Array.isArray(result.pages)
    );
  }

  function setSiteStatusBadge(text, kind) {
    if (!siteStatusBadge) {
      return;
    }
    siteStatusBadge.textContent = text;
    siteStatusBadge.classList.remove("is-idle", "is-loading", "is-ok", "is-warning", "is-error");
    siteStatusBadge.classList.add(`is-${kind}`);
  }

  function setSiteStatusMessage(message, kind = "default") {
    if (!siteStatusMessage) {
      return;
    }
    siteStatusMessage.textContent = message;
    siteStatusMessage.classList.toggle("is-error", kind === "error");
    siteStatusMessage.classList.toggle("is-warning", kind === "warning");
    siteStatusMessage.classList.toggle("is-success", kind === "success");
  }

  function formatExists(value) {
    return value === true ? "있음" : "없음";
  }

  function formatPresence(value) {
    return value === true ? "있음" : "누락";
  }

  function formatBoolean(value) {
    return value === true ? "예" : "아니오";
  }

  function formatDuplicateUrls(global) {
    const duplicates = normalArray(global.duplicateSitemapUrls);
    return global.sitemapHasDuplicateUrls ? `있음 (${duplicates.length.toLocaleString("ko-KR")}개)` : "없음";
  }

  function formatClassification(value) {
    const labels = {
      public: "public",
      excluded: "excluded",
      review_required: "review_required",
    };
    return labels[value] || "review_required";
  }

  function normalArray(value) {
    return Array.isArray(value) ? value : [];
  }

  function safeCount(value) {
    return Number.isSafeInteger(value) && value >= 0 ? value : 0;
  }

  function setText(element, value) {
    if (element) {
      element.textContent = value;
    }
  }

  function selectedAnalyticsPeriod() {
    const selected = document.querySelector('input[name="analytics-period"]:checked');
    return selected && allowedAnalyticsPeriods.has(selected.value) ? selected.value : analyticsSummaryState.period;
  }

  function resetAnalyticsSummary(period) {
    analyticsSummaryState = {
      period: allowedAnalyticsPeriods.has(period) ? period : "7d",
      status: "idle",
      result: null,
      errorCode: null,
      message: null,
      retryAt: null,
    };
    clearAnalyticsSummaryValues();
    setAnalyticsSummaryMessage("선택한 기간을 새로고침하여 확인하십시오.");
  }

  function clearAnalyticsSummaryValues() {
    analyticsVisitors.textContent = "—";
    analyticsPageviews.textContent = "—";
    analyticsRange.textContent = "확인 전";
    analyticsFetchedAt.textContent = "없음";
    analyticsCacheStatus.textContent = "확인 전";
  }

  function isValidAnalyticsSummary(result, requestedPeriod) {
    return (
      result !== null &&
      typeof result === "object" &&
      result.status === "ok" &&
      result.period === requestedPeriod &&
      allowedAnalyticsPeriods.has(result.period) &&
      typeof result.rangeStart === "string" &&
      result.rangeStart.length > 0 &&
      typeof result.rangeEnd === "string" &&
      result.rangeEnd.length > 0 &&
      typeof result.fetchedAt === "string" &&
      result.fetchedAt.length > 0 &&
      Number.isSafeInteger(result.pageviews) &&
      result.pageviews >= 0 &&
      Number.isSafeInteger(result.visitors) &&
      result.visitors >= 0 &&
      typeof result.cached === "boolean"
    );
  }

  function renderAnalyticsSummarySuccess(result) {
    analyticsSummaryState = {
      period: result.period,
      status: "success",
      result: {
        period: result.period,
        rangeStart: result.rangeStart,
        rangeEnd: result.rangeEnd,
        fetchedAt: result.fetchedAt,
        pageviews: result.pageviews,
        visitors: result.visitors,
        cached: result.cached,
      },
      errorCode: null,
      message: null,
      retryAt: null,
    };

    analyticsVisitors.textContent = `${result.visitors.toLocaleString("ko-KR")}명`;
    analyticsPageviews.textContent = `${result.pageviews.toLocaleString("ko-KR")}회`;
    analyticsRange.textContent = `${formatAnalyticsTimestamp(result.rangeStart)} ~ ${formatAnalyticsTimestamp(result.rangeEnd)}`;
    analyticsFetchedAt.textContent = formatAnalyticsTimestamp(result.fetchedAt);
    analyticsCacheStatus.textContent = result.cached ? "캐시 사용" : "새로 조회";
    setAnalyticsSummaryMessage(
      result.cached ? "저장된 5분 캐시 결과를 표시합니다." : "Analytics 요약을 새로 조회했습니다.",
      "success",
    );
    dashboardAnalyticsStatus.textContent = "연결됨";
  }

  function renderAnalyticsSummaryError(errorCode, retryAt) {
    const message = analyticsErrorMessages[errorCode] || "Analytics 데이터를 조회할 수 없습니다.";
    const retryMessage =
      errorCode === "rate_limited" && retryAt ? ` ${formatAnalyticsTimestamp(retryAt)} 이후 다시 시도해 주십시오.` : "";

    analyticsSummaryState = {
      period: selectedAnalyticsPeriod(),
      status: "error",
      result: null,
      errorCode,
      message,
      retryAt,
    };
    clearAnalyticsSummaryValues();
    setAnalyticsSummaryMessage(`${message}${retryMessage}`, "error");

    if (errorCode === "not_configured") {
      dashboardAnalyticsStatus.textContent = "연결되지 않음";
    } else if (
      [
        "unauthorized",
        "forbidden",
        "plan_or_billing_required",
        "rate_limited",
        "timeout",
        "network_error",
        "service_unavailable",
      ].includes(errorCode)
    ) {
      dashboardAnalyticsStatus.textContent = "연결 오류";
    }
  }

  function formatAnalyticsTimestamp(value) {
    const date = new Date(value);
    return Number.isNaN(date.getTime()) ? "확인할 수 없음" : `${analyticsDateFormatter.format(date)} KST`;
  }

  function setAnalyticsSummaryMessage(message, kind = "default") {
    analyticsSummaryMessage.textContent = message;
    analyticsSummaryMessage.classList.toggle("is-error", kind === "error");
    analyticsSummaryMessage.classList.toggle("is-success", kind === "success");
  }

  function setAnalyticsSummaryLoading(isLoading) {
    analyticsSummaryLoading = isLoading;
    analyticsSummary.setAttribute("aria-busy", String(isLoading));
    if (isLoading) {
      analyticsSummaryState.status = "loading";
    }
    updateAnalyticsControls();
  }

  async function refreshVercelConnectionStatus() {
    setAnalyticsLoading(true, "저장된 자격 증명을 확인하고 있습니다.");
    try {
      const result = await window.__TAURI__.core.invoke("get_vercel_connection_status");
      renderAnalyticsStatus(result);
      if (result.status === "credential_stored") {
        setAnalyticsMessage("자격증명이 저장되어 있습니다. 연결 시험을 실행해 주십시오.");
      } else if (result.status === "not_configured") {
        setAnalyticsMessage("Vercel Access Token을 저장한 후 연결 상태를 확인할 수 있습니다.");
      } else {
        setAnalyticsMessage(result.message || "연결 상태를 확인할 수 없습니다.", "error");
      }
    } catch (error) {
      renderAnalyticsError("관리자 앱과 통신할 수 없습니다.");
    } finally {
      setAnalyticsLoading(false);
    }
  }

  function setAnalyticsLoading(isLoading, message = "") {
    analyticsConnectionLoading = isLoading;
    updateAnalyticsControls();
    if (isLoading) {
      setAnalyticsMessage(message);
    }
  }

  function updateAnalyticsControls() {
    const analyticsBusy = analyticsConnectionLoading || analyticsSummaryLoading;
    saveVercelTokenButton.disabled = analyticsBusy;
    vercelAccessToken.disabled = analyticsBusy;
    testVercelConnectionButton.disabled = analyticsBusy || !vercelCredentialStored;
    deleteVercelTokenButton.disabled = analyticsBusy || !vercelCredentialStored;
    refreshAnalyticsSummaryButton.disabled =
      analyticsConnectionLoading || analyticsSummaryLoading || !vercelCredentialStored;
    analyticsPeriodInputs.forEach((input) => {
      input.disabled = analyticsConnectionLoading || analyticsSummaryLoading;
    });
  }

  function renderAnalyticsStatus(result) {
    vercelCredentialStored = result.tokenStored === true;
    analyticsCredentialStatus.textContent = vercelCredentialStored ? "토큰 저장됨" : "저장되지 않음";
    analyticsLastChecked.textContent = result.lastCheckedAt
      ? new Date(result.lastCheckedAt).toLocaleString("ko-KR")
      : "없음";

    analyticsConnectionBadge.classList.remove("is-connected", "is-error");
    if (result.status === "connected") {
      analyticsConnectionBadge.textContent = "연결됨";
      analyticsConnectionBadge.classList.add("is-connected");
      analyticsTestStatus.textContent = "연결 성공";
      dashboardAnalyticsStatus.textContent = "연결됨";
    } else if (result.status === "credential_stored") {
      analyticsConnectionBadge.textContent = "자격증명 저장됨";
      analyticsTestStatus.textContent = "확인 전";
      dashboardAnalyticsStatus.textContent = "연결되지 않음";
    } else if (result.status === "not_configured") {
      analyticsConnectionBadge.textContent = "연결되지 않음";
      analyticsTestStatus.textContent = "확인 전";
      dashboardAnalyticsStatus.textContent = "연결되지 않음";
    } else {
      analyticsConnectionBadge.textContent = "오류";
      analyticsConnectionBadge.classList.add("is-error");
      analyticsTestStatus.textContent = result.status === "rate_limited" ? "요청 제한" : "연결 실패";
      dashboardAnalyticsStatus.textContent = "연결 오류";
    }

    setAnalyticsLoading(false);
  }

  function renderAnalyticsError(message) {
    renderAnalyticsStatus({ status: "error", tokenStored: vercelCredentialStored, lastCheckedAt: null });
    setAnalyticsMessage(message, "error");
  }

  function setAnalyticsMessage(message, kind = "default") {
    analyticsMessage.textContent = message;
    analyticsMessage.classList.toggle("is-error", kind === "error");
    analyticsMessage.classList.toggle("is-success", kind === "success");
  }

  function renderWarnings() {
    const messages = [...connectedRepository.warnings];
    if (connectedRepository.truncated) {
      messages.unshift("탐색 제한에 도달하여 일부 항목만 표시합니다.");
    } else if (connectedRepository.partial && messages.length === 0) {
      messages.push("읽을 수 없는 일부 항목을 제외하고 표시합니다.");
    }

    repositoryWarning.replaceChildren();
    if (messages.length === 0) {
      repositoryWarning.hidden = true;
      return;
    }

    const heading = document.createElement("strong");
    heading.textContent = "일부 항목 안내";
    const list = document.createElement("ul");
    messages.forEach((message) => {
      const item = document.createElement("li");
      item.textContent = message;
      list.append(item);
    });
    repositoryWarning.append(heading, list);
    repositoryWarning.hidden = false;
  }

  function renderTree() {
    const rootList = document.createElement("ul");
    rootList.className = "tree-list tree-root-list";
    rootList.setAttribute("role", "group");

    const rootItem = document.createElement("li");
    rootItem.className = "tree-item tree-root-item";
    rootItem.setAttribute("role", "treeitem");
    rootItem.setAttribute("aria-expanded", "true");

    const rootLabel = document.createElement("div");
    rootLabel.className = "tree-row tree-root-row";
    const rootTwisty = document.createElement("span");
    rootTwisty.className = "tree-twisty";
    rootTwisty.textContent = "▾";
    const rootName = document.createElement("strong");
    rootName.textContent = connectedRepository.repositoryName;
    rootLabel.append(rootTwisty, rootName);

    const children = createTreeList(connectedRepository.tree);
    rootItem.append(rootLabel, children);
    rootList.append(rootItem);
    repositoryTree.replaceChildren(rootList);
  }

  function createTreeList(nodes) {
    const list = document.createElement("ul");
    list.className = "tree-list";
    list.setAttribute("role", "group");

    nodes.forEach((node) => {
      const item = document.createElement("li");
      item.className = "tree-item";
      item.setAttribute("role", "treeitem");

      if (node.kind === "directory") {
        const button = document.createElement("button");
        button.className = "tree-row tree-directory";
        button.type = "button";
        button.setAttribute("aria-expanded", "false");

        const twisty = document.createElement("span");
        twisty.className = "tree-twisty";
        twisty.textContent = "▸";
        const icon = document.createElement("span");
        icon.className = "tree-icon";
        icon.setAttribute("aria-hidden", "true");
        icon.textContent = "📁";
        const name = document.createElement("span");
        name.textContent = node.name;
        button.append(twisty, icon, name);

        const children = createTreeList(node.children);
        children.hidden = true;
        if (node.children.length === 0) {
          button.classList.add("is-empty");
        }

        button.addEventListener("click", () => {
          const isExpanded = button.getAttribute("aria-expanded") === "true";
          button.setAttribute("aria-expanded", String(!isExpanded));
          twisty.textContent = isExpanded ? "▸" : "▾";
          children.hidden = isExpanded;
        });

        item.append(button, children);
      } else {
        const row = document.createElement("div");
        row.className = "tree-row tree-file";
        const spacer = document.createElement("span");
        spacer.className = "tree-twisty";
        const icon = document.createElement("span");
        icon.className = "tree-icon";
        icon.setAttribute("aria-hidden", "true");
        icon.textContent = "📄";
        const name = document.createElement("span");
        name.textContent = node.name;
        row.append(spacer, icon, name);
        item.append(row);
      }

      list.append(item);
    });

    return list;
  }
});

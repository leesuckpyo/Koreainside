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
  const exportFormatInputs = document.querySelectorAll('input[name="export-format"]');
  const previewExportButton = document.querySelector("#preview-export");
  const saveExportButton = document.querySelector("#save-export");
  const exportStatus = document.querySelector("#export-status");
  const exportPreview = document.querySelector("#export-preview");
  const exportHistory = document.querySelector("#export-history");
  const exportHistoryEmpty = document.querySelector("#export-history-empty");
  const viewTitles = {
    dashboard: "운영 대시보드",
    analytics: "Vercel Analytics",
    explorer: "프로젝트 탐색기",
  };
  let connectedRepository = null;
  let vercelCredentialStored = false;

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

  vercelTokenForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    let token = vercelAccessToken.value;
    vercelAccessToken.value = "";
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

  function setLoading(isLoading) {
    selectRepositoryButton.disabled = isLoading;
    disconnectRepositoryButton.disabled = isLoading;
    repositoryLoading.hidden = !isLoading;
    repositoryConnected.setAttribute("aria-busy", String(isLoading));
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
    saveVercelTokenButton.disabled = isLoading;
    vercelAccessToken.disabled = isLoading;
    testVercelConnectionButton.disabled = isLoading || !vercelCredentialStored;
    deleteVercelTokenButton.disabled = isLoading || !vercelCredentialStored;
    if (isLoading) {
      setAnalyticsMessage(message);
    }
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

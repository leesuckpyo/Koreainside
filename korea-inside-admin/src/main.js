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
  const viewTitles = {
    dashboard: "운영 대시보드",
    explorer: "프로젝트 탐색기",
  };
  let connectedRepository = null;

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

  disconnectRepositoryButton.addEventListener("click", () => {
    connectedRepository = null;
    clearError();
    renderDisconnectedRepository();
  });

  initializationState.textContent = "관리자 앱 기반 구성이 완료되었습니다.";

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

window.addEventListener("DOMContentLoaded", () => {
  const viewTitle = document.querySelector("#view-title");
  const initializationState = document.querySelector("#initialization-state");
  const navigationItems = document.querySelectorAll(".nav-item[data-view]");
  const viewPanels = document.querySelectorAll("[data-panel]");
  const viewTitles = {
    dashboard: "운영 대시보드",
    explorer: "프로젝트 탐색기",
  };

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

  initializationState.textContent = "관리자 앱 기반 구성이 완료되었습니다.";
});

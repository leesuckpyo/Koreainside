window.addEventListener("DOMContentLoaded", () => {
  const viewTitle = document.querySelector("#view-title");
  const initializationState = document.querySelector("#initialization-state");
  const navigationItems = document.querySelectorAll(".nav-item[data-view]");
  const viewPanels = document.querySelectorAll("[data-panel]");
  const viewTitles = {
    dashboard: "운영 대시보드",
    explorer: "Project Explorer",
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

  initializationState.textContent = "Foundation initialized";
});

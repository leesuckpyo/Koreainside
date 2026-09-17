(() => {
  const currentYear = new Intl.DateTimeFormat("en", {
    timeZone: "Asia/Seoul",
    year: "numeric"
  }).format(new Date());

  document
    .querySelectorAll('[data-guide-year="current"]')
    .forEach((element) => {
      element.textContent = currentYear;
    });
})();

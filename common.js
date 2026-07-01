(function () {
  const header = document.querySelector('[data-common-header]');
  if (!header) return;

  const toggle = header.querySelector('.nav-toggle');
  const nav = header.querySelector('.nav');
  const navLinks = header.querySelectorAll('.nav__link');

  const setMenuOpen = (isOpen) => {
    if (!toggle || !nav) return;

    toggle.setAttribute('aria-expanded', String(isOpen));
    toggle.setAttribute('aria-label', isOpen ? 'Close menu' : 'Open menu');
    nav.classList.toggle('is-open', isOpen);
    header.classList.toggle('header--menu-open', isOpen);
  };

  if (toggle && nav) {
    toggle.addEventListener('click', () => {
      const isOpen = toggle.getAttribute('aria-expanded') === 'true';
      setMenuOpen(!isOpen);
    });

    navLinks.forEach((link) => {
      link.addEventListener('click', () => setMenuOpen(false));
    });

    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        setMenuOpen(false);
      }
    });
  }

  const updateHeaderState = () => {
    header.classList.toggle('header--scrolled', window.scrollY > 40);
  };

  window.addEventListener('scroll', updateHeaderState, { passive: true });
  updateHeaderState();
}());

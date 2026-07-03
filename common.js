(function () {
  const headers = document.querySelectorAll('[data-common-header]');
  if (!headers.length) return;

  const desktopQuery = window.matchMedia('(min-width: 960px)');

  const updateBodyLock = () => {
    const hasOpenMenu = Array.from(headers).some((header) => header.classList.contains('header--menu-open'));
    document.body.classList.toggle('body--nav-open', hasOpenMenu);
  };

  headers.forEach((header) => {
    const toggle = header.querySelector('.nav-toggle');
    const nav = header.querySelector('.nav');
    const navLinks = header.querySelectorAll('.nav__link');

    if (!toggle || !nav) return;

    const setMenuOpen = (isOpen) => {
      toggle.setAttribute('aria-expanded', String(isOpen));
      toggle.setAttribute('aria-label', isOpen ? 'Close menu' : 'Open menu');
      nav.classList.toggle('is-open', isOpen);
      header.classList.toggle('header--menu-open', isOpen);
      updateBodyLock();
    };

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

    const closeMenuOnDesktop = () => {
      if (desktopQuery.matches) {
        setMenuOpen(false);
      }
    };

    if (desktopQuery.addEventListener) {
      desktopQuery.addEventListener('change', closeMenuOnDesktop);
    } else {
      desktopQuery.addListener(closeMenuOnDesktop);
    }
  });

  const updateHeaderState = () => {
    headers.forEach((header) => {
      header.classList.toggle('header--scrolled', window.scrollY > 40);
    });
  };

  window.addEventListener('scroll', updateHeaderState, { passive: true });
  updateHeaderState();
}());

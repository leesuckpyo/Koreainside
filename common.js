(function () {
  const initializeCommonNavigation = () => {
  const headers = document.querySelectorAll('[data-common-header]');
  if (!headers.length) return;

  if (document.documentElement.dataset.commonNavigationInitialized === 'true') return;
  document.documentElement.dataset.commonNavigationInitialized = 'true';

  const desktopQuery = window.matchMedia('(min-width: 1100px)');

  const updateBodyLock = () => {
    const hasOpenMenu = Array.from(headers).some((header) => header.classList.contains('site-header--menu-open'));
    document.body.classList.toggle('body--nav-open', hasOpenMenu);
  };

  headers.forEach((header) => {
    const toggle = header.querySelector('.site-nav-toggle');
    const nav = header.querySelector('.site-nav');
    const items = Array.from(header.querySelectorAll('.site-nav__item'));
    const triggers = items.map((item) => item.querySelector('.site-nav__trigger')).filter(Boolean);
    const navLinks = Array.from(header.querySelectorAll('.site-nav__link'));

    if (!toggle || !nav) return;

    const closeSubmenus = (focusTrigger) => {
      items.forEach((item) => {
        const trigger = item.querySelector('.site-nav__trigger');
        const panel = item.querySelector('.site-nav__panel');
        if (!trigger || !panel) return;
        item.classList.remove('is-open');
        trigger.setAttribute('aria-expanded', 'false');
        panel.hidden = true;
      });
      if (focusTrigger) focusTrigger.focus();
    };

    const openSubmenu = (item, focusTarget) => {
      const trigger = item.querySelector('.site-nav__trigger');
      const panel = item.querySelector('.site-nav__panel');
      if (!trigger || !panel) return;
      closeSubmenus(false);
      item.classList.add('is-open');
      trigger.setAttribute('aria-expanded', 'true');
      panel.hidden = false;
      const links = Array.from(panel.querySelectorAll('.site-nav__link'));
      if (focusTarget === 'first' && links.length) links[0].focus();
      if (focusTarget === 'last' && links.length) links[links.length - 1].focus();
    };

    const setMenuOpen = (isOpen, returnFocus) => {
      toggle.setAttribute('aria-expanded', String(isOpen));
      toggle.setAttribute('aria-label', isOpen ? 'Close menu' : 'Open menu');
      nav.classList.toggle('is-open', isOpen);
      header.classList.toggle('site-header--menu-open', isOpen);
      if (!isOpen) {
        closeSubmenus(false);
        if (returnFocus) toggle.focus();
      }
      updateBodyLock();
    };

    toggle.addEventListener('click', () => {
      const isOpen = toggle.getAttribute('aria-expanded') === 'true';
      setMenuOpen(!isOpen);
    });

    items.forEach((item, index) => {
      const trigger = item.querySelector('.site-nav__trigger');
      const panel = item.querySelector('.site-nav__panel');
      if (!trigger || !panel) return;

      panel.hidden = true;

      trigger.addEventListener('click', () => {
        const isOpen = trigger.getAttribute('aria-expanded') === 'true';
        if (isOpen) closeSubmenus(false);
        else openSubmenu(item, false);
      });

      trigger.addEventListener('keydown', (event) => {
        if (event.key === 'ArrowRight' || event.key === 'ArrowLeft') {
          event.preventDefault();
          const direction = event.key === 'ArrowRight' ? 1 : -1;
          triggers[(index + direction + triggers.length) % triggers.length].focus();
        } else if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
          event.preventDefault();
          openSubmenu(item, event.key === 'ArrowDown' ? 'first' : 'last');
        }
      });

      panel.addEventListener('keydown', (event) => {
        const links = Array.from(panel.querySelectorAll('.site-nav__link'));
        const currentIndex = links.indexOf(document.activeElement);
        if (currentIndex < 0) return;
        let nextIndex = null;
        if (event.key === 'ArrowDown') nextIndex = (currentIndex + 1) % links.length;
        if (event.key === 'ArrowUp') nextIndex = (currentIndex - 1 + links.length) % links.length;
        if (event.key === 'Home') nextIndex = 0;
        if (event.key === 'End') nextIndex = links.length - 1;
        if (nextIndex !== null) {
          event.preventDefault();
          links[nextIndex].focus();
        }
      });
    });

    navLinks.forEach((link) => {
      link.addEventListener('click', () => {
        if (!desktopQuery.matches) setMenuOpen(false);
        else closeSubmenus(false);
      });
    });

    document.addEventListener('click', (event) => {
      if (header.contains(event.target)) return;
      if (!desktopQuery.matches && nav.classList.contains('is-open')) setMenuOpen(false);
      else closeSubmenus(false);
    });

    document.addEventListener('keydown', (event) => {
      if (event.key !== 'Escape') return;
      const openItem = items.find((item) => item.classList.contains('is-open'));
      if (openItem) {
        event.preventDefault();
        closeSubmenus(openItem.querySelector('.site-nav__trigger'));
      } else if (!desktopQuery.matches && nav.classList.contains('is-open')) {
        event.preventDefault();
        setMenuOpen(false, true);
      }
    });

    const resetNavigationState = () => setMenuOpen(false);

    resetNavigationState();
    window.addEventListener('pageshow', resetNavigationState);

    if (desktopQuery.addEventListener) {
      desktopQuery.addEventListener('change', resetNavigationState);
    } else {
      desktopQuery.addListener(resetNavigationState);
    }
  });

  const updateHeaderState = () => {
    headers.forEach((header) => {
      header.classList.toggle('header--scrolled', window.scrollY > 40);
    });
  };

  window.addEventListener('scroll', updateHeaderState, { passive: true });
  updateHeaderState();
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeCommonNavigation, { once: true });
  } else {
    initializeCommonNavigation();
  }
}());

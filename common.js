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

  const initializeLanguageSwitchers = () => {
    const language = document.documentElement.lang.toLowerCase().split('-')[0];
    if (language !== 'en' && language !== 'es') return;

    const languages = {
      en: { label: 'English' },
      es: { label: 'Español' }
    };
    const alternates = Array.from(document.querySelectorAll('link[rel~="alternate"][hreflang]')).reduce((items, link) => {
      const alternateLanguage = link.getAttribute('hreflang').toLowerCase().split('-')[0];
      if (languages[alternateLanguage] && alternateLanguage !== language && link.href) {
        items.set(alternateLanguage, link.href);
      }
      return items;
    }, new Map());

    document.querySelectorAll('.language-switcher').forEach((switcher, switcherIndex) => {
      const button = switcher.querySelector('.language-switcher__button');
      if (!button) return;

      const options = Object.keys(languages).filter((optionLanguage) => (
        optionLanguage === language || alternates.has(optionLanguage)
      ));
      if (options.length < 2) return;

      const menuId = `language-switcher-menu-${switcherIndex + 1}`;
      const menu = document.createElement('div');
      menu.className = 'language-switcher__menu';
      menu.id = menuId;
      menu.setAttribute('role', 'menu');
      menu.setAttribute('aria-label', switcher.getAttribute('aria-label') || 'Language selector');
      menu.hidden = true;

      options.forEach((optionLanguage) => {
        const isCurrent = optionLanguage === language;
        const option = document.createElement(isCurrent ? 'button' : 'a');
        option.className = `language-switcher__option${isCurrent ? ' language-switcher__option--current' : ''}`;
        option.setAttribute('role', 'menuitem');

        if (isCurrent) {
          option.type = 'button';
          option.setAttribute('aria-current', 'page');
        } else {
          option.href = alternates.get(optionLanguage);
          option.hreflang = optionLanguage;
        }

        const label = document.createElement('span');
        label.textContent = languages[optionLanguage].label;
        option.appendChild(label);

        const indicator = document.createElement('span');
        indicator.className = 'language-switcher__option-indicator';
        indicator.setAttribute('aria-hidden', 'true');
        indicator.textContent = isCurrent ? '✓' : '→';
        option.appendChild(indicator);

        menu.appendChild(option);
      });

      switcher.appendChild(menu);

      const menuItems = Array.from(menu.querySelectorAll('[role="menuitem"]'));
      const closeMenu = (returnFocus) => {
        menu.hidden = true;
        button.setAttribute('aria-expanded', 'false');
        switcher.classList.remove('is-open');
        if (returnFocus) button.focus();
      };
      const openMenu = (focusPosition) => {
        menu.hidden = false;
        button.setAttribute('aria-expanded', 'true');
        switcher.classList.add('is-open');
        if (focusPosition === 'first') menuItems[0].focus();
        if (focusPosition === 'last') menuItems[menuItems.length - 1].focus();
      };

      button.disabled = false;
      button.removeAttribute('aria-disabled');
      button.setAttribute('aria-haspopup', 'menu');
      button.setAttribute('aria-expanded', 'false');
      button.setAttribute('aria-controls', menuId);
      button.setAttribute('aria-label', switcher.getAttribute('aria-label') || 'Language selector');
      button.addEventListener('click', () => {
        if (menu.hidden) openMenu(false);
        else closeMenu(false);
      });
      button.addEventListener('keydown', (event) => {
        if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp') return;
        event.preventDefault();
        openMenu(event.key === 'ArrowDown' ? 'first' : 'last');
      });

      menu.addEventListener('click', (event) => {
        const selectedOption = event.target.closest('.language-switcher__option');
        if (!selectedOption || !menu.contains(selectedOption)) return;

        if (selectedOption.classList.contains('language-switcher__option--current')) {
          closeMenu(true);
          return;
        }

        const isPrimaryUnmodifiedClick = event.button === 0
          && !event.altKey
          && !event.ctrlKey
          && !event.metaKey
          && !event.shiftKey;
        if (selectedOption.matches('a[href]') && isPrimaryUnmodifiedClick) {
          event.preventDefault();
          window.location.assign(selectedOption.href);
        }
      });
      menu.addEventListener('keydown', (event) => {
        const currentIndex = menuItems.indexOf(document.activeElement);
        let nextIndex = null;

        if (event.key === 'Escape') {
          event.preventDefault();
          closeMenu(true);
          return;
        }
        if (event.key === 'Tab') {
          closeMenu(false);
          return;
        }
        if (event.key === 'ArrowDown') nextIndex = currentIndex < 0 ? 0 : (currentIndex + 1) % menuItems.length;
        if (event.key === 'ArrowUp') nextIndex = currentIndex < 0 ? menuItems.length - 1 : (currentIndex - 1 + menuItems.length) % menuItems.length;
        if (event.key === 'Home') nextIndex = 0;
        if (event.key === 'End') nextIndex = menuItems.length - 1;

        if (nextIndex !== null) {
          event.preventDefault();
          menuItems[nextIndex].focus();
        }
      });

      document.addEventListener('click', (event) => {
        if (!switcher.contains(event.target)) closeMenu(false);
      });
      document.addEventListener('keydown', (event) => {
        if (event.key === 'Escape' && !menu.hidden) closeMenu(true);
      });
      window.addEventListener('pageshow', () => closeMenu(false));
    });
  };

  const initializeWebAppInstall = () => {
    const language = document.documentElement.lang.toLowerCase().split('-')[0];
    const isSpanish = language === 'es';
    const manifestHref = isSpanish ? '/es/manifest.webmanifest' : '/manifest.webmanifest';
    const existingManifest = document.querySelector('link[rel~="manifest"]');

    if (!existingManifest) {
      const manifest = document.createElement('link');
      manifest.rel = 'manifest';
      manifest.href = manifestHref;
      document.head.appendChild(manifest);
    }

    if (document.documentElement.dataset.webAppInstallInitialized === 'true') return;
    document.documentElement.dataset.webAppInstallInitialized = 'true';

    const standaloneQuery = window.matchMedia('(display-mode: standalone)');
    const isStandalone = () => standaloneQuery.matches || window.navigator.standalone === true;
    const isIos = /iPad|iPhone|iPod/i.test(window.navigator.userAgent)
      || (window.navigator.platform === 'MacIntel' && window.navigator.maxTouchPoints > 1);
    const nav = document.querySelector('[data-common-header] .site-nav');
    if (!nav || isStandalone()) return;

    const copy = isSpanish ? {
      button: 'Instalar Korea Inside',
      close: 'Cerrar',
      iosTitle: 'Añade Korea Inside a tu pantalla de inicio',
      iosSteps: ['Toca Compartir', 'Toca Añadir a pantalla de inicio'],
      fallback: 'Abre el menú del navegador y elige “Instalar aplicación” o “Añadir a pantalla de inicio”.'
    } : {
      button: 'Install Korea Inside',
      close: 'Close',
      iosTitle: 'Add Korea Inside to your Home Screen',
      iosSteps: ['Tap Share', 'Tap Add to Home Screen'],
      fallback: 'Open your browser menu and choose “Install app” or “Add to Home screen.”'
    };

    const utility = document.createElement('div');
    utility.className = 'site-nav-install';

    const installButton = document.createElement('button');
    installButton.className = 'site-nav-install__button';
    installButton.type = 'button';
    installButton.textContent = copy.button;
    utility.appendChild(installButton);
    nav.appendChild(utility);

    const dialog = document.createElement('div');
    dialog.className = 'web-app-install-dialog';
    dialog.setAttribute('role', 'dialog');
    dialog.setAttribute('aria-modal', 'true');
    dialog.setAttribute('aria-labelledby', 'web-app-install-dialog-title');
    dialog.hidden = true;

    const dialogPanel = document.createElement('div');
    dialogPanel.className = 'web-app-install-dialog__panel';

    const closeButton = document.createElement('button');
    closeButton.className = 'web-app-install-dialog__close';
    closeButton.type = 'button';
    closeButton.setAttribute('aria-label', copy.close);
    closeButton.textContent = '×';

    const dialogTitle = document.createElement('h2');
    dialogTitle.className = 'web-app-install-dialog__title';
    dialogTitle.id = 'web-app-install-dialog-title';

    const dialogContent = document.createElement('div');
    dialogContent.className = 'web-app-install-dialog__content';

    dialogPanel.append(closeButton, dialogTitle, dialogContent);
    dialog.appendChild(dialogPanel);
    document.body.appendChild(dialog);

    let deferredInstallPrompt = null;

    const closeDialog = (returnFocus) => {
      dialog.hidden = true;
      if (returnFocus && document.body.contains(installButton)) installButton.focus();
    };

    const openInstructions = (showIosSteps) => {
      dialogTitle.textContent = showIosSteps ? copy.iosTitle : copy.button;
      dialogContent.replaceChildren();

      if (showIosSteps) {
        const steps = document.createElement('ol');
        steps.className = 'web-app-install-dialog__steps';
        copy.iosSteps.forEach((step) => {
          const item = document.createElement('li');
          item.textContent = step;
          steps.appendChild(item);
        });
        dialogContent.appendChild(steps);
      } else {
        const message = document.createElement('p');
        message.textContent = copy.fallback;
        dialogContent.appendChild(message);
      }

      dialog.hidden = false;
      closeButton.focus();
    };

    const hideInstallUtility = () => {
      deferredInstallPrompt = null;
      closeDialog(false);
      utility.remove();
    };

    window.addEventListener('beforeinstallprompt', (event) => {
      event.preventDefault();
      deferredInstallPrompt = event;
    });

    window.addEventListener('appinstalled', hideInstallUtility);

    const handleDisplayModeChange = () => {
      if (isStandalone()) hideInstallUtility();
    };
    if (standaloneQuery.addEventListener) {
      standaloneQuery.addEventListener('change', handleDisplayModeChange);
    } else {
      standaloneQuery.addListener(handleDisplayModeChange);
    }

    installButton.addEventListener('click', async () => {
      if (isIos) {
        openInstructions(true);
        return;
      }

      if (!deferredInstallPrompt) {
        openInstructions(false);
        return;
      }

      const promptEvent = deferredInstallPrompt;
      deferredInstallPrompt = null;
      installButton.disabled = true;
      try {
        await promptEvent.prompt();
        const choice = await promptEvent.userChoice;
        if (choice.outcome !== 'accepted') installButton.disabled = false;
      } catch (error) {
        installButton.disabled = false;
        openInstructions(false);
      }
    });

    closeButton.addEventListener('click', () => closeDialog(true));
    dialog.addEventListener('click', (event) => {
      if (event.target === dialog) closeDialog(true);
    });
    dialog.addEventListener('keydown', (event) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        closeDialog(true);
      }
    });
  };

  const initializeCommonHeader = () => {
    initializeCommonNavigation();
    initializeLanguageSwitchers();
    initializeWebAppInstall();
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeCommonHeader, { once: true });
  } else {
    initializeCommonHeader();
  }
}());

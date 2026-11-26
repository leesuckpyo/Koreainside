(() => {
  const toc = document.querySelector('.departure-toc');
  if (!toc) return;

  const panel = toc.querySelector('.departure-toc__panel');
  const list = toc.querySelector('.departure-toc__list');
  const toggle = toc.querySelector('.departure-toc__toggle');
  const close = toc.querySelector('.departure-toc__close');
  const header = document.querySelector('.site-header');
  const footer = document.querySelector('.footer');
  const calculator = document.getElementById('departure-plan-form');
  const calculatorContent = calculator.querySelectorAll(
    'label, input, select, button, small, p, dt, dd, .departure-plan__error'
  );
  const content = document.querySelectorAll(
    '.departure-hero > .container, .departure-quick-nav > .container, .departure-section > .container'
  );
  const entries = [...list.querySelectorAll('a')].map(link => ({
    link,
    section: document.getElementById(link.hash.slice(1))
  }));
  if (entries.some(entry => !entry.section)) return;

  const touchOnly = window.matchMedia('(hover: none) and (pointer: coarse)');
  let mode;
  let open = false;
  let active;
  let lift = 0;
  let headerHeight = 0;
  let top = 0;
  let frame = 0;
  let layoutPending = false;

  function keepActiveVisible() {
    if (!active || panel.hidden || list.contains(document.activeElement)) return;

    const item = active.getBoundingClientRect();
    const viewport = list.getBoundingClientRect();
    if (item.top < viewport.top) list.scrollTop -= viewport.top - item.top;
    if (item.bottom > viewport.bottom) list.scrollTop += item.bottom - viewport.bottom;
  }

  function setLift(amount) {
    if (lift === amount) return;
    lift = amount;
    toc.style.setProperty('--departure-toc-lift', amount + 'px');
  }

  function setOpen(expanded, returnFocus = false) {
    open = expanded;
    toc.toggleAttribute('data-open', open);
    toggle.setAttribute('aria-expanded', String(open));
    panel.hidden = mode === 'sheet' && !open;

    if (open) {
      setLift(0);
      toc.removeAttribute('data-obscured');
      close.focus({ preventScroll: true });
      keepActiveVisible();
    } else if (returnFocus) {
      toggle.focus({ preventScroll: true });
    }
    schedule();
  }

  function updateLayout() {
    const right = Math.max(...Array.from(content, element => element.getBoundingClientRect().right));
    const style = getComputedStyle(toc);
    const width = parseFloat(style.getPropertyValue('--departure-toc-width'));
    const gap = parseFloat(style.getPropertyValue('--departure-toc-gap'));
    const edge = parseFloat(style.getPropertyValue('--departure-toc-edge'));
    const canDock = !touchOnly.matches && document.documentElement.clientWidth - right >= width + gap + edge;

    headerHeight = header.offsetHeight;
    top = headerHeight + 12;
    toc.style.setProperty('--departure-toc-top', top + 'px');
    toc.style.setProperty('--departure-toc-left', (right + gap) + 'px');

    const next = canDock ? 'dock' : 'sheet';
    if (mode === next) return;

    const focused = document.activeElement;
    mode = next;
    toc.dataset.mode = mode;
    toc.removeAttribute('data-obscured');
    toggle.hidden = canDock;
    close.hidden = canDock;
    setOpen(false, !canDock && panel.contains(focused));
    if (canDock && (focused === toggle || focused === close)) {
      (active || entries[0].link).focus({ preventScroll: true });
    }
    keepActiveVisible();
  }

  function positionSheet() {
    if (open) return;

    const button = toggle.getBoundingClientRect();
    const restingBottom = button.bottom + lift;
    const form = calculator.getBoundingClientRect();
    let bottom = restingBottom;

    if (form.top < bottom && form.bottom > top) {
      const obstacles = Array.from(calculatorContent, element => element.getBoundingClientRect())
        .filter(box => box.height && box.right > button.left && box.left < button.right)
        .sort((first, second) => second.top - first.top);

      for (const box of obstacles) {
        if (bottom > box.top - 6 && bottom - button.height < box.bottom + 6) bottom = box.top - 6;
      }
    }

    const fits = bottom - button.height >= top;
    toc.toggleAttribute('data-obscured', !fits);
    setLift(fits ? Math.max(0, restingBottom - bottom) : 0);
  }

  function update() {
    frame = 0;
    const remeasured = layoutPending;
    if (remeasured) {
      layoutPending = false;
      updateLayout();
    }

    if (mode === 'dock') {
      const height = Math.min(window.innerHeight, footer.getBoundingClientRect().top) - top - 16;
      toc.style.setProperty('--departure-toc-dock-height', Math.max(0, height) + 'px');
      toc.toggleAttribute('data-obscured', height < 90);
    } else {
      positionSheet();
    }

    const viewport = document.documentElement;
    const atEnd = window.scrollY + viewport.clientHeight >= viewport.scrollHeight - 2;
    const entry = atEnd
      ? entries[entries.length - 1]
      : entries.findLast(({ section }) => section.getBoundingClientRect().top <= headerHeight + 30) || entries[0];

    if (entry.link === active) {
      if (remeasured) keepActiveVisible();
      return;
    }
    active?.removeAttribute('aria-current');
    active = entry.link;
    active.setAttribute('aria-current', 'location');
    keepActiveVisible();
  }

  function schedule(remeasure = false) {
    if (remeasure) layoutPending = true;
    if (!frame) frame = requestAnimationFrame(update);
  }

  toggle.addEventListener('click', () => setOpen(!open, open));
  close.addEventListener('click', () => setOpen(false, true));
  list.addEventListener('click', event => {
    if (event.defaultPrevented || event.button !== 0 || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;

    const entry = entries.find(({ link }) => link === event.target.closest('a'));
    if (!entry) return;
    const heading = entry.section.querySelector('h2');
    if (open) setOpen(false);

    // Keep the native fragment navigation and move focus after it has started.
    requestAnimationFrame(() => {
      heading.tabIndex = -1;
      heading.focus({ preventScroll: true });
    });
  });
  document.addEventListener('click', event => {
    if (open && !toc.contains(event.target)) setOpen(false, panel.contains(document.activeElement));
  });
  toc.addEventListener('focusout', event => {
    if (open && event.relatedTarget && !toc.contains(event.relatedTarget)) setOpen(false);
  });
  document.addEventListener('keydown', event => {
    if (!open || event.key !== 'Escape') return;
    event.preventDefault();
    setOpen(false, true);
  });

  window.addEventListener('scroll', () => schedule(), { passive: true });
  window.addEventListener('hashchange', () => schedule());
  window.addEventListener('resize', () => schedule(true));
  window.addEventListener('pageshow', () => schedule(true));
  touchOnly.addEventListener('change', () => schedule(true));
  const resize = new ResizeObserver(() => schedule(true));
  resize.observe(header);
  resize.observe(document.querySelector('main'));
  schedule(true);
})();

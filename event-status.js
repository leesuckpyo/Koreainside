(function () {
  'use strict';

  const TIMEZONE = 'Asia/Seoul';
  const STATUS_CLASSES = {
    UPCOMING: 'ki-event-status--upcoming',
    'HAPPENING NOW': 'ki-event-status--happening',
    ENDED: 'ki-event-status--ended'
  };
  const SPANISH_STATUS_LABELS = {
    UPCOMING: 'PRÓXIMAMENTE',
    'HAPPENING NOW': 'EN CURSO',
    ENDED: 'FINALIZADO'
  };
  const THAI_STATUS_LABELS = {
    UPCOMING: 'เร็วๆ นี้',
    'HAPPENING NOW': 'กำลังจัดอยู่',
    ENDED: 'สิ้นสุดแล้ว'
  };
  const FRENCH_STATUS_LABELS = {
    UPCOMING: 'À VENIR',
    'HAPPENING NOW': 'EN COURS',
    ENDED: 'TERMINÉ'
  };
  const HONGDAE_STATUS_LABELS = {
    ja: {
      UPCOMING: '開催予定',
      'HAPPENING NOW': '開催中',
      ENDED: '終了',
      'PLANNED — NOT CONFIRMED': '計画中 — 開催未確定',
      'PLANNED DATE PASSED — NOT VERIFIED': '計画日経過 — 開催実績未確認'
    },
    'zh-TW': {
      UPCOMING: '即將舉辦',
      'HAPPENING NOW': '進行中',
      ENDED: '已結束',
      'PLANNED — NOT CONFIRMED': '規劃中 — 尚未確認舉辦',
      'PLANNED DATE PASSED — NOT VERIFIED': '預定日期已過 — 尚未確認實際舉辦'
    },
    es: {
      UPCOMING: 'PRÓXIMAMENTE',
      'HAPPENING NOW': 'EN CURSO',
      ENDED: 'FINALIZADO',
      'PLANNED — NOT CONFIRMED': 'PREVISTO — SIN CONFIRMAR',
      'PLANNED DATE PASSED — NOT VERIFIED': 'FECHA PREVISTA TRANSCURRIDA — CELEBRACIÓN NO VERIFICADA'
    },
    'es-MX': {
      UPCOMING: 'PRÓXIMAMENTE',
      'HAPPENING NOW': 'EN CURSO',
      ENDED: 'FINALIZADO',
      'PLANNED — NOT CONFIRMED': 'PREVISTO — SIN CONFIRMAR',
      'PLANNED DATE PASSED — NOT VERIFIED': 'PASÓ LA FECHA PREVISTA — NO SE HA VERIFICADO SU REALIZACIÓN'
    }
  };

  function isDate(value) {
    if (typeof value !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
    const timestamp = Date.parse(`${value}T00:00:00Z`);
    return Number.isFinite(timestamp) && new Date(timestamp).toISOString().slice(0, 10) === value;
  }

  function getSeoulDate(now = new Date()) {
    const parts = new Intl.DateTimeFormat('en-US', {
      timeZone: TIMEZONE,
      calendar: 'gregory',
      numberingSystem: 'latn',
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    }).formatToParts(now);
    const date = Object.fromEntries(parts.map(part => [part.type, part.value]));
    return `${date.year}-${date.month}-${date.day}`;
  }

  function getEventStatus(event, today) {
    if (!event || !isDate(today) || !isDate(event.end)
        || (event.start !== null && (!isDate(event.start) || event.start > event.end))) {
      throw new Error('Invalid event date or date range.');
    }
    if (event.start !== null && today < event.start) return 'UPCOMING';
    return today <= event.end ? 'HAPPENING NOW' : 'ENDED';
  }

  function indexEvents(data) {
    if (!data || data.timezone !== TIMEZONE || !Array.isArray(data.events)) {
      throw new Error('Invalid event data or timezone.');
    }
    const events = new Map();
    for (const event of data.events) {
      if (!event || typeof event.id !== 'string' || !event.id || events.has(event.id)) {
        throw new Error('Missing or duplicate event id.');
      }
      getEventStatus(event, event.end);
      events.set(event.id, event);
    }
    return events;
  }

  function placeHongdaeEvent(article, container, level, before = null) {
    const heading = article.firstElementChild;
    if (heading.localName !== level) {
      const replacement = document.createElement(level);
      replacement.textContent = heading.textContent;
      heading.replaceWith(replacement);
    }
    if (article.parentElement !== container) container.insertBefore(article, before);
  }

  function prepareHongdaeSchedule(root, events, plannedEvents) {
    const confirmed = root.querySelector('[data-hongdae-confirmed-event]');
    const planned = root.querySelector('[data-hongdae-planned-event]');
    const confirmedEvent = events.get(confirmed.dataset.hongdaeConfirmedEvent);
    const plan = plannedEvents?.find((event) => event.id === planned.dataset.hongdaePlannedEvent);
    if (!confirmedEvent || !isDate(plan?.plannedDate)) {
      throw new Error('Hongdae event dates unavailable.');
    }

    const upcoming = root.querySelector('#upcoming');
    const current = root.querySelector('#happening-now');
    const pastPlans = root.querySelector('#past-plans');
    const upcomingList = upcoming.querySelector('.events-list');
    const currentList = current.querySelector('.events-list');
    const pastPlansList = pastPlans.querySelector('.events-list');
    const archiveHeading = root.querySelector('#archive-2026-heading');
    const upcomingJump = root.querySelectorAll('[data-hongdae-upcoming-jump]');
    const confirmedLabel = confirmed.querySelector('[data-hongdae-status]');
    const plannedLabel = planned.querySelector('[data-hongdae-status]');
    const labels = HONGDAE_STATUS_LABELS[document.documentElement.lang];

    return (today) => {
      const status = getEventStatus(confirmedEvent, today);
      const planPassed = today > plan.plannedDate;
      const planStatus = planPassed ? 'PLANNED DATE PASSED — NOT VERIFIED' : 'PLANNED — NOT CONFIRMED';
      const focused = document.activeElement;

      current.hidden = status !== 'HAPPENING NOW';
      pastPlans.hidden = !planPassed;
      upcoming.hidden = status !== 'UPCOMING' && planPassed;
      upcomingJump.forEach((element) => { element.hidden = upcoming.hidden; });

      if (status === 'ENDED') {
        placeHongdaeEvent(confirmed, archiveHeading.parentElement, 'h4', archiveHeading.nextElementSibling);
      } else if (status === 'HAPPENING NOW') {
        placeHongdaeEvent(confirmed, currentList, 'h3');
      } else {
        placeHongdaeEvent(confirmed, upcomingList, 'h3', upcomingList.firstElementChild);
      }
      placeHongdaeEvent(planned, planPassed ? pastPlansList : upcomingList, 'h3');

      const confirmedText = labels?.[status] ?? status;
      const plannedText = labels?.[planStatus] ?? planStatus;
      if (confirmedLabel.textContent !== confirmedText) confirmedLabel.textContent = confirmedText;
      if (plannedLabel.textContent !== plannedText) plannedLabel.textContent = plannedText;
      if (focused !== document.activeElement && (confirmed.contains(focused) || planned.contains(focused))) {
        focused.focus();
      }
    };
  }

  async function initialize() {
    const badges = Array.from(document.querySelectorAll('.ki-event-status[data-event-id]'));
    const hongdaeSchedule = document.querySelector('main[data-hongdae-event-schedule]');
    if (!badges.length && !hongdaeSchedule) return;

    try {
      const response = await fetch('/data/events.json', { cache: 'no-cache' });
      if (!response.ok) throw new Error(`Event data request failed (${response.status}).`);
      const data = await response.json();
      const events = indexEvents(data);
      const refreshHongdae = hongdaeSchedule && prepareHongdaeSchedule(hongdaeSchedule, events, data.plannedEvents);
      const matched = [];
      for (const badge of badges) {
        const event = events.get(badge.dataset.eventId);
        if (event) {
          matched.push({ badge, event });
        } else {
          console.warn('[Korea Inside events] Unknown event id:', badge.dataset.eventId);
        }
      }
      if (!matched.length && !refreshHongdae) return;

      let timer;
      function refresh() {
        clearTimeout(timer);
        const now = new Date();
        const today = getSeoulDate(now);
        if (refreshHongdae) refreshHongdae(today);
        for (const { badge, event } of matched) {
          const status = getEventStatus(event, today);
          badge.classList.remove(...Object.values(STATUS_CLASSES));
          badge.classList.add(STATUS_CLASSES[status]);
          badge.textContent = document.documentElement.lang === 'es'
            ? SPANISH_STATUS_LABELS[status]
            : document.documentElement.lang === 'th'
              ? THAI_STATUS_LABELS[status]
              : document.documentElement.lang === 'fr'
                ? FRENCH_STATUS_LABELS[status]
                : status;
        }
        // Keep an open page current at the next KST date boundary (UTC+09:00).
        const midnight = Date.parse(`${today}T00:00:00+09:00`) + 86400000;
        timer = setTimeout(refresh, Math.max(1, midnight - now.getTime()) + 50);
      }

      refresh();
      document.addEventListener('visibilitychange', function () {
        if (document.visibilityState === 'visible') refresh();
      });
      if (refreshHongdae) window.addEventListener('pageshow', refresh);
    } catch (error) {
      console.warn('[Korea Inside events] Status unavailable:', error.message);
    }
  }

  // Allow the same pure date logic to be checked without a browser or test clock in the UI.
  if (typeof module !== 'undefined' && module.exports) {
    module.exports = { getSeoulDate, getEventStatus, indexEvents };
  }
  if (typeof document !== 'undefined') initialize();
}());

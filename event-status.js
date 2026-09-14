(function () {
  'use strict';

  const TIMEZONE = 'Asia/Seoul';
  const STATUS_CLASSES = {
    UPCOMING: 'ki-event-status--upcoming',
    'HAPPENING NOW': 'ki-event-status--happening',
    ENDED: 'ki-event-status--ended'
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

  async function initialize() {
    const badges = Array.from(document.querySelectorAll('.ki-event-status[data-event-id]'));
    if (!badges.length) return;

    try {
      const response = await fetch('/data/events.json', { cache: 'no-cache' });
      if (!response.ok) throw new Error(`Event data request failed (${response.status}).`);
      const events = indexEvents(await response.json());
      const matched = [];
      for (const badge of badges) {
        const event = events.get(badge.dataset.eventId);
        if (event) {
          matched.push({ badge, event });
        } else {
          console.warn('[Korea Inside events] Unknown event id:', badge.dataset.eventId);
        }
      }
      if (!matched.length) return;

      let timer;
      function refresh() {
        clearTimeout(timer);
        const now = new Date();
        const today = getSeoulDate(now);
        for (const { badge, event } of matched) {
          const status = getEventStatus(event, today);
          badge.classList.remove(...Object.values(STATUS_CLASSES));
          badge.classList.add(STATUS_CLASSES[status]);
          badge.textContent = status;
        }
        // Keep an open page current at the next KST date boundary (UTC+09:00).
        const midnight = Date.parse(`${today}T00:00:00+09:00`) + 86400000;
        timer = setTimeout(refresh, Math.max(1, midnight - now.getTime()) + 50);
      }

      refresh();
      document.addEventListener('visibilitychange', function () {
        if (document.visibilityState === 'visible') refresh();
      });
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

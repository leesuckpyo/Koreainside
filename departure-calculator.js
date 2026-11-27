(() => {
  const form = document.querySelector('#departure-plan-form');
  if (!form) return;

  const locales = {
    en: {
      format: 'en-US',
      timezone: 'KST',
      previousDay: 'You need to start your journey on the previous day.',
      earlierDate: 'You need to start your journey before your flight date.',
      invalid: 'Enter a valid flight date, time and travel duration.',
      noTerminal: 'Confirm your departure terminal before leaving.',
      noTransport: 'Choose your transport and check its timetable before leaving.'
    },
    ja: {
      format: 'ja-JP',
      timezone: 'KST',
      previousDay: 'フライトの前日に空港へ向けて出発する必要があります。',
      earlierDate: 'フライトの出発日より前に空港へ向けて出発する必要があります。',
      invalid: '有効なフライトの出発日・時刻と移動時間を入力してください。',
      noTerminal: '出発前に利用するターミナルを確認してください。',
      noTransport: '移動手段を選び、出発前に時刻表を確認してください。'
    }
  };
  const locale = locales[document.documentElement.lang] || locales.en;
  const minute = 60_000;
  const day = 24 * 60 * minute;
  const kstOffset = 9 * 60 * minute;
  const dateFormat = new Intl.DateTimeFormat(locale.format, {
    timeZone: 'Asia/Seoul',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
  const timeFormat = new Intl.DateTimeFormat(locale.format, {
    timeZone: 'Asia/Seoul',
    hour: '2-digit',
    minute: '2-digit',
    hourCycle: 'h23'
  });
  const hoursFormat = new Intl.NumberFormat(locale.format, { style: 'unit', unit: 'hour', unitDisplay: 'long' });
  const minutesFormat = new Intl.NumberFormat(locale.format, { style: 'unit', unit: 'minute', unitDisplay: 'long' });
  const flightMonth = form.querySelector('#flight-month');
  const flightDay = form.querySelector('#flight-day');
  const flightYear = form.querySelector('#flight-year');
  const flightHour = form.querySelector('#flight-hour');
  const flightMinute = form.querySelector('#flight-minute');
  const dateFields = [...form.querySelectorAll('.departure-plan__date-fields input, .departure-plan__date-fields select')];
  const timeFields = [flightHour, flightMinute];
  const terminal = form.querySelector('#departure-plan-terminal');
  const allowanceChoice = form.querySelector('#departure-plan-allowance');
  const customAllowanceField = form.querySelector('#departure-plan-custom');
  const customAllowance = form.querySelector('#departure-plan-custom-allowance');
  const transport = form.querySelector('#departure-plan-transport');
  const travel = form.querySelector('#departure-plan-travel');
  const buffer = form.querySelector('#departure-plan-buffer');
  const requiredFields = [...dateFields, ...timeFields, customAllowance, travel, buffer];
  const result = form.querySelector('#departure-plan-result');
  const error = form.querySelector('#departure-plan-error');
  const rollover = form.querySelector('#departure-plan-rollover');
  const terminalWarning = form.querySelector('#departure-plan-terminal-warning');
  const transportWarning = form.querySelector('#departure-plan-transport-warning');
  const hotelOutput = form.querySelector('#departure-plan-hotel-output');
  const arrivalOutput = form.querySelector('#departure-plan-arrival-output');
  const totalOutput = form.querySelector('#departure-plan-total-output');
  const flightOutput = form.querySelector('#departure-plan-flight-output');

  function validateDate() {
    flightYear.setCustomValidity('');
    flightDay.setCustomValidity('');

    const year = Number(flightYear.value);
    if (flightYear.validity.valid && year < 1989) flightYear.setCustomValidity(locale.invalid);
    if (dateFields.some(isInvalid)) return NaN;

    const dayOfMonth = Number(flightDay.value);
    const calendarDate = new Date(Date.UTC(year, Number(flightMonth.value) - 1, dayOfMonth));
    if (calendarDate.getUTCDate() !== dayOfMonth) {
      flightDay.setCustomValidity(locale.invalid);
      return NaN;
    }
    return calendarDate.getTime();
  }

  // Interpret the entered calendar day and clock time as KST.
  function flightInKst(calendarDate) {
    const clockMinutes = Number(flightHour.value) * 60 + Number(flightMinute.value);
    return calendarDate + clockMinutes * minute - kstOffset;
  }

  function setTime(output, timestamp) {
    const value = document.createElement('time');
    value.dateTime = new Date(timestamp).toISOString();
    value.translate = false;
    value.textContent = `${dateFormat.format(timestamp)} — ${timeFormat.format(timestamp)} ${locale.timezone}`;
    output.replaceChildren(value);
  }

  function clearResult() {
    result.hidden = true;
    [hotelOutput, arrivalOutput, flightOutput, totalOutput].forEach(output => output.replaceChildren());
  }

  function clearErrors() {
    error.textContent = '';
    requiredFields.forEach(field => field.removeAttribute('aria-invalid'));
  }

  function showCustomAllowance(show) {
    customAllowanceField.hidden = !show;
    customAllowance.disabled = !show;
    if (!show) customAllowance.removeAttribute('aria-invalid');
  }

  function isInvalid(field) {
    return !field.disabled && (!field.validity.valid ||
      (field.type === 'number' && !Number.isFinite(field.valueAsNumber)));
  }

  function handleEdit({ target, type }) {
    clearResult();
    if (target === allowanceChoice) showCustomAllowance(allowanceChoice.value === 'custom');
    if (dateFields.includes(target)) validateDate();
    if (type === 'change' && timeFields.includes(target) && /^[0-9]$/.test(target.value)) {
      target.value = target.value.padStart(2, '0');
    }
    requiredFields.forEach(field => {
      if (field.hasAttribute('aria-invalid') && !isInvalid(field)) field.removeAttribute('aria-invalid');
    });
    if (!form.querySelector('[aria-invalid="true"]')) error.textContent = '';
  }

  form.addEventListener('submit', event => {
    event.preventDefault();
    clearResult();
    clearErrors();

    const calendarDate = validateDate();
    const invalidFields = requiredFields.filter(isInvalid);
    if (invalidFields.length) {
      error.textContent = locale.invalid;
      invalidFields.forEach(field => field.setAttribute('aria-invalid', 'true'));
      invalidFields[0].focus();
      return;
    }

    const allowance = customAllowance.disabled ? Number(allowanceChoice.value) : customAllowance.valueAsNumber;
    const flight = flightInKst(calendarDate);
    const totalMinutes = allowance + travel.valueAsNumber + buffer.valueAsNumber;
    const wholeHours = Math.floor(totalMinutes / 60);
    const remainingMinutes = totalMinutes % 60;
    const airportArrival = flight - allowance * minute;
    const hotelDeparture = flight - totalMinutes * minute;
    const daysEarlier = Math.floor((flight + kstOffset) / day) - Math.floor((hotelDeparture + kstOffset) / day);

    setTime(hotelOutput, hotelDeparture);
    setTime(arrivalOutput, airportArrival);
    setTime(flightOutput, flight);
    totalOutput.textContent = [
      wholeHours ? hoursFormat.format(wholeHours) : '',
      remainingMinutes ? minutesFormat.format(remainingMinutes) : ''
    ].filter(Boolean).join(' ');
    rollover.hidden = daysEarlier === 0;
    rollover.textContent = daysEarlier === 1 ? locale.previousDay : locale.earlierDate;
    terminalWarning.hidden = Boolean(terminal.value);
    terminalWarning.textContent = locale.noTerminal;
    transportWarning.hidden = Boolean(transport.value);
    transportWarning.textContent = locale.noTransport;
    result.hidden = false;
  });

  form.addEventListener('input', handleEdit);
  form.addEventListener('change', handleEdit);
  form.addEventListener('reset', () => {
    showCustomAllowance(false);
    flightYear.setCustomValidity('');
    flightDay.setCustomValidity('');
    clearErrors();
    clearResult();
  });
  showCustomAllowance(allowanceChoice.value === 'custom');
  form.querySelector('fieldset').disabled = false;
  form.querySelector('#departure-plan-manual').hidden = true;
})();

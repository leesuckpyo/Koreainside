# Korea Inside — Spanish Localization Batch 6 — Maps + Apps 2

**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED
**Date:** 2026-09-23
**Target language:** Spanish (`es`)
**Source document:** `Korea_Inside_Maps_Apps2_ES_Localization_Source_Batch6_2026-09-23.md`
**Total localized ITEMs:** 549

> Este documento contiene la redacción en español propuesta para los 549 ITEM del Source MD.
>
> Debe leerse junto con el Source MD, que sigue siendo la fuente de verdad para el texto English, file/line context, protected tokens, exclusiones estructurales, SHA-256, URL, tracking y estructura.
>
> No autoriza implementación hasta que el usuario apruebe el Batch completo.

## Batch pages

1. `maps.html` — 303 ITEMs
2. `apps.html` — 246 ITEMs

## Localization decisions

- Spanish natural y neutral para contenido de viaje; no traducción literal palabra por palabra.
- Se preservan hechos, cifras, juicios editoriales, orden de recomendación, nombres propios, marcas, productos, fechas, rutas, condiciones operativas y demás valores protegidos del Source MD.
- `Korea`, `Korea Inside`, nombres de apps, marcas y nombres oficiales protegidos permanecen intactos.
- Los adjetivos descriptivos `Korean` no protegidos como nombre propio se localizan de forma natural como `coreano/coreana/coreanos/coreanas`.
- Las rutas de interfaz que el Source presenta como etiquetas exactas en inglés, como `MY → Settings → Language → English` y `Preferred Language`, se conservan como texto de interfaz para que coincidan con la app.
- Global navigation, language switcher y footer siguen excluidos conforme al Source MD.
- El Source MD contiene **0** valores `data-*` user-facing en este Batch; no hay `data-*` pendientes de localizar.
- Los `data-*` funcionales, tracking, analytics, affiliate y event siguen protegidos.

## Localization QA

- Localized ITEMs: **549 / 549**
- ITEM numbering: **001–549**, gap **0**
- Empty Spanish ITEMs: **0**
- Duplicate Exact English with inconsistent Spanish: **0**
- Numeric-value/order mismatch: **0**
- Protected-token mismatch: **0**

## Approval status

**APPROVED BY USER — CONTENT LOCKED**

Approval date: **2026-09-23**

Implementation contract:
- no retranslate
- no rewrite
- no summarization
- no expansion
- no recommendation change
- Codex exact implementation solamente

---

# PAGE: `maps.html`

**English source SHA-256:** `14f843c1cccfcb1c0b0e77ba9026920449189d11bf22376d802e08ea94944739`
**Localized ITEM count:** 303

### ITEM 001

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Usa Naver Map, KakaoMap y Google Maps en Korea. Aprende a buscar lugares coreanos, comprobar las salidas del metro, seguir rutas de autobús y resolver problemas cuando la búsqueda en inglés falla.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 002

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Mejor app de mapas para Korea: Naver Map, KakaoMap y Google Maps | Korea Inside
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea Inside`

### ITEM 003

- Source context: L12 - `html > head > meta[property="og:title"] @content`
- Element/type: Open Graph title
- Spanish:

  ```text
  Mejor app de mapas para Korea: Naver Map, KakaoMap y Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 004

- Source context: L13 - `html > head > meta[property="og:description"] @content`
- Element/type: Open Graph description
- Spanish:

  ```text
  Usa Naver Map, KakaoMap y Google Maps en Korea. Aprende a buscar lugares coreanos, comprobar las salidas del metro, seguir rutas de autobús y resolver problemas cuando la búsqueda en inglés falla.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 005

- Source context: L17 - `html > head > meta[name="twitter:title"] @content`
- Element/type: Twitter card title
- Spanish:

  ```text
  Mejor app de mapas para Korea: Naver Map, KakaoMap y Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 006

- Source context: L18 - `html > head > meta[name="twitter:description"] @content`
- Element/type: Twitter card description
- Spanish:

  ```text
  Usa Naver Map como app principal de navegación, KakaoMap como respaldo y Google Maps para planificar, guardar lugares y consultar reseñas.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 007

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Mejor app de mapas para Korea: Naver Map, KakaoMap y Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 008

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].description`
- Element/type: JSON-LD user-facing description
- Spanish:

  ```text
  Usa Naver Map, KakaoMap y Google Maps en Korea. Aprende a buscar lugares coreanos, comprobar las salidas del metro, seguir rutas de autobús y resolver problemas cuando la búsqueda en inglés falla.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`, `Korea`

### ITEM 009

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].isPartOf.name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Korea Inside
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 010

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[0].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Inicio
  ```
- Protected tokens: None identified in this item.

### ITEM 011

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[1].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Mapas
  ```
- Protected tokens: None identified in this item.

### ITEM 012

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de mapas para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 013

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Naver Map es la mejor primera opción para la mayoría de los viajeros porque combina la búsqueda de lugares locales con rutas a pie, en metro y en autobús. Mantén KakaoMap como respaldo y usa Google Maps para planificar, guardar lugares y consultar reseñas internacionales.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 014

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Naver Map está disponible en inglés?
  ```
- Protected tokens: `Naver Map`

### ITEM 015

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Sí. En la app, abre MY y después Settings, Language y English. En iPhone, la app puede dirigirte a Settings, NAVER Map y Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 016

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Google Maps funciona en Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 017

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Sí. Resulta útil para lugares guardados, listas compartidas, reseñas internacionales y la planificación general del viaje. La disponibilidad de rutas y navegación puede variar, así que confirma la ruta exacta en tu dispositivo y ten preparada una app de mapas local.
  ```
- Protected tokens: None identified in this item.

### ITEM 018

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿También debería instalar KakaoMap?
  ```
- Protected tokens: `KakaoMap`

### ITEM 019

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Es un respaldo útil, no un requisito para todos los viajeros. Instálala si quieres comprobar un lugar con otra fuente, comparar una ruta a pie, verificar información de autobuses o abrir un enlace de un lugar de Kakao.
  ```
- Protected tokens: `Kakao`

### ITEM 020

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué hago si falla la búsqueda en inglés?
  ```
- Protected tokens: None identified in this item.

### ITEM 021

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Busca el nombre oficial en coreano del lugar o la dirección vial en tu reserva, en la web del negocio o en otra fuente fiable. Pégalo en Naver Map y confirma después la dirección, el número de teléfono y las fotos.
  ```
- Protected tokens: `Naver Map`

### ITEM 022

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cómo encuentro la salida correcta del metro?
  ```
- Protected tokens: None identified in this item.

### ITEM 023

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Abre la ruta antes de bajar del tren y confirma el número de salida, el lado de la calle y la distancia restante a pie. En una estación grande, comprueba también si necesitas ascensor y cuánto hay que caminar por el interior.
  ```
- Protected tokens: None identified in this item.

### ITEM 024

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cómo evito elegir la parada de autobús equivocada?
  ```
- Protected tokens: None identified in this item.

### ITEM 025

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Comprueba el número o la dirección de la parada, el destino del autobús y el lado de la calle antes de subir. Paradas con nombres parecidos pueden corresponder a sentidos de viaje opuestos.
  ```
- Protected tokens: None identified in this item.

### ITEM 026

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Puedo usar mapas sin conexión en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 027

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Google indica que las descargas de mapas sin conexión no están disponibles en algunos países o regiones y que las indicaciones sin conexión para transporte público, recorridos a pie y bicicleta no están disponibles. Comprueba la disponibilidad de descarga en tu dispositivo y conserva datos móviles para la navegación en tiempo real.
  ```
- Protected tokens: `Google`

### ITEM 028

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cómo guardo y comparto la dirección de mi hotel?
  ```
- Protected tokens: None identified in this item.

### ITEM 029

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Guarda la ficha exacta del hotel en Naver Map, copia su nombre en coreano y la dirección vial, y comparte el enlace del lugar con tus compañeros de viaje. Conserva también ese texto en la confirmación de la reserva.
  ```
- Protected tokens: `Naver Map`

### ITEM 030

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Necesito datos móviles para usar apps de mapas en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 031

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Se recomiendan mucho los datos móviles para cambios de ruta, información del transporte en tiempo real, detalles de lugares y compartir ubicaciones. Prepara un plan de eSIM, SIM o roaming antes de depender de la navegación en directo.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 032

- Source context: L217 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > div.maps-page-hero__copy > h1#maps-page-title`
- Element/type: H1 heading
- Spanish:

  ```text
  Mejor app de mapas para Korea: cómo usar Naver Map, KakaoMap y Google Maps
  ```
- Protected tokens: `Korea`, `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 033

- Source context: L218 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > div.maps-page-hero__copy > p.maps-page-hero__intro`
- Element/type: Hero / lead copy
- Spanish:

  ```text
  Usa Naver Map como tu app principal de navegación en Korea. Mantén KakaoMap como respaldo y utiliza Google Maps para lugares guardados, reseñas y planificación global del viaje.
  ```
- Protected tokens: `Naver Map`, `Korea`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 034

- Source context: L220 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Comparación rápida de apps de mapas
  ```
- Protected tokens: None identified in this item.

### ITEM 035

- Source context: L223 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(1) > dt`
- Element/type: Visible label
- Spanish:

  ```text
  Navegación principal
  ```
- Protected tokens: None identified in this item.

### ITEM 036

- Source context: L224 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(1) > dd`
- Element/type: Visible description
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 037

- Source context: L227 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(2) > dt`
- Element/type: Visible label
- Spanish:

  ```text
  Rutas alternativas y comprobaciones locales
  ```
- Protected tokens: None identified in this item.

### ITEM 038

- Source context: L228 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(2) > dd`
- Element/type: Visible description
- Spanish:

  ```text
  KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 039

- Source context: L231 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(3) > dt`
- Element/type: Visible label
- Spanish:

  ```text
  Planificación, lugares guardados y reseñas internacionales
  ```
- Protected tokens: None identified in this item.

### ITEM 040

- Source context: L232 - `html > body.maps-page > main > section.maps-page-hero > div.container.maps-page-hero__inner > aside.maps-quick-answer > dl > div:nth-of-type(3) > dd`
- Element/type: Visible description
- Spanish:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 041

- Source context: L244 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-section__heading:nth-of-type(1) > div > h2#comparison-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Comparativa de apps de mapas para Korea
  ```
- Protected tokens: `Korea`

### ITEM 042

- Source context: L245 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Elige la app según el problema de viaje que necesites resolver, no solo por familiaridad con la marca.
  ```
- Protected tokens: None identified in this item.

### ITEM 043

- Source context: L248 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Apps de mapas según la necesidad del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 044

- Source context: L252 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(1)`
- Element/type: Table header
- Spanish:

  ```text
  Necesidad del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 045

- Source context: L253 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(2)`
- Element/type: Table header
- Spanish:

  ```text
  Mejor primera opción
  ```
- Protected tokens: None identified in this item.

### ITEM 046

- Source context: L254 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > thead > tr > th:nth-of-type(3)`
- Element/type: Table header
- Spanish:

  ```text
  Uso de respaldo o secundario
  ```
- Protected tokens: None identified in this item.

### ITEM 047

- Source context: L259 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > th`
- Element/type: Table header
- Spanish:

  ```text
  Recorridos a pie y navegación local
  ```
- Protected tokens: None identified in this item.

### ITEM 048

- Source context: L260 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 049

- Source context: L261 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(1) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  KakaoMap para una ruta alternativa
  ```
- Protected tokens: `KakaoMap`

### ITEM 050

- Source context: L264 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > th`
- Element/type: Table header
- Spanish:

  ```text
  Rutas de metro y autobús
  ```
- Protected tokens: None identified in this item.

### ITEM 051

- Source context: L265 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 052

- Source context: L266 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(2) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  KakaoMap para una comprobación local
  ```
- Protected tokens: `KakaoMap`

### ITEM 053

- Source context: L269 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > th`
- Element/type: Table header
- Spanish:

  ```text
  Encontrar la sucursal exacta
  ```
- Protected tokens: None identified in this item.

### ITEM 054

- Source context: L270 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 055

- Source context: L271 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(3) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  KakaoMap para confirmar la ficha
  ```
- Protected tokens: `KakaoMap`

### ITEM 056

- Source context: L274 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > th`
- Element/type: Table header
- Spanish:

  ```text
  Lugares guardados y planificación del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 057

- Source context: L275 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 058

- Source context: L276 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(4) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map para la ficha local definitiva
  ```
- Protected tokens: `Naver Map`

### ITEM 059

- Source context: L279 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > th`
- Element/type: Table header
- Spanish:

  ```text
  Fallo de la búsqueda en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 060

- Source context: L280 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map con el nombre o la dirección en coreano
  ```
- Protected tokens: `Naver Map`

### ITEM 061

- Source context: L281 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(5) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  KakaoMap para comprobar el resultado
  ```
- Protected tokens: `KakaoMap`

### ITEM 062

- Source context: L284 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > th`
- Element/type: Table header
- Spanish:

  ```text
  Reseñas internacionales
  ```
- Protected tokens: None identified in this item.

### ITEM 063

- Source context: L285 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 064

- Source context: L286 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(6) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map para detalles locales y rutas
  ```
- Protected tokens: `Naver Map`

### ITEM 065

- Source context: L289 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > th`
- Element/type: Table header
- Spanish:

  ```text
  Conducción
  ```
- Protected tokens: None identified in this item.

### ITEM 066

- Source context: L290 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 067

- Source context: L291 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(7) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  KakaoMap; verifica Google Maps en tu dispositivo
  ```
- Protected tokens: `KakaoMap`, `Google Maps`, `Google`

### ITEM 068

- Source context: L294 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > th`
- Element/type: Table header
- Spanish:

  ```text
  Uso sin conexión
  ```
- Protected tokens: None identified in this item.

### ITEM 069

- Source context: L295 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Comprueba la disponibilidad de descargas en Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 070

- Source context: L296 - `html > body.maps-page > main > article.maps-guide > div.container > section#compare.maps-section:nth-of-type(1) > div.maps-table-scroll:nth-of-type(2) > table.maps-comparison-table > tbody > tr:nth-of-type(8) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Mantén los datos móviles y guarda las direcciones en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 071

- Source context: L306 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > div.maps-section__heading > div > h2#set-up-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Configura tus mapas antes de llegar
  ```
- Protected tokens: None identified in this item.

### ITEM 072

- Source context: L307 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Prepara las apps, el idioma y una ruta de prueba mientras aún tengas Wi-Fi fiable.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 073

- Source context: L311 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Instala Naver Map Acción Descarga la app oficial y ábrela una vez. Si falla Busca “NAVER Maps, Navigation” en la tienda y confirma que el desarrollador sea NAVER Corp.
  ```
- Protected tokens: `Naver Map`, `NAVER`, `NAVER Corp.`, `NAVER Corp`

### ITEM 074

- Source context: L313 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Instala Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 075

- Source context: L314 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Descarga la app oficial y ábrela una vez.
  ```
- Protected tokens: None identified in this item.

### ITEM 076

- Source context: L315 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(1) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Busca “NAVER Maps, Navigation” en la tienda y confirma que el desarrollador sea NAVER Corp.
  ```
- Protected tokens: `NAVER`, `NAVER Corp.`, `NAVER Corp`

### ITEM 077

- Source context: L318 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Instala KakaoMap como respaldo Acción Tenla preparada para rutas alternativas y comprobaciones locales. Si falla Continúa con Naver Map y abre el servicio web oficial de KakaoMap cuando lo necesites.
  ```
- Protected tokens: `KakaoMap`, `Naver Map`

### ITEM 078

- Source context: L320 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Instala KakaoMap como respaldo
  ```
- Protected tokens: `KakaoMap`

### ITEM 079

- Source context: L321 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Tenla preparada para rutas alternativas y comprobaciones locales.
  ```
- Protected tokens: None identified in this item.

### ITEM 080

- Source context: L322 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(2) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Continúa con Naver Map y abre el servicio web oficial de KakaoMap cuando lo necesites.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 081

- Source context: L325 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Conserva Google Maps Acción Mantén tus lugares guardados, listas compartidas y el plan de viaje que ya conoces. Si falla Copia el nombre o la dirección del lugar en una app de mapas local para obtener la ruta final.
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 082

- Source context: L327 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Conserva Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 083

- Source context: L328 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Mantén tus lugares guardados, listas compartidas y el plan de viaje que ya conoces.
  ```
- Protected tokens: None identified in this item.

### ITEM 084

- Source context: L329 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(3) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Copia el nombre o la dirección del lugar en una app de mapas local para obtener la ruta final.
  ```
- Protected tokens: None identified in this item.

### ITEM 085

- Source context: L332 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Permite el acceso a la ubicación Acción Permite la ubicación mientras uses cada app para que el punto de partida sea preciso. Si falla Abre los ajustes de privacidad o permisos de la app del teléfono y activa la ubicación precisa.
  ```
- Protected tokens: None identified in this item.

### ITEM 086

- Source context: L334 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Permite el acceso a la ubicación
  ```
- Protected tokens: None identified in this item.

### ITEM 087

- Source context: L335 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Permite la ubicación mientras uses cada app para que el punto de partida sea preciso.
  ```
- Protected tokens: None identified in this item.

### ITEM 088

- Source context: L336 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(4) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Abre los ajustes de privacidad o permisos de la app del teléfono y activa la ubicación precisa.
  ```
- Protected tokens: None identified in this item.

### ITEM 089

- Source context: L339 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Cambia Naver Map al inglés Acción Abre MY, Settings, Language y elige English. Si falla En iPhone, usa Settings, NAVER Map y Preferred Language.
  ```
- Protected tokens: `Naver Map`, `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 090

- Source context: L341 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Cambia Naver Map al inglés
  ```
- Protected tokens: `Naver Map`

### ITEM 091

- Source context: L342 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Abre MY, Settings, Language y elige English.
  ```
- Protected tokens: None identified in this item.

### ITEM 092

- Source context: L343 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(5) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla En iPhone, usa Settings, NAVER Map y Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 093

- Source context: L346 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Guarda el nombre y la dirección en coreano de tu hotel Acción Guarda la ficha exacta, la dirección vial en coreano y el número de teléfono. Si falla Copia el texto en coreano de tu hotel o de la confirmación de la reserva.
  ```
- Protected tokens: None identified in this item.

### ITEM 094

- Source context: L348 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Guarda el nombre y la dirección en coreano de tu hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 095

- Source context: L349 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Guarda la ficha exacta, la dirección vial en coreano y el número de teléfono.
  ```
- Protected tokens: None identified in this item.

### ITEM 096

- Source context: L350 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(6) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Copia el texto en coreano de tu hotel o de la confirmación de la reserva.
  ```
- Protected tokens: None identified in this item.

### ITEM 097

- Source context: L353 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Prueba una ruta del aeropuerto al hotel Acción Selecciona la terminal correcta del aeropuerto y compara el transporte con el último tramo a pie. Si falla Guarda una captura de pantalla y vuelve a comprobar la ruta después de aterrizar.
  ```
- Protected tokens: None identified in this item.

### ITEM 098

- Source context: L355 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Prueba una ruta del aeropuerto al hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 099

- Source context: L356 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Acción Selecciona la terminal correcta del aeropuerto y compara el transporte con el último tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 100

- Source context: L357 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > ol.maps-action-rows > li:nth-of-type(7) > div > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si falla Guarda una captura de pantalla y vuelve a comprobar la ruta después de aterrizar.
  ```
- Protected tokens: None identified in this item.

### ITEM 101

- Source context: L361 - `html > body.maps-page > main > article.maps-guide > div.container > section#set-up.maps-section:nth-of-type(2) > p.maps-context-link`
- Element/type: Body text
- Spanish:

  ```text
  Las rutas en tiempo real funcionan mejor con datos móviles. Prepara una eSIM para Korea, compara opciones de eSIM, revisa la guía de apps esenciales y añade estos pasos a tu lista de viaje por Korea.
  ```
- Protected tokens: `eSIM`, `Korea`

### ITEM 102

- Source context: L367 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-section__heading:nth-of-type(1) > div > h2#naver-map-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Cómo usar Naver Map en inglés
  ```
- Protected tokens: `Naver Map`

### ITEM 103

- Source context: L368 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Sigue esta secuencia desde la configuración hasta la salida final del metro o la parada de autobús.
  ```
- Protected tokens: None identified in this item.

### ITEM 104

- Source context: L373 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > h3#naver-language-title`
- Element/type: H3 heading
- Spanish:

  ```text
  A. Cambia el idioma de la app al inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 105

- Source context: L376 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(1) > p.maps-small-label:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Android
  ```
- Protected tokens: `Android`

### ITEM 106

- Source context: L377 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(1) > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  MY → Settings → Language → English
  ```
- Protected tokens: None identified in this item.

### ITEM 107

- Source context: L380 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p.maps-small-label:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  iPhone
  ```
- Protected tokens: `iPhone`

### ITEM 108

- Source context: L381 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  MY → Settings → Language
  ```
- Protected tokens: None identified in this item.

### ITEM 109

- Source context: L382 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > div.maps-setting-paths > div:nth-of-type(2) > p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Si es necesario: iPhone Settings → NAVER Map → Preferred Language → English
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 110

- Source context: L386 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(1) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Cómo cambiar NAVER Map al inglés desde los ajustes de la app
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 111

- Source context: L390 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > h3#naver-search-title`
- Element/type: H3 heading
- Spanish:

  ```text
  B. Busca un lugar
  ```
- Protected tokens: None identified in this item.

### ITEM 112

- Source context: L391 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Prueba con información cada vez más específica. Que una búsqueda en inglés falle no significa que el lugar no exista.
  ```
- Protected tokens: None identified in this item.

### ITEM 113

- Source context: L393 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Nombre del lugar en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 114

- Source context: L394 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Dirección vial en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 115

- Source context: L395 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Nombre del lugar en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 116

- Source context: L396 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Dirección vial en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 117

- Source context: L397 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Número de teléfono
  ```
- Protected tokens: None identified in this item.

### ITEM 118

- Source context: L401 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > h3#naver-branch-title`
- Element/type: H3 heading
- Spanish:

  ```text
  C. Confirma la sucursal correcta
  ```
- Protected tokens: None identified in this item.

### ITEM 119

- Source context: L402 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > p`
- Element/type: Body text
- Spanish:

  ```text
  Hoteles, cafés y restaurantes pueden compartir nombre en distintos barrios. Comprueba la ficha antes de iniciar la ruta.
  ```
- Protected tokens: None identified in this item.

### ITEM 120

- Source context: L404 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Barrio o distrito
  ```
- Protected tokens: None identified in this item.

### ITEM 121

- Source context: L405 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Dirección vial en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 122

- Source context: L406 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Número de teléfono
  ```
- Protected tokens: None identified in this item.

### ITEM 123

- Source context: L407 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Fotos
  ```
- Protected tokens: None identified in this item.

### ITEM 124

- Source context: L408 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Estación de metro más cercana
  ```
- Protected tokens: None identified in this item.

### ITEM 125

- Source context: L409 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Distancia desde la estación
  ```
- Protected tokens: None identified in this item.

### ITEM 126

- Source context: L410 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > ul.maps-check-list > li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Información de apertura
  ```
- Protected tokens: None identified in this item.

### ITEM 127

- Source context: L413 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(3) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Cómo buscar en NAVER Map y comparar resultados de lugares similares
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 128

- Source context: L417 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > h3#naver-directions-title`
- Element/type: H3 heading
- Spanish:

  ```text
  D. Obtén indicaciones
  ```
- Protected tokens: None identified in this item.

### ITEM 129

- Source context: L419 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Establece tu ubicación actual u otro punto de partida.
  ```
- Protected tokens: None identified in this item.

### ITEM 130

- Source context: L420 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Selecciona el destino confirmado.
  ```
- Protected tokens: None identified in this item.

### ITEM 131

- Source context: L421 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Elige transporte público o recorrido a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 132

- Source context: L422 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Compara el tiempo de viaje, los transbordos y la distancia final a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 133

- Source context: L423 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(4) > ul > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Guarda la ruta antes de dejar la Wi-Fi.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 134

- Source context: L427 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > h3#naver-exits-title`
- Element/type: H3 heading
- Spanish:

  ```text
  E. Comprueba las salidas del metro
  ```
- Protected tokens: None identified in this item.

### ITEM 135

- Source context: L429 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  El nombre de una estación por sí solo no basta.
  ```
- Protected tokens: None identified in this item.

### ITEM 136

- Source context: L430 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Confirma el número de salida antes de bajar del tren.
  ```
- Protected tokens: None identified in this item.

### ITEM 137

- Source context: L431 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba a qué lado de la calle sale esa salida.
  ```
- Protected tokens: None identified in this item.

### ITEM 138

- Source context: L432 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(5) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  En algunas estaciones grandes, cuenta con una caminata larga por el interior.
  ```
- Protected tokens: None identified in this item.

### ITEM 139

- Source context: L436 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > h3#naver-bus-title`
- Element/type: H3 heading
- Spanish:

  ```text
  F. Comprueba las paradas de autobús
  ```
- Protected tokens: None identified in this item.

### ITEM 140

- Source context: L438 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Dos paradas pueden tener nombres parecidos.
  ```
- Protected tokens: None identified in this item.

### ITEM 141

- Source context: L439 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Confirma el número o la dirección de la parada.
  ```
- Protected tokens: None identified in this item.

### ITEM 142

- Source context: L440 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba el destino del autobús antes de subir.
  ```
- Protected tokens: None identified in this item.

### ITEM 143

- Source context: L441 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Revisa el último tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 144

- Source context: L444 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(6) > figure.maps-guide-figure > img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Cómo comparar rutas en NAVER Map y confirmar los puntos de subida y bajada
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 145

- Source context: L448 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > h3#naver-save-title`
- Element/type: H3 heading
- Spanish:

  ```text
  G. Guarda y comparte un lugar
  ```
- Protected tokens: None identified in this item.

### ITEM 146

- Source context: L450 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Guarda tu hotel y la terminal exacta del aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 147

- Source context: L451 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Guarda restaurantes solo después de confirmar la sucursal.
  ```
- Protected tokens: None identified in this item.

### ITEM 148

- Source context: L452 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Copia la dirección en coreano como texto.
  ```
- Protected tokens: None identified in this item.

### ITEM 149

- Source context: L453 - `html > body.maps-page > main > article.maps-guide > div.container > section#naver-map-guide.maps-section:nth-of-type(3) > div.maps-manual:nth-of-type(2) > section:nth-of-type(7) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Comparte el enlace exacto del lugar con tus compañeros de viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 150

- Source context: L462 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-section__heading:nth-of-type(1) > div > h2#search-recovery-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Qué hacer cuando falla la búsqueda en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 151

- Source context: L463 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Recupera el resultado cambiando un nombre traducido por la información exacta de la ficha coreana.
  ```
- Protected tokens: None identified in this item.

### ITEM 152

- Source context: L466 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Proceso para resolver una búsqueda en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 153

- Source context: L467 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Falla el nombre en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 154

- Source context: L468 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Busca el nombre oficial en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 155

- Source context: L469 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Copia el texto en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 156

- Source context: L470 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Pégalo en Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 157

- Source context: L471 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Confirma dirección, teléfono y fotos
  ```
- Protected tokens: None identified in this item.

### ITEM 158

- Source context: L472 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > ol.maps-recovery-flow > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Compruébalo también en KakaoMap si hace falta
  ```
- Protected tokens: `KakaoMap`

### ITEM 159

- Source context: L476 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > h3#korean-name-source-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Dónde encontrar el nombre en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 160

- Source context: L478 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Hotel o confirmación de reserva
  ```
- Protected tokens: None identified in this item.

### ITEM 161

- Source context: L479 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Sitio web oficial del negocio
  ```
- Protected tokens: None identified in this item.

### ITEM 162

- Source context: L480 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Perfil oficial de Instagram
  ```
- Protected tokens: `Instagram`

### ITEM 163

- Source context: L481 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Resultado de búsqueda de Google
  ```
- Protected tokens: `Google`

### ITEM 164

- Source context: L482 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Dirección vial en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 165

- Source context: L483 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(1) > ul > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Número de teléfono
  ```
- Protected tokens: None identified in this item.

### ITEM 166

- Source context: L487 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > h3#search-failure-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Por qué un resultado puede seguir siendo incorrecto
  ```
- Protected tokens: None identified in this item.

### ITEM 167

- Source context: L489 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  El nombre del sitio de reservas no coincide con el nombre registrado localmente.
  ```
- Protected tokens: None identified in this item.

### ITEM 168

- Source context: L490 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Otra sucursal utiliza el mismo nombre.
  ```
- Protected tokens: None identified in this item.

### ITEM 169

- Source context: L491 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  El restaurante está registrado con el nombre de un edificio.
  ```
- Protected tokens: None identified in this item.

### ITEM 170

- Source context: L492 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  El negocio se mudó, cerró o es un pop-up temporal.
  ```
- Protected tokens: None identified in this item.

### ITEM 171

- Source context: L493 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  La tienda está dentro de un centro comercial subterráneo.
  ```
- Protected tokens: None identified in this item.

### ITEM 172

- Source context: L494 - `html > body.maps-page > main > article.maps-guide > div.container > section#search-recovery.maps-section:nth-of-type(4) > div.maps-recovery-notes:nth-of-type(2) > section:nth-of-type(2) > ul > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  La grafía en inglés varía.
  ```
- Protected tokens: None identified in this item.

### ITEM 173

- Source context: L503 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > div.maps-section__heading > div > h2#kakaomap-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Cuándo usar KakaoMap como respaldo
  ```
- Protected tokens: `KakaoMap`

### ITEM 174

- Source context: L504 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  KakaoMap aporta un segundo resultado local cuando conviene confirmar. No tiene que sustituir a tu app principal.
  ```
- Protected tokens: `KakaoMap`

### ITEM 175

- Source context: L508 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Los resultados de búsqueda de Naver no están claros.
  ```
- Protected tokens: None identified in this item.

### ITEM 176

- Source context: L509 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Quieres comparar una ruta a pie alternativa.
  ```
- Protected tokens: None identified in this item.

### ITEM 177

- Source context: L510 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Necesitas comprobar la información del autobús con otra fuente.
  ```
- Protected tokens: None identified in this item.

### ITEM 178

- Source context: L511 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Una persona local te envía un enlace de un lugar de Kakao.
  ```
- Protected tokens: `Kakao`

### ITEM 179

- Source context: L512 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Quieres confirmar el nombre o la dirección de un lugar.
  ```
- Protected tokens: None identified in this item.

### ITEM 180

- Source context: L513 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > ul.maps-editorial-list > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Quieres otra ruta antes de un trayecto nocturno.
  ```
- Protected tokens: None identified in this item.

### ITEM 181

- Source context: L515 - `html > body.maps-page > main > article.maps-guide > div.container > section#kakaomap-backup.maps-section:nth-of-type(5) > p.maps-conclusion-line`
- Element/type: Body text
- Spanish:

  ```text
  Primero Naver Map → KakaoMap cuando necesites confirmar o buscar otra ruta.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 182

- Source context: L521 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > div.maps-section__heading > div > h2#google-maps-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Qué puede y qué no puede hacer Google Maps en Korea
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 183

- Source context: L522 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Conserva Google Maps, pero distingue las funciones disponibles hoy de posibles mejoras futuras.
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 184

- Source context: L525 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  South Korea aprobó la exportación por Google de datos cartográficos a escala 1:5,000 en February 2026 bajo estrictas condiciones de seguridad. Esto puede facilitar mejoras futuras, pero el calendario de despliegue y la disponibilidad de funciones pueden seguir variando según el dispositivo, la ruta y la región.
  ```
- Protected tokens: `South Korea`, `Korea`, `Google`, `1:5,000`, `February 2026`

### ITEM 185

- Source context: L526 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > h3:nth-of-type(1)`
- Element/type: H3 heading
- Spanish:

  ```text
  Funciones útiles ahora mismo
  ```
- Protected tokens: None identified in this item.

### ITEM 186

- Source context: L528 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Guarda lugares antes del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 187

- Source context: L529 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Organiza listas compartidas.
  ```
- Protected tokens: None identified in this item.

### ITEM 188

- Source context: L530 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Lee reseñas internacionales.
  ```
- Protected tokens: None identified in this item.

### ITEM 189

- Source context: L531 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Comprende la relación entre las distintas zonas de la ciudad.
  ```
- Protected tokens: None identified in this item.

### ITEM 190

- Source context: L532 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Comparte una ubicación conocida de Google.
  ```
- Protected tokens: `Google`

### ITEM 191

- Source context: L533 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul.maps-check-list:nth-of-type(1) > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba las funciones disponibles actualmente en tu dispositivo.
  ```
- Protected tokens: None identified in this item.

### ITEM 192

- Source context: L535 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > p.maps-caution:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Importante No des por hecho que todas las funciones de rutas a pie, conducción o navegación estén ya totalmente disponibles. Confirma la ruta en tu dispositivo y ten preparados Naver Map o KakaoMap.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 193

- Source context: L536 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > h3:nth-of-type(2)`
- Element/type: H3 heading
- Spanish:

  ```text
  Mapas sin conexión de Google
  ```
- Protected tokens: `Google`

### ITEM 194

- Source context: L538 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  En algunos países o regiones puede no estar permitida la descarga de mapas.
  ```
- Protected tokens: None identified in this item.

### ITEM 195

- Source context: L539 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Las indicaciones sin conexión para transporte público, recorridos a pie y bicicleta no están disponibles.
  ```
- Protected tokens: None identified in this item.

### ITEM 196

- Source context: L540 - `html > body.maps-page > main > article.maps-guide > div.container > section#google-maps-role.maps-section:nth-of-type(6) > ul:nth-of-type(2) > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Los datos móviles siguen siendo importantes para la navegación en tiempo real y los cambios de ruta.
  ```
- Protected tokens: None identified in this item.

### ITEM 197

- Source context: L547 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > div.maps-section__heading > div > h2#travel-scenarios-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Usa el mapa adecuado en situaciones reales de viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 198

- Source context: L548 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Empieza por la información que puede cambiar la ruta: terminal, sucursal, salida, sentido de la parada u horario de funcionamiento.
  ```
- Protected tokens: None identified in this item.

### ITEM 199

- Source context: L552 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  De Incheon Airport a tu hotel Guarda la terminal exacta y la dirección en coreano del hotel. Compara el tren del aeropuerto, el autobús y el último tramo a pie. Usa la Guía de traslados del aeropuerto para comparar el trayecto completo. Consulta la Guía de Incheon Airport y la Guía de llegada si la terminal o los pasos de llegada afectan al plan.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 200

- Source context: L554 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  De Incheon Airport a tu hotel
  ```
- Protected tokens: `Incheon Airport`

### ITEM 201

- Source context: L556 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Guarda la terminal exacta y la dirección en coreano del hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 202

- Source context: L557 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Compara el tren del aeropuerto, el autobús y el último tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 203

- Source context: L558 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Usa la Guía de traslados del aeropuerto para comparar el trayecto completo.
  ```
- Protected tokens: None identified in this item.

### ITEM 204

- Source context: L559 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(1) > div > ul > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Consulta la Guía de Incheon Airport y la Guía de llegada si la terminal o los pasos de llegada afectan al plan.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 205

- Source context: L563 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Del hotel a un restaurante Busca el nombre del restaurante en coreano. Confirma la sucursal, la información de apertura y el último tramo a pie. Usa la Guía de alojamiento cuando el verdadero problema sea la ubicación del hotel o el acceso a la estación.
  ```
- Protected tokens: None identified in this item.

### ITEM 206

- Source context: L565 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Del hotel a un restaurante
  ```
- Protected tokens: None identified in this item.

### ITEM 207

- Source context: L567 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Busca el nombre del restaurante en coreano.
  ```
- Protected tokens: None identified in this item.

### ITEM 208

- Source context: L568 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Confirma la sucursal, la información de apertura y el último tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 209

- Source context: L569 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(2) > div > ul > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Usa la Guía de alojamiento cuando el verdadero problema sea la ubicación del hotel o el acceso a la estación.
  ```
- Protected tokens: None identified in this item.

### ITEM 210

- Source context: L573 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Encontrar la salida correcta del metro Comprueba la salida antes de bajar del tren. Revisa la dirección de la calle y si necesitas ascensor.
  ```
- Protected tokens: None identified in this item.

### ITEM 211

- Source context: L575 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Encontrar la salida correcta del metro
  ```
- Protected tokens: None identified in this item.

### ITEM 212

- Source context: L577 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba la salida antes de bajar del tren.
  ```
- Protected tokens: None identified in this item.

### ITEM 213

- Source context: L578 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(3) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Revisa la dirección de la calle y si necesitas ascensor.
  ```
- Protected tokens: None identified in this item.

### ITEM 214

- Source context: L582 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Tomar un autobús local Confirma el sentido de la parada, el número del autobús y el destino. Vigila la parada final y el tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 215

- Source context: L584 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Tomar un autobús local
  ```
- Protected tokens: None identified in this item.

### ITEM 216

- Source context: L586 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Confirma el sentido de la parada, el número del autobús y el destino.
  ```
- Protected tokens: None identified in this item.

### ITEM 217

- Source context: L587 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(4) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Vigila la parada final y el tramo a pie.
  ```
- Protected tokens: None identified in this item.

### ITEM 218

- Source context: L591 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Regreso nocturno Confirma la última opción de transporte público que puedas usar. Comprueba también un punto de recogida de taxi y ten preparada la Guía de taxis.
  ```
- Protected tokens: None identified in this item.

### ITEM 219

- Source context: L593 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Regreso nocturno
  ```
- Protected tokens: None identified in this item.

### ITEM 220

- Source context: L595 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Confirma la última opción de transporte público que puedas usar.
  ```
- Protected tokens: None identified in this item.

### ITEM 221

- Source context: L596 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(5) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba también un punto de recogida de taxi y ten preparada la Guía de taxis.
  ```
- Protected tokens: None identified in this item.

### ITEM 222

- Source context: L600 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Compartir un punto de encuentro Comparte la ficha exacta del lugar y su dirección o nombre en coreano. No envíes solo el nombre de un barrio.
  ```
- Protected tokens: None identified in this item.

### ITEM 223

- Source context: L602 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Compartir un punto de encuentro
  ```
- Protected tokens: None identified in this item.

### ITEM 224

- Source context: L604 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > ul > li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Comparte la ficha exacta del lugar y su dirección o nombre en coreano.
  ```
- Protected tokens: None identified in this item.

### ITEM 225

- Source context: L605 - `html > body.maps-page > main > article.maps-guide > div.container > section#travel-scenarios.maps-section:nth-of-type(7) > ol.maps-scenario-rows > li:nth-of-type(6) > div > ul > li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  No envíes solo el nombre de un barrio.
  ```
- Protected tokens: None identified in this item.

### ITEM 226

- Source context: L615 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > div.maps-section__heading > div > h2#map-mistakes-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Errores habituales con los mapas que conviene evitar
  ```
- Protected tokens: None identified in this item.

### ITEM 227

- Source context: L616 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Pequeñas comprobaciones evitan los problemas más comunes de lugar equivocado y dirección incorrecta.
  ```
- Protected tokens: None identified in this item.

### ITEM 228

- Source context: L620 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(1) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Buscar solo en inglés
  ```
- Protected tokens: None identified in this item.

### ITEM 229

- Source context: L620 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Ten preparados el nombre en coreano, la dirección vial y el número de teléfono.
  ```
- Protected tokens: None identified in this item.

### ITEM 230

- Source context: L621 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(2) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Elegir la sucursal equivocada
  ```
- Protected tokens: None identified in this item.

### ITEM 231

- Source context: L621 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Haz coincidir el barrio, las fotos y la estación más cercana.
  ```
- Protected tokens: None identified in this item.

### ITEM 232

- Source context: L622 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(3) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Ignorar la salida del metro
  ```
- Protected tokens: None identified in this item.

### ITEM 233

- Source context: L622 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Confirma la salida antes de bajar del tren.
  ```
- Protected tokens: None identified in this item.

### ITEM 234

- Source context: L623 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(4) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Usar la hora de llegada prevista en vez de la hora real de salida
  ```
- Protected tokens: None identified in this item.

### ITEM 235

- Source context: L623 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Configura la ruta para la hora a la que realmente vas a salir.
  ```
- Protected tokens: None identified in this item.

### ITEM 236

- Source context: L624 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(5) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Comparar solo el tiempo total de viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 237

- Source context: L624 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Comprueba también los transbordos, la espera y la fiabilidad.
  ```
- Protected tokens: None identified in this item.

### ITEM 238

- Source context: L625 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(6) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Ignorar el último tramo a pie con equipaje
  ```
- Protected tokens: None identified in this item.

### ITEM 239

- Source context: L625 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(6) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Un trayecto corto en tren aún puede terminar con una caminata complicada.
  ```
- Protected tokens: None identified in this item.

### ITEM 240

- Source context: L626 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(7) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Elegir el lado equivocado de una parada de autobús
  ```
- Protected tokens: None identified in this item.

### ITEM 241

- Source context: L626 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(7) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Verifica el sentido o el número de la parada antes de subir.
  ```
- Protected tokens: None identified in this item.

### ITEM 242

- Source context: L627 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(8) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Depender de una sola app sin respaldo
  ```
- Protected tokens: None identified in this item.

### ITEM 243

- Source context: L627 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(8) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Usa KakaoMap u otra ficha cuando un resultado no esté claro.
  ```
- Protected tokens: `KakaoMap`

### ITEM 244

- Source context: L628 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(9) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Salir del aeropuerto sin datos móviles
  ```
- Protected tokens: None identified in this item.

### ITEM 245

- Source context: L628 - `html > body.maps-page > main > article.maps-guide > div.container > section#map-mistakes.maps-section:nth-of-type(8) > ul.maps-mistake-list > li:nth-of-type(9) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Activa tu plan de eSIM, SIM o roaming antes de depender de la navegación en tiempo real.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 246

- Source context: L635 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-section__heading:nth-of-type(1) > div > h2#download-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Descarga las apps de mapas antes del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 247

- Source context: L636 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-section__heading:nth-of-type(1) > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Usa solo la ficha oficial de la tienda o el sitio web oficial del servicio.
  ```
- Protected tokens: None identified in this item.

### ITEM 248

- Source context: L642 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div:nth-of-type(1) > h3#download-naver-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 249

- Source context: L643 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Navegación principal
  ```
- Protected tokens: None identified in this item.

### ITEM 250

- Source context: L646 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 251

- Source context: L647 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 252

- Source context: L648 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(1) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Sitio web oficial
  ```
- Protected tokens: None identified in this item.

### ITEM 253

- Source context: L653 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div:nth-of-type(1) > h3#download-kakao-title`
- Element/type: H3 heading
- Spanish:

  ```text
  KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 254

- Source context: L654 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Mapa local de respaldo
  ```
- Protected tokens: None identified in this item.

### ITEM 255

- Source context: L657 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 256

- Source context: L658 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 257

- Source context: L659 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(2) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Sitio web oficial
  ```
- Protected tokens: None identified in this item.

### ITEM 258

- Source context: L664 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div:nth-of-type(1) > h3#download-google-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Google Maps
  ```
- Protected tokens: `Google Maps`, `Google`

### ITEM 259

- Source context: L665 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Planificación y lugares guardados
  ```
- Protected tokens: None identified in this item.

### ITEM 260

- Source context: L668 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  App Store
  ```
- Protected tokens: `App Store`

### ITEM 261

- Source context: L669 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Google Play
  ```
- Protected tokens: `Google Play`, `Google`

### ITEM 262

- Source context: L670 - `html > body.maps-page > main > article.maps-guide > div.container > section#download-apps.maps-section:nth-of-type(9) > div.maps-download-rows:nth-of-type(2) > section:nth-of-type(3) > div.maps-link-group:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Sitio web oficial
  ```
- Protected tokens: None identified in this item.

### ITEM 263

- Source context: L679 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > div.maps-section__heading > div > h2#faq-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Preguntas frecuentes
  ```
- Protected tokens: None identified in this item.

### ITEM 264

- Source context: L680 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  Respuestas breves a los problemas con mapas que los viajeros necesitan resolver con más frecuencia.
  ```
- Protected tokens: None identified in this item.

### ITEM 265

- Source context: L684 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(1) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de mapas para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 266

- Source context: L685 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(1) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Naver Map es la mejor primera opción para la mayoría de los viajeros porque combina la búsqueda de lugares locales con rutas a pie, en metro y en autobús. Mantén KakaoMap como respaldo y usa Google Maps para planificar, guardar lugares y consultar reseñas internacionales.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`, `Google Maps`, `Google`

### ITEM 267

- Source context: L688 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(2) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Naver Map está disponible en inglés?
  ```
- Protected tokens: `Naver Map`

### ITEM 268

- Source context: L689 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(2) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sí. En la app, abre MY y después Settings, Language y English. En iPhone, la app puede dirigirte a Settings, NAVER Map y Preferred Language.
  ```
- Protected tokens: `iPhone`, `NAVER Map`, `NAVER`, `Preferred Language`

### ITEM 269

- Source context: L692 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(3) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Google Maps funciona en Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 270

- Source context: L693 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(3) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sí. Resulta útil para lugares guardados, listas compartidas, reseñas internacionales y la planificación general del viaje. La disponibilidad de rutas y navegación puede variar, así que confirma la ruta exacta en tu dispositivo y ten preparada una app de mapas local.
  ```
- Protected tokens: None identified in this item.

### ITEM 271

- Source context: L696 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(4) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿También debería instalar KakaoMap?
  ```
- Protected tokens: `KakaoMap`

### ITEM 272

- Source context: L697 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(4) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Es un respaldo útil, no un requisito para todos los viajeros. Instálala si quieres comprobar un lugar con otra fuente, comparar una ruta a pie, verificar información de autobuses o abrir un enlace de un lugar de Kakao.
  ```
- Protected tokens: `Kakao`

### ITEM 273

- Source context: L700 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(5) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué hago si falla la búsqueda en inglés?
  ```
- Protected tokens: None identified in this item.

### ITEM 274

- Source context: L701 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(5) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Busca el nombre oficial en coreano del lugar o la dirección vial en tu reserva, en la web del negocio o en otra fuente fiable. Pégalo en Naver Map y confirma después la dirección, el número de teléfono y las fotos.
  ```
- Protected tokens: `Naver Map`

### ITEM 275

- Source context: L704 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(6) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo encuentro la salida correcta del metro?
  ```
- Protected tokens: None identified in this item.

### ITEM 276

- Source context: L705 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(6) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Abre la ruta antes de bajar del tren y confirma el número de salida, el lado de la calle y la distancia restante a pie. En una estación grande, comprueba también si necesitas ascensor y cuánto hay que caminar por el interior.
  ```
- Protected tokens: None identified in this item.

### ITEM 277

- Source context: L708 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(7) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo evito elegir la parada de autobús equivocada?
  ```
- Protected tokens: None identified in this item.

### ITEM 278

- Source context: L709 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(7) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Comprueba el número o la dirección de la parada, el destino del autobús y el lado de la calle antes de subir. Paradas con nombres parecidos pueden corresponder a sentidos de viaje opuestos.
  ```
- Protected tokens: None identified in this item.

### ITEM 279

- Source context: L712 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(8) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo usar mapas sin conexión en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 280

- Source context: L713 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(8) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Google indica que las descargas de mapas sin conexión no están disponibles en algunos países o regiones y que las indicaciones sin conexión para transporte público, recorridos a pie y bicicleta no están disponibles. Comprueba la disponibilidad de descarga en tu dispositivo y conserva datos móviles para la navegación en tiempo real.
  ```
- Protected tokens: `Google`

### ITEM 281

- Source context: L716 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(9) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo guardo y comparto la dirección de mi hotel?
  ```
- Protected tokens: None identified in this item.

### ITEM 282

- Source context: L717 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(9) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Guarda la ficha exacta del hotel en Naver Map, copia su nombre en coreano y la dirección vial, y comparte el enlace del lugar con tus compañeros de viaje. Conserva también ese texto en la confirmación de la reserva.
  ```
- Protected tokens: `Naver Map`

### ITEM 283

- Source context: L720 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(10) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Necesito datos móviles para usar apps de mapas en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 284

- Source context: L721 - `html > body.maps-page > main > article.maps-guide > div.container > section#faq.maps-section.maps-faq:nth-of-type(10) > details:nth-of-type(10) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Se recomiendan mucho los datos móviles para cambios de ruta, información del transporte en tiempo real, detalles de lugares y compartir ubicaciones. Prepara un plan de eSIM, SIM o roaming antes de depender de la navegación en directo.
  ```
- Protected tokens: `eSIM`, `SIM`

### ITEM 285

- Source context: L728 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > div.maps-section__heading > div > h2#sources-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Fuentes oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 286

- Source context: L729 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > div.maps-section__heading > div > p`
- Element/type: Body text
- Spanish:

  ```text
  La documentación de los proveedores respalda las instrucciones de producto que aparecen a continuación. El orden de apps y los consejos de respaldo de Korea Inside son recomendaciones editoriales.
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 287

- Source context: L733 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(1) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  NAVER Map Help: ajustes de idioma de la app
  ```
- Protected tokens: `NAVER Map Help`, `NAVER Map`, `NAVER`

### ITEM 288

- Source context: L733 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Rutas oficiales en Android y iPhone para cambiar el idioma de la app.
  ```
- Protected tokens: `Android`, `iPhone`

### ITEM 289

- Source context: L734 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(2) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Google Maps Help: obtener indicaciones y mostrar rutas
  ```
- Protected tokens: `Google Maps Help`, `Google Maps`, `Google`

### ITEM 290

- Source context: L734 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Guía oficial sobre modos de ruta y disponibilidad de funciones.
  ```
- Protected tokens: None identified in this item.

### ITEM 291

- Source context: L735 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(3) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Google Maps Help: descargar zonas y navegar sin conexión
  ```
- Protected tokens: `Google Maps Help`, `Google Maps`, `Google`

### ITEM 292

- Source context: L735 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Límites oficiales de descarga y modos de ruta no disponibles sin conexión.
  ```
- Protected tokens: None identified in this item.

### ITEM 293

- Source context: L736 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(4) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Republic of Korea policy briefing: aprobación de Google para exportar mapas a escala 1:5,000
  ```
- Protected tokens: `Republic of Korea policy briefing`, `Republic of Korea`, `Korea`, `Google`, `1:5,000`

### ITEM 294

- Source context: L736 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Decisión del Gobierno y condiciones de seguridad aplicables antes de exportar los datos.
  ```
- Protected tokens: None identified in this item.

### ITEM 295

- Source context: L737 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(5) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Kakao Corp: documentación oficial de funciones de KakaoMap
  ```
- Protected tokens: `Kakao Corp`, `Kakao`, `KakaoMap`

### ITEM 296

- Source context: L737 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Resumen oficial de funciones de rutas, navegación y transporte público.
  ```
- Protected tokens: None identified in this item.

### ITEM 297

- Source context: L739 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-group-title:nth-of-type(1) :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Páginas oficiales de las tiendas de apps
  ```
- Protected tokens: None identified in this item.

### ITEM 298

- Source context: L741 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Naver Map para iPhone
  ```
- Protected tokens: `Naver Map`, `iPhone`

### ITEM 299

- Source context: L742 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Naver Map para Android
  ```
- Protected tokens: `Naver Map`, `Android`

### ITEM 300

- Source context: L743 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  KakaoMap para iPhone
  ```
- Protected tokens: `KakaoMap`, `iPhone`

### ITEM 301

- Source context: L744 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(4)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  KakaoMap para Android
  ```
- Protected tokens: `KakaoMap`, `Android`

### ITEM 302

- Source context: L745 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(5)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Google Maps para iPhone
  ```
- Protected tokens: `Google Maps`, `Google`, `iPhone`

### ITEM 303

- Source context: L746 - `html > body.maps-page > main > article.maps-guide > div.container > section#sources.maps-section.maps-sources:nth-of-type(11) > ul > li:nth-of-type(6) > span.maps-source-links:nth-of-type(2) > a:nth-of-type(6)`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Google Maps para Android
  ```
- Protected tokens: `Google Maps`, `Google`, `Android`

# PAGE: `apps.html`

**English source SHA-256:** `b80f02a398c9b562baa3bfc8977961c25fe4081f92f6262ed3945fcbd185b734`
**Localized ITEM count:** 246

### ITEM 304

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Las apps esenciales para viajar por Korea, incluidas Naver Map, Papago, k.ride, reservas de restaurantes, entrega de comida, pagos, trenes y ayuda de emergencia para visitantes extranjeros.
  ```
- Protected tokens: `Korea`, `Naver Map`, `Papago`, `k.ride`

### ITEM 305

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Mejores apps para viajar por Korea: mapas, taxis y traducción | Korea Inside
  ```
- Protected tokens: `Korea`, `Korea Inside`

### ITEM 306

- Source context: L12 - `html > head > meta[property="og:title"] @content`
- Element/type: Open Graph title
- Spanish:

  ```text
  Mejores apps para viajar por Korea: mapas, taxis y traducción
  ```
- Protected tokens: `Korea`

### ITEM 307

- Source context: L13 - `html > head > meta[property="og:description"] @content`
- Element/type: Open Graph description
- Spanish:

  ```text
  Elige las apps esenciales para viajar por Korea según necesites mapas, traducción, taxis, restaurantes, entrega de comida, pagos, trenes y ayuda de emergencia.
  ```
- Protected tokens: `Korea`

### ITEM 308

- Source context: L17 - `html > head > meta[name="twitter:title"] @content`
- Element/type: Twitter card title
- Spanish:

  ```text
  Mejores apps para viajar por Korea: mapas, taxis y traducción
  ```
- Protected tokens: `Korea`

### ITEM 309

- Source context: L18 - `html > head > meta[name="twitter:description"] @content`
- Element/type: Twitter card description
- Spanish:

  ```text
  Elige las apps esenciales para viajar por Korea según necesites mapas, traducción, taxis, restaurantes, entrega de comida, pagos, trenes y ayuda de emergencia.
  ```
- Protected tokens: `Korea`

### ITEM 310

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Mejores apps para viajar por Korea: mapas, taxis y traducción | Korea Inside
  ```
- Protected tokens: `Korea`, `Korea Inside`

### ITEM 311

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].description`
- Element/type: JSON-LD user-facing description
- Spanish:

  ```text
  Las apps esenciales para viajar por Korea, incluidas Naver Map, Papago, k.ride, reservas de restaurantes, entrega de comida, pagos, trenes y ayuda de emergencia para visitantes extranjeros.
  ```
- Protected tokens: `Korea`, `Naver Map`, `Papago`, `k.ride`

### ITEM 312

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[0].isPartOf.name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Korea Inside
  ```
- Protected tokens: `Korea Inside`, `Korea`

### ITEM 313

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[0].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Inicio
  ```
- Protected tokens: None identified in this item.

### ITEM 314

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[1].itemListElement[1].name`
- Element/type: JSON-LD user-facing name
- Spanish:

  ```text
  Apps
  ```
- Protected tokens: None identified in this item.

### ITEM 315

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué apps debería instalar antes de viajar a Korea?
  ```
- Protected tokens: `Korea`

### ITEM 316

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[0].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  La mayoría de quienes visitan Korea por primera vez deberían instalar Naver Map, Papago y k.ride antes del viaje. Añade Catchtable, Shuttle Delivery, KakaoTalk y apps de pagos o trenes solo cuando tus planes las necesiten.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`, `Korea`, `Catchtable`, `Shuttle Delivery`, `Shuttle`, `KakaoTalk`

### ITEM 317

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de mapas para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 318

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[1].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Naver Map es la mejor app de mapas predeterminada para la mayoría de los visitantes porque ofrece búsquedas locales detalladas y rutas en metro, autobús, a pie y en coche. Mantén KakaoMap como comprobación opcional.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 319

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Google Maps funciona en Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 320

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[2].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Google Maps funciona para guardar lugares y orientarse de forma básica, pero la búsqueda local y los detalles de las rutas pueden ser menos fiables en Korea. Usa Naver Map como app principal de navegación y comprueba el nombre o la dirección en coreano cuando falle la búsqueda.
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`, `Naver Map`

### ITEM 321

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de traducción para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 322

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[3].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Papago es la mejor primera app de traducción para muchos viajes por Korea. Admite traducción de texto, imagen, voz, conversación y sin conexión, aunque los nombres, el argot y el contexto de los menús siguen requiriendo criterio.
  ```
- Protected tokens: `Papago`, `Korea`

### ITEM 323

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué app de taxi es más fácil para visitantes extranjeros?
  ```
- Protected tokens: None identified in this item.

### ITEM 324

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[4].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  k.ride es la opción predeterminada más sencilla para muchos visitantes extranjeros porque está diseñada para viajeros internacionales y admite búsqueda multilingüe de destinos, chat traducido con el conductor y registro de tarjetas emitidas en el extranjero. Kakao T y Uber Taxi siguen siendo alternativas útiles.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 325

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Necesito KakaoTalk como turista?
  ```
- Protected tokens: `KakaoTalk`

### ITEM 326

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[5].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Solo necesitas KakaoTalk si esperas comunicarte con amigos coreanos, anfitriones, operadores turísticos o negocios locales. Los viajeros sin contactos locales normalmente pueden prescindir de ella.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 327

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué app debería usar para reservar restaurantes?
  ```
- Protected tokens: None identified in this item.

### ITEM 328

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[6].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Usa Catchtable cuando un restaurante acepte reservas o registro en lista de espera mediante el servicio. Confirma la sucursal, la fecha, el número de personas, el depósito y las condiciones de cancelación antes de reservar.
  ```
- Protected tokens: `Catchtable`

### ITEM 329

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Los turistas pueden pedir comida a domicilio en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 330

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[7].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Los turistas pueden pedir comida a domicilio en algunas zonas. Shuttle Delivery es la primera opción más sencilla para muchos visitantes extranjeros porque admite usuarios y tarjetas internacionales, aunque la cobertura varía; la ayuda del hotel, la recogida y acudir directamente al restaurante son alternativas fiables.
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 331

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Las apps coreanas exigen un número de teléfono coreano?
  ```
- Protected tokens: None identified in this item.

### ITEM 332

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[8].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Algunas apps coreanas funcionan con un número extranjero, mientras que otras exigen SMS coreano, verificación de identidad o de pago para determinadas funciones. Prueba el inicio de sesión y la recuperación antes de salir y nunca des por hecho que una eSIM solo de datos incluye un número de teléfono coreano.
  ```
- Protected tokens: `SMS`, `eSIM`

### ITEM 333

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué apps debería configurar antes de mi vuelo?
  ```
- Protected tokens: None identified in this item.

### ITEM 334

- Source context: L22 - `script[type="application/ld+json"] #1 > $.@graph[2].mainEntity[9].acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Antes del vuelo, configura Naver Map, Papago y k.ride y prueba después el inicio de sesión, el idioma, los permisos y el pago. Guarda el nombre en coreano de tu hotel, la dirección vial y el número de teléfono, además de capturas de pantalla de las reservas importantes.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 335

- Source context: L217 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__copy:nth-of-type(1) > h1#apps-title`
- Element/type: H1 heading
- Spanish:

  ```text
  Mejores apps para viajar por Korea: qué instalar antes de llegar
  ```
- Protected tokens: `Korea`

### ITEM 336

- Source context: L218 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__copy:nth-of-type(1) > p.apps-hero__intro`
- Element/type: Hero / lead copy
- Spanish:

  ```text
  Para un viaje corto no necesitas llenar el teléfono de apps coreanas. Naver Map cubre la mayor parte de la navegación local, Papago ayuda cuando el texto o el habla en coreano se convierten en un obstáculo y k.ride cubre los momentos en que un taxi resulta más fácil que el transporte público.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 337

- Source context: L220 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__answer:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Apps que conviene instalar antes de aterrizar
  ```
- Protected tokens: None identified in this item.

### ITEM 338

- Source context: L221 - `html > body > main > section.apps-hero > div.container > div.apps-hero__grid > div.apps-hero__answer:nth-of-type(2) > p`
- Element/type: Hero / lead copy
- Spanish:

  ```text
  KakaoTalk importa cuando un contacto local espera usarla. Las apps de restaurantes, entrega de comida, pagos, trenes y seguridad pueden esperar hasta que el itinerario les dé una función concreta.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 339

- Source context: L231 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > header.apps-section__heading > h2#apps-at-a-glance-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Apps que conviene conocer antes de viajar a Korea
  ```
- Protected tokens: `Korea`

### ITEM 340

- Source context: L232 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  “Instalar antes de llegar” significa completar la configuración básica y probar la función que piensas usar, no simplemente descargar la app.
  ```
- Protected tokens: None identified in this item.

### ITEM 341

- Source context: L234 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Comparación de apps para viajar por Korea
  ```
- Protected tokens: `Korea`

### ITEM 342

- Source context: L238 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(1)`
- Element/type: Table header
- Spanish:

  ```text
  App
  ```
- Protected tokens: None identified in this item.

### ITEM 343

- Source context: L239 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(2)`
- Element/type: Table header
- Spanish:

  ```text
  Para qué sirve
  ```
- Protected tokens: None identified in this item.

### ITEM 344

- Source context: L240 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(3)`
- Element/type: Table header
- Spanish:

  ```text
  Cuándo importa la configuración
  ```
- Protected tokens: None identified in this item.

### ITEM 345

- Source context: L241 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > thead > tr > th:nth-of-type(4)`
- Element/type: Table header
- Spanish:

  ```text
  Qué puede complicarlo
  ```
- Protected tokens: None identified in this item.

### ITEM 346

- Source context: L246 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > th`
- Element/type: Table header
- Spanish:

  ```text
  Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 347

- Source context: L247 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Búsqueda local y rutas
  ```
- Protected tokens: None identified in this item.

### ITEM 348

- Source context: L248 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 349

- Source context: L249 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(1) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Las búsquedas en inglés pueden no encontrar un lugar que figura con su nombre en coreano.
  ```
- Protected tokens: None identified in this item.

### ITEM 350

- Source context: L252 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > th`
- Element/type: Table header
- Spanish:

  ```text
  Papago
  ```
- Protected tokens: `Papago`

### ITEM 351

- Source context: L253 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Menús, carteles y conversaciones breves
  ```
- Protected tokens: None identified in this item.

### ITEM 352

- Source context: L254 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 353

- Source context: L255 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(2) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  La traducción automática puede equivocarse con nombres, argot y contexto.
  ```
- Protected tokens: None identified in this item.

### ITEM 354

- Source context: L258 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > th`
- Element/type: Table header
- Spanish:

  ```text
  k.ride
  ```
- Protected tokens: `k.ride`

### ITEM 355

- Source context: L259 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Pedir taxis para visitantes internacionales
  ```
- Protected tokens: None identified in this item.

### ITEM 356

- Source context: L260 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 357

- Source context: L261 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(3) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  La disponibilidad de vehículos y el éxito del pago pueden variar según el lugar y la hora.
  ```
- Protected tokens: None identified in this item.

### ITEM 358

- Source context: L264 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > th`
- Element/type: Table header
- Spanish:

  ```text
  KakaoTalk
  ```
- Protected tokens: `KakaoTalk`

### ITEM 359

- Source context: L265 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Mensajería con contactos coreanos
  ```
- Protected tokens: None identified in this item.

### ITEM 360

- Source context: L266 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Solo si la necesitas
  ```
- Protected tokens: None identified in this item.

### ITEM 361

- Source context: L267 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(4) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Conviene probar con antelación la verificación del teléfono y la recuperación de la cuenta.
  ```
- Protected tokens: None identified in this item.

### ITEM 362

- Source context: L270 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > th`
- Element/type: Table header
- Spanish:

  ```text
  Catchtable
  ```
- Protected tokens: `Catchtable`

### ITEM 363

- Source context: L271 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Reservas de restaurantes y listas de espera
  ```
- Protected tokens: None identified in this item.

### ITEM 364

- Source context: L272 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Si comer bien es una prioridad
  ```
- Protected tokens: None identified in this item.

### ITEM 365

- Source context: L273 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(5) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  No todos los restaurantes ni todos los horarios están disponibles.
  ```
- Protected tokens: None identified in this item.

### ITEM 366

- Source context: L276 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > th`
- Element/type: Table header
- Spanish:

  ```text
  Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 367

- Source context: L277 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Entrega de comida fácil para extranjeros
  ```
- Protected tokens: None identified in this item.

### ITEM 368

- Source context: L278 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Si piensas hacer pedidos
  ```
- Protected tokens: None identified in this item.

### ITEM 369

- Source context: L279 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(6) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  La selección de restaurantes y la cobertura de entrega dependen de la ubicación.
  ```
- Protected tokens: None identified in this item.

### ITEM 370

- Source context: L282 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > th`
- Element/type: Table header
- Spanish:

  ```text
  WOWPASS
  ```
- Protected tokens: `WOWPASS`

### ITEM 371

- Source context: L283 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Respaldo de pago prepago y gestión de la tarjeta
  ```
- Protected tokens: None identified in this item.

### ITEM 372

- Source context: L284 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Si encaja con tu plan de pagos
  ```
- Protected tokens: None identified in this item.

### ITEM 373

- Source context: L285 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(7) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Los saldos de pago y de T-money son independientes.
  ```
- Protected tokens: `T-money`

### ITEM 374

- Source context: L288 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > th`
- Element/type: Table header
- Spanish:

  ```text
  Mobile Tmoney
  ```
- Protected tokens: `Mobile Tmoney`, `Tmoney`

### ITEM 375

- Source context: L289 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Pago del transporte con el teléfono
  ```
- Protected tokens: None identified in this item.

### ITEM 376

- Source context: L290 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Solo después de comprobar la compatibilidad
  ```
- Protected tokens: None identified in this item.

### ITEM 377

- Source context: L291 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(8) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Los requisitos de teléfono, monedero, NFC y recarga varían según el dispositivo.
  ```
- Protected tokens: `NFC`

### ITEM 378

- Source context: L294 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > th`
- Element/type: Table header
- Spanish:

  ```text
  Emergency Ready
  ```
- Protected tokens: `Emergency Ready`

### ITEM 379

- Source context: L295 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Alertas oficiales e información de seguridad cercana
  ```
- Protected tokens: None identified in this item.

### ITEM 380

- Source context: L296 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Recomendado
  ```
- Protected tokens: None identified in this item.

### ITEM 381

- Source context: L297 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(9) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Para aprovecharla por completo se necesitan permisos de ubicación y notificaciones.
  ```
- Protected tokens: None identified in this item.

### ITEM 382

- Source context: L300 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > th`
- Element/type: Table header
- Spanish:

  ```text
  VisitKorea
  ```
- Protected tokens: `VisitKorea`

### ITEM 383

- Source context: L301 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Información oficial de viaje y planificación
  ```
- Protected tokens: None identified in this item.

### ITEM 384

- Source context: L302 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Opcional
  ```
- Protected tokens: None identified in this item.

### ITEM 385

- Source context: L303 - `html > body > main > article.apps-guide > section#apps-at-a-glance.apps-section:nth-of-type(1) > div.container > div.apps-table-scroll > table.apps-table > tbody > tr:nth-of-type(10) > td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Complementa una app de mapas específica, no la sustituye.
  ```
- Protected tokens: None identified in this item.

### ITEM 386

- Source context: L314 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > header.apps-section__heading > h2#naver-map-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Cuando una búsqueda en inglés no encuentra el lugar
  ```
- Protected tokens: None identified in this item.

### ITEM 387

- Source context: L315 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Naver Map es el punto de partida práctico para rutas y fichas locales. Su mapa e interfaz admiten coreano, inglés, japonés y chino, pero el nombre o la dirección exactos en coreano suelen importar más que probar otra grafía en inglés.
  ```
- Protected tokens: `Naver Map`

### ITEM 388

- Source context: L319 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Las rutas y los detalles locales están en el mismo lugar
  ```
- Protected tokens: None identified in this item.

### ITEM 389

- Source context: L320 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Las rutas de metro, autobús, a pie y en coche aparecen junto a horarios comerciales, accesos y sucursales cercanas. Los lugares importantes y favoritos son más fáciles de recuperar si se guardan antes de dejar una Wi-Fi fiable.
  ```
- Protected tokens: `Wi-Fi`

### ITEM 390

- Source context: L323 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Una búsqueda fallida necesita un texto de origen mejor
  ```
- Protected tokens: None identified in this item.

### ITEM 391

- Source context: L324 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Que no aparezca ningún resultado en inglés no significa que el lugar no exista. El nombre exacto en coreano, la dirección vial o el número de teléfono de una reserva pueden recuperar la ficha; después, el pin y las fotos de la fachada ayudan a distinguir una sucursal de otra.
  ```
- Protected tokens: None identified in this item.

### ITEM 392

- Source context: L327 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  KakaoMap es una segunda referencia local
  ```
- Protected tokens: `KakaoMap`

### ITEM 393

- Source context: L328 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Spanish:

  ```text
  KakaoMap puede ayudar cuando un pin, una sucursal o una ruta a pie no están claros. El mismo nombre o dirección vial en coreano funciona mejor que cambiar repetidamente la grafía en inglés, y el número de teléfono y las fotos de la fachada ofrecen una comprobación final.
  ```
- Protected tokens: `KakaoMap`

### ITEM 394

- Source context: L331 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Recuperación de búsqueda: nombre del lugar en inglés → nombre del lugar en coreano → dirección vial en coreano → número de teléfono. Para modos de ruta y tácticas de mapas más detalladas, usa la guía de mapas de Korea.
  ```
- Protected tokens: `Korea`

### ITEM 395

- Source context: L332 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Descargas de Naver Map
  ```
- Protected tokens: `Naver Map`

### ITEM 396

- Source context: L333 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Naver Map en App Store
  ```
- Protected tokens: `Naver Map`, `App Store`

### ITEM 397

- Source context: L334 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Naver Map en Google Play
  ```
- Protected tokens: `Naver Map`, `Google Play`, `Google`

### ITEM 398

- Source context: L336 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(3) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Página oficial de KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 399

- Source context: L337 - `html > body > main > article.apps-guide > section#naver-map.apps-section.apps-section--soft:nth-of-type(2) > div.container > p.apps-download-links:nth-of-type(3) > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Página oficial del servicio KakaoMap
  ```
- Protected tokens: `KakaoMap`

### ITEM 400

- Source context: L345 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > header.apps-section__heading > h2#papago-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Menús, carteles e intercambios breves necesitan modos de traducción distintos
  ```
- Protected tokens: None identified in this item.

### ITEM 401

- Source context: L346 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Papago admite traducción de texto, imagen, voz, conversación y sin conexión, así que lo útil es elegir el método de entrada que mejor encaje con lo que tienes delante.
  ```
- Protected tokens: `Papago`

### ITEM 402

- Source context: L350 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  La cámara resuelve el texto que no puedes copiar
  ```
- Protected tokens: None identified in this item.

### ITEM 403

- Source context: L351 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  La traducción por imagen puede leer el menú de un restaurante, la etiqueta de un medicamento, un quiosco, un aviso o una captura de pantalla. El modo de voz o conversación resulta más natural en un intercambio breve en el que escribir ralentizaría a ambas personas.
  ```
- Protected tokens: None identified in this item.

### ITEM 404

- Source context: L354 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  La compatibilidad sin conexión no es idéntica en todas las funciones
  ```
- Protected tokens: None identified in this item.

### ITEM 405

- Source context: L355 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Los datos de idioma necesarios deben descargarse y probarse antes de salir. La disponibilidad sin conexión varía según el idioma y la función, y las direcciones importantes y los datos de las reservas son más seguros si se guardan por separado.
  ```
- Protected tokens: None identified in this item.

### ITEM 406

- Source context: L358 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La traducción sigue requiriendo criterio. Los nombres, el argot, los términos alimentarios y el significado específico de un menú pueden ser incorrectos aunque la frase suene fluida. Cuando la precisión importe, muestra el coreano original junto con la traducción.
  ```
- Protected tokens: None identified in this item.

### ITEM 407

- Source context: L359 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Descargas de Papago
  ```
- Protected tokens: `Papago`

### ITEM 408

- Source context: L360 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Papago en App Store
  ```
- Protected tokens: `Papago`, `App Store`

### ITEM 409

- Source context: L361 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Papago en Google Play
  ```
- Protected tokens: `Papago`, `Google Play`, `Google`

### ITEM 410

- Source context: L362 - `html > body > main > article.apps-guide > section#papago.apps-section:nth-of-type(3) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Papago en la web
  ```
- Protected tokens: `Papago`

### ITEM 411

- Source context: L370 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > header.apps-section__heading > h2#taxi-apps-title`
- Element/type: H2 heading
- Spanish:

  ```text
  La app de taxi tiene que funcionar antes del trayecto nocturno
  ```
- Protected tokens: None identified in this item.

### ITEM 412

- Source context: L371 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  k.ride se creó para viajeros internacionales, mientras que Kakao T y Uber Taxi son más relevantes cuando ya tienes una cuenta y una de ellas te resulta familiar.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 413

- Source context: L375 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  k.ride elimina varias barreras para visitantes
  ```
- Protected tokens: `k.ride`

### ITEM 414

- Source context: L376 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Kakao Mobility creó k.ride para viajeros internacionales. Admite una interfaz multilingüe, búsqueda de destinos y traducción del chat con el conductor en muchos idiomas, además del registro de tarjetas emitidas en el extranjero. Prueba el inicio de sesión y el pago antes del viaje.
  ```
- Protected tokens: `Kakao Mobility`, `Kakao`, `k.ride`

### ITEM 415

- Source context: L379 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Kakao T encaja con usuarios que ya usan Kakao
  ```
- Protected tokens: `Kakao T`, `Kakao`

### ITEM 416

- Source context: L380 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Es un servicio local sólido para quien ya se siente cómodo con el ecosistema de Kakao. Las condiciones de cuenta, idioma y pago pueden variar, así que no hagas que el primer uso dependa de un trayecto nocturno.
  ```
- Protected tokens: `Kakao`

### ITEM 417

- Source context: L383 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Uber Taxi es una alternativa familiar
  ```
- Protected tokens: `Uber Taxi`

### ITEM 418

- Source context: L384 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Spanish:

  ```text
  Puede ser cómodo si ya tienes una cuenta de Uber y prefieres su interfaz. Los tipos de vehículo disponibles, la asignación y las opciones de pago siguen variando según el lugar y la hora.
  ```
- Protected tokens: None identified in this item.

### ITEM 419

- Source context: L387 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Guarda tu destino en coreano y conserva un segundo método de pago incluso si ya has configurado el pago dentro de la app. Consulta la guía completa de taxis de Korea para tarifas, comprobaciones del punto de recogida y estrategia nocturna. Si prefieres un trayecto reservado de antemano después de un vuelo largo, compara las opciones en la guía de traslados del aeropuerto.
  ```
- Protected tokens: `Korea`

### ITEM 420

- Source context: L388 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Descargas de apps de taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 421

- Source context: L389 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  k.ride en App Store
  ```
- Protected tokens: `k.ride`, `App Store`

### ITEM 422

- Source context: L390 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  k.ride en Google Play
  ```
- Protected tokens: `k.ride`, `Google Play`, `Google`

### ITEM 423

- Source context: L391 - `html > body > main > article.apps-guide > section#taxi-apps.apps-section.apps-section--soft:nth-of-type(4) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Guía oficial de k.ride
  ```
- Protected tokens: `k.ride`

### ITEM 424

- Source context: L399 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > header.apps-section__heading > h2#kakaotalk-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Los contactos locales y las reservas de restaurantes dependen del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 425

- Source context: L400 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  KakaoTalk y Catchtable se vuelven relevantes por una persona, negocio o reserva concretos, no simplemente porque sean apps coreanas populares.
  ```
- Protected tokens: `KakaoTalk`, `Catchtable`

### ITEM 426

- Source context: L404 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  KakaoTalk depende del contacto
  ```
- Protected tokens: `KakaoTalk`

### ITEM 427

- Source context: L405 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Amigos coreanos, anfitriones de alojamientos, operadores turísticos y negocios locales pueden esperar comunicarse allí. Open Chat, Voice Talk y Face Talk pueden facilitar la coordinación cuando esos contactos los usan; un viaje corto e independiente sin contactos locales puede no necesitar nunca la app.
  ```
- Protected tokens: `Open Chat`, `Voice Talk`, `Face Talk`

### ITEM 428

- Source context: L408 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  El acceso a la cuenta tiene que sobrevivir al viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 429

- Source context: L409 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Conviene probar antes de salir la verificación por SMS y la recuperación de cuenta si KakaoTalk va a llevar una conversación importante. Perder el acceso en el extranjero puede importar más que cualquier función de la app.
  ```
- Protected tokens: `SMS`, `KakaoTalk`

### ITEM 430

- Source context: L412 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div#restaurant-reservations.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Catchtable depende del restaurante
  ```
- Protected tokens: `Catchtable`

### ITEM 431

- Source context: L413 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div#restaurant-reservations.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Spanish:

  ```text
  El servicio global permite descubrir locales, consultar reseñas, hacer reservas y apuntarse a listas de espera en establecimientos participantes. Nombres parecidos en inglés pueden corresponder a distritos o sucursales distintos, así que la dirección en coreano, el barrio y las fotos deben coincidir antes de confirmar.
  ```
- Protected tokens: None identified in this item.

### ITEM 432

- Source context: L416 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Las condiciones de reserva importan después de encontrar mesa
  ```
- Protected tokens: None identified in this item.

### ITEM 433

- Source context: L417 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > p`
- Element/type: Body text
- Spanish:

  ```text
  El número de personas, el depósito, las condiciones de cancelación, la hora de llegada y si la ficha ofrece una reserva o solo una lista de espera pueden cambiar el valor de un horario disponible.
  ```
- Protected tokens: None identified in this item.

### ITEM 434

- Source context: L420 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(1) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Página oficial de KakaoTalk
  ```
- Protected tokens: `KakaoTalk`

### ITEM 435

- Source context: L421 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(1) > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Página oficial del servicio KakaoTalk
  ```
- Protected tokens: `KakaoTalk`

### ITEM 436

- Source context: L423 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Página oficial de Catchtable
  ```
- Protected tokens: `Catchtable`

### ITEM 437

- Source context: L424 - `html > body > main > article.apps-guide > section#kakaotalk.apps-section:nth-of-type(5) > div.container > p.apps-download-links:nth-of-type(2) > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Abrir CATCHTABLE Global
  ```
- Protected tokens: `CATCHTABLE Global`, `CATCHTABLE`

### ITEM 438

- Source context: L432 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > header.apps-section__heading > h2#food-delivery-title`
- Element/type: H2 heading
- Spanish:

  ```text
  La entrega a domicilio es donde la configuración de apps se complica
  ```
- Protected tokens: None identified in this item.

### ITEM 439

- Source context: L433 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Lo difícil a menudo no es encontrar comida. La verificación del teléfono, el formato de la dirección de entrega, la aceptación del pago y la cobertura pueden detener un pedido después de haber elegido el menú.
  ```
- Protected tokens: None identified in this item.

### ITEM 440

- Source context: L437 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Shuttle reduce parte de la fricción para visitantes
  ```
- Protected tokens: `Shuttle`

### ITEM 441

- Source context: L438 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Shuttle admite usuarios internacionales, pedidos multilingües y métodos de pago extranjeros sin exigir un número de teléfono coreano para el registro estándar. Su selección de restaurantes y su cobertura son más limitadas que las de las mayores plataformas nacionales, por lo que la dirección real de entrega es la primera limitación práctica.
  ```
- Protected tokens: `Shuttle`

### ITEM 442

- Source context: L441 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Las plataformas nacionales exigen más a la cuenta
  ```
- Protected tokens: None identified in this item.

### ITEM 443

- Source context: L442 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  Baemin y Coupang Eats pueden tener más sentido durante una estancia larga o cuando resulta manejable introducir una dirección local, configurar la cuenta y resolver problemas de pago. Baemin ofrece soporte multilingüe en el flujo principal del pedido, pero eso no garantiza todos los menús, tarjetas o ubicaciones de entrega. Si la configuración falla, la ayuda del hotel, la recogida, acudir directamente al restaurante o una tienda de conveniencia evitan que el problema se apodere de la noche.
  ```
- Protected tokens: `Baemin`, `Coupang Eats`

### ITEM 444

- Source context: L445 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > p.apps-download-links @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Página oficial de Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 445

- Source context: L446 - `html > body > main > article.apps-guide > section#food-delivery.apps-section.apps-section--soft:nth-of-type(6) > div.container > p.apps-download-links > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Abrir Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 446

- Source context: L454 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > header.apps-section__heading > h2#payment-transit-apps-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Los pagos, los trenes y los servicios oficiales dependen del viaje
  ```
- Protected tokens: None identified in this item.

### ITEM 447

- Source context: L455 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Estos servicios resuelven problemas distintos. Solo tienen sentido en el teléfono cuando el método de pago, la ruta interurbana, el estilo de planificación o una necesidad de seguridad les dan una función clara.
  ```
- Protected tokens: None identified in this item.

### ITEM 448

- Source context: L459 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(1) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  WOWPASS gestiona una tarjeta con dos saldos
  ```
- Protected tokens: `WOWPASS`

### ITEM 449

- Source context: L460 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  La app gestiona la tarjeta de pago prepago, las transacciones y las funciones de seguridad. El saldo de pago y el saldo de transporte T-money integrado siguen siendo independientes, por lo que la guía de WOWPASS resulta útil antes de la primera recarga.
  ```
- Protected tokens: `T-money`, `WOWPASS`

### ITEM 450

- Source context: L463 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Mobile Tmoney depende del dispositivo y de la forma de recarga
  ```
- Protected tokens: `Mobile Tmoney`, `Tmoney`

### ITEM 451

- Source context: L464 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list:nth-of-type(1) > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  El pago del transporte con el móvil está disponible en teléfonos y monederos compatibles, incluidos dispositivos Apple compatibles y configuraciones Android. Importan el dispositivo, NFC, el monedero y las condiciones de recarga; si alguna parte no está clara, una tarjeta física ofrece menos fricción. Usa la guía de T-money para la configuración y el uso.
  ```
- Protected tokens: `Android`, `NFC`, `T-money`

### ITEM 452

- Source context: L467 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-context-note:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Una tarjeta de transporte y un servicio de pago más amplio cumplen funciones distintas. La guía de pagos de Korea explica dónde encaja cada uno sin exigir por defecto ambas apps.
  ```
- Protected tokens: `Korea`

### ITEM 453

- Source context: L468 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Páginas oficiales de apps de pago
  ```
- Protected tokens: None identified in this item.

### ITEM 454

- Source context: L469 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Sitio oficial de WOWPASS
  ```
- Protected tokens: `WOWPASS`

### ITEM 455

- Source context: L470 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(2) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Servicios móviles de Tmoney
  ```
- Protected tokens: `Tmoney`

### ITEM 456

- Source context: L474 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(2) > div#train-apps.apps-editorial-row > h3#train-apps-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Las reservas de KORAIL pueden hacerse desde el navegador
  ```
- Protected tokens: `KORAIL`

### ITEM 457

- Source context: L475 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(2) > div#train-apps.apps-editorial-row > p`
- Element/type: Body text
- Spanish:

  ```text
  El sitio de reservas en inglés permite a los visitantes extranjeros buscar y reservar KTX y otros trenes operados por KORAIL sin instalar una app. Una tarjeta inscrita en 3-D Secure, el nombre exacto del pasajero y una confirmación guardada importan más que KorailTalk para un trayecto ferroviario ocasional. KORAIL y SRT siguen siendo sistemas separados.
  ```
- Protected tokens: `KTX`, `KORAIL`, `3-D Secure`, `KorailTalk`, `SRT`

### ITEM 458

- Source context: L478 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(3) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Reserva oficial de KORAIL
  ```
- Protected tokens: `KORAIL`

### ITEM 459

- Source context: L479 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(3) > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Reserva en el sitio oficial de KORAIL
  ```
- Protected tokens: `KORAIL`

### ITEM 460

- Source context: L483 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(3) > div#visitkorea.apps-editorial-row > h3#visitkorea-title`
- Element/type: H3 heading
- Spanish:

  ```text
  VisitKorea es una referencia oficial, no otro mapa
  ```
- Protected tokens: `VisitKorea`

### ITEM 461

- Source context: L484 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(3) > div#visitkorea.apps-editorial-row > p`
- Element/type: Body text
- Spanish:

  ```text
  La plataforma Korea Tourism Organization cubre atracciones, comida, alojamiento, festivales, aspectos básicos del viaje, itinerarios sugeridos y apoyo para la planificación. La app añade planes guardados o notificaciones, mientras que el sitio web es suficiente cuando solo necesitas una referencia oficial.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 462

- Source context: L487 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(4) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Información oficial de la app VisitKorea
  ```
- Protected tokens: `VisitKorea`

### ITEM 463

- Source context: L488 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(4) > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Información de la app VisitKorea
  ```
- Protected tokens: `VisitKorea`

### ITEM 464

- Source context: L492 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(4) > div#emergency-ready.apps-editorial-row > h3#emergency-ready-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Emergency Ready mantiene cerca la ayuda oficial
  ```
- Protected tokens: `Emergency Ready`

### ITEM 465

- Source context: L493 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > div.apps-editorial-list.apps-editorial-list--continued:nth-of-type(4) > div#emergency-ready.apps-editorial-row > p`
- Element/type: Body text
- Spanish:

  ```text
  La app Ministry of the Interior and Safety ofrece alertas multilingües de desastres, refugios e instalaciones de emergencia cercanos, información de embajadas y orientación oficial de respuesta. El acceso a la ubicación y a las notificaciones permite que esas funciones trabajen cuando se necesitan.
  ```
- Protected tokens: `Ministry of the Interior and Safety`

### ITEM 466

- Source context: L496 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-context-note:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Conviene distinguirlos: 1330 es una línea de información turística e interpretación, no un número de despacho de emergencias. Llama al 112 para la policía o al 119 para incendios y emergencias médicas.
  ```
- Protected tokens: `112`, `119`

### ITEM 467

- Source context: L497 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Información oficial de Emergency Ready
  ```
- Protected tokens: `Emergency Ready`

### ITEM 468

- Source context: L498 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) > a:nth-of-type(1)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Emergency Ready en Google Play
  ```
- Protected tokens: `Emergency Ready`, `Google Play`, `Google`

### ITEM 469

- Source context: L499 - `html > body > main > article.apps-guide > section#payment-transit-apps.apps-section:nth-of-type(7) > div.container > p.apps-download-links:nth-of-type(6) > a:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Resumen oficial de Emergency Ready
  ```
- Protected tokens: `Emergency Ready`

### ITEM 470

- Source context: L507 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > header.apps-section__heading > h2#setup-before-you-fly-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Completa la configuración delicada mientras tus cuentas habituales aún funcionan
  ```
- Protected tokens: None identified in this item.

### ITEM 471

- Source context: L508 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Una red conocida mantiene al alcance tu número de teléfono habitual, tarjetas, correo electrónico y gestor de contraseñas mientras pruebas el inicio de sesión, los permisos y el pago.
  ```
- Protected tokens: None identified in this item.

### ITEM 472

- Source context: L511 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(1) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Completa el acceso a las cuentas
  ```
- Protected tokens: None identified in this item.

### ITEM 473

- Source context: L511 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Instala solo las apps que tengan una función en el itinerario, configura el idioma y después cierra y vuelve a iniciar sesión una vez para que cualquier contraseña olvidada o inicio social bloqueado aparezca antes del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 474

- Source context: L512 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(2) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Prueba los permisos que necesitas
  ```
- Protected tokens: None identified in this item.

### ITEM 475

- Source context: L512 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Concede a las apps de mapas, taxi y seguridad el acceso mínimo a la ubicación que necesiten y prueba la traducción con cámara de Papago sobre una captura de pantalla.
  ```
- Protected tokens: `Papago`

### ITEM 476

- Source context: L513 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(3) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Prueba el pago del taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 477

- Source context: L513 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Añade a k.ride el método de pago que piensas usar mientras mantienes disponible otra tarjeta o efectivo por si falla el primer cobro.
  ```
- Protected tokens: `k.ride`

### ITEM 478

- Source context: L514 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(4) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Haz que el hotel sea fácil de identificar
  ```
- Protected tokens: None identified in this item.

### ITEM 479

- Source context: L514 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Guarda juntos en notas el nombre exacto en coreano, la dirección vial y el número de teléfono para que un resultado del mapa, un conductor o la recepción del hotel puedan confirmar el mismo lugar.
  ```
- Protected tokens: None identified in this item.

### ITEM 480

- Source context: L515 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(5) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Haz capturas de las reservas importantes
  ```
- Protected tokens: None identified in this item.

### ITEM 481

- Source context: L515 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Visible list description
- Spanish:

  ```text
  Guarda los datos de trenes, restaurantes, aeropuerto y alojamiento para poder usarlos sin una sesión de cuenta activa.
  ```
- Protected tokens: None identified in this item.

### ITEM 482

- Source context: L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > strong :: text()[1]`
- Element/type: Visible list label
- Spanish:

  ```text
  Confirma el acceso a datos y SMS
  ```
- Protected tokens: `SMS`

### ITEM 483

- Source context: L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > span > a`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  eSIM de Korea
  ```
- Protected tokens: `Korea`, `eSIM`

### ITEM 484

- Source context: L518 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > p.apps-context-note`
- Element/type: Body text
- Spanish:

  ```text
  Añade estos pasos a tu lista general de viaje por Korea para terminar la configuración de las apps antes del día del aeropuerto.
  ```
- Protected tokens: `Korea`

### ITEM 485

- Source context: L525 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > header.apps-section__heading > h2#common-app-problems-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Dónde pueden complicarse las apps coreanas
  ```
- Protected tokens: None identified in this item.

### ITEM 486

- Source context: L526 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Los mismos obstáculos pueden afectar a mapas, taxis, entrega y mensajería: acceso a la cuenta, reglas de pago, campos de dirección coreana y pérdida de una conexión activa.
  ```
- Protected tokens: None identified in this item.

### ITEM 487

- Source context: L530 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#real-travel-situations.apps-editorial-row:nth-of-type(1) > h3#real-travel-situations-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Los nombres en inglés pueden ocultar la sucursal correcta
  ```
- Protected tokens: None identified in this item.

### ITEM 488

- Source context: L531 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#real-travel-situations.apps-editorial-row:nth-of-type(1) > p`
- Element/type: Body text
- Spanish:

  ```text
  Un nombre de lugar en coreano, una dirección vial o un número de teléfono suelen dar a una búsqueda local más información con la que trabajar. Después, los nombres de distrito, las fotos de la fachada y la imagen de la reserva ayudan a separar una sucursal de otra.
  ```
- Protected tokens: None identified in this item.

### ITEM 489

- Source context: L534 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Un fallo de SMS puede convertirse en un problema de recuperación de cuenta
  ```
- Protected tokens: `SMS`

### ITEM 490

- Source context: L535 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(2) > p`
- Element/type: Body text
- Spanish:

  ```text
  El código de país, el estado del roaming y el acceso al número original influyen en la verificación. La recuperación por correo, redes sociales, pasaporte o dispositivo de confianza solo ayuda cuando el servicio ofrece oficialmente esa vía.
  ```
- Protected tokens: None identified in this item.

### ITEM 491

- Source context: L538 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Una tarjeta extranjera puede fallar después de que la configuración funcione
  ```
- Protected tokens: None identified in this item.

### ITEM 492

- Source context: L539 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(3) > p`
- Element/type: Body text
- Spanish:

  ```text
  Repetir el cobro no cambia las reglas de pago de un comercio. Tener preparada otra tarjeta, efectivo o una vía de pago presencial evita que una transacción rechazada bloquee el siguiente paso.
  ```
- Protected tokens: None identified in this item.

### ITEM 493

- Source context: L542 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Las direcciones coreanas tienen que ajustarse al formato del servicio
  ```
- Protected tokens: None identified in this item.

### ITEM 494

- Source context: L543 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(4) > p`
- Element/type: Body text
- Spanish:

  ```text
  Añadir texto en inglés puede impedir que un campo de dirección acepte una ubicación que por lo demás es correcta. La dirección vial completa en coreano y las instrucciones de entrega del alojamiento son más fiables; si no hay cobertura, conviene recurrir a recogida, ayuda del hotel o una visita directa.
  ```
- Protected tokens: None identified in this item.

### ITEM 495

- Source context: L546 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(5) > h3`
- Element/type: H3 heading
- Spanish:

  ```text
  Los permisos y los ajustes de idioma pueden restablecerse sin que lo notes
  ```
- Protected tokens: None identified in this item.

### ITEM 496

- Source context: L547 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div.apps-editorial-row:nth-of-type(5) > p`
- Element/type: Body text
- Spanish:

  ```text
  Los ajustes de idioma de la app y del teléfono pueden importar a la vez. Las funciones que dependen de la ubicación también necesitan el permiso mínimo necesario, seguido de una nueva comprobación del marcador de recogida antes de confirmar.
  ```
- Protected tokens: None identified in this item.

### ITEM 497

- Source context: L550 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#common-mistakes.apps-editorial-row:nth-of-type(6) > h3#common-mistakes-title`
- Element/type: H3 heading
- Spanish:

  ```text
  Las capturas mantienen el viaje en marcha cuando desaparece el acceso en tiempo real
  ```
- Protected tokens: None identified in this item.

### ITEM 498

- Source context: L551 - `html > body > main > article.apps-guide > section#common-app-problems.apps-section:nth-of-type(9) > div.container > div.apps-editorial-list > div#common-mistakes.apps-editorial-row:nth-of-type(6) > p`
- Element/type: Body text
- Spanish:

  ```text
  Las reservas guardadas, las direcciones coreanas y los datos de la ruta de regreso siguen siendo utilizables aunque se pierda la sesión o se corte la conexión de datos. Una Wi-Fi fiable del alojamiento o de una estación puede ayudar a recuperar una conexión de roaming, SIM o eSIM sin convertir el acceso en tiempo real en el único plan.
  ```
- Protected tokens: `Wi-Fi`, `SIM`, `eSIM`

### ITEM 499

- Source context: L560 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > header.apps-section__heading > h2#faq-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Preguntas frecuentes
  ```
- Protected tokens: None identified in this item.

### ITEM 500

- Source context: L564 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(1) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué apps debería instalar antes de viajar a Korea?
  ```
- Protected tokens: `Korea`

### ITEM 501

- Source context: L565 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(1) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  La mayoría de quienes visitan Korea por primera vez deberían instalar Naver Map, Papago y k.ride antes del viaje. Añade Catchtable, Shuttle Delivery, KakaoTalk y apps de pagos o trenes solo cuando tus planes las necesiten.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`, `Korea`, `Catchtable`, `Shuttle Delivery`, `Shuttle`, `KakaoTalk`

### ITEM 502

- Source context: L568 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(2) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de mapas para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 503

- Source context: L569 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(2) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Naver Map es la mejor app de mapas predeterminada para la mayoría de los visitantes porque ofrece búsquedas locales detalladas y rutas en metro, autobús, a pie y en coche. Mantén KakaoMap como comprobación opcional.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 504

- Source context: L572 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(3) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Google Maps funciona en Korea?
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`

### ITEM 505

- Source context: L573 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(3) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Google Maps funciona para guardar lugares y orientarse de forma básica, pero la búsqueda local y los detalles de las rutas pueden ser menos fiables en Korea. Usa Naver Map como app principal de navegación y comprueba el nombre o la dirección en coreano cuando falle la búsqueda.
  ```
- Protected tokens: `Google Maps`, `Google`, `Korea`, `Naver Map`

### ITEM 506

- Source context: L576 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(4) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuál es la mejor app de traducción para Korea?
  ```
- Protected tokens: `Korea`

### ITEM 507

- Source context: L577 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(4) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Papago es la mejor primera app de traducción para muchos viajes por Korea. Admite traducción de texto, imagen, voz, conversación y sin conexión, aunque los nombres, el argot y el contexto de los menús siguen requiriendo criterio.
  ```
- Protected tokens: `Papago`, `Korea`

### ITEM 508

- Source context: L580 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(5) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué app de taxi es más fácil para visitantes extranjeros?
  ```
- Protected tokens: None identified in this item.

### ITEM 509

- Source context: L581 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(5) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  k.ride es la opción predeterminada más sencilla para muchos visitantes extranjeros porque está diseñada para viajeros internacionales y admite búsqueda multilingüe de destinos, chat traducido con el conductor y registro de tarjetas emitidas en el extranjero. Kakao T y Uber Taxi siguen siendo alternativas útiles.
  ```
- Protected tokens: `k.ride`, `Kakao T`, `Kakao`, `Uber Taxi`

### ITEM 510

- Source context: L584 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(6) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Necesito KakaoTalk como turista?
  ```
- Protected tokens: `KakaoTalk`

### ITEM 511

- Source context: L585 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(6) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Solo necesitas KakaoTalk si esperas comunicarte con amigos coreanos, anfitriones, operadores turísticos o negocios locales. Los viajeros sin contactos locales normalmente pueden prescindir de ella.
  ```
- Protected tokens: `KakaoTalk`

### ITEM 512

- Source context: L588 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(7) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué app debería usar para reservar restaurantes?
  ```
- Protected tokens: None identified in this item.

### ITEM 513

- Source context: L589 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(7) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Usa Catchtable cuando un restaurante acepte reservas o registro en lista de espera mediante el servicio. Confirma la sucursal, la fecha, el número de personas, el depósito y las condiciones de cancelación antes de reservar.
  ```
- Protected tokens: `Catchtable`

### ITEM 514

- Source context: L592 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(8) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Los turistas pueden pedir comida a domicilio en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 515

- Source context: L593 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(8) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Los turistas pueden pedir comida a domicilio en algunas zonas. Shuttle Delivery es la primera opción más sencilla para muchos visitantes extranjeros porque admite usuarios y tarjetas internacionales, aunque la cobertura varía; la ayuda del hotel, la recogida y acudir directamente al restaurante son alternativas fiables.
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 516

- Source context: L596 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(9) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Las apps coreanas exigen un número de teléfono coreano?
  ```
- Protected tokens: None identified in this item.

### ITEM 517

- Source context: L597 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(9) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Algunas apps coreanas funcionan con un número extranjero, mientras que otras exigen SMS coreano, verificación de identidad o de pago para determinadas funciones. Prueba el inicio de sesión y la recuperación antes de salir y nunca des por hecho que una eSIM solo de datos incluye un número de teléfono coreano.
  ```
- Protected tokens: `SMS`, `eSIM`

### ITEM 518

- Source context: L600 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(10) > summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué apps debería configurar antes de mi vuelo?
  ```
- Protected tokens: None identified in this item.

### ITEM 519

- Source context: L601 - `html > body > main > article.apps-guide > section#faq.apps-section.apps-section--soft.apps-faq:nth-of-type(10) > div.container > div.apps-faq__list > details:nth-of-type(10) > p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Antes del vuelo, configura Naver Map, Papago y k.ride y prueba después el inicio de sesión, el idioma, los permisos y el pago. Guarda el nombre en coreano de tu hotel, la dirección vial y el número de teléfono, además de capturas de pantalla de las reservas importantes.
  ```
- Protected tokens: `Naver Map`, `Papago`, `k.ride`

### ITEM 520

- Source context: L610 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > header.apps-section__heading > h2#official-sources-title`
- Element/type: H2 heading
- Spanish:

  ```text
  Fuentes oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 521

- Source context: L611 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > header.apps-section__heading > p`
- Element/type: Body text
- Spanish:

  ```text
  Las funciones de las apps y las condiciones de verificación y pago pueden cambiar. Confirma cualquier función crítica en el servicio oficial antes de incorporarla a tu plan de viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 522

- Source context: L615 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(1) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  VISITKOREA — Apps y recursos útiles
  ```
- Protected tokens: `VISITKOREA`

### ITEM 523

- Source context: L616 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(1) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Resumen oficial de Korea Tourism Organization sobre herramientas de mapas, transporte, entrega y seguridad orientadas a visitantes.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 524

- Source context: L619 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(2) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Ayuda oficial de NAVER Map — idiomas compatibles
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 525

- Source context: L620 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(2) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Información oficial sobre la interfaz y los idiomas del mapa.
  ```
- Protected tokens: None identified in this item.

### ITEM 526

- Source context: L623 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(3) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Página oficial de la app NAVER Map
  ```
- Protected tokens: `NAVER Map`, `NAVER`

### ITEM 527

- Source context: L624 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(3) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Funciones actuales de rutas, búsqueda, transporte público, navegación y guardado de NAVER Corp.
  ```
- Protected tokens: `NAVER Corp.`, `NAVER Corp`, `NAVER`

### ITEM 528

- Source context: L627 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(4) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Página oficial de la app Papago
  ```
- Protected tokens: `Papago`

### ITEM 529

- Source context: L628 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(4) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Funciones actuales de traducción de texto, imagen, voz, conversación y sin conexión de NAVER Corp.
  ```
- Protected tokens: `NAVER Corp.`, `NAVER Corp`, `NAVER`

### ITEM 530

- Source context: L631 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(5) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Kakao Mobility — k.ride
  ```
- Protected tokens: `Kakao Mobility`, `Kakao`, `k.ride`

### ITEM 531

- Source context: L632 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(5) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Funciones oficiales de taxi orientadas a visitantes, idiomas compatibles y descargas de la app.
  ```
- Protected tokens: None identified in this item.

### ITEM 532

- Source context: L635 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(6) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  CATCHTABLE Global
  ```
- Protected tokens: `CATCHTABLE Global`, `CATCHTABLE`

### ITEM 533

- Source context: L636 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(6) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Servicio oficial para descubrir restaurantes, hacer reservas y apuntarse a listas de espera.
  ```
- Protected tokens: None identified in this item.

### ITEM 534

- Source context: L639 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(7) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Shuttle Delivery
  ```
- Protected tokens: `Shuttle Delivery`, `Shuttle`

### ITEM 535

- Source context: L640 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(7) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Servicio oficial de pedidos y disponibilidad actual de entrega para usuarios internacionales.
  ```
- Protected tokens: None identified in this item.

### ITEM 536

- Source context: L643 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(8) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Baemin — experiencia de pedidos multilingüe
  ```
- Protected tokens: `Baemin`

### ITEM 537

- Source context: L644 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(8) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Resumen técnico oficial de Woowa Brothers sobre el flujo principal de pedidos multilingüe.
  ```
- Protected tokens: `Woowa Brothers`

### ITEM 538

- Source context: L647 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(9) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Guía oficial de WOWPASS
  ```
- Protected tokens: `WOWPASS`

### ITEM 539

- Source context: L648 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(9) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Instrucciones oficiales sobre pagos, recargas y saldo independiente de T-money.
  ```
- Protected tokens: `T-money`

### ITEM 540

- Source context: L651 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(10) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Tmoney — compatibilidad con tarjeta de transporte móvil
  ```
- Protected tokens: `Tmoney`

### ITEM 541

- Source context: L652 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(10) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Guía oficial sobre disponibilidad de Mobile Tmoney y dispositivos compatibles.
  ```
- Protected tokens: `Tmoney`

### ITEM 542

- Source context: L655 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(11) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Reserva oficial de KORAIL en inglés
  ```
- Protected tokens: `KORAIL`

### ITEM 543

- Source context: L656 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(11) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Canal oficial de reservas para KTX y otros trenes operados por KORAIL.
  ```
- Protected tokens: `KTX`, `KORAIL`

### ITEM 544

- Source context: L659 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(12) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  App móvil de VISITKOREA
  ```
- Protected tokens: `VISITKOREA`

### ITEM 545

- Source context: L660 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(12) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Información oficial de viaje, funciones de planificación e itinerarios de Korea Tourism Organization.
  ```
- Protected tokens: `Korea Tourism Organization`, `Korea`

### ITEM 546

- Source context: L663 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(13) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  Ministry of the Interior and Safety — Emergency Ready
  ```
- Protected tokens: `Ministry of the Interior and Safety`, `Emergency Ready`

### ITEM 547

- Source context: L664 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(13) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Resumen oficial de alertas, refugios, instalaciones de emergencia, información de embajadas y orientación de seguridad.
  ```
- Protected tokens: None identified in this item.

### ITEM 548

- Source context: L667 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(14) > a`
- Element/type: Official-source visible link text
- Spanish:

  ```text
  VISITKOREA — 1330 Korea Travel Helpline
  ```
- Protected tokens: `VISITKOREA`, `1330 Korea Travel Helpline`, `Korea`

### ITEM 549

- Source context: L668 - `html > body > main > article.apps-guide > section#official-sources.apps-section.apps-sources:nth-of-type(11) > div.container > ul > li:nth-of-type(14) > span :: text()[1]`
- Element/type: Official-source description
- Spanish:

  ```text
  Canal oficial de información turística y apoyo de interpretación, distinto del despacho de policía y de bomberos o emergencias médicas.
  ```
- Protected tokens: None identified in this item.

## Supplemental approved correction

This correction is CONTENT LOCKED and does not change the Batch ITEM count of 549.

- Source context: `apps.html` L516 - `html > body > main > article.apps-guide > section#setup-before-you-fly.apps-section.apps-section--soft:nth-of-type(8) > div.container > ul.apps-checklist > li:nth-of-type(6) > span :: mixed text around a`
- Element/type: Visible list description
- Exact English:

  ```text
  A roaming plan, SIM or Korea eSIM may be data-only while an account still sends recovery codes to the home number.
  ```
- Approved Spanish:

  ```text
  Un plan de roaming, una SIM o una eSIM de Korea puede ser solo de datos, mientras una cuenta sigue enviando códigos de recuperación al número original.
  ```
- Protected tokens: `SIM`, `Korea`, `eSIM`

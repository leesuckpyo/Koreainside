# Korea Inside — Spanish Localization Batch 4 — Airport 6

**Status:** APPROVED PUBLIC COPY — CONTENT LOCKED  
**Date:** 2026-09-23  
**Target language:** Spanish (`es`)  
**Source document:** `Korea_Inside_Airport6_ES_Localization_Source_Batch4_2026-09-23.md`  
**Total localized ITEMs:** 929

> Este documento contiene la redacción en español propuesta para los 929 ITEM del Source MD.
>
> Debe leerse junto con el Source MD, que sigue siendo la fuente de verdad para el texto English, file/line context, protected tokens, exclusiones estructurales, SHA-256, URL, tracking y estructura.
>
> No autoriza implementación hasta que el usuario apruebe el Batch completo.

## Batch pages

1. `airport.html` — 110 ITEMs
2. `arrival.html` — 111 ITEMs
3. `airport-transfer.html` — 166 ITEMs
4. `arex.html` — 221 ITEMs
5. `airport-bus.html` — 182 ITEMs
6. `incheon-airport-private-transfer.html` — 139 ITEMs

## Localization decisions

- Spanish natural y neutral para contenido de viaje; no traducción literal palabra por palabra.
- Se preservan hechos, cifras, juicios editoriales, orden de decisión, nombres propios, marcas, estaciones, rutas, productos, fechas, horarios, distancias, medidas y demás valores protegidos del Source MD.
- Global navigation, language switcher y footer siguen excluidos conforme al Source MD.
- El Source MD confirma **0** `data-label` user-facing en estas seis páginas; no se inventa ninguno.
- Los `data-*` funcionales, de tracking, analytics, affiliate y event siguen protegidos y no se localizan.
- Cuando el Source marca un token exacto como protegido, se conserva literalmente aunque el resto de la frase se localice al español. Los adjetivos no protegidos como `Korean` se localizan de forma natural cuando corresponde.
- Visible FAQ y JSON-LD repetidos usan exactamente la misma localización española para mantener paridad semántica.

## Approval status

**APPROVED PUBLIC COPY — CONTENT LOCKED**

User approval: 2026-09-23

- no retranslate
- no rewrite
- no summarize
- no expand
- Codex exact implementation solamente

---

# PAGE: `airport.html`

**English source SHA-256:** `0a5f143aefca411fa88739fda825dae24257f05f73016003effb00c57eef844d`  
**Localized ITEM count:** 110

### ITEM 001

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Después de inmigración y aduanas, usa esta guía de la sala de llegadas de Incheon Airport para conectarte, guardar tu dirección, comprobar cómo pagar y elegir el transporte.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 002

- Source context: L8 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Guía de la sala de llegadas de Incheon Airport: primeros 30 Minutes | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `30 Minutes`, `Korea Inside`

### ITEM 003

- Source context: L80 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 004

- Source context: L81 - `h1.airport-page-hero__title`
- Element/type: H1
- Spanish:

  ```text
  Tus primeros 30 Minutes en la sala de llegadas de Incheon Airport
  ```
- Protected tokens: `30 Minutes`, `Incheon Airport`

### ITEM 005

- Source context: L82 - `p.airport-page-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Esta guía empieza cuando ya has pasado inmigración, recogido el equipaje, superado aduanas y entrado en la sala pública de llegadas. Antes de dirigirte a la ciudad, dedica unos minutos a comprobar que tu teléfono tiene conexión, guarda los datos de tu alojamiento en un formato que pueda utilizarse en Korea, prepara una alternativa de pago y revisa el trayecto completo hasta tu alojamiento.
  ```
- Protected tokens: `Korea`

### ITEM 006

- Source context: L86 - `a.airport-pill:nth-of-type(1)`
- Element/type: Visible link text
- Spanish:

  ```text
  Primeros pasos
  ```
- Protected tokens: None identified in this item.

### ITEM 007

- Source context: L87 - `a.airport-pill:nth-of-type(2)`
- Element/type: Visible link text
- Spanish:

  ```text
  Transporte
  ```
- Protected tokens: None identified in this item.

### ITEM 008

- Source context: L88 - `a.airport-pill:nth-of-type(3)`
- Element/type: Visible link text
- Spanish:

  ```text
  Problemas
  ```
- Protected tokens: None identified in this item.

### ITEM 009

- Source context: L89 - `a.airport-pill:nth-of-type(4)`
- Element/type: Visible link text
- Spanish:

  ```text
  Preguntas frecuentes
  ```
- Protected tokens: None identified in this item.

### ITEM 010

- Source context: L93 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Ilustración de los primeros 30 minutes en la sala de llegadas de Incheon Airport: conectarse, guardar la dirección, comprobar el pago y elegir el transporte
  ```
- Protected tokens: `30 minutes`, `Incheon Airport`

### ITEM 011

- Source context: L110 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  Cuatro cosas que conviene resolver antes de salir de la terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 012

- Source context: L111 - `p.section__subtitle`
- Element/type: Body text
- Spanish:

  ```text
  Ninguna de estas tareas lleva mucho tiempo, pero es más fácil resolverlas mientras todavía tienes cerca el Wi-Fi del aeropuerto, los mostradores de información y un lugar cómodo donde detenerte. T-money es útil para autobuses y metro, aunque puede esperar si tu primer trayecto será en taxi, traslado reservado con antelación o coche de alquiler.
  ```
- Protected tokens: `T-money`

### ITEM 013

- Source context: L117 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba que los datos móviles funcionan sin Wi-Fi. El Wi-Fi de la sala de llegadas puede ocultar un problema con el roaming o la eSIM. Desactiva brevemente el Wi-Fi y abre un mapa o una página web usando datos móviles, en lugar de fiarte solo del símbolo LTE o 5G. Si no carga nada, es mucho más fácil seguir las instrucciones del proveedor o contactar con soporte antes de salir de la terminal. Abrir la guía de eSIM
  ```
- Protected tokens: `LTE`, `5G`

### ITEM 014

- Source context: L119 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Comprueba que los datos móviles funcionan sin Wi-Fi
  ```
- Protected tokens: None identified in this item.

### ITEM 015

- Source context: L120 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El Wi-Fi de la sala de llegadas puede ocultar un problema con el roaming o la eSIM. Desactiva brevemente el Wi-Fi y abre un mapa o una página web usando datos móviles, en lugar de fiarte solo del símbolo LTE o 5G. Si no carga nada, es mucho más fácil seguir las instrucciones del proveedor o contactar con soporte antes de salir de la terminal.
  ```
- Protected tokens: `LTE`, `5G`

### ITEM 016

- Source context: L124 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Guarda tu destino en un formato que la gente pueda utilizar. El nombre del hotel en inglés puede no bastar para un taxista o para buscarlo en una app de mapas coreana. Guarda juntos el nombre coreano del lugar, la dirección vial en coreano, el número de teléfono y la confirmación de la reserva, y conserva además una captura disponible sin conexión. Abrir la guía de mapas
  ```
- Protected tokens: None identified in this item.

### ITEM 017

- Source context: L126 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Guarda tu destino en un formato que la gente pueda utilizar
  ```
- Protected tokens: None identified in this item.

### ITEM 018

- Source context: L127 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El nombre del hotel en inglés puede no bastar para un taxista o para buscarlo en una app de mapas coreana. Guarda juntos el nombre coreano del lugar, la dirección vial en coreano, el número de teléfono y la confirmación de la reserva, y conserva además una captura disponible sin conexión.
  ```
- Protected tokens: None identified in this item.

### ITEM 019

- Source context: L131 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Ten una alternativa de pago. Una llegada larga se vuelve más estresante cuando una sola tarjeta es tu única forma de pagar. Guarda una segunda tarjeta o algo de efectivo separado de la tarjeta principal para que un rechazo en una máquina o mostrador no bloquee el resto del trayecto. Abrir la guía de pagos
  ```
- Protected tokens: None identified in this item.

### ITEM 020

- Source context: L133 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Ten una alternativa de pago
  ```
- Protected tokens: None identified in this item.

### ITEM 021

- Source context: L134 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Una llegada larga se vuelve más estresante cuando una sola tarjeta es tu única forma de pagar. Guarda una segunda tarjeta o algo de efectivo separado de la tarjeta principal para que un rechazo en una máquina o mostrador no bloquee el resto del trayecto.
  ```
- Protected tokens: None identified in this item.

### ITEM 022

- Source context: L138 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Mira el trayecto completo, no solo el primer tren o autobús. El primer tramo más rápido o barato puede terminar en otro transbordo, escaleras o una caminata larga con equipaje. Tu destino, hora de llegada, tamaño del grupo y el tramo final desde la parada o estación importan más que el tiempo anunciado del primer trayecto. Ver en qué se diferencian las opciones de traslado desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 023

- Source context: L140 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Mira el trayecto completo, no solo el primer tren o autobús
  ```
- Protected tokens: None identified in this item.

### ITEM 024

- Source context: L141 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El primer tramo más rápido o barato puede terminar en otro transbordo, escaleras o una caminata larga con equipaje. Tu destino, hora de llegada, tamaño del grupo y el tramo final desde la parada o estación importan más que el tiempo anunciado del primer trayecto.
  ```
- Protected tokens: None identified in this item.

### ITEM 025

- Source context: L153 - `h2.section__title`
- Element/type: H2
- Spanish:

  ```text
  Usa el mapa de la terminal a la que realmente llegaste
  ```
- Protected tokens: None identified in this item.

### ITEM 026

- Source context: L154 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Terminal 1 y Terminal 2 tienen distribuciones diferentes, así que primero confirma la terminal indicada para tu vuelo. Después cambia el mapa oficial a esa terminal y a la planta de llegadas antes de buscar transporte, un mostrador de información u otra instalación.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 027

- Source context: L157 - `a.airport-official-map-link__button`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Abrir el mapa oficial del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 028

- Source context: L165 - `p.airport-official-map-link__tip`
- Element/type: Body text
- Spanish:

  ```text
  El mapa puede abrirse en otra terminal o planta. Ajusta ambas opciones a tu llegada actual antes de seguir sus indicaciones.
  ```
- Protected tokens: None identified in this item.

### ITEM 029

- Source context: L175 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  ¿Todavía estás pasando inmigración o recogiendo el equipaje?
  ```
- Protected tokens: None identified in this item.

### ITEM 030

- Source context: L178 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Si todavía buscas instrucciones sobre conexiones, inmigración, recogida de equipaje o aduanas, empieza por la Guía de Llegada. Esta página comienza únicamente cuando ya has entrado en la sala pública de llegadas y estás listo para resolver los primeros aspectos prácticos del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 031

- Source context: L179 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Seguir el recorrido desde el avión hasta la sala de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 032

- Source context: L188 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  Cómo llegar del aeropuerto a tu alojamiento
  ```
- Protected tokens: None identified in this item.

### ITEM 033

- Source context: L189 - `p.section__subtitle`
- Element/type: Body text
- Spanish:

  ```text
  No existe un único traslado que sea el mejor para todos. La comparación útil es el viaje completo puerta a puerta: dónde te alojas, a qué hora llegas, cuánto equipaje llevas, con quién viajas y qué ocurre después del primer tren o autobús. Aun así, comprueba las rutas y horarios actuales para el día del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 034

- Source context: L194 - `h3`
- Element/type: H3
- Spanish:

  ```text
  AREX
  ```
- Protected tokens: `AREX`

### ITEM 035

- Source context: L196 - `p`
- Element/type: Body text
- Spanish:

  ```text
  AREX es fácil de entender cuando la ruta te lleva hacia Seoul Station, Hongdae u otro destino bien conectado por tren. Sin embargo, el tren es solo una parte del viaje. Si para llegar al hotel aún necesitas otro transbordo de metro y una caminata larga con equipaje, un autobús o taxi puede ser más sencillo puerta a puerta aunque el tramo en tren sea más rápido.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 036

- Source context: L197 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Leer la guía de AREX
  ```
- Protected tokens: `AREX`

### ITEM 037

- Source context: L202 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 038

- Source context: L204 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús del aeropuerto puede resultar sorprendentemente cómodo cuando para cerca de tu alojamiento. Evita mover maletas por una estación grande, pero el tráfico y la caminata desde la parada real siguen importando. Revisa la ruta y el horario actuales en lugar de asumir que la parada cuyo nombre suena más cercana será la más fácil.
  ```
- Protected tokens: None identified in this item.

### ITEM 039

- Source context: L205 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Leer la guía del autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 040

- Source context: L210 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 041

- Source context: L212 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un taxi cuesta más que el transporte público, pero la comparación cambia cuando viajan varias personas, hay niños, equipaje pesado o una llegada tardía. Ir puerta a puerta puede eliminar varios transbordos justo cuando más cansan. Usa una parada oficial de taxis y ten preparado el nombre coreano del destino, la dirección y el número de teléfono.
  ```
- Protected tokens: None identified in this item.

### ITEM 042

- Source context: L213 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Leer la guía de taxis
  ```
- Protected tokens: None identified in this item.

### ITEM 043

- Source context: L218 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Traslado reservado con antelación
  ```
- Protected tokens: None identified in this item.

### ITEM 044

- Source context: L220 - `p`
- Element/type: Body text
- Spanish:

  ```text
  La principal ventaja no es simplemente que haya un coche esperando. Es llegar con un punto de encuentro ya organizado, algo que puede ser valioso con niños, adultos mayores o mucho equipaje. Guarda sin conexión el contacto de la reserva, el punto de encuentro y las instrucciones de la terminal antes del vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 045

- Source context: L221 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar opciones de traslado desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 046

- Source context: L227 - `h3#airport-driving-note-title`
- Element/type: H3
- Spanish:

  ```text
  ¿Vas a conducir fuera de Seoul?
  ```
- Protected tokens: `Seoul`

### ITEM 047

- Source context: L228 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un coche de alquiler es una decisión distinta de las cuatro formas habituales de entrar en Seoul. Cobra sentido cuando un viaje regional por carretera o un itinerario fuera de la ciudad hace que conducir resulte realmente útil. Los requisitos del permiso de conducir, el seguro, las instrucciones de recogida y la terminal correcta deberían quedar resueltos antes de llegar.
  ```
- Protected tokens: `Seoul`

### ITEM 048

- Source context: L229 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Leer la guía de alquiler de coches
  ```
- Protected tokens: None identified in this item.

### ITEM 049

- Source context: L231 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La zona donde te alojas cambia cuál es realmente la ruta más fácil desde el aeropuerto. Si todavía estás organizando el viaje, compara Hongdae, Gongdeok, Seoul Station y Myeongdong por el trayecto completo desde el aeropuerto hasta el hotel.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 050

- Source context: L232 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar zonas de Seoul por acceso al aeropuerto →
  ```
- Protected tokens: `Seoul`

### ITEM 051

- Source context: L239 - `h2.section__title`
- Element/type: H2
- Spanish:

  ```text
  Guarda esto antes de salir del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 052

- Source context: L240 - `p.section__subtitle`
- Element/type: Body text
- Spanish:

  ```text
  Conserva la información que puedas necesitar incluso si dejan de funcionar internet o tu método de pago.
  ```
- Protected tokens: None identified in this item.

### ITEM 053

- Source context: L244 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Datos del alojamiento en coreano. Guarda juntos el nombre en coreano, la dirección vial y el número de teléfono para que un conductor, un mostrador de información o una app de mapas coreana pueda identificar el lugar.
  ```
- Protected tokens: None identified in this item.

### ITEM 054

- Source context: L245 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Confirmación de la reserva. Guarda la confirmación y cualquier instrucción de llegada o check-in en un lugar al que puedas acceder sin datos móviles.
  ```
- Protected tokens: None identified in this item.

### ITEM 055

- Source context: L246 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Ruta completa de transporte. Anota la terminal, la ruta, la parada o estación, la salida útil y la caminata o transbordo final hasta el alojamiento, no solo el primer tren o autobús.
  ```
- Protected tokens: None identified in this item.

### ITEM 056

- Source context: L247 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Datos de soporte de los datos móviles. Guarda la información de activación de la eSIM, el contacto de soporte y el código QR por si necesitas volver a utilizarlo. Sigue las instrucciones del proveedor antes de modificar o eliminar un perfil.
  ```
- Protected tokens: None identified in this item.

### ITEM 057

- Source context: L248 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Referencia de mapa sin conexión. Guarda el destino en Naver Map o KakaoMap y conserva una captura o un pin que puedas mostrar incluso cuando la conexión sea inestable.
  ```
- Protected tokens: `Naver Map`, `KakaoMap`

### ITEM 058

- Source context: L249 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Una segunda forma de pagar. Guarda otra tarjeta o algo de efectivo separado de la tarjeta principal para que una transacción rechazada no interrumpa el resto del trayecto.
  ```
- Protected tokens: None identified in this item.

### ITEM 059

- Source context: L258 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  Resuelve los problemas en la sala pública de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 060

- Source context: L259 - `p.section__subtitle`
- Element/type: Body text
- Spanish:

  ```text
  Un problema de datos, pago o transporte es más fácil de resolver antes de salir de la terminal. En la sala de llegadas tienes Wi-Fi, señalización oficial y mostradores de información mientras buscas una alternativa.
  ```
- Protected tokens: None identified in this item.

### ITEM 061

- Source context: L264 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Tu eSIM sigue sin tener datos
  ```
- Protected tokens: None identified in this item.

### ITEM 062

- Source context: L266 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Quédate unos minutos más conectado al Wi-Fi del aeropuerto. Comprueba que la línea de viaje esté activada y seleccionada para datos móviles, y después sigue las instrucciones de activación y roaming del proveedor. Si sigue sin conectarse, contacta con el proveedor antes de eliminar el perfil de la eSIM; tus capturas sin conexión pueden cubrir el trayecto inmediato mientras recibes asistencia.
  ```
- Protected tokens: None identified in this item.

### ITEM 063

- Source context: L267 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Revisar la guía de eSIM
  ```
- Protected tokens: None identified in this item.

### ITEM 064

- Source context: L272 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Tu tarjeta extranjera fue rechazada
  ```
- Protected tokens: None identified in this item.

### ITEM 065

- Source context: L274 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un rechazo no tiene por qué detener el viaje. Prueba en un mostrador atendido o con otra tarjeta y revisa la configuración del emisor mientras aún tienes Wi-Fi. Una segunda tarjeta o algo de efectivo permite separar la decisión de transporte del problema con el primer método de pago.
  ```
- Protected tokens: None identified in this item.

### ITEM 066

- Source context: L275 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Revisar la guía de pagos
  ```
- Protected tokens: None identified in this item.

### ITEM 067

- Source context: L280 - `h3`
- Element/type: H3
- Spanish:

  ```text
  No encuentras AREX, el autobús ni un taxi
  ```
- Protected tokens: `AREX`

### ITEM 068

- Source context: L282 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Empieza por la terminal y la planta que aparecen en el mapa oficial del aeropuerto, y después sigue las señales actuales para la ruta que elegiste. Un mostrador de información puede indicarte la entrada, parada o parada oficial de taxis correcta. Es más seguro y claro que seguir una oferta de transporte no solicitada.
  ```
- Protected tokens: None identified in this item.

### ITEM 069

- Source context: L283 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar el transporte desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 070

- Source context: L288 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Llegaste tarde por la noche
  ```
- Protected tokens: None identified in this item.

### ITEM 071

- Source context: L290 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Una llegada tardía cambia el orden de la decisión: consulta primero el horario oficial actual de tu terminal antes de caminar hacia una ruta diurna que recuerdas de memoria. Puede seguir habiendo un autobús nocturno. Si no, utiliza una parada oficial de taxis o las instrucciones de encuentro guardadas para una recogida reservada con antelación.
  ```
- Protected tokens: None identified in this item.

### ITEM 072

- Source context: L291 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Consultar la guía del autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 073

- Source context: L296 - `h3`
- Element/type: H3
- Spanish:

  ```text
  El conductor no puede identificar tu alojamiento
  ```
- Protected tokens: None identified in this item.

### ITEM 074

- Source context: L298 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Muestra el nombre coreano del alojamiento, la dirección vial y el número de teléfono en lugar de depender solo del nombre comercial en inglés. Una ubicación guardada en una app de mapas coreana o la captura de la reserva puede darle otra referencia al conductor, y el Wi-Fi del aeropuerto te permite contactar con el alojamiento si el destino sigue sin estar claro.
  ```
- Protected tokens: None identified in this item.

### ITEM 075

- Source context: L299 - `a.airport-text-link`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Revisar la guía de mapas
  ```
- Protected tokens: None identified in this item.

### ITEM 076

- Source context: L309 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes: los primeros 30 Minutes en Incheon Airport
  ```
- Protected tokens: `Incheon Airport`, `30 Minutes`

### ITEM 077

- Source context: L310 - `p.section__subtitle.section__subtitle--center`
- Element/type: Body text
- Spanish:

  ```text
  Respuestas breves para las decisiones que necesitas tomar en la sala pública de llegadas.
  ```
- Protected tokens: None identified in this item.

### ITEM 078

- Source context: L315 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué debo hacer primero al entrar en la sala de llegadas?
  ```
- Protected tokens: None identified in this item.

### ITEM 079

- Source context: L316 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Usa el Wi-Fi del aeropuerto como red de seguridad mientras pruebas los datos móviles, guardas los datos del alojamiento en coreano, compruebas que tienes una segunda forma de pagar y revisas la ruta completa hasta tu alojamiento. Estas pequeñas tareas se vuelven más difíciles una vez que sales con el equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 080

- Source context: L319 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Tengo que comprar T-money inmediatamente?
  ```
- Protected tokens: `T-money`

### ITEM 081

- Source context: L320 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. T-money es útil cuando el primer trayecto incluye autobús o metro, pero no tiene por qué ser la primera compra si vas a salir en taxi, traslado reservado con antelación o coche de alquiler. En esos primeros minutos puede ser más importante tener conexión y entender bien la ruta.
  ```
- Protected tokens: `T-money`

### ITEM 082

- Source context: L323 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo elijo entre AREX, autobús del aeropuerto y taxi?
  ```
- Protected tokens: `AREX`

### ITEM 083

- Source context: L324 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No te quedes solo con el primer tramo. AREX es sencillo para destinos bien conectados por tren, un autobús del aeropuerto puede ser más fácil si para cerca del alojamiento y un taxi elimina transbordos cuando el equipaje, los niños, el tamaño del grupo o una llegada tardía los hacen especialmente pesados.
  ```
- Protected tokens: `AREX`

### ITEM 084

- Source context: L327 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué debo hacer si llego tarde por la noche?
  ```
- Protected tokens: None identified in this item.

### ITEM 085

- Source context: L328 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Consulta el horario oficial actual de tu terminal antes de seguir una ruta pensada para el día. Puede seguir funcionando un autobús nocturno; si no, utiliza una parada oficial de taxis o las instrucciones de encuentro guardadas para una recogida reservada con antelación.
  ```
- Protected tokens: None identified in this item.

### ITEM 086

- Source context: L331 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde puedo comprobar mi terminal y las instalaciones del aeropuerto?
  ```
- Protected tokens: None identified in this item.

### ITEM 087

- Source context: L332 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Consulta la página oficial de vuelos de llegada para confirmar tu terminal y usa el mapa oficial del aeropuerto para localizar las instalaciones. Si tienes que cambiar de terminal en la zona pública, utiliza el autobús lanzadera gratuito entre terminales o el Airport Railroad de pago y confirma los datos operativos actuales. El tren lanzadera de la zona de embarque es para conexiones entre vuelos, no para desplazamientos normales entre terminales en la zona pública.
  ```
- Protected tokens: `Airport Railroad`

### ITEM 088

- Source context: L335 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué información debo guardar antes de salir del aeropuerto?
  ```
- Protected tokens: None identified in this item.

### ITEM 089

- Source context: L336 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Guarda el nombre coreano del alojamiento, la dirección vial y el número de teléfono, la confirmación de la reserva, la ruta completa de transporte, capturas útiles de mapas, los datos de activación y soporte de datos móviles, cualquier código QR importante y un plan de pago alternativo.
  ```
- Protected tokens: None identified in this item.

### ITEM 090

- Source context: L341 - `p.related-links__title`
- Element/type: Related-guide text
- Spanish:

  ```text
  Continúa con la parte que necesitas
  ```
- Protected tokens: None identified in this item.

### ITEM 091

- Source context: L342 - `a.chip:nth-of-type(1)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Configurar los datos móviles
  ```
- Protected tokens: None identified in this item.

### ITEM 092

- Source context: L343 - `a.chip:nth-of-type(2)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Entender T-money
  ```
- Protected tokens: `T-money`

### ITEM 093

- Source context: L344 - `a.chip:nth-of-type(3)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Preparar alternativas de pago
  ```
- Protected tokens: None identified in this item.

### ITEM 094

- Source context: L345 - `a.chip:nth-of-type(4)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Configurar mapas coreanos
  ```
- Protected tokens: None identified in this item.

### ITEM 095

- Source context: L346 - `a.chip:nth-of-type(5)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Comparar la ruta completa desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 096

- Source context: L354 - `h2.section__title.section__title--lg`
- Element/type: H2
- Spanish:

  ```text
  Información oficial del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 097

- Source context: L355 - `p.section__subtitle`
- Element/type: Body text
- Spanish:

  ```text
  Las terminales asignadas, los horarios de transporte y la ubicación de las instalaciones pueden cambiar. Estas son las páginas oficiales de Incheon Airport que conviene utilizar cuando necesitas información actualizada.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 098

- Source context: L359 - `li.airport-copy-row:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Mapa del aeropuerto. Selecciona Terminal 1 o Terminal 2 y después usa la planta de llegadas para localizar instalaciones y accesos al transporte. Abrir el mapa oficial del aeropuerto
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 099

- Source context: L360 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Mapa del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 100

- Source context: L362 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Selecciona Terminal 1 o Terminal 2 y después usa la planta de llegadas para localizar instalaciones y accesos al transporte.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 101

- Source context: L366 - `li.airport-copy-row:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Vuelos de llegada. Usa la información de vuelos en tiempo real para confirmar la terminal real de llegada en lugar de depender de una lista fija de aerolíneas. Consultar vuelos de llegada oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 102

- Source context: L367 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Vuelos de llegada
  ```
- Protected tokens: None identified in this item.

### ITEM 103

- Source context: L369 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Usa la información de vuelos en tiempo real para confirmar la terminal real de llegada en lugar de depender de una lista fija de aerolíneas.
  ```
- Protected tokens: None identified in this item.

### ITEM 104

- Source context: L373 - `li.airport-copy-row:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Transporte entre terminales. Revisa el autobús lanzadera de la zona pública y las opciones de Airport Railroad, junto con la información actual de operación y tarifas. Consultar el transporte oficial entre terminales
  ```
- Protected tokens: `Airport Railroad`

### ITEM 105

- Source context: L374 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Transporte entre terminales
  ```
- Protected tokens: None identified in this item.

### ITEM 106

- Source context: L376 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Revisa el autobús lanzadera de la zona pública y las opciones de Airport Railroad, junto con la información actual de operación y tarifas.
  ```
- Protected tokens: `Airport Railroad`

### ITEM 107

- Source context: L380 - `li.airport-copy-row:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos. Usa la página correspondiente a la terminal a la que llegaste para consultar las rutas, paradas y horarios actuales. Terminal 1 Terminal 2
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 108

- Source context: L381 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Autobuses nocturnos
  ```
- Protected tokens: None identified in this item.

### ITEM 109

- Source context: L383 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Usa la página correspondiente a la terminal a la que llegaste para consultar las rutas, paradas y horarios actuales.
  ```
- Protected tokens: None identified in this item.

### ITEM 110

- Source context: L384 - `p.airport-source-links:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Terminal 1 Terminal 2
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

---

# PAGE: `arrival.html`

**English source SHA-256:** `89fb08d898bd93bb26c2069464129d5b9cbc8ee7b2a54b0dc68db37cb276df0f`  
**Localized ITEM count:** 111

### ITEM 111

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Guía de llegada a Incheon Airport con comprobación de terminal, inmigración, K-ETA y e-Arrival Card, recogida de equipaje, aduanas y sala pública de llegadas.
  ```
- Protected tokens: `Incheon Airport`, `K-ETA`, `e-Arrival Card`

### ITEM 112

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Guía de llegada a Incheon Airport: inmigración, equipaje y aduanas | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Korea Inside`

### ITEM 113

- Source context: L22 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cómo sé si mi vuelo llega a Terminal 1 o Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 114

- Source context: L25 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Consulta tu billete electrónico o busca el número de vuelo en la página oficial de llegadas de Incheon Airport. Es más seguro que depender de una lista fija de aerolíneas, especialmente en vuelos de código compartido.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 115

- Source context: L30 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Paso por inmigración antes de recoger el equipaje?
  ```
- Protected tokens: None identified in this item.

### ITEM 116

- Source context: L33 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Sí, si vas a entrar en Korea. El orden habitual es inmigración, recogida de equipaje, aduanas y después la sala pública de llegadas. Los pasajeros que conectan con otro vuelo deben seguir en su lugar las señales de Transfer o Connecting Flights y las instrucciones de su aerolínea.
  ```
- Protected tokens: `Korea`

### ITEM 117

- Source context: L38 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué debo hacer si mi equipaje no llega?
  ```
- Protected tokens: None identified in this item.

### ITEM 118

- Source context: L41 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Primero comprueba que la cinta de equipaje sigue correspondiendo a tu vuelo. Si la maleta falta o está dañada, contacta con tu aerolínea o con el mostrador de equipajes antes de salir de la zona de recogida y ten a mano la etiqueta del equipaje y los datos del vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 119

- Source context: L46 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Puedo caminar entre Terminal 1 y Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 120

- Source context: L49 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No. Terminal 1 y Terminal 2 son edificios separados y no existe una ruta peatonal pública entre ellos. Si necesitas cambiar de terminal en la zona pública, utiliza las indicaciones oficiales del aeropuerto sobre transporte entre terminales.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 121

- Source context: L54 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué transporte debo elegir de Incheon Airport a Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 122

- Source context: L57 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Toma esa decisión cuando llegues a la sala pública de llegadas. La mejor opción depende del alojamiento exacto, la hora de llegada, el equipaje y el tamaño del grupo. La Guía de Traslados desde el Aeropuerto compara AREX, autobús del aeropuerto, taxi y traslado reservado con antelación según el trayecto completo hasta tu alojamiento.
  ```
- Protected tokens: `AREX`

### ITEM 123

- Source context: L62 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué deben hacer los pasajeros en conexión?
  ```
- Protected tokens: None identified in this item.

### ITEM 124

- Source context: L65 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Sigue las señales de Transfer o Connecting Flights y las instrucciones de tu aerolínea. Confirma la terminal de conexión, el tiempo disponible y si el equipaje facturado se transfiere al siguiente vuelo, en lugar de asumir que debes seguir el recorrido general de inmigración.
  ```
- Protected tokens: None identified in this item.

### ITEM 125

- Source context: L138 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Aeropuerto / Guía de Llegada
  ```
- Protected tokens: None identified in this item.

### ITEM 126

- Source context: L139 - `h1#arrival-title.arrival-hero__title`
- Element/type: H1
- Spanish:

  ```text
  Qué hacer después de aterrizar en Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 127

- Source context: L140 - `p.arrival-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Llegar a Incheon Airport es bastante sencillo cuando sabes qué señales importan. Si vas a entrar en Korea, la ruta habitual es Arrivals, inmigración, recogida de equipaje, aduanas y después la sala pública de llegadas.
  ```
- Protected tokens: `Incheon Airport`, `Korea`

### ITEM 128

- Source context: L141 - `p.arrival-hero__desc:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Si conectas con otro vuelo, no sigas esa secuencia automáticamente. Sigue en su lugar las señales de Transfer o Connecting Flights y las instrucciones correspondientes a tu itinerario.
  ```
- Protected tokens: None identified in this item.

### ITEM 129

- Source context: L144 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Viajeros con equipaje caminando bajo una señal de Arrivals dentro de Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 130

- Source context: L145 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Llegadas · Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 131

- Source context: L154 - `h2#flight-check-title`
- Element/type: H2
- Spanish:

  ```text
  Comprueba qué terminal utiliza realmente tu vuelo
  ```
- Protected tokens: None identified in this item.

### ITEM 132

- Source context: L155 - `p`
- Element/type: Body text
- Spanish:

  ```text
  La terminal asignada puede cambiar, especialmente en vuelos de código compartido. Tu billete electrónico y la búsqueda de llegadas en tiempo real del aeropuerto son más fiables que una lista de aerolíneas guardada.
  ```
- Protected tokens: None identified in this item.

### ITEM 133

- Source context: L158 - `a.arrival-button.arrival-button--light @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Consultar tu vuelo de llegada en el sitio oficial de Incheon Airport, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Incheon Airport`

### ITEM 134

- Source context: L158 - `a.arrival-button.arrival-button--light`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Consultar tu vuelo de llegada ↗
  ```
- Protected tokens: None identified in this item.

### ITEM 135

- Source context: L169 - `h2#journey-type-title`
- Element/type: H2
- Spanish:

  ```text
  ¿Vas a entrar en Korea o solo haces conexión?
  ```
- Protected tokens: `Korea`

### ITEM 136

- Source context: L173 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Entrar en Korea
  ```
- Protected tokens: `Korea`

### ITEM 137

- Source context: L174 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Si este es tu vuelo final y vas a entrar en Korea, sigue las señales de Arrivals o Immigration. La secuencia habitual es primero inmigración, después el equipaje facturado, luego aduanas y finalmente la sala pública de llegadas.
  ```
- Protected tokens: `Korea`

### ITEM 138

- Source context: L175 - `p.arrival-route-line:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Inmigración, después recogida de equipaje, luego aduanas y finalmente sala de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 139

- Source context: L175 - `p.arrival-route-line:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Inmigración → Recogida de equipaje → Aduanas → Sala de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 140

- Source context: L180 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Conexión con otro vuelo
  ```
- Protected tokens: None identified in this item.

### ITEM 141

- Source context: L181 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Si solo haces conexión en Incheon, sigue las señales de Transfer o Connecting Flights en lugar de incorporarte automáticamente al flujo general de llegadas. El procedimiento puede depender de la aerolínea, la terminal y de si tu equipaje facturado se transfiere al siguiente vuelo.
  ```
- Protected tokens: `Incheon`

### ITEM 142

- Source context: L182 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si algo no está claro, sigue las instrucciones de conexión de tu aerolínea en lugar de asumir que todas las conexiones requieren pasar por inmigración coreana.
  ```
- Protected tokens: None identified in this item.

### ITEM 143

- Source context: L191 - `h2#arrival-steps-title`
- Element/type: H2
- Spanish:

  ```text
  Del avión a la sala de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 144

- Source context: L192 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Para los viajeros que entran en Korea, el recorrido suele ser sencillo. La distancia a pie puede variar según la puerta y la terminal, pero el orden de los pasos principales es el mismo.
  ```
- Protected tokens: `Korea`

### ITEM 145

- Source context: L196 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Viajeros siguiendo las señales de Immigration y Baggage Claim dentro de Incheon International Airport
  ```
- Protected tokens: `Incheon International Airport`

### ITEM 146

- Source context: L197 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Sigue las señales del aeropuerto en orden; la ruta exacta a pie depende de la puerta y la terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 147

- Source context: L200 - `li.arrival-step:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  01 Sigue las señales de Arrivals. Después de bajar del avión, sigue las señales de Arrivals o Immigration. Las señales de Transfer y Connecting Flights conducen a otro proceso, así que úsalas solo si vas a continuar en otro vuelo.
  ```
- Protected tokens: `01`

### ITEM 148

- Source context: L203 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Sigue las señales de Arrivals
  ```
- Protected tokens: None identified in this item.

### ITEM 149

- Source context: L204 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Después de bajar del avión, sigue las señales de Arrivals o Immigration. Las señales de Transfer y Connecting Flights conducen a otro proceso, así que úsalas solo si vas a continuar en otro vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 150

- Source context: L207 - `li.arrival-step:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  02 Inmigración. Ten preparados el pasaporte y los documentos de entrada que correspondan a tu situación de viaje. Las reglas no son iguales para todos los pasaportes, visados o estatus de residencia, así que consulta la información oficial coreana antes del vuelo en lugar de copiar la lista de otro viajero. K-ETA y e-Arrival Card no son lo mismo. La exención temporal de K-ETA de Korea para los países ya cubiertos por la medida se ha ampliado hasta el December 31, 2026. Estar exento de K-ETA no significa automáticamente estar exento de la declaración de llegada. Quien tenga una K-ETA válida está exento de la declaración de llegada. Quien utilice la exención temporal de K-ETA todavía puede tener que presentar una e-Arrival Card, según su situación. La e-Arrival Card oficial es gratuita y puede enviarse dentro de los tres días anteriores a la llegada a Korea. El sitio oficial también incluye un asistente que indica si necesitas presentarla. El sitio oficial de e-Arrival Card no cobra ninguna tarifa. No introduzcas datos de pago en un sitio que afirme vender la tarjeta de llegada coreana. K-ETA oficial ↗ e-Arrival Card oficial ↗
  ```
- Protected tokens: `02`, `K-ETA`, `e-Arrival Card`, `Korea`, `December 31, 2026`

### ITEM 151

- Source context: L210 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Inmigración
  ```
- Protected tokens: None identified in this item.

### ITEM 152

- Source context: L211 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Ten preparados el pasaporte y los documentos de entrada que correspondan a tu situación de viaje. Las reglas no son iguales para todos los pasaportes, visados o estatus de residencia, así que consulta la información oficial coreana antes del vuelo en lugar de copiar la lista de otro viajero.
  ```
- Protected tokens: None identified in this item.

### ITEM 153

- Source context: L213 - `h4`
- Element/type: H4
- Spanish:

  ```text
  K-ETA y e-Arrival Card no son lo mismo
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 154

- Source context: L214 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La exención temporal de K-ETA de Korea para los países ya cubiertos por la medida se ha ampliado hasta el December 31, 2026. Estar exento de K-ETA no significa automáticamente estar exento de la declaración de llegada.
  ```
- Protected tokens: `Korea`, `K-ETA`, `December 31, 2026`

### ITEM 155

- Source context: L215 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Quien tenga una K-ETA válida está exento de la declaración de llegada. Quien utilice la exención temporal de K-ETA todavía puede tener que presentar una e-Arrival Card, según su situación.
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 156

- Source context: L216 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  La e-Arrival Card oficial es gratuita y puede enviarse dentro de los tres días anteriores a la llegada a Korea. El sitio oficial también incluye un asistente que indica si necesitas presentarla.
  ```
- Protected tokens: `e-Arrival Card`, `Korea`

### ITEM 157

- Source context: L217 - `p.arrival-entry-note__warning:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  El sitio oficial de e-Arrival Card no cobra ninguna tarifa. No introduzcas datos de pago en un sitio que afirme vender la tarjeta de llegada coreana.
  ```
- Protected tokens: `e-Arrival Card`

### ITEM 158

- Source context: L218 - `p.arrival-context-links:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  K-ETA oficial ↗ e-Arrival Card oficial ↗
  ```
- Protected tokens: `K-ETA`, `e-Arrival Card`

### ITEM 159

- Source context: L219 - `a:nth-of-type(1) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Sitio oficial de K-ETA, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `K-ETA`

### ITEM 160

- Source context: L220 - `a:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Sitio oficial de Korea e-Arrival Card, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 161

- Source context: L225 - `li.arrival-step:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  03 Recogida de equipaje. Después de inmigración, comprueba en las pantallas del aeropuerto la cinta asignada a tu vuelo y recoge el equipaje facturado antes de continuar hacia aduanas. Si tu maleta no aparece, primero confirma que la cinta sigue correspondiendo a tu vuelo. Si realmente falta o está dañada, habla con tu aerolínea o con el mostrador de equipajes mientras todavía estás dentro de la zona de recogida. Conserva la etiqueta de equipaje del check-in y los datos del vuelo.
  ```
- Protected tokens: `03`

### ITEM 162

- Source context: L228 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Recogida de equipaje
  ```
- Protected tokens: None identified in this item.

### ITEM 163

- Source context: L229 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Después de inmigración, comprueba en las pantallas del aeropuerto la cinta asignada a tu vuelo y recoge el equipaje facturado antes de continuar hacia aduanas.
  ```
- Protected tokens: None identified in this item.

### ITEM 164

- Source context: L230 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si tu maleta no aparece, primero confirma que la cinta sigue correspondiendo a tu vuelo. Si realmente falta o está dañada, habla con tu aerolínea o con el mostrador de equipajes mientras todavía estás dentro de la zona de recogida. Conserva la etiqueta de equipaje del check-in y los datos del vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 165

- Source context: L233 - `li.arrival-step:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  04 Aduanas. Después de recoger el equipaje, sigue las señales de aduanas hacia la salida. Si llevas artículos que puedan requerir declaración, consulta Korea Customs en lugar de basarte en las normas de otro país. Los artículos que superen la franquicia libre de impuestos y los bienes restringidos pueden requerir declaración. Si no tienes nada que declarar, sigue el canal de aduanas correspondiente indicado en el aeropuerto. Korea Customs Service — Travelers ↗
  ```
- Protected tokens: `04`, `Korea`, `Korea Customs Service`

### ITEM 166

- Source context: L236 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Aduanas
  ```
- Protected tokens: None identified in this item.

### ITEM 167

- Source context: L237 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Después de recoger el equipaje, sigue las señales de aduanas hacia la salida.
  ```
- Protected tokens: None identified in this item.

### ITEM 168

- Source context: L238 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si llevas artículos que puedan requerir declaración, consulta Korea Customs en lugar de basarte en las normas de otro país. Los artículos que superen la franquicia libre de impuestos y los bienes restringidos pueden requerir declaración.
  ```
- Protected tokens: `Korea`

### ITEM 169

- Source context: L239 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Si no tienes nada que declarar, sigue el canal de aduanas correspondiente indicado en el aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 170

- Source context: L240 - `p.arrival-context-link:nth-of-type(4)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Korea Customs Service — Viajeros ↗
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 171

- Source context: L240 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Información para viajeros de Korea Customs Service, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 172

- Source context: L243 - `li.arrival-step:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  05 Sala de llegadas. Una vez que pasas aduanas y entras en la sala pública de llegadas, la parte de inmigración del viaje ha terminado. Antes de dirigirte a la ciudad, quizá todavía necesites configurar los datos móviles, guardar los datos coreanos del alojamiento, comprobar una alternativa de pago o decidir cómo llegar hasta donde te alojas. Esos primeros pasos prácticos se explican por separado en la Guía de la Sala de Llegadas de Incheon Airport.
  ```
- Protected tokens: `05`, `Incheon Airport`

### ITEM 173

- Source context: L246 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Sala de llegadas
  ```
- Protected tokens: None identified in this item.

### ITEM 174

- Source context: L247 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Una vez que pasas aduanas y entras en la sala pública de llegadas, la parte de inmigración del viaje ha terminado.
  ```
- Protected tokens: None identified in this item.

### ITEM 175

- Source context: L248 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Antes de dirigirte a la ciudad, quizá todavía necesites configurar los datos móviles, guardar los datos coreanos del alojamiento, comprobar una alternativa de pago o decidir cómo llegar hasta donde te alojas. Esos primeros pasos prácticos se explican por separado en la Guía de la Sala de Llegadas de Incheon Airport.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 176

- Source context: L259 - `h2#terminal-title`
- Element/type: H2
- Spanish:

  ```text
  Terminal 1 y Terminal 2 son edificios separados
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 177

- Source context: L260 - `p`
- Element/type: Body text
- Spanish:

  ```text
  No necesitas memorizar una lista fija de aerolíneas para cada terminal. Comprueba la terminal indicada para tu número de vuelo real y después usa el mapa para orientarte tras aterrizar.
  ```
- Protected tokens: None identified in this item.

### ITEM 178

- Source context: L265 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 179

- Source context: L266 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Terminal 1 es una terminal de pasajeros independiente, con su propia inmigración, recogida de equipaje y sala de llegadas. Sigue las señales correspondientes al vuelo en el que realmente llegaste en lugar de utilizar una lista de aerolíneas guardada.
  ```
- Protected tokens: `Terminal 1`

### ITEM 180

- Source context: L269 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 181

- Source context: L270 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Terminal 2 tiene sus propias instalaciones de llegada y no está conectada con Terminal 1 mediante una ruta peatonal pública. Si necesitas desplazarte entre terminales después de entrar en la zona pública, utiliza la información actual del aeropuerto sobre transporte entre terminales.
  ```
- Protected tokens: `Terminal 2`, `Terminal 1`

### ITEM 182

- Source context: L271 - `p.arrival-context-link:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Incheon Airport — Transporte entre terminales ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 183

- Source context: L271 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Información de Incheon Airport sobre transporte entre terminales, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Incheon Airport`

### ITEM 184

- Source context: L273 - `p.arrival-terminal-orientation`
- Element/type: Body text
- Spanish:

  ```text
  El mapa sirve para entender dónde se encuentran inmigración, recogida de equipaje, aduanas y la sala pública de llegadas entre sí. Para información en tiempo real sobre puertas, cintas o instalaciones, sigue la señalización actual del aeropuerto y la información de vuelos.
  ```
- Protected tokens: None identified in this item.

### ITEM 185

- Source context: L276 - `a.arrival-terminal-map__media @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Abrir a tamaño completo en una pestaña nueva el mapa de llegadas de Incheon Airport Terminal 1 y Terminal 2
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`

### ITEM 186

- Source context: L277 - `img.arrival-terminal-map__image @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Mapas combinados de llegadas de Incheon Airport Terminal 1 y Terminal 2 que muestran inmigración, recogida de equipaje, aduanas, salas de llegadas, transporte y ubicaciones de servicios
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`

### ITEM 187

- Source context: L279 - `figcaption.arrival-terminal-map__caption`
- Element/type: Figcaption
- Spanish:

  ```text
  Mapas de llegadas de Terminal 1 y Terminal 2. Úsalos para orientarte; la señalización y la información de vuelos en tiempo real del aeropuerto tienen prioridad. Abrir el mapa de la terminal a tamaño completo ↗
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 188

- Source context: L290 - `h2#arrival-handoff-title`
- Element/type: H2
- Spanish:

  ```text
  Ya has terminado — ¿qué viene ahora?
  ```
- Protected tokens: None identified in this item.

### ITEM 189

- Source context: L291 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Cuando ya estás en la sala pública de llegadas, el proceso de entrada ha quedado atrás. Lo siguiente depende de tu viaje: conectar el teléfono, guardar la dirección del alojamiento en coreano, tener una alternativa de pago y elegir una ruta que siga siendo práctica con tu equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 190

- Source context: L292 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Continúa con la Guía de la Sala de Llegadas de Incheon Airport para esos primeros pasos prácticos.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 191

- Source context: L293 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Si la única duda que te queda es cómo llegar a tu alojamiento, la Guía de Traslados desde el Aeropuerto compara AREX, autobús del aeropuerto, taxi y traslado reservado con antelación según el viaje completo puerta a puerta.
  ```
- Protected tokens: `AREX`

### ITEM 192

- Source context: L300 - `h2#faq-title`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes sobre la llegada a Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 193

- Source context: L304 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo sé si mi vuelo llega a Terminal 1 o Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 194

- Source context: L305 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Consulta tu billete electrónico o busca el número de vuelo en la página oficial de llegadas de Incheon Airport. Es más seguro que depender de una lista fija de aerolíneas, especialmente en vuelos de código compartido.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 195

- Source context: L308 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Paso por inmigración antes de recoger el equipaje?
  ```
- Protected tokens: None identified in this item.

### ITEM 196

- Source context: L309 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sí, si vas a entrar en Korea. El orden habitual es inmigración, recogida de equipaje, aduanas y después la sala pública de llegadas. Los pasajeros que conectan con otro vuelo deben seguir en su lugar las señales de Transfer o Connecting Flights y las instrucciones de su aerolínea.
  ```
- Protected tokens: `Korea`

### ITEM 197

- Source context: L312 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué debo hacer si mi equipaje no llega?
  ```
- Protected tokens: None identified in this item.

### ITEM 198

- Source context: L313 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Primero comprueba que la cinta de equipaje sigue correspondiendo a tu vuelo. Si la maleta falta o está dañada, contacta con tu aerolínea o con el mostrador de equipajes antes de salir de la zona de recogida y ten a mano la etiqueta del equipaje y los datos del vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 199

- Source context: L316 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo caminar entre Terminal 1 y Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 200

- Source context: L317 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. Terminal 1 y Terminal 2 son edificios separados y no existe una ruta peatonal pública entre ellos. Si necesitas cambiar de terminal en la zona pública, utiliza las indicaciones oficiales del aeropuerto sobre transporte entre terminales.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 201

- Source context: L320 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué transporte debo elegir de Incheon Airport a Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 202

- Source context: L321 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Toma esa decisión cuando llegues a la sala pública de llegadas. La mejor opción depende del alojamiento exacto, la hora de llegada, el equipaje y el tamaño del grupo. La Guía de Traslados desde el Aeropuerto compara AREX, autobús del aeropuerto, taxi y traslado reservado con antelación según el trayecto completo hasta tu alojamiento.
  ```
- Protected tokens: `AREX`

### ITEM 203

- Source context: L324 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué deben hacer los pasajeros en conexión?
  ```
- Protected tokens: None identified in this item.

### ITEM 204

- Source context: L325 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sigue las señales de Transfer o Connecting Flights y las instrucciones de tu aerolínea. Confirma la terminal de conexión, el tiempo disponible y si el equipaje facturado se transfiere al siguiente vuelo, en lugar de asumir que debes seguir el recorrido general de inmigración.
  ```
- Protected tokens: None identified in this item.

### ITEM 205

- Source context: L334 - `h2#official-sources-title`
- Element/type: H2
- Spanish:

  ```text
  Fuentes oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 206

- Source context: L335 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Las normas de entrada y las operaciones del aeropuerto pueden cambiar. Usa estas fuentes oficiales cuando un detalle afecte a tu propio viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 207

- Source context: L338 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Incheon Airport — Procedimientos de llegada ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 208

- Source context: L338 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Procedimientos de llegada de Incheon Airport, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Incheon Airport`

### ITEM 209

- Source context: L339 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Incheon Airport — Vuelos de llegada ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 210

- Source context: L339 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Vuelos de llegada de Incheon Airport, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Incheon Airport`

### ITEM 211

- Source context: L340 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Incheon Airport — Transporte entre terminales ↗
  ```
- Protected tokens: `Incheon Airport`

### ITEM 212

- Source context: L340 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Transporte entre terminales de Incheon Airport, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Incheon Airport`

### ITEM 213

- Source context: L341 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  K-ETA ↗
  ```
- Protected tokens: `K-ETA`

### ITEM 214

- Source context: L341 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Sitio oficial de K-ETA, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `K-ETA`

### ITEM 215

- Source context: L342 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Korea e-Arrival Card oficial ↗
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 216

- Source context: L342 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Sitio oficial de Korea e-Arrival Card, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Korea e-Arrival Card`

### ITEM 217

- Source context: L343 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Korea Customs Service ↗
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 218

- Source context: L343 - `a @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Korea Customs Service, sitio externo, se abre en una pestaña nueva
  ```
- Protected tokens: `Korea Customs Service`

### ITEM 219

- Source context: L348 - `aside.arrival-service-notice @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Aviso de revisión de la información
  ```
- Protected tokens: None identified in this item.

### ITEM 220

- Source context: L350 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Las operaciones del aeropuerto, los requisitos de entrada y los servicios de transporte pueden cambiar. Confirma los detalles sensibles al tiempo en el sitio oficial correspondiente o con tu aerolínea.
  ```
- Protected tokens: None identified in this item.

### ITEM 221

- Source context: L351 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Última revisión: August 2026
  ```
- Protected tokens: `August 2026`

---

# PAGE: `airport-transfer.html`

**English source SHA-256:** `b8649faf6bf1b61b234e0331a83f2d053f3281fb3ea69b95b53a14aeaa559ae4`  
**Localized ITEM count:** 166

### ITEM 222

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Compara AREX, autobuses del aeropuerto, taxis, Call Van y traslados privados de Incheon Airport a Seoul, con tarifas actuales, equipaje y opciones para llegadas nocturnas.
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 223

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Incheon Airport a Seoul: AREX, autobús, taxi y traslado | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`, `Korea Inside`

### ITEM 224

- Source context: L21 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Es mejor AREX o el autobús del aeropuerto?
  ```
- Protected tokens: `AREX`

### ITEM 225

- Source context: L24 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  AREX es más predecible y funciona especialmente bien para Seoul Station o Hongdae. El autobús del aeropuerto puede ser más fácil cuando su parada real está cerca del hotel y evita otro transbordo en estación. La caminata final importa tanto como el tiempo de viaje anunciado.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 226

- Source context: L29 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cuál es la forma más barata de ir de Incheon Airport a Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 227

- Source context: L32 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  El AREX All-stop Train suele ser la opción ferroviaria más barata. Las tarifas actuales para adultos con tarjeta de transporte son ₩4,650 desde T1 y ₩5,250 desde T2 hasta Hongik University, y ₩4,750 desde T1 y ₩5,350 desde T2 hasta Seoul Station.
  ```
- Protected tokens: `AREX All-stop Train`, `₩4,650`, `T1`, `₩5,250`, `T2`, `Hongik University`, `₩4,750`, `₩5,350`, `Seoul Station`

### ITEM 228

- Source context: L37 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cuál es la forma más rápida de llegar a Seoul Station?
  ```
- Protected tokens: `Seoul Station`

### ITEM 229

- Source context: L40 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  El AREX Express publica tiempos de viaje desde el aeropuerto hasta Seoul Station de 43 minutes desde T1 y 51 minutes desde T2. El viaje completo todavía incluye la caminata hasta la estación del aeropuerto y todo lo que venga después de Seoul Station.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 230

- Source context: L45 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿El AREX Express para en Hongdae?
  ```
- Protected tokens: `AREX Express`, `Hongdae`

### ITEM 231

- Source context: L48 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No. El AREX Express va directamente a Seoul Station y no para en Hongik University. El All-stop Train sí para en Hongik University y suele ser la opción ferroviaria más directa para Hongdae.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `Hongik University`, `All-stop Train`, `Hongdae`

### ITEM 232

- Source context: L53 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Puedo usar T-money en AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 233

- Source context: L56 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  T-money puede utilizarse en el AREX All-stop Train. El AREX Express utiliza un billete independiente y un sistema de asientos reservados.
  ```
- Protected tokens: `T-money`, `AREX All-stop Train`, `AREX Express`

### ITEM 234

- Source context: L61 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué funciona mejor con varias maletas grandes?
  ```
- Protected tokens: None identified in this item.

### ITEM 235

- Source context: L64 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  La respuesta cambia según el número y tamaño de las maletas. Un autobús del aeropuerto puede funcionar bien cuando su parada está cerca del hotel, mientras que un taxi más grande, un Call Van o un traslado reservado con antelación resultan más útiles cuando mover el equipaje por una estación sería difícil. La capacidad de pasajeros y la capacidad de equipaje son límites distintos.
  ```
- Protected tokens: None identified in this item.

### ITEM 236

- Source context: L69 - `script[type="application/ld+json"] > mainEntity.6.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué puedo usar después de medianoche?
  ```
- Protected tokens: None identified in this item.

### ITEM 237

- Source context: L72 - `script[type="application/ld+json"] > mainEntity.6.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Incheon Airport incluye actualmente servicios nocturnos como N6000, N6002, N6701 y N6703, con horarios de salida diferentes en T1 y T2. Los taxis oficiales y los vehículos reservados con antelación son alternativas cuando ya ha terminado la franja útil de trenes o autobuses nocturnos.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `T1`, `T2`

### ITEM 238

- Source context: L77 - `script[type="application/ld+json"] > mainEntity.7.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Cómo funcionan los taxis desde Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 239

- Source context: L80 - `script[type="application/ld+json"] > mainEntity.7.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Las paradas oficiales de taxis del aeropuerto ofrecen servicios regular, Deluxe, Jumbo e International Taxi. Un taxi regular de Seoul parte actualmente de ₩4,800 por 1.6 km, mientras que los servicios Deluxe, SUV y Jumbo parten de ₩7,000 por 3 km. La tarifa final depende del servicio, el destino, el tráfico y la hora del viaje.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 240

- Source context: L85 - `script[type="application/ld+json"] > mainEntity.8.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Terminal 1 o Terminal 2 cambian la ruta?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 241

- Source context: L88 - `script[type="application/ld+json"] > mainEntity.8.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Las principales opciones de transporte son similares, pero las zonas de venta de billetes, los niveles ferroviarios, las paradas de taxi y los puntos de Call Van son diferentes. T1 y T2 también tienen horarios de salida distintos para algunos servicios nocturnos.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 242

- Source context: L93 - `script[type="application/ld+json"] > mainEntity.9.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Call Van es lo mismo que un traslado privado?
  ```
- Protected tokens: None identified in this item.

### ITEM 243

- Source context: L96 - `script[type="application/ld+json"] > mainEntity.9.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No. El Call Van de Incheon Airport es un servicio comercial coreano de furgonetas con sus propias condiciones aeroportuarias. “Traslado privado” es un término de reserva más amplio para un vehículo reservado para un solo grupo. Algunos traslados privados pueden utilizar furgonetas, pero los términos no son intercambiables.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 244

- Source context: L169 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Traslado desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 245

- Source context: L170 - `h1.airport-page-hero__title`
- Element/type: H1
- Spanish:

  ```text
  Incheon Airport a Seoul: ¿qué traslado conviene más?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 246

- Source context: L171 - `p.transfer-review-date:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Tarifas oficiales y datos operativos comprobados a fecha de August 18, 2026.
  ```
- Protected tokens: `August 18, 2026`

### ITEM 247

- Source context: L173 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Dónde te alojas importa más que el tiempo de viaje anunciado. El AREX Express encaja muy bien con Seoul Station, mientras que el All-stop Train llega a Hongik University para Hongdae. Un autobús limusina del aeropuerto puede ser más fácil cuando su parada está cerca del hotel.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `All-stop Train`, `Hongik University`, `Hongdae`

### ITEM 248

- Source context: L174 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Con una familia, varias maletas grandes o una llegada tardía, el equilibrio cambia. Un taxi, Call Van o traslado privado reservado con antelación cuesta más, pero puede eliminar el transbordo en estación, las escaleras y la caminata final que hacen que un viaje desde el aeropuerto se sienta mucho más largo de lo que parece en el horario.
  ```
- Protected tokens: None identified in this item.

### ITEM 249

- Source context: L183 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Importa el viaje completo
  ```
- Protected tokens: None identified in this item.

### ITEM 250

- Source context: L184 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un trayecto ferroviario de 43 minutos termina en Seoul Station, no en tu hotel. Si después viene otro transbordo de metro, escaleras o una caminata larga, el primer tramo más rápido puede no ser la llegada más fácil.
  ```
- Protected tokens: `43`, `Seoul Station`

### ITEM 251

- Source context: L185 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  La misma ruta también puede sentirse muy distinta con equipaje. Una maleta suele ser manejable. Varias maletas grandes, un cochecito o el equipaje de toda una familia pueden convertir un simple transbordo en estación en la parte más difícil del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 252

- Source context: L186 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Por eso la comparación útil llega hasta la puerta del hotel: dónde te deja el tren o autobús, cuánto equipaje llevas, cuántas personas comparten el viaje y a qué hora estás realmente listo para salir del aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 253

- Source context: L194 - `h2#transfer-comparison-heading`
- Element/type: H2
- Spanish:

  ```text
  Incheon Airport a Seoul de un vistazo
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 254

- Source context: L195 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Estas son las tarifas publicadas actualmente y las diferencias prácticas que más importan. Las tarifas de autobús varían según la ruta y el operador, mientras que el total de taxi y traslado privado depende del destino, el vehículo y las condiciones del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 255

- Source context: L202 - `th:nth-of-type(1)`
- Element/type: Table header
- Spanish:

  ```text
  Opción
  ```
- Protected tokens: None identified in this item.

### ITEM 256

- Source context: L203 - `th:nth-of-type(2)`
- Element/type: Table header
- Spanish:

  ```text
  Tarifa actual
  ```
- Protected tokens: None identified in this item.

### ITEM 257

- Source context: L204 - `th:nth-of-type(3)`
- Element/type: Table header
- Spanish:

  ```text
  Cómo es el trayecto
  ```
- Protected tokens: None identified in this item.

### ITEM 258

- Source context: L205 - `th:nth-of-type(4)`
- Element/type: Table header
- Spanish:

  ```text
  Limitación principal
  ```
- Protected tokens: None identified in this item.

### ITEM 259

- Source context: L210 - `th`
- Element/type: Table header
- Spanish:

  ```text
  AREX Express
  ```
- Protected tokens: `AREX Express`

### ITEM 260

- Source context: L211 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Adulto ₩13,000 · Niño ₩9,500
  ```
- Protected tokens: `₩13,000`, `₩9,500`

### ITEM 261

- Source context: L212 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Tren directo sin paradas a Seoul Station con asiento reservado. El tiempo de viaje publicado es de 43 minutes desde T1 y 51 minutes desde T2.
  ```
- Protected tokens: `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 262

- Source context: L213 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  No para en Hongik University, y el hotel todavía puede requerir otro transbordo o una caminata.
  ```
- Protected tokens: `Hongik University`

### ITEM 263

- Source context: L216 - `th`
- Element/type: Table header
- Spanish:

  ```text
  AREX All-stop
  ```
- Protected tokens: `AREX All-stop`

### ITEM 264

- Source context: L217 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Seoul Station: ₩4,750 desde T1 / ₩5,350 desde T2 · Hongik University: ₩4,650 desde T1 / ₩5,250 desde T2
  ```
- Protected tokens: `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`, `Hongik University`, `₩4,650`, `₩5,250`

### ITEM 265

- Source context: L218 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Servicio ferroviario directo a Hongik University y Seoul Station, usando una tarjeta de transporte o un billete de un solo uso.
  ```
- Protected tokens: `Hongik University`, `Seoul Station`

### ITEM 266

- Source context: L219 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Es más lento, no tiene asientos reservados y llevas tu equipaje contigo.
  ```
- Protected tokens: None identified in this item.

### ITEM 267

- Source context: L222 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Autobús limusina del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 268

- Source context: L223 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Las principales rutas de Seoul cuestan actualmente ₩16,000 to ₩18,000 para adultos. En los ejemplos comprobados, la tarifa infantil es de ₩12,000.
  ```
- Protected tokens: `Seoul`, `₩16,000 to ₩18,000`, `₩12,000`

### ITEM 269

- Source context: L224 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Equipaje almacenado y menos transbordos en estación cuando la parada del autobús está cerca del hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 270

- Source context: L225 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  El tráfico varía, y la caminata final desde la parada real puede cambiar todo el viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 271

- Source context: L228 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Taxi regular / Deluxe-Jumbo
  ```
- Protected tokens: None identified in this item.

### ITEM 272

- Source context: L229 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  El taxi regular parte de ₩4,800 por 1.6 km. Los servicios Deluxe, SUV y Jumbo parten de ₩7,000 por 3 km.
  ```
- Protected tokens: `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 273

- Source context: L230 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Trayecto puerta a puerta desde la parada oficial de taxis del aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 274

- Source context: L231 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  El total depende del destino, el tráfico, la hora y el tipo de vehículo. Tener suficientes asientos no garantiza espacio suficiente para el equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 275

- Source context: L234 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Official Airport Call Van
  ```
- Protected tokens: `Official Airport Call Van`

### ITEM 276

- Source context: L235 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Tarifa según distancia; los peajes se cobran aparte.
  ```
- Protected tokens: None identified in this item.

### ITEM 277

- Source context: L236 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Servicio comercial de furgoneta descrito por Incheon Airport para grupos de cinco personas o menos con 20 kg de equipaje por persona.
  ```
- Protected tokens: `Incheon Airport`, `20 kg`

### ITEM 278

- Source context: L237 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  El horario publicado es 08:00–21:00 y se aplican las condiciones de elegibilidad del aeropuerto.
  ```
- Protected tokens: `08:00`, `21:00`

### ITEM 279

- Source context: L240 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Traslado privado reservado con antelación
  ```
- Protected tokens: None identified in this item.

### ITEM 280

- Source context: L241 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Depende del proveedor.
  ```
- Protected tokens: None identified in this item.

### ITEM 281

- Source context: L242 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Vehículo reservado con antelación para un solo grupo, con un punto de encuentro acordado antes de llegar.
  ```
- Protected tokens: None identified in this item.

### ITEM 282

- Source context: L243 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  El tamaño del vehículo, el equipaje, el tiempo de espera y las condiciones de cancelación dependen de cada reserva.
  ```
- Protected tokens: None identified in this item.

### ITEM 283

- Source context: L249 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Las tarifas y los datos operativos se comprobaron con los operadores de transporte correspondientes y con Incheon Airport el August 18, 2026. Las rutas, horarios y condiciones del servicio pueden cambiar.
  ```
- Protected tokens: `Incheon Airport`, `August 18, 2026`

### ITEM 284

- Source context: L256 - `h2`
- Element/type: H2
- Spanish:

  ```text
  AREX: Seoul Station y Hongdae son viajes distintos
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 285

- Source context: L258 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  El Express está pensado alrededor de Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 286

- Source context: L259 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El AREX Express tiene más sentido cuando Seoul Station es realmente útil para el resto del viaje. Va sin paradas desde el aeropuerto y ofrece asientos reservados. La tarifa de venta actual es de ₩13,000 para adultos y ₩9,500 para niños, con tiempos publicados de 43 minutes desde T1 y 51 minutes desde T2.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `₩13,000`, `₩9,500`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 287

- Source context: L260 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Las primeras salidas actuales son a las 05:16 desde T2 y a las 05:24 desde T1. Las últimas son a las 22:40 desde T2 y a las 22:48 desde T1. Aun así, esos horarios tienen que encajar con inmigración, recogida de equipaje y la caminata desde la sala de llegadas hasta la estación del aeropuerto.
  ```
- Protected tokens: `05:16`, `T2`, `05:24`, `T1`, `22:40`, `22:48`

### ITEM 288

- Source context: L262 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Hongdae es diferente
  ```
- Protected tokens: `Hongdae`

### ITEM 289

- Source context: L263 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Para Hongdae, el All-stop Train suele ser el trayecto ferroviario más directo porque para en Hongik University. Las tarifas actuales para adultos con tarjeta de transporte son ₩4,650 desde T1 y ₩5,250 desde T2 hasta Hongik University.
  ```
- Protected tokens: `Hongdae`, `All-stop Train`, `Hongik University`, `₩4,650`, `T1`, `₩5,250`, `T2`

### ITEM 290

- Source context: L264 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Hasta Seoul Station, las tarifas equivalentes son ₩4,750 desde T1 y ₩5,350 desde T2. Los tiempos de viaje publicados son de unos 59 minutes desde T1 y 66 minutes desde T2, aunque algunos trenes tardan unos minutos más.
  ```
- Protected tokens: `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`, `59 minutes`, `66 minutes`

### ITEM 291

- Source context: L266 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  El equipaje cambia la experiencia
  ```
- Protected tokens: None identified in this item.

### ITEM 292

- Source context: L267 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Las normas de transporte de AREX permiten como máximo dos piezas por pasajero, cada una de menos de 32 kg y menos de 158 cm en dimensiones totales. Incluso dentro de esos límites, varias maletas grandes pueden hacer que el transbordo en estación y la caminata final cansen más que el propio viaje en tren.
  ```
- Protected tokens: `AREX`, `32 kg`, `158`

### ITEM 293

- Source context: L268 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  Para detalles sobre billetes, acceso a estaciones y la diferencia entre ambos trenes, consulta la guía completa de AREX. La guía de T-money explica el uso de la tarjeta de transporte en el All-stop Train.
  ```
- Protected tokens: `AREX`, `T-money`, `All-stop Train`

### ITEM 294

- Source context: L276 - `h2`
- Element/type: H2
- Spanish:

  ```text
  El autobús del aeropuerto es más fácil cuando la parada está cerca
  ```
- Protected tokens: None identified in this item.

### ITEM 295

- Source context: L277 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús limusina del aeropuerto puede eliminar un transbordo en estación y ofrecer un compartimento específico para maletas grandes. Lo importante no es el nombre del barrio en el mapa de la ruta, sino dónde te deja realmente el autobús.
  ```
- Protected tokens: None identified in this item.

### ITEM 296

- Source context: L278 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Una parada a una manzana del hotel puede hacer que el autobús sea mucho más fácil que el tren. Una parada al otro lado de una avenida ancha, varias manzanas cuesta arriba o en el lado equivocado de una intersección complicada puede eliminar esa ventaja.
  ```
- Protected tokens: None identified in this item.

### ITEM 297

- Source context: L280 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Las tarifas actuales varían según la ruta
  ```
- Protected tokens: None identified in this item.

### ITEM 298

- Source context: L281 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Las rutas 6001, 6002 y 6015 de Airport Limousine cuestan actualmente ₩17,000 para adultos y ₩12,000 para niños. La ruta 6003 cuesta ₩16,000 para adultos y ₩12,000 para niños. Las rutas de K Airport Limousine por el centro de Seoul cuestan actualmente ₩18,000 para adultos y ₩12,000 para niños.
  ```
- Protected tokens: `6001`, `6002`, `6015`, `₩17,000`, `₩12,000`, `6003`, `₩16,000`, `K Airport Limousine`, `Seoul`, `₩18,000`

### ITEM 299

- Source context: L282 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Son ejemplos actuales, no una tarifa universal para los autobuses del aeropuerto de Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 300

- Source context: L284 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Las normas de equipaje no son idénticas
  ```
- Protected tokens: None identified in this item.

### ITEM 301

- Source context: L285 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Airport Limousine describe actualmente como equipaje gratuito dos piezas de hasta 28 inches y 20 kg cada una, o una pieza de más de 28 inches.
  ```
- Protected tokens: `28 inches`, `20 kg`

### ITEM 302

- Source context: L286 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  Las FAQ en inglés y las condiciones de transporte de K Airport Limousine no son totalmente coherentes entre sí, por lo que un equipaje especialmente grande o pesado debe tratarse como una condición específica del operador, no como una única regla para todos los autobuses del aeropuerto.
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 303

- Source context: L288 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  Terminal 2 tiene un procedimiento de billetes diferente
  ```
- Protected tokens: `Terminal 2`

### ITEM 304

- Source context: L289 - `p:nth-of-type(7)`
- Element/type: Body text
- Spanish:

  ```text
  En T1, la venta de billetes del autobús del aeropuerto está en la planta de llegadas. En T2, la venta de billetes y el embarque se concentran en Transportation Center B1.
  ```
- Protected tokens: `T1`, `T2`, `Transportation Center`, `B1`

### ITEM 305

- Source context: L290 - `p:nth-of-type(8)`
- Element/type: Body text
- Spanish:

  ```text
  Desde March 5, 2026, los servicios de Airport Limousine con destino a Seoul desde T2 requieren comprar un billete en el mostrador atendido o en una máquina antes de subir.
  ```
- Protected tokens: `March 5, 2026`, `Seoul`, `T2`

### ITEM 306

- Source context: L291 - `p:nth-of-type(9)`
- Element/type: Body text
- Spanish:

  ```text
  El tráfico sigue siendo la principal desventaja. Un autobús puede ofrecer el trayecto puerta a puerta más sencillo y aun así tardar más cuando las carreteras están congestionadas.
  ```
- Protected tokens: None identified in this item.

### ITEM 307

- Source context: L292 - `p:nth-of-type(10)`
- Element/type: Body text
- Spanish:

  ```text
  La guía del autobús del aeropuerto explica con más detalle las rutas y las paradas de cada terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 308

- Source context: L300 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Taxi: puerta a puerta, hasta que el equipaje cambia el vehículo
  ```
- Protected tokens: None identified in this item.

### ITEM 309

- Source context: L301 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un taxi elimina por completo el transbordo en estación. En las paradas oficiales del aeropuerto, un taxi regular de Seoul parte actualmente de ₩4,800 por los primeros 1.6 km. Los servicios Deluxe, SUV y Jumbo parten de ₩7,000 por los primeros 3 km.
  ```
- Protected tokens: `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 310

- Source context: L302 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Los taxis regulares aplican un recargo nocturno del 20% entre 22:00–23:00, del 40% entre 23:00–02:00 y del 20% entre 02:00–04:00. Los servicios Deluxe, SUV y van aplican un recargo nocturno del 20% entre 22:00–04:00.
  ```
- Protected tokens: `20`, `22:00`, `23:00`, `40`, `02:00`, `04:00`, `SUV`

### ITEM 311

- Source context: L303 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  El número de pasajeros es solo la mitad del problema de espacio. Cuatro personas pueden caber en un coche y cuatro maletas grandes no. Las categorías de taxi más grandes resultan más útiles cuando el límite real es el equipaje, no los asientos.
  ```
- Protected tokens: None identified in this item.

### ITEM 312

- Source context: L305 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Paradas oficiales de taxi
  ```
- Protected tokens: None identified in this item.

### ITEM 313

- Source context: L306 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  En T1, los taxis regulares de Seoul usan 5C, 6C y 6D. Los taxis Deluxe y Jumbo usan 7C y 8C, mientras que International Taxi usa 4C.
  ```
- Protected tokens: `T1`, `Seoul`, `5C`, `6C`, `7C`, `8C`, `International Taxi`, `4C`

### ITEM 314

- Source context: L307 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  En T2, los taxis regulares de Seoul usan 7C, los taxis Deluxe y Jumbo usan 7D e International Taxi usa 3C.
  ```
- Protected tokens: `T2`, `Seoul`, `7C`, `7D`, `International Taxi`, `3C`

### ITEM 315

- Source context: L309 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  International Taxi
  ```
- Protected tokens: `International Taxi`

### ITEM 316

- Source context: L310 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  International Taxi utiliza tarifas por zonas de Seoul en lugar del taxímetro ordinario para su servicio desde el aeropuerto. Los rangos publicados actualmente son ₩70,000–95,000 para sedanes y ₩100,000–140,000 para vehículos más grandes, según la zona de destino.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩70,000–95,000`, `₩100,000–140,000`

### ITEM 317

- Source context: L311 - `p:nth-of-type(7)`
- Element/type: Body text
- Spanish:

  ```text
  La información oficial no es totalmente coherente sobre cómo se describen los peajes en todos los servicios de International Taxi, por lo que las condiciones vinculadas a la reserva real son una referencia más segura que aplicar una única regla de peajes a todas las reservas.
  ```
- Protected tokens: `International Taxi`

### ITEM 318

- Source context: L312 - `p:nth-of-type(8)`
- Element/type: Body text
- Spanish:

  ```text
  La guía de taxis explica con más detalle los tipos de taxi de Seoul, los pagos y la estructura de tarifas.
  ```
- Protected tokens: `Seoul`

### ITEM 319

- Source context: L320 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Call Van y traslados privados reservados con antelación
  ```
- Protected tokens: None identified in this item.

### ITEM 320

- Source context: L321 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El Call Van del aeropuerto en Korea es un servicio comercial de furgonetas. No es el nombre genérico en inglés para todos los traslados privados vendidos por internet.
  ```
- Protected tokens: `Korea`

### ITEM 321

- Source context: L322 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Incheon Airport describe actualmente este servicio para grupos de cinco personas o menos con 20 kg de equipaje por persona. El mostrador de información está entre las salidas 12 y 13 en T1 y cerca de la salida 7 en T2. La recogida es en 10C en T1 y 8D en T2. El horario publicado es 08:00–21:00, las tarifas dependen de la distancia y los peajes se cobran aparte.
  ```
- Protected tokens: `Incheon Airport`, `20 kg`, `12`, `13`, `T1`, `7`, `T2`, `10C`, `8D`, `08:00`, `21:00`

### ITEM 322

- Source context: L323 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  “Traslado privado” es el término de reserva más amplio que los viajeros ven en las plataformas internacionales. El vehículo, la franquicia de equipaje, el punto de encuentro, el tiempo de espera y las condiciones de cancelación pertenecen a cada reserva concreta, no al término “traslado privado” en sí.
  ```
- Protected tokens: None identified in this item.

### ITEM 323

- Source context: L324 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Para grupos más grandes, capacidad de equipaje y detalles de la reserva anticipada, consulta la guía de Call Van / Traslado privado.
  ```
- Protected tokens: None identified in this item.

### ITEM 324

- Source context: L332 - `h2`
- Element/type: H2
- Spanish:

  ```text
  En una llegada tardía importa cuándo sales de la terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 325

- Source context: L333 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un vuelo que aterriza tarde por la noche no significa que salgas de la terminal a esa hora. Primero vienen inmigración, recogida de equipaje y aduanas, y ese intervalo importa cuando se acerca el último tren o autobús conveniente.
  ```
- Protected tokens: None identified in this item.

### ITEM 326

- Source context: L334 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Incheon Airport incluye actualmente N6000, N6002, N6701 y N6703 entre sus servicios nocturnos de autobús. N6000 y N6002 cuestan actualmente ₩17,000 para adultos y ₩10,000 para niños. N6701 y N6703 cuestan ₩18,000 para adultos y ₩12,000 para niños.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `₩17,000`, `₩10,000`, `₩18,000`, `₩12,000`

### ITEM 327

- Source context: L335 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  T1 y T2 tienen horarios de salida diferentes, por lo que el horario actual del aeropuerto resulta más útil que una lista estática copiada en un plan de viaje.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 328

- Source context: L336 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Si ya terminó la franja útil de trenes o autobuses nocturnos, la alternativa práctica pasa a ser una parada oficial de taxis o un vehículo reservado con antelación.
  ```
- Protected tokens: None identified in this item.

### ITEM 329

- Source context: L337 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  La Guía de Llegada explica los pasos del aeropuerto que vienen antes de entrar en la sala pública de llegadas.
  ```
- Protected tokens: None identified in this item.

### ITEM 330

- Source context: L345 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Terminal 1 y Terminal 2 funcionan de forma diferente
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 331

- Source context: L346 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Las opciones de transporte son en gran parte las mismas en ambas terminales, pero las zonas de venta de billetes, los niveles ferroviarios y las paradas de vehículos no lo son.
  ```
- Protected tokens: None identified in this item.

### ITEM 332

- Source context: L348 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 333

- Source context: L349 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Los autobuses del aeropuerto salen desde la planta de llegadas. Hay venta de billetes en el interior cerca de las salidas 4 y 9, con instalaciones adicionales en el exterior cerca de las salidas 4, 6, 7, 8, 11 y 13.
  ```
- Protected tokens: `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 334

- Source context: L350 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  La venta de billetes y la información de AREX están en Transportation Center B1, mientras que los trenes utilizan B4.
  ```
- Protected tokens: `AREX`, `Transportation Center`, `B1`, `B4`

### ITEM 335

- Source context: L351 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Los taxis regulares de Seoul usan 5C, 6C y 6D; los taxis Deluxe y Jumbo usan 7C y 8C; International Taxi usa 4C.
  ```
- Protected tokens: `Seoul`, `5C`, `6C`, `7C`, `8C`, `International Taxi`, `4C`

### ITEM 336

- Source context: L352 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  El mostrador de información de Call Van está entre las salidas 12 y 13, y la recogida se realiza en 10C.
  ```
- Protected tokens: `12`, `13`, `10C`

### ITEM 337

- Source context: L354 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 338

- Source context: L355 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  La venta de billetes, la información y el embarque del autobús del aeropuerto están en Transportation Center B1.
  ```
- Protected tokens: `Transportation Center`, `B1`

### ITEM 339

- Source context: L356 - `p:nth-of-type(7)`
- Element/type: Body text
- Spanish:

  ```text
  La venta de billetes y la información de AREX también están en B1, mientras que los trenes utilizan B3.
  ```
- Protected tokens: `AREX`, `B1`, `B3`

### ITEM 340

- Source context: L357 - `p:nth-of-type(8)`
- Element/type: Body text
- Spanish:

  ```text
  Los taxis regulares de Seoul usan 7C; los taxis Deluxe y Jumbo usan 7D; International Taxi usa 3C.
  ```
- Protected tokens: `Seoul`, `7C`, `7D`, `International Taxi`, `3C`

### ITEM 341

- Source context: L358 - `p:nth-of-type(9)`
- Element/type: Body text
- Spanish:

  ```text
  El mostrador de información de Call Van está cerca de la salida 7, y la recogida se realiza en 8D.
  ```
- Protected tokens: `7`, `8D`

### ITEM 342

- Source context: L359 - `p:nth-of-type(10)`
- Element/type: Body text
- Spanish:

  ```text
  La guía de Incheon Airport ofrece un contexto más amplio de la sala de llegadas para ambas terminales.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 343

- Source context: L367 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Qué traslado desde el aeropuerto encaja con tu viaje?
  ```
- Protected tokens: None identified in this item.

### ITEM 344

- Source context: L368 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Seoul Station es el caso más claro para el AREX Express. Hongdae suele ser más sencillo con el All-stop Train porque para en Hongik University.
  ```
- Protected tokens: `Seoul Station`, `AREX Express`, `Hongdae`, `All-stop Train`, `Hongik University`

### ITEM 345

- Source context: L369 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El autobús del aeropuerto se vuelve atractivo cuando su parada real está cerca del hotel. El taxi resulta más útil cuanto más difícil es gestionar la caminata final, el equipaje o el tamaño del grupo.
  ```
- Protected tokens: None identified in this item.

### ITEM 346

- Source context: L370 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Para una familia, varias maletas grandes o una llegada tardía, un Call Van o un traslado privado reservado con antelación puede justificar el coste extra al eliminar un transbordo en estación y el último kilómetro con equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 347

- Source context: L371 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  El primer trayecto más rápido no siempre produce la llegada más sencilla. El viaje que importa es el que termina en la puerta del hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 348

- Source context: L372 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Tu elección de transporte y la zona del hotel están relacionadas. Si aún no has fijado el alojamiento, compara Hongdae, Gongdeok, Seoul Station y Myeongdong según la gestión del equipaje, la caminata final y el resto de tu itinerario por Seoul.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`, `Seoul`

### ITEM 349

- Source context: L373 - `p:nth-of-type(6)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar dónde alojarse para facilitar el acceso al aeropuerto →
  ```
- Protected tokens: None identified in this item.

### ITEM 350

- Source context: L381 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes
  ```
- Protected tokens: None identified in this item.

### ITEM 351

- Source context: L385 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Es mejor AREX o el autobús del aeropuerto?
  ```
- Protected tokens: `AREX`

### ITEM 352

- Source context: L386 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  AREX es más predecible y funciona especialmente bien para Seoul Station o Hongdae. El autobús del aeropuerto puede ser más fácil cuando su parada real está cerca del hotel y evita otro transbordo en estación. La caminata final importa tanto como el tiempo de viaje anunciado.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`

### ITEM 353

- Source context: L389 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuál es la forma más barata de ir de Incheon Airport a Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 354

- Source context: L390 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  El AREX All-stop Train suele ser la opción ferroviaria más barata. Las tarifas actuales para adultos con tarjeta de transporte son ₩4,650 desde T1 y ₩5,250 desde T2 hasta Hongik University, y ₩4,750 desde T1 y ₩5,350 desde T2 hasta Seoul Station.
  ```
- Protected tokens: `AREX All-stop Train`, `₩4,650`, `T1`, `₩5,250`, `T2`, `Hongik University`, `₩4,750`, `₩5,350`, `Seoul Station`

### ITEM 355

- Source context: L393 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuál es la forma más rápida de llegar a Seoul Station?
  ```
- Protected tokens: `Seoul Station`

### ITEM 356

- Source context: L394 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  El AREX Express publica tiempos de viaje desde el aeropuerto hasta Seoul Station de 43 minutes desde T1 y 51 minutes desde T2. El viaje completo todavía incluye la caminata hasta la estación del aeropuerto y todo lo que venga después de Seoul Station.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `43 minutes`, `T1`, `51 minutes`, `T2`

### ITEM 357

- Source context: L397 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿El AREX Express para en Hongdae?
  ```
- Protected tokens: `AREX Express`, `Hongdae`

### ITEM 358

- Source context: L398 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. El AREX Express va directamente a Seoul Station y no para en Hongik University. El All-stop Train sí para en Hongik University y suele ser la opción ferroviaria más directa para Hongdae.
  ```
- Protected tokens: `AREX Express`, `Seoul Station`, `Hongik University`, `All-stop Train`, `Hongdae`

### ITEM 359

- Source context: L401 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo usar T-money en AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 360

- Source context: L402 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  T-money puede utilizarse en el AREX All-stop Train. El AREX Express utiliza un billete independiente y un sistema de asientos reservados.
  ```
- Protected tokens: `T-money`, `AREX All-stop Train`, `AREX Express`

### ITEM 361

- Source context: L405 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué funciona mejor con varias maletas grandes?
  ```
- Protected tokens: None identified in this item.

### ITEM 362

- Source context: L406 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  La respuesta cambia según el número y tamaño de las maletas. Un autobús del aeropuerto puede funcionar bien cuando su parada está cerca del hotel, mientras que un taxi más grande, un Call Van o un traslado reservado con antelación resultan más útiles cuando mover el equipaje por una estación sería difícil. La capacidad de pasajeros y la capacidad de equipaje son límites distintos.
  ```
- Protected tokens: None identified in this item.

### ITEM 363

- Source context: L409 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué puedo usar después de medianoche?
  ```
- Protected tokens: None identified in this item.

### ITEM 364

- Source context: L410 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Incheon Airport incluye actualmente servicios nocturnos como N6000, N6002, N6701 y N6703, con horarios de salida diferentes en T1 y T2. Los taxis oficiales y los vehículos reservados con antelación son alternativas cuando ya ha terminado la franja útil de trenes o autobuses nocturnos.
  ```
- Protected tokens: `Incheon Airport`, `N6000`, `N6002`, `N6701`, `N6703`, `T1`, `T2`

### ITEM 365

- Source context: L413 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo funcionan los taxis desde Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 366

- Source context: L414 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Las paradas oficiales de taxis del aeropuerto ofrecen servicios regular, Deluxe, Jumbo e International Taxi. Un taxi regular de Seoul parte actualmente de ₩4,800 por 1.6 km, mientras que los servicios Deluxe, SUV y Jumbo parten de ₩7,000 por 3 km. La tarifa final depende del servicio, el destino, el tráfico y la hora del viaje.
  ```
- Protected tokens: `International Taxi`, `Seoul`, `₩4,800`, `1.6 km`, `SUV`, `₩7,000`, `3 km`

### ITEM 367

- Source context: L417 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Terminal 1 o Terminal 2 cambian la ruta?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 368

- Source context: L418 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Las principales opciones de transporte son similares, pero las zonas de venta de billetes, los niveles ferroviarios, las paradas de taxi y los puntos de Call Van son diferentes. T1 y T2 también tienen horarios de salida distintos para algunos servicios nocturnos.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 369

- Source context: L421 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Call Van es lo mismo que un traslado privado?
  ```
- Protected tokens: None identified in this item.

### ITEM 370

- Source context: L422 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. El Call Van de Incheon Airport es un servicio comercial coreano de furgonetas con sus propias condiciones aeroportuarias. “Traslado privado” es un término de reserva más amplio para un vehículo reservado para un solo grupo. Algunos traslados privados pueden utilizar furgonetas, pero los términos no son intercambiables.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 371

- Source context: L431 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Fuentes oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 372

- Source context: L432 - `p.transfer-source-intro`
- Element/type: Body text
- Spanish:

  ```text
  Las tarifas, los horarios, las normas de equipaje y los lugares de operación pueden cambiar. Los enlaces siguientes son las fuentes principales utilizadas para la información de esta página.
  ```
- Protected tokens: None identified in this item.

### ITEM 373

- Source context: L435 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Tarifas y tiempos de viaje de AREX Express
  ```
- Protected tokens: `AREX Express`

### ITEM 374

- Source context: L436 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Horario de AREX Express
  ```
- Protected tokens: `AREX Express`

### ITEM 375

- Source context: L437 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Tarifas de AREX All-stop
  ```
- Protected tokens: `AREX All-stop`

### ITEM 376

- Source context: L438 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Servicio AREX All-stop
  ```
- Protected tokens: `AREX All-stop`

### ITEM 377

- Source context: L439 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Condiciones de transporte y equipaje de AREX
  ```
- Protected tokens: `AREX`

### ITEM 378

- Source context: L440 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Información de autobuses de Incheon Airport — Terminal 1
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`

### ITEM 379

- Source context: L441 - `li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Información de autobuses de Incheon Airport — Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 2`

### ITEM 380

- Source context: L442 - `li:nth-of-type(8)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Incheon Airport — Terminal 1
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`

### ITEM 381

- Source context: L443 - `li:nth-of-type(9)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Incheon Airport — Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 2`

### ITEM 382

- Source context: L444 - `li:nth-of-type(10)`
- Element/type: List text
- Spanish:

  ```text
  Guía de taxis de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 383

- Source context: L445 - `li:nth-of-type(11)`
- Element/type: List text
- Spanish:

  ```text
  Guía de Call Van de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 384

- Source context: L446 - `li:nth-of-type(12)`
- Element/type: List text
- Spanish:

  ```text
  Rutas y tarifas de Airport Limousine
  ```
- Protected tokens: None identified in this item.

### ITEM 385

- Source context: L447 - `li:nth-of-type(13)`
- Element/type: List text
- Spanish:

  ```text
  Tarifas de K Airport Limousine
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 386

- Source context: L448 - `li:nth-of-type(14)`
- Element/type: List text
- Spanish:

  ```text
  Información sobre taxis de Seoul
  ```
- Protected tokens: `Seoul`

### ITEM 387

- Source context: L449 - `li:nth-of-type(15)`
- Element/type: List text
- Spanish:

  ```text
  International Taxi
  ```
- Protected tokens: `International Taxi`

---

# PAGE: `arex.html`

**English source SHA-256:** `17d3fb49924732eb4db5fab1871b224bd11a8ff52799ad1c13f485c19b57d598`  
**Localized ITEM count:** 221

### ITEM 388

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Compara AREX Express y los trenes All-Stop de Incheon Airport a Seoul por precio, tiempo de viaje, paradas, billetes, T-money, equipaje y opciones para llegadas tardías.
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul`, `T-money`

### ITEM 389

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  AREX de Incheon Airport a Seoul: precio, tiempo y guía de trenes | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`, `Korea Inside`

### ITEM 390

- Source context: L80 - `p.page-hero__breadcrumb`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Aeropuerto / AREX
  ```
- Protected tokens: `AREX`

### ITEM 391

- Source context: L81 - `h1.arex-hero__title`
- Element/type: H1
- Spanish:

  ```text
  De Incheon Airport a Seoul en AREX: Express vs All-Stop
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `AREX`

### ITEM 392

- Source context: L83 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Tren AREX viajando desde Incheon Airport hacia Seoul con las opciones de ruta Express y All-Stop
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 393

- Source context: L92 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  AREX tiene dos tipos de tren. Express va desde las terminales del aeropuerto hasta Seoul Station con asiento asignado, mientras que All-Stop sirve estaciones intermedias como Hongik University, Gongdeok y Gimpo Airport.
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongik University`, `Gongdeok`, `Gimpo Airport`

### ITEM 394

- Source context: L93 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Express es más rápido hasta Seoul Station, pero el tren más conveniente depende de dónde te alojes. Para algunos hoteles, el All-Stop Train más lento evita un transbordo. El tren más rápido hasta Seoul Station no siempre es la forma más rápida de llegar a tu hotel.
  ```
- Protected tokens: `Seoul Station`, `All-Stop Train`

### ITEM 395

- Source context: L95 - `div.transfer-fast-choice:nth-of-type(2) @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Elección rápida de AREX
  ```
- Protected tokens: `AREX`

### ITEM 396

- Source context: L97 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Express tiene sentido cuando...
  ```
- Protected tokens: None identified in this item.

### ITEM 397

- Source context: L98 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Tu hotel está cerca de Seoul Station, tu siguiente conexión empieza allí o te importa más tener asiento asignado que pagar la tarifa más baja.
  ```
- Protected tokens: `Seoul Station`

### ITEM 398

- Source context: L101 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop tiene más sentido cuando...
  ```
- Protected tokens: None identified in this item.

### ITEM 399

- Source context: L102 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Te alojas cerca de Hongdae o Gongdeok, haces conexión en Gimpo Airport o utilizas T-money para aprovechar la tarifa más baja calculada por distancia.
  ```
- Protected tokens: `Hongdae`, `Gongdeok`, `Gimpo Airport`, `T-money`

### ITEM 400

- Source context: L105 - `nav.arex-quick-links @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Capítulos de la guía de AREX
  ```
- Protected tokens: `AREX`

### ITEM 401

- Source context: L106 - `a.arex-pill:nth-of-type(1)`
- Element/type: Visible link text
- Spanish:

  ```text
  Express vs All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 402

- Source context: L107 - `a.arex-pill:nth-of-type(2)`
- Element/type: Visible link text
- Spanish:

  ```text
  Estaciones
  ```
- Protected tokens: None identified in this item.

### ITEM 403

- Source context: L108 - `a.arex-pill:nth-of-type(3)`
- Element/type: Visible link text
- Spanish:

  ```text
  Billetes
  ```
- Protected tokens: None identified in this item.

### ITEM 404

- Source context: L109 - `a.arex-pill:nth-of-type(4)`
- Element/type: Visible link text
- Spanish:

  ```text
  Llegadas tardías
  ```
- Protected tokens: None identified in this item.

### ITEM 405

- Source context: L119 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Express o All-Stop?
  ```
- Protected tokens: None identified in this item.

### ITEM 406

- Source context: L120 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Ambos trenes conectan Incheon Airport con Seoul, pero siguen patrones de paradas distintos y utilizan billetes diferentes.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 407

- Source context: L124 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Express Train
  ```
- Protected tokens: `Express Train`

### ITEM 408

- Source context: L125 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express va desde Terminal 2 pasando por Terminal 1 hasta Seoul Station sin paradas intermedias para pasajeros. Tiene asiento asignado y requiere un billete independiente, por lo que encaja bien con un viaje directo a Seoul Station o una conexión posterior en KTX si dispones de tiempo suficiente para el transbordo.
  ```
- Protected tokens: `Terminal 2`, `Terminal 1`, `Seoul Station`, `KTX`

### ITEM 409

- Source context: L128 - `h3`
- Element/type: H3
- Spanish:

  ```text
  All-Stop Train
  ```
- Protected tokens: `All-Stop Train`

### ITEM 410

- Source context: L129 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop para en todas las estaciones de AREX, incluidas Gimpo Airport, Hongik University y Gongdeok. Funciona como un tren de cercanías y acepta T-money, por lo que puede dejarte más cerca del hotel antes de Seoul Station y evitar un regreso innecesario hacia el oeste de la ciudad.
  ```
- Protected tokens: `AREX`, `Gimpo Airport`, `Hongik University`, `Gongdeok`, `T-money`, `Seoul Station`

### ITEM 411

- Source context: L132 - `p`
- Element/type: Body text
- Spanish:

  ```text
  “Seoul” no es un único destino. Un tren que ahorra tiempo en el tramo del aeropuerto a Seoul Station puede perder esa ventaja durante un transbordo, una caminata larga dentro de la estación o un trayecto de vuelta hacia el oeste. Cuando la ruta ferroviaria resulta incómoda con equipaje, compara las otras opciones de traslado desde Incheon Airport; un autobús del aeropuerto o un taxi oficial puede requerir menos manipulación de maletas.
  ```
- Protected tokens: `Seoul`, `Seoul Station`, `Incheon Airport`

### ITEM 412

- Source context: L139 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Comparación rápida: Express vs All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 413

- Source context: L140 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Esta tabla resume las diferencias que resultan útiles frente a la máquina de billetes. Las secciones por destino que aparecen después explican qué ocurre una vez que bajas del tren.
  ```
- Protected tokens: None identified in this item.

### ITEM 414

- Source context: L142 - `div.transfer-comparison.table-scroll @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Comparación entre AREX Express y All-Stop
  ```
- Protected tokens: `AREX Express`

### ITEM 415

- Source context: L144 - `caption`
- Element/type: Table caption
- Spanish:

  ```text
  AREX desde Incheon Airport hacia Seoul
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 416

- Source context: L147 - `th:nth-of-type(1)`
- Element/type: Table header
- Spanish:

  ```text
  Característica
  ```
- Protected tokens: None identified in this item.

### ITEM 417

- Source context: L148 - `th:nth-of-type(2)`
- Element/type: Table header
- Spanish:

  ```text
  Express Train
  ```
- Protected tokens: `Express Train`

### ITEM 418

- Source context: L149 - `th:nth-of-type(3)`
- Element/type: Table header
- Spanish:

  ```text
  All-Stop Train
  ```
- Protected tokens: `All-Stop Train`

### ITEM 419

- Source context: L153 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Paradas
  ```
- Protected tokens: None identified in this item.

### ITEM 420

- Source context: L153 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Sin paradas entre las terminales del aeropuerto y Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 421

- Source context: L153 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Todas las estaciones, incluidas Gimpo Airport, Hongik University y Gongdeok
  ```
- Protected tokens: `Gimpo Airport`, `Hongik University`, `Gongdeok`

### ITEM 422

- Source context: L154 - `th`
- Element/type: Table header
- Spanish:

  ```text
  T1 a Seoul Station
  ```
- Protected tokens: `T1`, `Seoul Station`

### ITEM 423

- Source context: L154 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  43 minutes
  ```
- Protected tokens: `43 minutes`

### ITEM 424

- Source context: L154 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  59 minutes; algunos trenes tardan 2–6 minutes más
  ```
- Protected tokens: `59 minutes`, `2–6 minutes`

### ITEM 425

- Source context: L155 - `th`
- Element/type: Table header
- Spanish:

  ```text
  T2 a Seoul Station
  ```
- Protected tokens: `T2`, `Seoul Station`

### ITEM 426

- Source context: L155 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  51 minutes
  ```
- Protected tokens: `51 minutes`

### ITEM 427

- Source context: L155 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  66 minutes; algunos trenes tardan 2–6 minutes más
  ```
- Protected tokens: `66 minutes`, `2–6 minutes`

### ITEM 428

- Source context: L156 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Tarifa de adulto
  ```
- Protected tokens: None identified in this item.

### ITEM 429

- Source context: L156 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Tarifa oficial de venta: ₩13,000
  ```
- Protected tokens: `₩13,000`

### ITEM 430

- Source context: L156 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Hasta Seoul Station con tarjeta de transporte: T1 ₩4,750; T2 ₩5,350
  ```
- Protected tokens: `Seoul Station`, `T1`, `₩4,750`, `T2`, `₩5,350`

### ITEM 431

- Source context: L157 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Asiento
  ```
- Protected tokens: None identified in this item.

### ITEM 432

- Source context: L157 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Asiento asignado
  ```
- Protected tokens: None identified in this item.

### ITEM 433

- Source context: L157 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Asientos abiertos tipo cercanías; no se garantiza asiento
  ```
- Protected tokens: None identified in this item.

### ITEM 434

- Source context: L158 - `th`
- Element/type: Table header
- Spanish:

  ```text
  T-money
  ```
- Protected tokens: `T-money`

### ITEM 435

- Source context: L158 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  No; usa un billete Express independiente
  ```
- Protected tokens: None identified in this item.

### ITEM 436

- Source context: L158 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí; valida una tarjeta de transporte con saldo suficiente
  ```
- Protected tokens: None identified in this item.

### ITEM 437

- Source context: L159 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Tipo de billete
  ```
- Protected tokens: None identified in this item.

### ITEM 438

- Source context: L159 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Billete reservado de Express con código QR o billete emitido en la estación
  ```
- Protected tokens: None identified in this item.

### ITEM 439

- Source context: L159 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  T-money u otra tarjeta de transporte aceptada, o una tarjeta de transporte de un solo uso
  ```
- Protected tokens: `T-money`

### ITEM 440

- Source context: L160 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Principal contrapartida
  ```
- Protected tokens: None identified in this item.

### ITEM 441

- Source context: L160 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Más rápido y con asiento, pero más caro y limitado a Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 442

- Source context: L160 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Más barato y con paradas más útiles, pero más lento y sin asiento asignado
  ```
- Protected tokens: None identified in this item.

### ITEM 443

- Source context: L170 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Cuánto cuesta AREX?
  ```
- Protected tokens: `AREX`

### ITEM 444

- Source context: L171 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Express tiene una tarifa de venta fija hasta Seoul Station. All-Stop cobra según la distancia, por lo que tanto tu terminal del aeropuerto como la estación donde bajas influyen en el precio.
  ```
- Protected tokens: `Seoul Station`

### ITEM 445

- Source context: L173 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Tarifa del Express Train
  ```
- Protected tokens: `Express Train`

### ITEM 446

- Source context: L174 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El sistema oficial de reservas muestra ₩13,000 para un adulto no miembro, ₩12,500 para un adulto miembro y ₩9,500 para un niño. Express utiliza un billete reservado independiente en lugar de una tarifa de transporte por distancia, y la tarifa de venta es la misma desde T1 o T2 hasta Seoul Station.
  ```
- Protected tokens: `₩13,000`, `₩12,500`, `₩9,500`, `T1`, `T2`, `Seoul Station`

### ITEM 447

- Source context: L175 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Tarifa de All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 448

- Source context: L176 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Con una tarjeta de transporte prepago o pospago, un adulto paga desde T1 ₩4,750 hasta Seoul Station o ₩4,650 hasta Hongik University. Desde T2, esas tarifas son ₩5,350 y ₩5,250. Los ₩600 adicionales reflejan la mayor distancia desde Terminal 2.
  ```
- Protected tokens: `T1`, `₩4,750`, `Seoul Station`, `₩4,650`, `Hongik University`, `T2`, `₩5,350`, `₩5,250`, `₩600`, `Terminal 2`

### ITEM 449

- Source context: L177 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  T-money frente a tarjeta de un solo uso
  ```
- Protected tokens: `T-money`

### ITEM 450

- Source context: L178 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  T-money funciona en All-Stop validando al entrar y salir; no es un billete Express. Una tarjeta de transporte de un solo uso para adulto cuesta la tarifa de la tarjeta de transporte más ₩100, con un depósito reembolsable de ₩500. El depósito se devuelve al introducir la tarjeta en una máquina de reembolso en el destino.
  ```
- Protected tokens: `T-money`, `₩100,`, `₩500`

### ITEM 451

- Source context: L185 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Cuánto tiempo ahorra realmente Express?
  ```
- Protected tokens: None identified in this item.

### ITEM 452

- Source context: L186 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  El trayecto publicado termina en Seoul Station. Tu viaje real también incluye la caminata hasta la estación del aeropuerto, la espera, la siguiente conexión y la caminata al hotel.
  ```
- Protected tokens: `Seoul Station`

### ITEM 453

- Source context: L189 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Desde Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 454

- Source context: L189 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express llega a Seoul Station en 43 minutes. All-Stop tiene un tiempo programado de 59 minutes, aunque algunos servicios tardan otros 2–6 minutes.
  ```
- Protected tokens: `Seoul Station`, `43 minutes`, `59 minutes`, `2–6 minutes`

### ITEM 455

- Source context: L190 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Desde Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 456

- Source context: L190 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express tarda 51 minutes hasta Seoul Station. All-Stop tarda 66 minutes, con la misma posible ampliación de 2–6 minutes en algunos trenes.
  ```
- Protected tokens: `51 minutes`, `Seoul Station`, `66 minutes`, `2–6`

### ITEM 457

- Source context: L192 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Eso hace que Express sea unos 15–16 minutes más rápido dentro del tren. La diferencia puede reducirse cuando Express te obliga a hacer un transbordo de metro en Seoul Station, mientras que All-Stop llega directamente a Hongik University o Gongdeok. El equipaje, los ascensores y la caminata final pueden importar más que la diferencia del horario.
  ```
- Protected tokens: `15–16 minutes`, `Seoul Station`, `Hongik University`, `Gongdeok`

### ITEM 458

- Source context: L199 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Principales estaciones y paradas de AREX
  ```
- Protected tokens: `AREX`

### ITEM 459

- Source context: L200 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Express sirve las dos terminales del aeropuerto y Seoul Station. All-Stop añade las estaciones intermedias que a menudo determinan si el resto del trayecto será sencillo.
  ```
- Protected tokens: `Seoul Station`

### ITEM 460

- Source context: L203 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Mapa de rutas de AREX Express y All-Stop con las terminales de Incheon Airport, las principales estaciones y Seoul Station
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul Station`

### ITEM 461

- Source context: L204 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Rutas AREX Express y All-Stop desde Incheon Airport hasta Seoul
  ```
- Protected tokens: `AREX Express`, `Incheon Airport`, `Seoul`

### ITEM 462

- Source context: L208 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Incheon Airport Terminal 2
  ```
- Protected tokens: `Incheon Airport Terminal 2`

### ITEM 463

- Source context: L209 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express y All-Stop comienzan aquí antes de parar en T1. La estación está conectada con el centro de transporte de Terminal 2, pero es distinta de la estación de T1, así que importa la terminal indicada en la reserva del vuelo.
  ```
- Protected tokens: `T1`, `Terminal 2`

### ITEM 464

- Source context: L212 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Incheon Airport Terminal 1
  ```
- Protected tokens: `Incheon Airport Terminal 1`

### ITEM 465

- Source context: L213 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Ambos trenes también paran en la estación situada bajo el centro de transporte de Terminal 1. La circulación del aeropuerto puede cambiar, por lo que las señales actuales de Airport Railroad / AREX son más fiables que un número de puerta guardado de una guía antigua.
  ```
- Protected tokens: `Terminal 1`, `Airport Railroad`, `AREX`

### ITEM 466

- Source context: L216 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Gimpo International Airport
  ```
- Protected tokens: `Gimpo International Airport`

### ITEM 467

- Source context: L217 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Solo All-Stop para en Gimpo Airport, donde los pasajeros pueden conectar con Subway Lines 5 and 9, Gimpo Goldline y otros servicios ferroviarios. Una conexión aérea todavía implica caminar hasta la terminal correcta, así que la hora de llegada del tren no es la hora de embarque.
  ```
- Protected tokens: `Gimpo Airport`, `Subway Lines 5 and 9`, `Gimpo Goldline`

### ITEM 468

- Source context: L220 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Hongik University
  ```
- Protected tokens: `Hongik University`

### ITEM 469

- Source context: L221 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop llega directamente a Hongik University, con conexiones a Subway Line 2 y Gyeongui-Jungang Line. Para un hotel en Hongdae, normalmente no tiene sentido tomar Express hasta Seoul Station y luego volver hacia el oeste. Sin embargo, la estación es grande y la salida correcta puede marcar una diferencia notable con equipaje.
  ```
- Protected tokens: `Hongik University`, `Subway Line 2`, `Gyeongui-Jungang Line`, `Hongdae`, `Seoul Station`

### ITEM 470

- Source context: L224 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Gongdeok
  ```
- Protected tokens: `Gongdeok`

### ITEM 471

- Source context: L225 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop para en Gongdeok antes de Seoul Station. Los hoteles cercanos no necesitan un segundo tren, mientras que Subway Lines 5 and 6 y Gyeongui-Jungang Line ofrecen conexiones posteriores. La ubicación exacta del hotel sigue determinando qué salida y qué transbordo son prácticos.
  ```
- Protected tokens: `Gongdeok`, `Seoul Station`, `Subway Lines 5 and 6`, `Gyeongui-Jungang Line`

### ITEM 472

- Source context: L228 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 473

- Source context: L229 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Ambos servicios terminan en Seoul Station, donde conectan Subway Lines 1 and 4, Gyeongui-Jungang Line y los servicios ferroviarios nacionales. Los andenes de AREX están en una zona profunda del complejo, y llegar a un andén de metro o KTX implica pasillos, ascensores o escaleras mecánicas y una caminata adicional.
  ```
- Protected tokens: `Seoul Station`, `Subway Lines 1 and 4`, `Gyeongui-Jungang Line`, `AREX`, `KTX`

### ITEM 474

- Source context: L238 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿A dónde vas después de AREX?
  ```
- Protected tokens: `AREX`

### ITEM 475

- Source context: L239 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  La pregunta útil no es solo qué tren es más rápido, sino dónde puedes bajar de AREX y cuánto trayecto queda después.
  ```
- Protected tokens: `AREX`

### ITEM 476

- Source context: L242 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Guía para elegir tren y estación de AREX para Seoul Station, Hongdae, Gongdeok, Myeongdong, Gangnam y otros destinos
  ```
- Protected tokens: `AREX`, `Seoul Station`, `Hongdae`, `Gongdeok`, `Myeongdong`, `Gangnam`

### ITEM 477

- Source context: L243 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Opciones de tren y estación de AREX para los principales destinos de Seoul
  ```
- Protected tokens: `AREX`, `Seoul`

### ITEM 478

- Source context: L246 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Seoul Station
  ```
- Protected tokens: `Seoul Station`

### ITEM 479

- Source context: L246 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express llega más rápido y ofrece asiento asignado; All-Stop llega a la misma estación por menos dinero. Un hotel cercano puede quedar a pie, pero uno situado al otro lado del complejo puede seguir implicando una salida larga. Si la caminata final parece incómoda con maletas, compara una parada de autobús o un punto de bajada de taxi cerca de la dirección.
  ```
- Protected tokens: None identified in this item.

### ITEM 480

- Source context: L247 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Hongdae
  ```
- Protected tokens: `Hongdae`

### ITEM 481

- Source context: L247 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop para directamente en Hongik University Station, así que hay pocas razones para tomar Express hasta Seoul Station y luego volver hacia el oeste para llegar a un hotel de Hongdae. Sin embargo, la estación es grande y la salida correcta puede marcar una diferencia notable cuando arrastras equipaje.
  ```
- Protected tokens: `Hongik University Station`, `Seoul Station`, `Hongdae`

### ITEM 482

- Source context: L248 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Gongdeok
  ```
- Protected tokens: `Gongdeok`

### ITEM 483

- Source context: L248 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop llega a Gongdeok sin transbordo. Los hoteles cercanos a la estación quedan a una caminata sencilla; otras direcciones pueden continuar por las líneas 5 o 6, y la ubicación exacta del hotel determina qué ruta funciona.
  ```
- Protected tokens: `Gongdeok`, `5`, `6`

### ITEM 484

- Source context: L249 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Myeongdong
  ```
- Protected tokens: `Myeongdong`

### ITEM 485

- Source context: L249 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express hasta Seoul Station seguido de Line 4 puede ser una ruta ferroviaria rápida, mientras que All-Stop cubre el tramo desde el aeropuerto por menos dinero. Ninguno elimina el transbordo. Con varias maletas, un autobús del aeropuerto que pare cerca del hotel puede ser más fácil que atravesar Seoul Station.
  ```
- Protected tokens: `Seoul Station`, `Line 4`

### ITEM 486

- Source context: L250 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Jongno
  ```
- Protected tokens: `Jongno`

### ITEM 487

- Source context: L250 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Jongno abarca un área demasiado amplia para dar una única respuesta con AREX. Algunos hoteles conectan de forma natural desde Gongdeok por Line 5; otros funcionan mejor desde Seoul Station o en autobús del aeropuerto. Usa la dirección exacta y la salida de la estación, no solo el nombre del distrito.
  ```
- Protected tokens: `Jongno`, `AREX`, `Gongdeok`, `Line 5`, `Seoul Station`

### ITEM 488

- Source context: L251 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Dongdaemun
  ```
- Protected tokens: `Dongdaemun`

### ITEM 489

- Source context: L251 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Una ruta ferroviaria habitual es Express hasta Seoul Station y después Line 4 hacia la estación más cercana al hotel. Sigue siendo un transbordo con equipaje, así que merece la pena comparar una parada de autobús del aeropuerto directa cerca del alojamiento.
  ```
- Protected tokens: `Seoul Station`, `Line 4`

### ITEM 490

- Source context: L252 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Gangnam
  ```
- Protected tokens: `Gangnam`

### ITEM 491

- Source context: L252 - `p`
- Element/type: Body text
- Spanish:

  ```text
  No existe un único transbordo de AREX que funcione para todo Gangnam. All-Stop puede conectar con Line 9 en Magongnaru o Gimpo Airport, pero la estación útil depende de la dirección exacta y de cualquier conexión posterior. Muchos hoteles de Gangnam son más sencillos de alcanzar en autobús del aeropuerto que llevando maletas por varios tramos ferroviarios.
  ```
- Protected tokens: `AREX`, `Gangnam`, `Line 9`, `Magongnaru`, `Gimpo Airport`

### ITEM 492

- Source context: L253 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Jamsil
  ```
- Protected tokens: `Jamsil`

### ITEM 493

- Source context: L253 - `p`
- Element/type: Body text
- Spanish:

  ```text
  AREX suele dejar al menos un trayecto posterior importante hasta Jamsil, y a menudo más de un tramo ferroviario. Un autobús del aeropuerto adecuado puede ser más sencillo, especialmente con equipaje, niños o movilidad limitada.
  ```
- Protected tokens: `AREX`, `Jamsil`

### ITEM 494

- Source context: L254 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Gimpo Airport
  ```
- Protected tokens: `Gimpo Airport`

### ITEM 495

- Source context: L254 - `p`
- Element/type: Body text
- Spanish:

  ```text
  All-Stop va directamente a Gimpo International Airport Station; Express pasa sin detenerse. Deja tiempo para la señalización, los ascensores y la caminata hasta la terminal de vuelo correcta o la siguiente línea de metro.
  ```
- Protected tokens: `Gimpo International Airport Station`

### ITEM 496

- Source context: L255 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Conexión con KTX
  ```
- Protected tokens: `KTX`

### ITEM 497

- Source context: L255 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Express suele ser el tren del aeropuerto más razonable cuando el siguiente viaje comienza en KTX en Seoul Station. La hora de llegada de AREX no es la hora a la que puedes subir al KTX: todavía tienes que salir del andén profundo de AREX, cruzar la estación y llegar al andén ferroviario nacional correcto.
  ```
- Protected tokens: `KTX`, `Seoul Station`, `AREX`

### ITEM 498

- Source context: L257 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Usa apps de mapas coreanas para comprobar el pin del hotel, la salida de la estación y la caminata final. El nombre de una zona por sí solo no basta para evaluar el transbordo.
  ```
- Protected tokens: None identified in this item.

### ITEM 499

- Source context: L258 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El acceso a AREX por sí solo no debería decidir dónde te alojas. Hongdae y Gongdeok tienen servicio directo All-Stop, Seoul Station añade Express y conexiones ferroviarias nacionales, mientras que algunos hoteles de Myeongdong pueden ser más fáciles de alcanzar en autobús del aeropuerto.
  ```
- Protected tokens: `AREX`, `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 500

- Source context: L259 - `p:nth-of-type(3)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar bases en Seoul cómodas para el aeropuerto →
  ```
- Protected tokens: `Seoul`

### ITEM 501

- Source context: L266 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Cómo encontrar AREX en Terminal 1 y Terminal 2
  ```
- Protected tokens: `AREX`, `Terminal 1`, `Terminal 2`

### ITEM 502

- Source context: L267 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  T1 y T2 tienen estaciones separadas, pero el recorrido desde la sala de llegadas sigue la misma secuencia básica.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 503

- Source context: L270 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  De la sala de llegadas al andén correcto de AREX
  ```
- Protected tokens: `AREX`

### ITEM 504

- Source context: L273 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Completa inmigración, recogida de equipaje y aduanas, y después entra en la sala pública de llegadas.
  ```
- Protected tokens: None identified in this item.

### ITEM 505

- Source context: L274 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Sigue las señales de Airport Railroad / AREX hacia el centro de transporte de la terminal.
  ```
- Protected tokens: `Airport Railroad`, `AREX`

### ITEM 506

- Source context: L275 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Continúa hasta la zona ferroviaria y distingue la entrada de Express de la entrada de All-Stop.
  ```
- Protected tokens: None identified in this item.

### ITEM 507

- Source context: L276 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Compra el billete correcto o prepara una tarjeta de transporte con saldo suficiente para All-Stop.
  ```
- Protected tokens: None identified in this item.

### ITEM 508

- Source context: L277 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Consulta la pantalla de salidas en tiempo real y confirma el andén con dirección a Seoul antes de entrar.
  ```
- Protected tokens: `Seoul`

### ITEM 509

- Source context: L280 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  En Terminal 1: la zona ferroviaria está en el centro de transporte de T1. Sigue las señales hacia ese edificio y no unas indicaciones escritas para T2.
  ```
- Protected tokens: `Terminal 1`, `T1`, `T2`

### ITEM 510

- Source context: L281 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  En Terminal 2: continúa hacia el centro de transporte de T2 y su propia estación. No es necesario ir primero a T1.
  ```
- Protected tokens: `Terminal 2`, `T2`, `T1`

### ITEM 511

- Source context: L286 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Indicaciones paso a paso desde las salas de llegadas de Incheon Airport Terminal 1 y Terminal 2 hasta los andenes de AREX
  ```
- Protected tokens: `Incheon Airport Terminal 1`, `Terminal 2`, `AREX`

### ITEM 512

- Source context: L287 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Cómo llegar a la estación de AREX desde Terminal 1 y Terminal 2
  ```
- Protected tokens: `AREX`, `Terminal 1`, `Terminal 2`

### ITEM 513

- Source context: L289 - `p`
- Element/type: Body text
- Spanish:

  ```text
  La circulación del aeropuerto y las rutas temporales pueden cambiar, por lo que la señalización actual es más fiable que un número de puerta o salida guardado. El proceso general de llegada a Incheon Airport explica qué ocurre antes de entrar en la sala pública.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 514

- Source context: L296 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Billetes para Express y All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 515

- Source context: L297 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Los dos trenes utilizan accesos y sistemas de billetes diferentes. T-money funciona en All-Stop, pero no sustituye una reserva de Express.
  ```
- Protected tokens: `T-money`

### ITEM 516

- Source context: L300 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Guía para elegir billete de AREX: reservas de Express, billetes de estación, T-money y tarjetas de un solo uso
  ```
- Protected tokens: `AREX`, `T-money`

### ITEM 517

- Source context: L301 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Tipos de billete de AREX para los trenes Express y All-Stop
  ```
- Protected tokens: `AREX`

### ITEM 518

- Source context: L305 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Billetes de Express
  ```
- Protected tokens: None identified in this item.

### ITEM 519

- Source context: L306 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El sitio y la app oficiales permiten buscar fechas de viaje desde hoy hasta un máximo de dos meses por adelantado. Una vez que un tren seleccionado se añade al carrito, el pago debe completarse en 20 minutes; ese límite no significa que las reservas sigan disponibles hasta 20 minutes antes de la salida. El billete pagado incluye un código QR e identifica el tren, el coche y el asiento asignado.
  ```
- Protected tokens: `20 minutes`

### ITEM 520

- Source context: L307 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Los billetes también se venden en máquinas de Express y centros de información al cliente de T1, T2 y Seoul Station. AREX incluye Visa, Mastercard, JCB, Diners Club, American Express y UnionPay entre las redes de tarjetas extranjeras compatibles para comprar billetes Express.
  ```
- Protected tokens: `T1`, `T2`, `Seoul Station`, `AREX`, `Visa`, `Mastercard`, `JCB`, `Diners Club`, `American Express`, `UnionPay`

### ITEM 521

- Source context: L310 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Billetes de All-Stop
  ```
- Protected tokens: None identified in this item.

### ITEM 522

- Source context: L311 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Una tarjeta de transporte prepago como T-money puede utilizarse si tiene saldo suficiente para el trayecto. Valida al entrar por el acceso de All-Stop y vuelve a validar al salir en el destino.
  ```
- Protected tokens: `T-money`

### ITEM 523

- Source context: L312 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Los viajeros sin tarjeta de transporte pueden comprar una tarjeta de transporte de un solo uso en una máquina de billetes de la estación, conservarla durante el trayecto y recuperar el depósito al salir. La guía oficial de All-Stop confirma la venta en máquinas, pero no indica claramente que acepten tarjetas emitidas en el extranjero, así que no dependas de un método de pago concreto sin comprobarlo en la estación.
  ```
- Protected tokens: None identified in this item.

### ITEM 524

- Source context: L315 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El sitio oficial de reservas de Express también ofrece funciones de cambio y devolución y muestra las condiciones aplicables antes de confirmar. Las reglas de los revendedores pueden diferir de las condiciones del propio operador.
  ```
- Protected tokens: None identified in this item.

### ITEM 525

- Source context: L322 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Seoul Station suele ser solo la mitad del viaje
  ```
- Protected tokens: `Seoul Station`

### ITEM 526

- Source context: L323 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Llegar a Seoul Station no es lo mismo que llegar a tu hotel.
  ```
- Protected tokens: `Seoul Station`

### ITEM 527

- Source context: L325 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Los andenes de AREX están en una zona profunda de un gran complejo ferroviario. Llegar al metro, a un andén de KTX o al nivel de la calle puede implicar pasillos largos, ascensor o escaleras mecánicas y tiempo siguiendo señales con equipaje.
  ```
- Protected tokens: `AREX`, `KTX`

### ITEM 528

- Source context: L326 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Conexiones de metro y hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 529

- Source context: L327 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Line 4 es la continuación habitual hacia Myeongdong y partes de Dongdaemun, mientras que Line 1 sirve otras conexiones del centro y del este. El viaje no termina en el torno del metro: la dirección correcta, la salida del lado del hotel y la caminata final influyen en lo manejable que resulta la ruta.
  ```
- Protected tokens: `Line 4`, `Myeongdong`, `Dongdaemun`, `Line 1`

### ITEM 530

- Source context: L328 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  KTX y ferrocarril nacional
  ```
- Protected tokens: `KTX`

### ITEM 531

- Source context: L329 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Una hora de llegada publicada de AREX no significa que estés listo para subir a un KTX. Incluye la caminata desde el andén de AREX, ascensores o escaleras mecánicas, cualquier control de billetes y el recorrido por la estación hasta el andén ferroviario nacional correcto.
  ```
- Protected tokens: `AREX`, `KTX`

### ITEM 532

- Source context: L330 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Usa apps de mapas coreanas para comprobar el pin del hotel, la salida de la estación y la caminata final. El nombre de una zona por sí solo no basta para evaluar el transbordo.
  ```
- Protected tokens: None identified in this item.

### ITEM 533

- Source context: L331 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  City Airport Terminal
  ```
- Protected tokens: None identified in this item.

### ITEM 534

- Source context: L332 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Seoul Station City Airport Terminal es un servicio de salida, no una ayuda para pasajeros que llegan desde Incheon Airport. Los pasajeros internacionales elegibles que vuelan el mismo día y utilizan Express pueden realizar allí ciertos trámites de salida, según las condiciones vigentes de la aerolínea y del servicio.
  ```
- Protected tokens: `Seoul Station City Airport Terminal`, `Incheon Airport`

### ITEM 535

- Source context: L339 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Equipaje, familias y accesibilidad
  ```
- Protected tokens: None identified in this item.

### ITEM 536

- Source context: L340 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Un viaje cómodo en tren no elimina el trabajo de mover las maletas por la estación del aeropuerto, un transbordo y el tramo final hasta el hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 537

- Source context: L342 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Dentro del tren
  ```
- Protected tokens: None identified in this item.

### ITEM 538

- Source context: L343 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Express ofrece asiento asignado y espacio específico para equipaje, lo que hace que el trayecto principal sea más predecible. All-Stop tiene asientos tipo cercanías, por lo que no se garantiza asiento y las horas de mayor afluencia pueden ser difíciles con maletas grandes, un cochecito o niños.
  ```
- Protected tokens: None identified in this item.

### ITEM 539

- Source context: L344 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Entre el tren y el hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 540

- Source context: L345 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Cada maleta todavía tiene que ir desde la sala de llegadas hasta la estación del aeropuerto, subir a AREX, pasar por cualquier transbordo en Seoul y recorrer la caminata final. Para una persona en silla de ruedas, alguien con movilidad limitada o una familia que maneja varias maletas, las rutas con ascensor y la distancia de transbordo pueden importar más que una diferencia de 15 minutos en el tren.
  ```
- Protected tokens: `AREX`, `Seoul`, `15`

### ITEM 541

- Source context: L346 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Cuando esa cadena incluye levantar equipaje repetidamente o pasillos largos, un autobús del aeropuerto que pare cerca del hotel o un taxi oficial del tamaño adecuado puede reducir el esfuerzo total.
  ```
- Protected tokens: None identified in this item.

### ITEM 542

- Source context: L348 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Situaciones en las que conviene considerar autobús del aeropuerto, taxi o recogida privada en lugar de AREX
  ```
- Protected tokens: `AREX`

### ITEM 543

- Source context: L349 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Cuándo un autobús del aeropuerto, taxi o recogida privada puede ser más fácil que AREX
  ```
- Protected tokens: `AREX`

### ITEM 544

- Source context: L357 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Llegada tardía? Calcula hacia atrás desde el tren, no desde la hora de aterrizaje
  ```
- Protected tokens: None identified in this item.

### ITEM 545

- Source context: L358 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  AREX no funciona 24 hours, y la hora de aterrizaje no es la hora de llegada al andén.
  ```
- Protected tokens: `AREX`, `24 hours`

### ITEM 546

- Source context: L360 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El horario oficial actual de Express muestra las primeras salidas desde el aeropuerto a las 05:16 desde T2 y 05:24 desde T1, y las últimas a las 22:40 desde T2 y 22:48 desde T1. All-Stop sigue un horario distinto, así que utiliza el horario oficial fechado de AREX en lugar de una captura antigua.
  ```
- Protected tokens: `05:16`, `T2`, `05:24`, `T1`, `22:40`, `22:48`, `AREX`

### ITEM 547

- Source context: L362 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Empieza por la última salida desde la terminal donde aterriza tu vuelo.
  ```
- Protected tokens: None identified in this item.

### ITEM 548

- Source context: L363 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Calcula hacia atrás incluyendo la caminata hasta el andén y la compra del billete.
  ```
- Protected tokens: None identified in this item.

### ITEM 549

- Source context: L364 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Después añade inmigración, recogida de equipaje y aduanas.
  ```
- Protected tokens: None identified in this item.

### ITEM 550

- Source context: L365 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Compara el resultado con la hora programada de aterrizaje y un margen realista para retrasos.
  ```
- Protected tokens: None identified in this item.

### ITEM 551

- Source context: L366 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Ten una alternativa que siga funcionando después de la hora a la que realmente puedas llegar al andén.
  ```
- Protected tokens: None identified in this item.

### ITEM 552

- Source context: L368 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Si el último tren ya no te sirve, busca primero un autobús nocturno oficial u otra opción de autobús del aeropuerto. Si no queda un servicio adecuado, utiliza una parada oficial de taxis del aeropuerto o una recogida verificada con condiciones claras de encuentro y retraso.
  ```
- Protected tokens: None identified in this item.

### ITEM 553

- Source context: L375 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Errores habituales con AREX
  ```
- Protected tokens: `AREX`

### ITEM 554

- Source context: L376 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  El tren en sí es sencillo. Los problemas suelen empezar cuando la ruta se planifica solo hasta una estación de AREX.
  ```
- Protected tokens: `AREX`

### ITEM 555

- Source context: L379 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Express no para en Hongik University. Para la mayoría de hoteles de Hongdae, All-Stop evita ir hasta Seoul Station y después volver hacia el oeste.
  ```
- Protected tokens: `Hongik University`, `Hongdae`, `Seoul Station`

### ITEM 556

- Source context: L380 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Seoul Station no es el final del viaje salvo que el hotel esté cerca. Los pasillos del metro, las salidas y la caminata final necesitan su propio tiempo.
  ```
- Protected tokens: `Seoul Station`

### ITEM 557

- Source context: L381 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Express y All-Stop tienen billetes y accesos separados. Entrar en la zona equivocada obliga a volver innecesariamente al vestíbulo.
  ```
- Protected tokens: None identified in this item.

### ITEM 558

- Source context: L382 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  T-money funciona en All-Stop, no como billete de Express. Express requiere su propia reserva o un billete emitido en la estación.
  ```
- Protected tokens: `T-money`

### ITEM 559

- Source context: L383 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  T1 y T2 son estaciones distintas. Las indicaciones y los horarios de salida deben coincidir con la terminal donde aterriza el vuelo.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 560

- Source context: L384 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Las tablas de tarifas antiguas, capturas de horarios y condiciones de revendedores pueden estar desactualizadas. Las páginas actuales del operador deberían ser la referencia final.
  ```
- Protected tokens: None identified in this item.

### ITEM 561

- Source context: L385 - `li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Un plan nocturno basado en la hora de aterrizaje deja fuera inmigración, equipaje, aduanas, la caminata hasta la estación y la compra del billete.
  ```
- Protected tokens: None identified in this item.

### ITEM 562

- Source context: L386 - `li:nth-of-type(8)`
- Element/type: List text
- Spanish:

  ```text
  El tren no es automáticamente más fácil con varias maletas grandes. Una parada de autobús cerca del hotel o un taxi oficial puede evitar levantar el equipaje repetidamente y hacer transbordos.
  ```
- Protected tokens: None identified in this item.

### ITEM 563

- Source context: L387 - `li:nth-of-type(9)`
- Element/type: List text
- Spanish:

  ```text
  Gangnam y Jongno son zonas demasiado amplias para tener una única estación de transbordo por defecto. La dirección del hotel, la línea de conexión y la caminata final pueden cambiar la ruta más razonable.
  ```
- Protected tokens: `Gangnam`, `Jongno`

### ITEM 564

- Source context: L395 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes sobre AREX
  ```
- Protected tokens: `AREX`

### ITEM 565

- Source context: L396 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Estos son los detalles prácticos que suelen importar una vez que ya están fijados el vuelo, el hotel y la hora de llegada.
  ```
- Protected tokens: None identified in this item.

### ITEM 566

- Source context: L399 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuánto cuesta AREX de Incheon Airport a Seoul?
  ```
- Protected tokens: `AREX`, `Incheon Airport`, `Seoul`

### ITEM 567

- Source context: L399 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  La tarifa de venta para adulto de Express es ₩13,000. Con tarjeta de transporte, All-Stop hasta Seoul Station cuesta ₩4,750 desde T1 o ₩5,350 desde T2. Comprueba la tarifa oficial vigente antes de viajar.
  ```
- Protected tokens: `₩13,000`, `Seoul Station`, `₩4,750`, `T1`, `₩5,350`, `T2`

### ITEM 568

- Source context: L400 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuánto tarda AREX hasta Seoul Station?
  ```
- Protected tokens: `AREX`, `Seoul Station`

### ITEM 569

- Source context: L400 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Express tarda 43 minutes desde T1 o 51 minutes desde T2. All-Stop tarda 59 minutes desde T1 o 66 minutes desde T2, aunque algunos trenes tardan 2–6 minutes más.
  ```
- Protected tokens: `43 minutes`, `T1`, `51 minutes`, `T2`, `59 minutes`, `66 minutes`, `2–6 minutes`

### ITEM 570

- Source context: L401 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿AREX Express es más rápido que All-Stop?
  ```
- Protected tokens: `AREX Express`

### ITEM 571

- Source context: L401 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Es más rápido hasta Seoul Station. Sin embargo, un hotel cerca de Hongik University, Gongdeok u otra parada intermedia puede alcanzarse antes con All-Stop porque evita regresar sobre el recorrido.
  ```
- Protected tokens: `Seoul Station`, `Hongik University`, `Gongdeok`

### ITEM 572

- Source context: L402 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo usar T-money en AREX?
  ```
- Protected tokens: `T-money`, `AREX`

### ITEM 573

- Source context: L402 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Usa T-money en el All-Stop Train. Express requiere un billete reservado independiente.
  ```
- Protected tokens: `T-money`, `All-Stop Train`

### ITEM 574

- Source context: L403 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué tren AREX debo tomar para Hongdae?
  ```
- Protected tokens: `AREX`, `Hongdae`

### ITEM 575

- Source context: L403 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  All-Stop va directamente a Hongik University Station. Express pasa de largo por Hongdae y termina en Seoul Station, por lo que normalmente añade un regreso innecesario hacia el oeste.
  ```
- Protected tokens: `Hongik University Station`, `Hongdae`, `Seoul Station`

### ITEM 576

- Source context: L404 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿AREX funciona 24 hours?
  ```
- Protected tokens: `AREX`, `24 hours`

### ITEM 577

- Source context: L404 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. Consulta el horario oficial para tu terminal y fecha de viaje, especialmente después de un vuelo tardío.
  ```
- Protected tokens: None identified in this item.

### ITEM 578

- Source context: L405 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde está AREX en Terminal 1?
  ```
- Protected tokens: `AREX`, `Terminal 1`

### ITEM 579

- Source context: L405 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Después de aduanas, sigue las señales de Airport Railroad / AREX desde la sala pública de llegadas hacia Terminal 1 Transportation Center y la zona ferroviaria.
  ```
- Protected tokens: `Airport Railroad`, `AREX`, `Terminal 1 Transportation Center`

### ITEM 580

- Source context: L406 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde está AREX en Terminal 2?
  ```
- Protected tokens: `AREX`, `Terminal 2`

### ITEM 581

- Source context: L406 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Después de aduanas, sigue las señales de Airport Railroad / AREX desde la sala pública de llegadas hacia Terminal 2 Transportation Center y la zona ferroviaria.
  ```
- Protected tokens: `Airport Railroad`, `AREX`, `Terminal 2 Transportation Center`

### ITEM 582

- Source context: L407 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿AREX es práctico con equipaje grande?
  ```
- Protected tokens: `AREX`

### ITEM 583

- Source context: L407 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Express tiene asientos asignados y espacio específico para equipaje, pero cada maleta sigue teniendo que pasar por la estación del aeropuerto y cualquier transbordo en Seoul. Un autobús o taxi puede requerir menos manipulación cuando hay varias maletas grandes.
  ```
- Protected tokens: `Seoul`

### ITEM 584

- Source context: L408 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Es AREX mejor que el autobús del aeropuerto?
  ```
- Protected tokens: `AREX`

### ITEM 585

- Source context: L408 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  AREX es mejor si valoras un tiempo ferroviario predecible y el hotel tiene una conexión sencilla desde la estación. Un autobús puede ser mejor cuando una parada oficial queda cerca del hotel y reduce los transbordos con equipaje.
  ```
- Protected tokens: `AREX`

### ITEM 586

- Source context: L409 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo comprar un billete de AREX con una tarjeta de crédito extranjera?
  ```
- Protected tokens: `AREX`

### ITEM 587

- Source context: L409 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Para billetes Express, AREX incluye Visa, Mastercard, JCB, Diners Club, American Express y UnionPay entre las redes de tarjetas emitidas en el extranjero compatibles. Para All-Stop, la guía oficial confirma que las tarjetas de transporte de un solo uso se venden en las máquinas de las estaciones, pero no confirma claramente la aceptación de tarjetas extranjeras en esas máquinas.
  ```
- Protected tokens: `AREX`, `Visa`, `Mastercard`, `JCB`, `Diners Club`, `American Express`, `UnionPay`

### ITEM 588

- Source context: L410 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué ocurre si pierdo el último tren?
  ```
- Protected tokens: None identified in this item.

### ITEM 589

- Source context: L410 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Comprueba primero los servicios oficiales de autobús nocturno y autobús del aeropuerto. Si ninguno encaja con tu destino y horario, utiliza una parada oficial de taxis del aeropuerto o una recogida verificada.
  ```
- Protected tokens: None identified in this item.

### ITEM 590

- Source context: L418 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Fuentes oficiales y guías relacionadas
  ```
- Protected tokens: None identified in this item.

### ITEM 591

- Source context: L419 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  Las tarifas, horarios, métodos de pago compatibles y condiciones de operación pueden cambiar. Estas páginas oficiales son las referencias para una comprobación final antes del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 592

- Source context: L422 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Introducción a AREX Express — paradas, tiempo de viaje, tarifa oficial de venta, asientos asignados y ubicación de estaciones.
  ```
- Protected tokens: `AREX Express`

### ITEM 593

- Source context: L423 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Horario de AREX Express — primeras, últimas y salidas por fecha.
  ```
- Protected tokens: `AREX Express`

### ITEM 594

- Source context: L424 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Guía de billetes de AREX Express — compra online y en estación, tarjetas extranjeras, billetes QR, cambios y devoluciones.
  ```
- Protected tokens: `AREX Express`

### ITEM 595

- Source context: L425 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Introducción a AREX All-Stop — estaciones, líneas de transbordo, tiempo de viaje y tipos de billete.
  ```
- Protected tokens: `AREX All-Stop`

### ITEM 596

- Source context: L426 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Tabla de tarifas de AREX All-Stop — tarifas por distancia, recargo de un solo uso y depósito reembolsable.
  ```
- Protected tokens: `AREX All-Stop`

### ITEM 597

- Source context: L427 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Guía ferroviaria de Incheon Airport — contexto del transporte del aeropuerto e información de terminales.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 598

- Source context: L428 - `li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Guía del city terminal de Incheon Airport — requisitos para Express y condiciones de los servicios de salida.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 599

- Source context: L429 - `li:nth-of-type(8)`
- Element/type: List text
- Spanish:

  ```text
  Guía de transporte aeroportuario de VISITKOREA — resumen oficial independiente de los tipos de tren AREX y sus tiempos.
  ```
- Protected tokens: `VISITKOREA`, `AREX`

### ITEM 600

- Source context: L431 - `nav.related-links @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Guías de viaje relacionadas sobre Korea
  ```
- Protected tokens: `Korea`

### ITEM 601

- Source context: L432 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Seguir planificando
  ```
- Protected tokens: None identified in this item.

### ITEM 602

- Source context: L434 - `li:nth-of-type(1)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Comparación de traslados desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 603

- Source context: L435 - `li:nth-of-type(2)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Guía del autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 604

- Source context: L436 - `li:nth-of-type(3)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Proceso de llegada a Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 605

- Source context: L437 - `li:nth-of-type(4)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Guía de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 606

- Source context: L438 - `li:nth-of-type(5)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Guía de T-money
  ```
- Protected tokens: `T-money`

### ITEM 607

- Source context: L439 - `li:nth-of-type(6)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Guía de apps de mapas coreanas
  ```
- Protected tokens: None identified in this item.

### ITEM 608

- Source context: L440 - `li:nth-of-type(7)`
- Element/type: Related-guide visible link
- Spanish:

  ```text
  Guía oficial de taxis
  ```
- Protected tokens: None identified in this item.

---

# PAGE: `airport-bus.html`

**English source SHA-256:** `f58295df09fa7122ee587696103b51f61e83d7f91127a10f550c362cdb872283`  
**Localized ITEM count:** 182

### ITEM 609

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Elige el autobús adecuado desde Incheon Airport hacia Seoul, Gyeonggi u otras ciudades. Consulta rutas, billetes de T1/T2, embarque, equipaje, autobuses nocturnos y el viaje de regreso.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Gyeonggi`, `T1`, `T2`

### ITEM 610

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Guía de autobuses de Incheon Airport: rutas, billetes y embarque | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Korea Inside`

### ITEM 611

- Source context: L79 - `p.page-hero__breadcrumb:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Aeropuerto / Autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 612

- Source context: L80 - `h1.page-hero__title`
- Element/type: H1
- Spanish:

  ```text
  Guía de autobuses de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 613

- Source context: L81 - `p.page-hero__desc:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús del aeropuerto puede ser una de las formas más fáciles de salir de Incheon Airport cuando para cerca de tu hotel. Evitas un transbordo de metro, la mayoría de las maletas grandes van en la bodega y el trayecto puede terminar a una distancia caminable de tu alojamiento.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 614

- Source context: L82 - `p.page-hero__desc:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  La parte difícil es que “autobús del aeropuerto” no significa una sola red. Los servicios de Seoul, Incheon, Gyeonggi, interurbanos y nocturnos utilizan rutas, operadores y reglas de billetes diferentes. Importa menos el número del autobús que el lugar donde realmente te deja.
  ```
- Protected tokens: `Seoul`, `Incheon`, `Gyeonggi`

### ITEM 615

- Source context: L88 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El autobús resulta especialmente útil cuando hay una parada a una caminata sencilla del hotel. Para Hongdae, Seoul Station u otra dirección con una conexión ferroviaria simple, AREX puede seguir siendo más fácil. Fuera de Seoul, busca por la ciudad o terminal real en lugar de empezar con una ruta limusina de Seoul.
  ```
- Protected tokens: `Hongdae`, `Seoul Station`, `AREX`, `Seoul`

### ITEM 616

- Source context: L94 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Señales de Terminal 2 para las zonas de embarque de autobuses del aeropuerto hacia ciudades locales, Seoul y Gyeonggi en Incheon Airport
  ```
- Protected tokens: `Terminal 2`, `Seoul`, `Gyeonggi`, `Incheon Airport`

### ITEM 617

- Source context: L95 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Terminal 2 separa las zonas de embarque de autobuses por destino. Los rangos de dársenas y la asignación de rutas pueden cambiar, así que confirma la señalización actual antes de subir.
  ```
- Protected tokens: `Terminal 2`

### ITEM 618

- Source context: L98 - `h2#choose-bus`
- Element/type: H2
- Spanish:

  ```text
  Empieza por la parada, no por el número del autobús
  ```
- Protected tokens: None identified in this item.

### ITEM 619

- Source context: L99 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El viaje más fácil en autobús del aeropuerto empieza en el hotel, no en el aeropuerto. Localiza tu alojamiento en un mapa coreano y después busca paradas que dejen una caminata final razonable.
  ```
- Protected tokens: None identified in this item.

### ITEM 620

- Source context: L100 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Una parada a 300 metres por una calle llana puede ser excelente. Otra ligeramente más cerca, pero al otro lado de una intersección ancha, cuesta arriba o varios niveles por debajo del hotel, puede resultar mucho menos agradable con dos maletas.
  ```
- Protected tokens: `300 metres`

### ITEM 621

- Source context: L101 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Esto importa especialmente en distritos grandes como Myeongdong, Jongno, Dongdaemun, Gangnam y Hongdae. Dos hoteles descritos como situados en el mismo barrio pueden tener accesos en autobús del aeropuerto muy diferentes.
  ```
- Protected tokens: `Myeongdong`, `Jongno`, `Dongdaemun`, `Gangnam`, `Hongdae`

### ITEM 622

- Source context: L107 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Seoul
  ```
- Protected tokens: `Seoul`

### ITEM 623

- Source context: L108 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Para un hotel en Seoul, un autobús limusina del aeropuerto resulta más atractivo cuando la parada está realmente cerca del alojamiento.
  ```
- Protected tokens: `Seoul`

### ITEM 624

- Source context: L109 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  La ventaja principal no es necesariamente la velocidad. Es evitar la secuencia de estación del aeropuerto, tren, estación de transbordo, salida de metro y caminata final cargando equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 625

- Source context: L110 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  El tráfico es la contrapartida. Un autobús puede tardar más que el tren cuando las carreteras están congestionadas, así que una dirección con una conexión sencilla a AREX todavía puede funcionar mejor.
  ```
- Protected tokens: `AREX`

### ITEM 626

- Source context: L111 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Si todavía estás eligiendo una base en Seoul, tener una estación directa de AREX no es automáticamente más fácil que una parada de autobús del aeropuerto cerca del hotel. Compara la llegada completa para Hongdae, Gongdeok, Seoul Station y Myeongdong antes de elegir la zona.
  ```
- Protected tokens: `Seoul`, `AREX`, `Hongdae`, `Gongdeok`, `Seoul Station`, `Myeongdong`

### ITEM 627

- Source context: L112 - `p.airport-bus-context-link:nth-of-type(5)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Comparar zonas de Seoul por acceso al aeropuerto →
  ```
- Protected tokens: `Seoul`

### ITEM 628

- Source context: L115 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Incheon y Gyeonggi
  ```
- Protected tokens: `Incheon`, `Gyeonggi`

### ITEM 629

- Source context: L116 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Los destinos fuera del centro de Seoul utilizan redes de autobús diferentes.
  ```
- Protected tokens: `Seoul`

### ITEM 630

- Source context: L117 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Quien vaya a Suwon, Seongnam, Goyang, Bucheon u otro destino de Gyeonggi debería buscar ese destino directamente en lugar de intentar adaptar una ruta de autobús del aeropuerto de Seoul.
  ```
- Protected tokens: `Suwon`, `Seongnam`, `Goyang`, `Bucheon`, `Gyeonggi`, `Seoul`

### ITEM 631

- Source context: L118 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Los números de ruta, operadores, sistemas de billetes y paradas varían, y los servicios nocturnos de Gyeonggi también se publican por separado de los de Seoul.
  ```
- Protected tokens: `Gyeonggi`, `Seoul`

### ITEM 632

- Source context: L119 - `p.airport-bus-context-link:nth-of-type(4)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Gyeonggi Bus Information
  ```
- Protected tokens: `Gyeonggi Bus Information`

### ITEM 633

- Source context: L122 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Otras ciudades de Corea
  ```
- Protected tokens: None identified in this item.

### ITEM 634

- Source context: L123 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Si el aeropuerto es solo el comienzo de un viaje más largo, un autobús interurbano puede evitar tener que entrar primero en Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 635

- Source context: L124 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Busca la ciudad real y el nombre de la terminal. Las ciudades grandes pueden tener más de una terminal de autobuses, y llegar al lado equivocado de la ciudad puede borrar la comodidad de haber tomado un autobús directo desde el aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 636

- Source context: L125 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  BusTago ofrece información de reservas online para las terminales interurbanas participantes, pero no todas las rutas admiten reserva por internet.
  ```
- Protected tokens: `BusTago`

### ITEM 637

- Source context: L128 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Por la noche
  ```
- Protected tokens: None identified in this item.

### ITEM 638

- Source context: L129 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús nocturno no es simplemente la ruta diurna funcionando hasta más tarde.
  ```
- Protected tokens: None identified in this item.

### ITEM 639

- Source context: L130 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Incheon Airport publica información nocturna separada para Terminal 1 y Terminal 2 y distingue los servicios de Seoul de los de Gyeonggi. Empieza por la terminal donde realmente aterriza tu vuelo y después mira dónde termina la ruta nocturna.
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`, `Terminal 2`, `Seoul`, `Gyeonggi`

### ITEM 640

- Source context: L131 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús que técnicamente circula a la 1 a. m. sirve de poco si su última parada te deja lejos del hotel con equipaje.
  ```
- Protected tokens: `1`

### ITEM 641

- Source context: L134 - `h3`
- Element/type: H3
- Spanish:

  ```text
  El shuttle gratuito del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 642

- Source context: L135 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El shuttle del aeropuerto es otro servicio distinto.
  ```
- Protected tokens: None identified in this item.

### ITEM 643

- Source context: L136 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Traslada pasajeros entre terminales e instalaciones de la zona aeroportuaria. No es transporte gratuito hacia Seoul o Gyeonggi.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`

### ITEM 644

- Source context: L139 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Cuándo el autobús no es la opción evidente
  ```
- Protected tokens: None identified in this item.

### ITEM 645

- Source context: L140 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un autobús no es automáticamente mejor por ser directo. Si la parada más cercana aún deja una caminata difícil, AREX puede ser más fácil para un hotel bien conectado por tren. Un taxi tiene más sentido cuando importa más ir puerta a puerta que el precio, mientras que un vehículo reservado con antelación puede ser útil para un grupo grande o equipaje poco habitual. La guía de traslados de Incheon Airport explica todas estas diferencias.
  ```
- Protected tokens: `AREX`, `Incheon Airport`

### ITEM 646

- Source context: L146 - `h2#find-route`
- Element/type: H2
- Spanish:

  ```text
  Encuentra una ruta que siga siendo práctica después de bajar
  ```
- Protected tokens: None identified in this item.

### ITEM 647

- Source context: L147 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Cuando ya tienes el pin del hotel, buscar la ruta resulta mucho más sencillo. La búsqueda oficial del aeropuerto te indica qué autobuses sirven la zona; la página del operador te da los detalles que pueden cambiar de una empresa a otra.
  ```
- Protected tokens: None identified in this item.

### ITEM 648

- Source context: L150 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Guarda el hotel en coreano. Conserva en el teléfono el nombre coreano del lugar, la dirección vial y el número de teléfono. Un pin en el mapa puede ser incluso más útil que el nombre del barrio.
  ```
- Protected tokens: None identified in this item.

### ITEM 649

- Source context: L151 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Mira el último tramo del viaje. Compara las paradas candidatas con la entrada del hotel, no solo con el centro del distrito. Las cuestas, pasos subterráneos e intersecciones anchas importan cuando llevas equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 650

- Source context: L152 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Usa la búsqueda de autobuses de Incheon Airport. Busca el destino e identifica la ruta, la parada y el operador. El aeropuerto separa los servicios de Seoul, Gyeonggi e interurbanos en lugar de tratarlos como un único sistema.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Gyeonggi`

### ITEM 651

- Source context: L153 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Después abre la página del propio operador. Los horarios, tarifas, billetes y normas de equipaje pueden diferir incluso entre autobuses que sirven zonas similares de Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 652

- Source context: L154 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Haz coincidir la ruta con T1 o T2. La terminal de tu vuelo afecta dónde compras el billete, dónde subes y, a veces, la hora de salida.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 653

- Source context: L155 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Deja margen para el propio aeropuerto. Incheon Airport aconseja a quienes reservan transporte público considerar aproximadamente 1–2 hours para inmigración después de la llegada del vuelo. El equipaje y aduanas añaden su propia incertidumbre, así que una conexión muy ajustada con el último autobús necesita un plan alternativo.
  ```
- Protected tokens: `Incheon Airport`, `1–2 hours`

### ITEM 654

- Source context: L159 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Guía para elegir autobús del aeropuerto según la zona del hotel en Seoul y regiones cercanas
  ```
- Protected tokens: `Seoul`

### ITEM 655

- Source context: L160 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Las rutas y los lugares de embarque pueden cambiar. Confirma la información más reciente en el sitio oficial del aeropuerto o del operador antes del viaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 656

- Source context: L166 - `h2#operators`
- Element/type: H2
- Spanish:

  ```text
  Operadores oficiales de autobuses del aeropuerto de Seoul
  ```
- Protected tokens: `Seoul`

### ITEM 657

- Source context: L167 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Los autobuses del aeropuerto de Seoul no los opera una sola empresa. Una vez que tienes una ruta, el nombre del operador te indica dónde comprobar las reglas que realmente se aplican.
  ```
- Protected tokens: `Seoul`

### ITEM 658

- Source context: L170 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Airport Limousine Co.
  ```
- Protected tokens: `Airport Limousine Co.`

### ITEM 659

- Source context: L171 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Muchas rutas del aeropuerto de Seoul las opera Airport Limousine Co. Su sitio publica rutas, horarios, tarifas, normas de equipaje e información de billetes.
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`

### ITEM 660

- Source context: L174 - `h3`
- Element/type: H3
- Spanish:

  ```text
  K Airport Limousine
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 661

- Source context: L175 - `p`
- Element/type: Body text
- Spanish:

  ```text
  K Airport Limousine opera su propia red de rutas y publica búsqueda de paradas, horarios, tarifas, información de billetes y seguimiento de autobuses.
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 662

- Source context: L178 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Seoul Airport Limousine
  ```
- Protected tokens: `Seoul Airport Limousine`

### ITEM 663

- Source context: L179 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Es otro operador independiente de Seoul, con rutas que sirven partes de Gangnam y el sureste de Seoul. Tiene su propio horario e información operativa.
  ```
- Protected tokens: `Seoul`, `Gangnam`

### ITEM 664

- Source context: L182 - `h3`
- Element/type: H3
- Spanish:

  ```text
  CALT
  ```
- Protected tokens: `CALT`

### ITEM 665

- Source context: L183 - `p`
- Element/type: Body text
- Spanish:

  ```text
  CALT opera sus propias rutas de autobús limusina del aeropuerto, incluida la conexión con la zona de COEX, con información separada de rutas y horarios.
  ```
- Protected tokens: `CALT`, `COEX`

### ITEM 666

- Source context: L186 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El nombre de la empresa no es un detalle administrativo irrelevante. Te indica qué tarifas, normas de equipaje y reglas de billetes se aplican realmente a tu autobús.
  ```
- Protected tokens: None identified in this item.

### ITEM 667

- Source context: L190 - `h2#tickets-boarding`
- Element/type: H2
- Spanish:

  ```text
  Comprar el billete y embarcar en T1 o T2
  ```
- Protected tokens: `T1`, `T2`

### ITEM 668

- Source context: L191 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Cuando llegas a la sala pública de llegadas, la mayor parte de la planificación ya está hecha. Solo necesitas hacer coincidir la ruta guardada con el punto correcto de venta de billetes y la zona de embarque.
  ```
- Protected tokens: None identified in this item.

### ITEM 669

- Source context: L195 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Señal superior de Incheon Airport que indica por separado Arrivals y Transfer
  ```
- Protected tokens: `Incheon Airport`

### ITEM 670

- Source context: L196 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Los pasajeros que entran en Korea deben seguir Arrivals. Las señales de Transfer son para pasajeros en conexión que siguen otro recorrido.
  ```
- Protected tokens: `Korea`

### ITEM 671

- Source context: L199 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Terminal 1
  ```
- Protected tokens: `Terminal 1`

### ITEM 672

- Source context: L200 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  En Terminal 1, la venta de billetes de autobús está en la planta de llegadas.
  ```
- Protected tokens: `Terminal 1`

### ITEM 673

- Source context: L201 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El aeropuerto indica actualmente mostradores de billetes dentro de la terminal cerca de las salidas 4 y 9, con mostradores exteriores adicionales alrededor de las salidas 4, 6, 7, 8, 11 y 13. Usa esos puntos para orientarte, pero sigue la señalización en tiempo real si la distribución ha cambiado.
  ```
- Protected tokens: `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 674

- Source context: L202 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  No hay razón para memorizar una dársena semanas antes del viaje. Compra o verifica primero el billete y después sigue la pantalla actual hasta el punto de embarque.
  ```
- Protected tokens: None identified in this item.

### ITEM 675

- Source context: L207 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Panel de información de transporte de Terminal 1 que muestra las zonas de embarque de autobuses del aeropuerto en Incheon Airport
  ```
- Protected tokens: `Terminal 1`, `Incheon Airport`

### ITEM 676

- Source context: L208 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Este panel de información de transporte de Terminal 1 muestra las zonas de autobuses por destino. Úsalo para orientarte y después confirma la ruta y la dársena en la pantalla actual del aeropuerto.
  ```
- Protected tokens: `Terminal 1`

### ITEM 677

- Source context: L211 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Terminal 2
  ```
- Protected tokens: `Terminal 2`

### ITEM 678

- Source context: L212 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  En Terminal 2, los autobuses se gestionan desde B1 Bus Terminal en Transportation Center.
  ```
- Protected tokens: `Terminal 2`, `B1 Bus Terminal`, `Transportation Center`

### ITEM 679

- Source context: L213 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Actualmente el aeropuerto dirige allí a los pasajeros para obtener información de autobuses y comprar billetes.
  ```
- Protected tokens: None identified in this item.

### ITEM 680

- Source context: L216 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Guía de lugares de embarque de autobuses de Incheon Airport para Terminal 1 y Terminal 2
  ```
- Protected tokens: `Incheon Airport`, `Terminal 1`, `Terminal 2`

### ITEM 681

- Source context: L217 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Guía visual de los lugares de embarque de autobuses de Terminal 1 y Terminal 2
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 682

- Source context: L222 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Señal de Terminal 2 para las dársenas 29 to 35 de autobuses del aeropuerto con destino a Seoul en Incheon Airport
  ```
- Protected tokens: `Terminal 2`, `Seoul`, `29 to 35`, `Incheon Airport`

### ITEM 683

- Source context: L223 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Esta foto muestra la zona de embarque de autobuses del aeropuerto con destino a Seoul para las dársenas 29 to 35 en Terminal 2. Las asignaciones de dársenas y rutas pueden cambiar, así que comprueba el billete y la pantalla en tiempo real antes de subir.
  ```
- Protected tokens: `Seoul`, `29 to 35`, `Terminal 2`

### ITEM 684

- Source context: L226 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Una regla de T2 que conviene conocer en 2026
  ```
- Protected tokens: `T2`, `2026`

### ITEM 685

- Source context: L227 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  No des por hecho que en todos los autobuses del aeropuerto de Seoul puedes simplemente subir y validar una tarjeta de transporte.
  ```
- Protected tokens: `Seoul`

### ITEM 686

- Source context: L228 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Airport Limousine Co. cambió su procedimiento de T2 en March 2026: los pasajeros que toman sus autobuses hacia Seoul deben comprar un billete antes de subir, usando un mostrador atendido o una máquina en B1. K Airport Limousine también indica a los pasajeros que salen de Incheon Airport que compren un billete de reserva en un mostrador o máquina.
  ```
- Protected tokens: `Airport Limousine Co.`, `T2`, `March 2026`, `Seoul`, `B1`, `K Airport Limousine`, `Incheon Airport`

### ITEM 687

- Source context: L229 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Por eso no conviene copiar las instrucciones de pago de una empresa de autobuses del aeropuerto y aplicarlas a otra.
  ```
- Protected tokens: None identified in this item.

### ITEM 688

- Source context: L233 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  De la sala de llegadas a tu asiento
  ```
- Protected tokens: None identified in this item.

### ITEM 689

- Source context: L235 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Entra en la sala pública de llegadas después de inmigración, recogida de equipaje y aduanas.
  ```
- Protected tokens: None identified in this item.

### ITEM 690

- Source context: L236 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Ve a la zona de billetes de autobús de tu terminal y muestra el destino guardado si no tienes clara la ruta.
  ```
- Protected tokens: None identified in this item.

### ITEM 691

- Source context: L237 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Lee el billete antes de alejarte del mostrador. La parada de destino y la terminal importan más que memorizar el número de ruta.
  ```
- Protected tokens: None identified in this item.

### ITEM 692

- Source context: L238 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Sigue la pantalla actual hasta la dársena. Si una dársena mostrada en una captura antigua no coincide con la pantalla del aeropuerto, usa la pantalla del aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 693

- Source context: L239 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Muestra el destino antes de que tu maleta vaya a la bodega del autobús. Conserva cualquier etiqueta de reclamación hasta que vuelvas a tener la maleta en tus manos.
  ```
- Protected tokens: None identified in this item.

### ITEM 694

- Source context: L240 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Mantén abierto el pin del hotel durante el trayecto. Las pantallas y anuncios a bordo ayudan, pero ver tu posición en el mapa hace que una parada desconocida sea más fácil de reconocer.
  ```
- Protected tokens: None identified in this item.

### ITEM 695

- Source context: L244 - `img @alt`
- Element/type: Image alt
- Spanish:

  ```text
  Guía paso a paso para usar el autobús limusina de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 696

- Source context: L245 - `figcaption`
- Element/type: Figcaption
- Spanish:

  ```text
  Proceso del autobús limusina del aeropuerto desde la comprobación de la ruta hasta la llegada al hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 697

- Source context: L248 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  El pago depende del operador, no de una regla única del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 698

- Source context: L249 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  No existe una única regla de pago para todos los autobuses que salen de Incheon Airport.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 699

- Source context: L250 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  K Airport Limousine, por ejemplo, publica compatibilidad con tarjeta de crédito, tarjeta de transporte y T-money, pero también exige billetes de reserva para autobuses que salen de Incheon Airport. Airport Limousine Co. tiene su propio procedimiento de billetes del aeropuerto.
  ```
- Protected tokens: `K Airport Limousine`, `T-money`, `Incheon Airport`, `Airport Limousine Co.`

### ITEM 700

- Source context: L251 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  Que un viajero haya podido validar T-money en una ruta no significa que el mismo procedimiento se aplique en el siguiente mostrador.
  ```
- Protected tokens: `T-money`

### ITEM 701

- Source context: L252 - `p:nth-of-type(7)`
- Element/type: Body text
- Spanish:

  ```text
  Una segunda tarjeta de pago y algo de won coreano siguen siendo alternativas sensatas para el día de llegada. La máquina de billetes o el mostrador atendido de la ruta real son la referencia final.
  ```
- Protected tokens: None identified in this item.

### ITEM 702

- Source context: L256 - `h2#luggage-problems`
- Element/type: H2
- Spanish:

  ```text
  Equipaje, familias y llegadas tardías
  ```
- Protected tokens: None identified in this item.

### ITEM 703

- Source context: L258 - `h3:nth-of-type(1)`
- Element/type: H3
- Spanish:

  ```text
  Equipaje
  ```
- Protected tokens: None identified in this item.

### ITEM 704

- Source context: L259 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El compartimento de equipaje es una de las mejores razones para tomar un autobús del aeropuerto. También es una de las partes donde los viajeros pueden hacer suposiciones equivocadas.
  ```
- Protected tokens: None identified in this item.

### ITEM 705

- Source context: L260 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  No existe una franquicia de equipaje universal para todos los autobuses de Incheon Airport.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 706

- Source context: L261 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Airport Limousine Co. publica actualmente una franquicia de hasta dos piezas si cada una mide menos de 28 inches y pesa menos de 20 kg, mientras que K Airport Limousine publica una franquicia distinta de dos piezas con un peso combinado de hasta 40 kg por pasajero de pago.
  ```
- Protected tokens: `Airport Limousine Co.`, `28 inches`, `20 kg`, `K Airport Limousine`, `40 kg`

### ITEM 707

- Source context: L262 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Son ejemplos de reglas de operadores, no una norma para todos los autobuses del aeropuerto.
  ```
- Protected tokens: None identified in this item.

### ITEM 708

- Source context: L263 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Lleva contigo pasaportes, medicación, dispositivos electrónicos y otros objetos de valor. Si una maleta va en la bodega del autobús, conserva la etiqueta correspondiente hasta que te la devuelvan.
  ```
- Protected tokens: None identified in this item.

### ITEM 709

- Source context: L265 - `h3:nth-of-type(2)`
- Element/type: H3
- Spanish:

  ```text
  Niños, cochecitos y grupos
  ```
- Protected tokens: None identified in this item.

### ITEM 710

- Source context: L266 - `p:nth-of-type(6)`
- Element/type: Body text
- Spanish:

  ```text
  Para una familia, el autobús puede resultar mucho más fácil que el tren cuando la parada está junto al hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 711

- Source context: L267 - `p:nth-of-type(7)`
- Element/type: Body text
- Spanish:

  ```text
  La comparación cambia si un cochecito, varias maletas o un niño cansado todavía tienen que recorrer otros 800 metres desde la parada del autobús.
  ```
- Protected tokens: `800 metres`

### ITEM 712

- Source context: L268 - `p:nth-of-type(8)`
- Element/type: Body text
- Spanish:

  ```text
  Las tarifas infantiles, las reglas sobre asiento separado y el manejo de cochecitos también dependen del operador, no de los “autobuses de Incheon Airport” en conjunto.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 713

- Source context: L269 - `p:nth-of-type(9)`
- Element/type: Body text
- Spanish:

  ```text
  Para una familia o grupo, compara el viaje completo después de bajar del autobús con el coste y la capacidad de un taxi o traslado reservado con antelación.
  ```
- Protected tokens: None identified in this item.

### ITEM 714

- Source context: L271 - `h3:nth-of-type(3)`
- Element/type: H3
- Spanish:

  ```text
  Llegadas tardías
  ```
- Protected tokens: None identified in this item.

### ITEM 715

- Source context: L272 - `p:nth-of-type(10)`
- Element/type: Body text
- Spanish:

  ```text
  Usa la hora a la que puedes llegar a la sala pública, no la hora de aterrizaje del avión.
  ```
- Protected tokens: None identified in this item.

### ITEM 716

- Source context: L273 - `p:nth-of-type(11)`
- Element/type: Body text
- Spanish:

  ```text
  Entre esos dos momentos están inmigración, recogida de equipaje y aduanas. El propio Incheon Airport indica a los viajeros que dejen margen para inmigración al reservar transporte público.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 717

- Source context: L274 - `p:nth-of-type(12)`
- Element/type: Body text
- Spanish:

  ```text
  Después consulta la página actual de autobuses nocturnos para tu terminal y zona de destino. Los servicios de Seoul y Gyeonggi se publican por separado, y T1 y T2 no comparten un horario idéntico.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`, `T1`, `T2`

### ITEM 718

- Source context: L275 - `p:nth-of-type(13)`
- Element/type: Body text
- Spanish:

  ```text
  Si el autobús que queda te deja lejos del hotel, un taxi oficial del aeropuerto puede ser la opción nocturna más práctica.
  ```
- Protected tokens: None identified in this item.

### ITEM 719

- Source context: L279 - `h2#problems`
- Element/type: H2
- Spanish:

  ```text
  Si algo sale mal
  ```
- Protected tokens: None identified in this item.

### ITEM 720

- Source context: L282 - `h3`
- Element/type: H3
- Spanish:

  ```text
  La tarjeta no funciona
  ```
- Protected tokens: None identified in this item.

### ITEM 721

- Source context: L283 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Que una tarjeta sea rechazada no significa necesariamente que no puedas usar el autobús. Pregunta en el mostrador atendido qué métodos de pago acepta esa ruta antes de probar la misma tarjeta repetidamente. Si no puedes comprar el billete con el método que tienes, compara otra ruta verificada o la parada oficial de taxis.
  ```
- Protected tokens: None identified in this item.

### ITEM 722

- Source context: L286 - `h3`
- Element/type: H3
- Spanish:

  ```text
  El billete es incorrecto
  ```
- Protected tokens: None identified in this item.

### ITEM 723

- Source context: L287 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Si el destino, la terminal o la salida son incorrectos, vuelve al vendedor antes de embarcar. Muestra la dirección del alojamiento y pregunta si el billete puede cambiarse o reembolsarse según las reglas del operador.
  ```
- Protected tokens: None identified in this item.

### ITEM 724

- Source context: L290 - `h3`
- Element/type: H3
- Spanish:

  ```text
  El autobús ya salió
  ```
- Protected tokens: None identified in this item.

### ITEM 725

- Source context: L291 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Mira la siguiente salida antes de abandonar la ruta. Esperar 20 minutos todavía puede ser más fácil que mover el equipaje hasta el tren; una espera mucho mayor puede hacer que AREX o un taxi tengan más sentido.
  ```
- Protected tokens: `20`, `AREX`

### ITEM 726

- Source context: L294 - `h3`
- Element/type: H3
- Spanish:

  ```text
  La dársena no está donde decía la guía antigua
  ```
- Protected tokens: None identified in this item.

### ITEM 727

- Source context: L295 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Usa la pantalla en tiempo real del aeropuerto. Las dársenas y las asignaciones de ruta pueden cambiar, por eso una captura guardada meses antes nunca debería tener prioridad sobre la señalización de la terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 728

- Source context: L298 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Buscaste la terminal equivocada
  ```
- Protected tokens: None identified in this item.

### ITEM 729

- Source context: L299 - `p`
- Element/type: Body text
- Spanish:

  ```text
  T1 y T2 tienen zonas de autobuses diferentes. Comprueba la terminal en la información del vuelo antes de seguir las instrucciones de billetes o embarque de otra terminal.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 730

- Source context: L302 - `h3`
- Element/type: H3
- Spanish:

  ```text
  La parada queda más lejos del hotel de lo esperado
  ```
- Protected tokens: None identified in this item.

### ITEM 731

- Source context: L303 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Abre la ruta a pie antes de bajar. Un taxi corto desde la parada puede evitar una caminata final difícil, pero si el problema es evidente antes de salir del aeropuerto, otra opción de transporte puede ser más sencilla desde el principio.
  ```
- Protected tokens: None identified in this item.

### ITEM 732

- Source context: L306 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Ya no queda un autobús útil
  ```
- Protected tokens: None identified in this item.

### ITEM 733

- Source context: L307 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Vuelve a comprobar la información de autobuses nocturnos específica de tu terminal. Si ninguno llega a una parada práctica, utiliza la parada oficial de taxis del aeropuerto o una recogida confirmada en lugar de aceptar una oferta no solicitada dentro de la terminal.
  ```
- Protected tokens: None identified in this item.

### ITEM 734

- Source context: L310 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Falta la etiqueta del equipaje
  ```
- Protected tokens: None identified in this item.

### ITEM 735

- Source context: L311 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Avísalo al conductor o al personal del autobús antes de abandonar la parada. Ten a mano el billete, número de ruta, hora del viaje y una descripción de la maleta mientras comprueban la propiedad.
  ```
- Protected tokens: None identified in this item.

### ITEM 736

- Source context: L317 - `h2#return-airport`
- Element/type: H2
- Spanish:

  ```text
  Regreso a Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 737

- Source context: L318 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La parada que te llevó a Seoul no es automáticamente la parada que te devuelve al aeropuerto.
  ```
- Protected tokens: `Seoul`

### ITEM 738

- Source context: L319 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Las paradas con destino al aeropuerto pueden estar al otro lado de una avenida grande o en un lugar completamente distinto. Localiza la parada de regreso antes del día de salida, guarda el nombre en coreano y el pin del mapa y revisa el acceso a pie con equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 739

- Source context: L321 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Busca por separado la parada con destino al aeropuerto. No inviertas la ruta de llegada por simple suposición.
  ```
- Protected tokens: None identified in this item.

### ITEM 740

- Source context: L322 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Comprueba la terminal de tu aerolínea. Confirma si necesitas T1 o T2 y si el autobús sirve ambas.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 741

- Source context: L323 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Consulta el horario actual el día antes de salir. Los vuelos tempranos requieren especial precaución.
  ```
- Protected tokens: None identified in this item.

### ITEM 742

- Source context: L324 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Deja más margen que el tiempo programado por carretera. Los autobuses del aeropuerto comparten el tráfico de Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 743

- Source context: L325 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Conoce la alternativa antes de salir del hotel. Si el primer autobús, la parada o la reserva dejan de funcionar, ya deberías tener guardado un plan con AREX o taxi.
  ```
- Protected tokens: `AREX`

### ITEM 744

- Source context: L327 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Para un vuelo temprano, importa más el primer autobús hacia el aeropuerto que la ruta que parecía cómoda durante el día. Si el horario deja poco margen para el check-in de la aerolínea, salir antes en tren o taxi es la decisión más segura.
  ```
- Protected tokens: None identified in this item.

### ITEM 745

- Source context: L331 - `h2#departure-checklist`
- Element/type: H2
- Spanish:

  ```text
  Antes de salir del hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 746

- Source context: L333 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Parada hacia el aeropuerto guardada en coreano
  ```
- Protected tokens: None identified in this item.

### ITEM 747

- Source context: L334 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  T1 o T2 confirmada
  ```
- Protected tokens: `T1`, `T2`

### ITEM 748

- Source context: L335 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Hora de salida actual comprobada
  ```
- Protected tokens: None identified in this item.

### ITEM 749

- Source context: L336 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Billete o método de pago entendido
  ```
- Protected tokens: None identified in this item.

### ITEM 750

- Source context: L337 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Ruta con equipaje hasta la parada comprobada
  ```
- Protected tokens: None identified in this item.

### ITEM 751

- Source context: L338 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Alternativa con AREX o taxi guardada
  ```
- Protected tokens: `AREX`

### ITEM 752

- Source context: L343 - `h2#faq`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes sobre el autobús del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 753

- Source context: L346 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué diferencia hay entre un autobús del aeropuerto y un autobús limusina del aeropuerto?
  ```
- Protected tokens: None identified in this item.

### ITEM 754

- Source context: L347 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  En Incheon Airport, “autobús del aeropuerto” abarca varios servicios distintos. “Autobús limusina” se usa habitualmente para rutas en autocar, especialmente hacia Seoul, pero no existe un único operador ni una regla universal de billetes. Importan más la ruta y la empresa que la etiqueta.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 755

- Source context: L350 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo encuentro el autobús adecuado para mi hotel?
  ```
- Protected tokens: None identified in this item.

### ITEM 756

- Source context: L351 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Empieza por el pin exacto del hotel. Busca paradas que dejen una caminata final razonable y después busca esos destinos en la herramienta oficial de autobuses de Incheon Airport. Cuando tengas la ruta, utiliza la página del operador para consultar el horario, la tarifa y las reglas de equipaje actuales.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 757

- Source context: L354 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Todos los autobuses del aeropuerto de Seoul los opera Airport Limousine Co.?
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`

### ITEM 758

- Source context: L355 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. Varias empresas operan autobuses del aeropuerto en Seoul, entre ellas Airport Limousine Co., K Airport Limousine, Seoul Airport Limousine y CALT. Sus reglas de rutas y billetes no son intercambiables.
  ```
- Protected tokens: `Seoul`, `Airport Limousine Co.`, `K Airport Limousine`, `Seoul Airport Limousine`, `CALT`

### ITEM 759

- Source context: L358 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde puedo comprar un billete de autobús en Terminal 1?
  ```
- Protected tokens: `Terminal 1`

### ITEM 760

- Source context: L359 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Incheon Airport indica actualmente mostradores de billetes en T1 dentro de la terminal cerca de las salidas 4 y 9 y otros puntos exteriores cerca de las salidas 4, 6, 7, 8, 11 y 13. Sigue la señalización actual si la distribución del aeropuerto ha cambiado.
  ```
- Protected tokens: `Incheon Airport`, `T1`, `4`, `9`, `6`, `7`, `8`, `11`, `13`

### ITEM 761

- Source context: L362 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde puedo comprar un billete de autobús en Terminal 2?
  ```
- Protected tokens: `Terminal 2`

### ITEM 762

- Source context: L363 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  La información de autobuses y la compra de billetes se gestionan en B1 Bus Terminal dentro de Terminal 2 Transportation Center. Algunos operadores exigen tener billete antes de llegar al autobús, así que identifica la empresa antes de ir a la dársena.
  ```
- Protected tokens: `B1 Bus Terminal`, `Terminal 2 Transportation Center`

### ITEM 763

- Source context: L366 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo usar T-money o una tarjeta de crédito extranjera?
  ```
- Protected tokens: `T-money`

### ITEM 764

- Source context: L367 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No hay una respuesta única para todo el aeropuerto. Algunos operadores aceptan tarjetas de transporte o crédito, mientras que ciertos servicios que salen del aeropuerto exigen comprar un billete en mostrador o máquina. Sigue la regla del operador real en lugar de asumir que el pago de un autobús se aplica a otro.
  ```
- Protected tokens: None identified in this item.

### ITEM 765

- Source context: L370 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cuánto equipaje puedo llevar?
  ```
- Protected tokens: None identified in this item.

### ITEM 766

- Source context: L371 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Las reglas de equipaje las fija la empresa de autobuses. Incluso los principales operadores de Seoul publican franquicias gratuitas distintas, así que consulta la empresa de tu ruta si llevas varias maletas grandes, equipamiento deportivo u otro artículo sobredimensionado.
  ```
- Protected tokens: `Seoul`

### ITEM 767

- Source context: L374 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué debo hacer si llego tarde por la noche?
  ```
- Protected tokens: None identified in this item.

### ITEM 768

- Source context: L375 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Usa la página oficial de autobuses nocturnos para la terminal donde aterriza tu vuelo y después distingue las rutas de Seoul de las de Gyeonggi. Si ningún autobús termina cerca de un destino práctico, usa la parada oficial de taxis o una recogida reservada y confirmada.
  ```
- Protected tokens: `Seoul`, `Gyeonggi`

### ITEM 769

- Source context: L378 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Hay autobuses desde Incheon Airport a ciudades fuera de Seoul?
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 770

- Source context: L379 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sí. Los servicios de Incheon, Gyeonggi e interurbanos conectan el aeropuerto con muchos otros destinos. Busca por la ciudad o terminal de autobuses exacta y después confirma el operador y el método de reserva.
  ```
- Protected tokens: `Incheon`, `Gyeonggi`

### ITEM 771

- Source context: L382 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Cómo tomo el autobús de regreso a Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 772

- Source context: L383 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Busca por separado la parada con destino al aeropuerto, comprueba si el autobús sirve T1 o T2 y vuelve a revisar el horario antes de salir. Deja margen adicional para el tráfico y ten disponible otra ruta al aeropuerto si el horario queda demasiado ajustado.
  ```
- Protected tokens: `T1`, `T2`

### ITEM 773

- Source context: L389 - `h2#official-sources`
- Element/type: H2
- Spanish:

  ```text
  Fuentes oficiales
  ```
- Protected tokens: None identified in this item.

### ITEM 774

- Source context: L391 - `li:nth-of-type(1)`
- Element/type: List text
- Spanish:

  ```text
  Información de autobús público de Incheon Airport Terminal 1
  ```
- Protected tokens: `Incheon Airport Terminal 1`

### ITEM 775

- Source context: L392 - `li:nth-of-type(2)`
- Element/type: List text
- Spanish:

  ```text
  Información de la terminal de autobuses de Incheon Airport Terminal 2
  ```
- Protected tokens: `Incheon Airport Terminal 2`

### ITEM 776

- Source context: L393 - `li:nth-of-type(3)`
- Element/type: List text
- Spanish:

  ```text
  Buscador oficial de autobuses de Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 777

- Source context: L394 - `li:nth-of-type(4)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Terminal 1 hacia Seoul
  ```
- Protected tokens: `Terminal 1`, `Seoul`

### ITEM 778

- Source context: L395 - `li:nth-of-type(5)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Terminal 2 hacia Seoul
  ```
- Protected tokens: `Terminal 2`, `Seoul`

### ITEM 779

- Source context: L396 - `li:nth-of-type(6)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Terminal 1 hacia Gyeonggi
  ```
- Protected tokens: `Terminal 1`, `Gyeonggi`

### ITEM 780

- Source context: L397 - `li:nth-of-type(7)`
- Element/type: List text
- Spanish:

  ```text
  Autobuses nocturnos de Terminal 2 hacia Gyeonggi
  ```
- Protected tokens: `Terminal 2`, `Gyeonggi`

### ITEM 781

- Source context: L398 - `li:nth-of-type(8)`
- Element/type: List text
- Spanish:

  ```text
  Airport Limousine Co.
  ```
- Protected tokens: `Airport Limousine Co.`

### ITEM 782

- Source context: L399 - `li:nth-of-type(9)`
- Element/type: List text
- Spanish:

  ```text
  K Airport Limousine
  ```
- Protected tokens: `K Airport Limousine`

### ITEM 783

- Source context: L400 - `li:nth-of-type(10)`
- Element/type: List text
- Spanish:

  ```text
  Seoul Airport Limousine
  ```
- Protected tokens: `Seoul Airport Limousine`

### ITEM 784

- Source context: L401 - `li:nth-of-type(11)`
- Element/type: List text
- Spanish:

  ```text
  CALT City Airport Limousine
  ```
- Protected tokens: `CALT City Airport Limousine`

### ITEM 785

- Source context: L402 - `li:nth-of-type(12)`
- Element/type: List text
- Spanish:

  ```text
  Gyeonggi Bus Information
  ```
- Protected tokens: `Gyeonggi Bus Information`

### ITEM 786

- Source context: L403 - `li:nth-of-type(13)`
- Element/type: List text
- Spanish:

  ```text
  BusTago
  ```
- Protected tokens: `BusTago`

### ITEM 787

- Source context: L407 - `nav.related-links @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Guías relacionadas
  ```
- Protected tokens: None identified in this item.

### ITEM 788

- Source context: L408 - `p.related-links__title:nth-of-type(1)`
- Element/type: Related-guide text
- Spanish:

  ```text
  Guías relacionadas
  ```
- Protected tokens: None identified in this item.

### ITEM 789

- Source context: L409 - `p:nth-of-type(2)`
- Element/type: Related-guide text
- Spanish:

  ```text
  Guía de Incheon Airport · Procedimientos de llegada · Guía de AREX · Alternativa en taxi
  ```
- Protected tokens: `Incheon Airport`, `AREX`

### ITEM 790

- Source context: L410 - `p:nth-of-type(3)`
- Element/type: Related-guide text
- Spanish:

  ```text
  Comparar traslados desde el aeropuerto · Mapas coreanos · Pagos
  ```
- Protected tokens: None identified in this item.

---

# PAGE: `incheon-airport-private-transfer.html`

**English source SHA-256:** `532ff0942a5eabb17d0d928e5c46cb4855901b5e985b9c2109a55cf6986bcbc8`  
**Localized ITEM count:** 139

### ITEM 791

- Source context: L6 - `html > head > meta[name="description"] @content`
- Element/type: Meta description
- Spanish:

  ```text
  Cuándo tiene sentido un traslado privado desde Incheon Airport, cómo influye el equipaje en el tamaño del vehículo, cómo funciona la recogida en el aeropuerto y dónde reservar para Seoul.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 792

- Source context: L9 - `html > head > title`
- Element/type: Title
- Spanish:

  ```text
  Traslado privado de Incheon Airport a Seoul | Korea Inside
  ```
- Protected tokens: `Incheon Airport`, `Seoul`, `Korea Inside`

### ITEM 793

- Source context: L21 - `script[type="application/ld+json"] > mainEntity.0.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Vale la pena un traslado privado para cuatro personas?
  ```
- Protected tokens: None identified in this item.

### ITEM 794

- Source context: L24 - `script[type="application/ld+json"] > mainEntity.0.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  A veces. Cuatro tarifas separadas de transporte público pueden reducir la diferencia de precio, especialmente si al llegar a Seoul necesitas otro taxi por el equipaje o por el tramo final. La comparación útil es el coste total puerta a puerta, no solo la primera tarifa que ves.
  ```
- Protected tokens: `Seoul`

### ITEM 795

- Source context: L29 - `script[type="application/ld+json"] > mainEntity.1.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Puede un taxi regular llevar a cuatro personas y cuatro maletas grandes?
  ```
- Protected tokens: None identified in this item.

### ITEM 796

- Source context: L32 - `script[type="application/ld+json"] > mainEntity.1.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No de forma fiable. La capacidad de pasajeros y el espacio del maletero son límites distintos. Cuando cada pasajero lleva una maleta grande, un taxi jumbo o un vehículo más grande reservado con antelación puede ser más práctico.
  ```
- Protected tokens: None identified in this item.

### ITEM 797

- Source context: L37 - `script[type="application/ld+json"] > mainEntity.2.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Qué pasa si mi vuelo se retrasa?
  ```
- Protected tokens: None identified in this item.

### ITEM 798

- Source context: L40 - `script[type="application/ld+json"] > mainEntity.2.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Muchos traslados reservados con antelación piden el número de vuelo y hacen seguimiento de los retrasos, pero el tiempo de espera gratuito varía según el proveedor y el producto. La regla que importa es la vigente en la reserva que hayas seleccionado.
  ```
- Protected tokens: None identified in this item.

### ITEM 799

- Source context: L45 - `script[type="application/ld+json"] > mainEntity.3.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Dónde me encontraré con el conductor en Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 800

- Source context: L48 - `script[type="application/ld+json"] > mainEntity.3.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  La organización exacta depende de la reserva. Entre los métodos habituales están un conductor esperando en la sala de llegadas con un cartel con tu nombre o un mensaje que te dirige a una salida o punto de encuentro concretos.
  ```
- Protected tokens: None identified in this item.

### ITEM 801

- Source context: L53 - `script[type="application/ld+json"] > mainEntity.4.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Importa si llego a Terminal 1 o Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 802

- Source context: L56 - `script[type="application/ld+json"] > mainEntity.4.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Sí. Las terminales tienen zonas de llegada y puntos de recogida separados. Al hacer la reserva, utiliza la terminal indicada para tu vuelo de llegada.
  ```
- Protected tokens: None identified in this item.

### ITEM 803

- Source context: L61 - `script[type="application/ld+json"] > mainEntity.5.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Un traslado privado es lo mismo que un call van en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 804

- Source context: L64 - `script[type="application/ld+json"] > mainEntity.5.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No. “Traslado privado” es un término internacional de reserva para un vehículo reservado para un solo grupo. “Call van” es un término coreano para un servicio comercial de furgonetas. Algunos vehículos de traslado desde el aeropuerto pueden pertenecer a esa categoría, pero los términos no son intercambiables.
  ```
- Protected tokens: None identified in this item.

### ITEM 805

- Source context: L69 - `script[type="application/ld+json"] > mainEntity.6.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Necesito reservar con antelación el transporte desde Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 806

- Source context: L72 - `script[type="application/ld+json"] > mainEntity.6.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  No en todos los viajes. Los viajeros solos y las parejas con equipaje manejable suelen tener opciones sencillas de tren, autobús o taxi. Reservar con antelación resulta más útil cuando el tamaño del vehículo, el espacio para equipaje, una llegada tardía o un grupo grande hacen más importante encontrar el vehículo adecuado al llegar.
  ```
- Protected tokens: None identified in this item.

### ITEM 807

- Source context: L77 - `script[type="application/ld+json"] > mainEntity.7.name`
- Element/type: JSON-LD FAQ question
- Spanish:

  ```text
  ¿Puedo reservar un traslado privado para una llegada nocturna?
  ```
- Protected tokens: None identified in this item.

### ITEM 808

- Source context: L80 - `script[type="application/ld+json"] > mainEntity.7.acceptedAnswer.text`
- Element/type: JSON-LD FAQ answer
- Spanish:

  ```text
  Muchos productos de traslado de Incheon Airport operan hasta tarde o las 24 horas, pero la disponibilidad y los posibles recargos nocturnos dependen del proveedor y del vehículo. La página de reserva para la fecha elegida debería mostrar las condiciones vigentes.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 809

- Source context: L153 - `p.page-hero__breadcrumb`
- Element/type: Body text
- Spanish:

  ```text
  Inicio / Transporte / Traslado privado
  ```
- Protected tokens: None identified in this item.

### ITEM 810

- Source context: L154 - `h1.airport-page-hero__title`
- Element/type: H1
- Spanish:

  ```text
  Traslado privado de Incheon Airport a Seoul
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 811

- Source context: L156 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Un traslado privado empieza a tener sentido cuando un solo vehículo puede llevar a todo tu grupo y todo el equipaje directamente del aeropuerto al hotel.
  ```
- Protected tokens: None identified in this item.

### ITEM 812

- Source context: L157 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Para una o dos personas que viajan ligeras, AREX o el autobús limusina del aeropuerto normalmente costarán menos. La comparación cambia con una familia o grupo, varias maletas grandes, un cochecito, un viajero mayor, una llegada tardía o un hotel que deja una caminata incómoda desde la estación más cercana.
  ```
- Protected tokens: `AREX`

### ITEM 813

- Source context: L158 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  La pregunta real no es simplemente si un coche privado cuesta más. Es si ese coste adicional compensa eliminar los problemas de equipaje, transbordos y último tramo después de un vuelo largo.
  ```
- Protected tokens: None identified in this item.

### ITEM 814

- Source context: L167 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Quién se beneficia más de un traslado privado desde el aeropuerto?
  ```
- Protected tokens: None identified in this item.

### ITEM 815

- Source context: L170 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Suele merecer la pena considerar un traslado privado cuando tu grupo y el equipaje caben cómodamente en un solo vehículo y el viaje puerta a puerta elimina un último tramo complicado.
  ```
- Protected tokens: None identified in this item.

### ITEM 816

- Source context: L171 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  ¿Viajas solo o en pareja con equipaje manejable? El transporte público normalmente será la opción más barata.
  ```
- Protected tokens: None identified in this item.

### ITEM 817

- Source context: L172 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  ¿Viajas en familia o grupo con varias maletas grandes, llegas tarde o te alojas lejos de una parada cómoda de autobús del aeropuerto o tren? La diferencia de precio puede reducirse mucho más de lo que parece al principio.
  ```
- Protected tokens: None identified in this item.

### ITEM 818

- Source context: L180 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Qué significa “traslado privado” en Korea
  ```
- Protected tokens: `Korea`

### ITEM 819

- Source context: L182 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  En los sitios internacionales de reservas, un traslado privado significa un vehículo reservado con antelación para tu grupo, no un viaje compartido.
  ```
- Protected tokens: None identified in this item.

### ITEM 820

- Source context: L183 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  La palabra “privado” describe el trayecto. No significa que un coche particular normal pueda operar legalmente como servicio de aeropuerto de pago.
  ```
- Protected tokens: None identified in this item.

### ITEM 821

- Source context: L184 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  En Korea, los traslados de aeropuerto de pago se prestan mediante transporte comercial u otras estructuras empresariales legalmente autorizadas. También puedes ver el término local “call van”. Un call van coreano es un servicio comercial de furgonetas y no debería tratarse como el nombre universal en inglés de todos los traslados privados vendidos por internet.
  ```
- Protected tokens: `Korea`

### ITEM 822

- Source context: L185 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Para el viajero, la parte práctica es más sencilla: reserva mediante un servicio establecido y comprueba el vehículo real, el límite de pasajeros y el límite de equipaje indicados para esa reserva.
  ```
- Protected tokens: None identified in this item.

### ITEM 823

- Source context: L192 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Cuándo empieza a tener sentido pagar más
  ```
- Protected tokens: None identified in this item.

### ITEM 824

- Source context: L194 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La economía cambia cuando varias personas viajan juntas.
  ```
- Protected tokens: None identified in this item.

### ITEM 825

- Source context: L195 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El tren del aeropuerto y los autobuses se cobran por pasajero. Un traslado privado normalmente se cobra por vehículo. Cuatro tarifas separadas de transporte público, seguidas por un taxi o una caminata larga al llegar a Seoul, pueden reducir la diferencia de precio sorprendentemente rápido.
  ```
- Protected tokens: `Seoul`

### ITEM 826

- Source context: L196 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  El equipaje puede importar incluso más que el número de personas. Un grupo que usaría el tren fácilmente sin maletas puede vivir una experiencia muy distinta con cuatro maletas grandes, equipaje de cabina y un cochecito.
  ```
- Protected tokens: None identified in this item.

### ITEM 827

- Source context: L197 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Las llegadas tardías crean otro problema. La hora programada de aterrizaje no es la hora a la que sales de la terminal. Inmigración, recogida de equipaje y aduanas pueden retrasar mucho la hora real de salida, algo importante cuando se acerca el último tren o autobús conveniente.
  ```
- Protected tokens: None identified in this item.

### ITEM 828

- Source context: L198 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  El viaje puerta a puerta también tiene más valor cuando el hotel está cuesta arriba, a varias manzanas de la estación o es difícil de alcanzar con equipaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 829

- Source context: L205 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Traslado privado frente a otras opciones desde el aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 830

- Source context: L207 - `div.table-scroll.transfer-comparison @aria-label`
- Element/type: ARIA label
- Spanish:

  ```text
  Comparación desplazable de opciones de traslado desde Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 831

- Source context: L209 - `caption`
- Element/type: Table caption
- Spanish:

  ```text
  Comparación de transporte de Incheon Airport a Seoul
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 832

- Source context: L212 - `th:nth-of-type(1)`
- Element/type: Table header
- Spanish:

  ```text
  Opción
  ```
- Protected tokens: None identified in this item.

### ITEM 833

- Source context: L213 - `th:nth-of-type(2)`
- Element/type: Table header
- Spanish:

  ```text
  Puerta a puerta
  ```
- Protected tokens: None identified in this item.

### ITEM 834

- Source context: L214 - `th:nth-of-type(3)`
- Element/type: Table header
- Spanish:

  ```text
  Equipaje
  ```
- Protected tokens: None identified in this item.

### ITEM 835

- Source context: L215 - `th:nth-of-type(4)`
- Element/type: Table header
- Spanish:

  ```text
  Llegada tardía
  ```
- Protected tokens: None identified in this item.

### ITEM 836

- Source context: L216 - `th:nth-of-type(5)`
- Element/type: Table header
- Spanish:

  ```text
  Reserva
  ```
- Protected tokens: None identified in this item.

### ITEM 837

- Source context: L221 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Traslado privado
  ```
- Protected tokens: None identified in this item.

### ITEM 838

- Source context: L222 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 839

- Source context: L223 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Depende del vehículo reservado
  ```
- Protected tokens: None identified in this item.

### ITEM 840

- Source context: L224 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Buena opción si se reserva con antelación
  ```
- Protected tokens: None identified in this item.

### ITEM 841

- Source context: L225 - `td:nth-of-type(4)`
- Element/type: Table cell
- Spanish:

  ```text
  Normalmente
  ```
- Protected tokens: None identified in this item.

### ITEM 842

- Source context: L228 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Taxi regular
  ```
- Protected tokens: None identified in this item.

### ITEM 843

- Source context: L229 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 844

- Source context: L230 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  El espacio del maletero puede ser limitado
  ```
- Protected tokens: None identified in this item.

### ITEM 845

- Source context: L231 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Generalmente disponible
  ```
- Protected tokens: None identified in this item.

### ITEM 846

- Source context: L232 - `td:nth-of-type(4)`
- Element/type: Table cell
- Spanish:

  ```text
  Normalmente no es necesario
  ```
- Protected tokens: None identified in this item.

### ITEM 847

- Source context: L235 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Taxi Jumbo
  ```
- Protected tokens: None identified in this item.

### ITEM 848

- Source context: L236 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Sí
  ```
- Protected tokens: None identified in this item.

### ITEM 849

- Source context: L237 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Más espacio que un taxi regular
  ```
- Protected tokens: None identified in this item.

### ITEM 850

- Source context: L238 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  La disponibilidad puede variar
  ```
- Protected tokens: None identified in this item.

### ITEM 851

- Source context: L239 - `td:nth-of-type(4)`
- Element/type: Table cell
- Spanish:

  ```text
  Reservar con antelación puede ayudar
  ```
- Protected tokens: None identified in this item.

### ITEM 852

- Source context: L242 - `th`
- Element/type: Table header
- Spanish:

  ```text
  Autobús limusina del aeropuerto
  ```
- Protected tokens: None identified in this item.

### ITEM 853

- Source context: L243 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  De parada a parada
  ```
- Protected tokens: None identified in this item.

### ITEM 854

- Source context: L244 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Buen espacio para equipaje, pero tú gestionas el último tramo
  ```
- Protected tokens: None identified in this item.

### ITEM 855

- Source context: L245 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  El horario importa
  ```
- Protected tokens: None identified in this item.

### ITEM 856

- Source context: L246 - `td:nth-of-type(4)`
- Element/type: Table cell
- Spanish:

  ```text
  Normalmente opcional
  ```
- Protected tokens: None identified in this item.

### ITEM 857

- Source context: L249 - `th`
- Element/type: Table header
- Spanish:

  ```text
  AREX
  ```
- Protected tokens: `AREX`

### ITEM 858

- Source context: L250 - `td:nth-of-type(1)`
- Element/type: Table cell
- Spanish:

  ```text
  Tren hasta Seoul Station / estaciones conectadas
  ```
- Protected tokens: `Seoul Station`

### ITEM 859

- Source context: L251 - `td:nth-of-type(2)`
- Element/type: Table cell
- Spanish:

  ```text
  Tú llevas tus propias maletas
  ```
- Protected tokens: None identified in this item.

### ITEM 860

- Source context: L252 - `td:nth-of-type(3)`
- Element/type: Table cell
- Spanish:

  ```text
  Importa la hora del último tren
  ```
- Protected tokens: None identified in this item.

### ITEM 861

- Source context: L253 - `td:nth-of-type(4)`
- Element/type: Table cell
- Spanish:

  ```text
  Opcional
  ```
- Protected tokens: None identified in this item.

### ITEM 862

- Source context: L258 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Para una comparación completa de AREX, autobuses del aeropuerto y taxis, consulta la guía principal de traslados de Incheon Airport.
  ```
- Protected tokens: `AREX`, `Incheon Airport`

### ITEM 863

- Source context: L265 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Un vehículo de siete plazas no siempre es un vehículo de aeropuerto para siete personas
  ```
- Protected tokens: None identified in this item.

### ITEM 864

- Source context: L267 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El número de asientos y la capacidad de equipaje en un traslado de aeropuerto son dos cosas distintas.
  ```
- Protected tokens: None identified in this item.

### ITEM 865

- Source context: L268 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Un vehículo puede tener técnicamente suficientes asientos para tu grupo y aun así quedarse sin espacio útil cuando cada pasajero lleva una maleta grande.
  ```
- Protected tokens: None identified in this item.

### ITEM 866

- Source context: L269 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Cuenta primero el equipaje antes de fijarte en el número de pasajeros. Maletas grandes, equipaje de cabina, cochecitos, bolsas de golf, cajas de bicicleta y otros artículos sobredimensionados pueden cambiar el vehículo que necesitas.
  ```
- Protected tokens: None identified in this item.

### ITEM 867

- Source context: L270 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Los sitios de reservas suelen mostrar por separado la capacidad de pasajeros y la capacidad de equipaje. Esos dos números importan más que el nombre del modelo de la furgoneta.
  ```
- Protected tokens: None identified in this item.

### ITEM 868

- Source context: L271 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Cuando el equipaje queda cerca del límite publicado, reservar un vehículo más grande puede ser más seguro que intentar ocupar todos los asientos disponibles.
  ```
- Protected tokens: None identified in this item.

### ITEM 869

- Source context: L278 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Qué ocurre después de reservar
  ```
- Protected tokens: None identified in this item.

### ITEM 870

- Source context: L280 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  La reserva normalmente pide tu número de vuelo, terminal de recogida y destino en Seoul.
  ```
- Protected tokens: `Seoul`

### ITEM 871

- Source context: L281 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  El número de vuelo importa porque permite al servicio de recogida seguir el vuelo real en lugar de depender solo de la hora programada de aterrizaje.
  ```
- Protected tokens: None identified in this item.

### ITEM 872

- Source context: L282 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Los datos del conductor pueden llegar el día anterior al viaje o más cerca de la llegada, según el proveedor. Algunos servicios usan la sala de llegadas y un cartel con tu nombre. Otros envían una salida o punto de encuentro concretos mediante la app de reserva, correo electrónico o un servicio de mensajería.
  ```
- Protected tokens: None identified in this item.

### ITEM 873

- Source context: L283 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  Mantén disponibles después de aterrizar la confirmación de la reserva y el canal de contacto del proveedor. El Wi-Fi del aeropuerto suele ser suficiente para recibir un mensaje si tus datos móviles todavía no están activos.
  ```
- Protected tokens: None identified in this item.

### ITEM 874

- Source context: L284 - `p:nth-of-type(5)`
- Element/type: Body text
- Spanish:

  ```text
  Terminal 1 y Terminal 2 no son intercambiables. La terminal de la reserva debe coincidir con la del vuelo de llegada.
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 875

- Source context: L291 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Retrasos de vuelo y tiempo de espera
  ```
- Protected tokens: None identified in this item.

### ITEM 876

- Source context: L293 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  El seguimiento del vuelo es habitual en los traslados reservados con antelación, pero la política de espera no es idéntica entre todos los proveedores ni entre todas las ofertas.
  ```
- Protected tokens: None identified in this item.

### ITEM 877

- Source context: L294 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  Algunas reservas ajustan automáticamente la hora de recogida cuando el vuelo se retrasa. Otras incluyen un periodo fijo de espera gratuita después del aterrizaje o de la hora de recogida acordada. Una espera adicional puede empezar a cobrarse cuando termina ese periodo.
  ```
- Protected tokens: None identified in this item.

### ITEM 878

- Source context: L295 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Esta es una de las pocas condiciones que conviene leer en la página real del producto antes de pagar, especialmente para una llegada nocturna o un viaje con equipaje facturado.
  ```
- Protected tokens: None identified in this item.

### ITEM 879

- Source context: L296 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  No fijes una única política de espera y la apliques a las cuatro empresas de reserva.
  ```
- Protected tokens: None identified in this item.

### ITEM 880

- Source context: L303 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Dónde reservar un traslado privado desde Incheon Airport
  ```
- Protected tokens: `Incheon Airport`

### ITEM 881

- Source context: L304 - `p.transfer-chapter__answer`
- Element/type: Body text
- Spanish:

  ```text
  La misma ruta Incheon Airport–Seoul aparece en varias plataformas de reserva, a veces con tamaños de vehículo, límites de equipaje, reglas de espera y condiciones de cancelación diferentes.
  ```
- Protected tokens: `Incheon Airport`, `Seoul`

### ITEM 882

- Source context: L306 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Korea Inside no clasifica a ninguna de estas empresas como la opción universalmente “mejor”. La comparación útil está en el vehículo real y las condiciones de reserva disponibles para tu fecha.
  ```
- Protected tokens: `Korea Inside`

### ITEM 883

- Source context: L309 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Klook
  ```
- Protected tokens: `Klook`

### ITEM 884

- Source context: L310 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Klook ofrece varias opciones de traslado desde Incheon Airport y distintos tamaños de vehículo. Los límites de pasajeros, la capacidad de equipaje y las condiciones de espera pueden cambiar entre ofertas, así que importa más el vehículo seleccionado que el nombre de la plataforma.
  ```
- Protected tokens: `Klook`, `Incheon Airport`

### ITEM 885

- Source context: L311 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Ver opciones de traslado en Klook
  ```
- Protected tokens: `Klook`

### ITEM 886

- Source context: L314 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Trip.com
  ```
- Protected tokens: `Trip.com`

### ITEM 887

- Source context: L315 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Trip.com tiene un flujo específico de reserva de traslados para Incheon Airport, con opciones de vehículo vinculadas al vuelo y a los datos de recogida introducidos durante la reserva. Las condiciones exactas de pasajeros, equipaje y cancelación dependen de la reserva seleccionada.
  ```
- Protected tokens: `Trip.com`, `Incheon Airport`

### ITEM 888

- Source context: L316 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Ver opciones de traslado en Trip.com
  ```
- Protected tokens: `Trip.com`

### ITEM 889

- Source context: L319 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Trazy
  ```
- Protected tokens: `Trazy`

### ITEM 890

- Source context: L320 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Trazy está centrado en viajes por Korea y sus ofertas de traslados desde el aeropuerto suelen mostrar con detalle los límites de pasajeros y equipaje. Los recargos nocturnos, tiempos de espera y reglas de cancelación pueden variar según el producto y deben mantenerse vinculados a la reserva concreta en lugar de generalizarse a toda la plataforma.
  ```
- Protected tokens: `Trazy`, `Korea`

### ITEM 891

- Source context: L321 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Ver opciones de traslado en Trazy
  ```
- Protected tokens: `Trazy`

### ITEM 892

- Source context: L324 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Booking.com
  ```
- Protected tokens: `Booking.com`

### ITEM 893

- Source context: L325 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Booking.com ofrece reservas de taxi de aeropuerto y traslados privados junto con sus servicios de viaje más amplios. El tamaño del vehículo, la capacidad de equipaje, la organización del encuentro y las condiciones de cancelación varían según la oferta.
  ```
- Protected tokens: `Booking.com`

### ITEM 894

- Source context: L326 - `p:nth-of-type(2)`
- Element/type: CTA / visible link text
- Spanish:

  ```text
  Ver opciones de traslado en Booking.com
  ```
- Protected tokens: `Booking.com`

### ITEM 895

- Source context: L335 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Errores habituales al reservar
  ```
- Protected tokens: None identified in this item.

### ITEM 896

- Source context: L339 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Reservar solo por el número de asientos
  ```
- Protected tokens: None identified in this item.

### ITEM 897

- Source context: L340 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Un vehículo con suficientes asientos puede seguir siendo demasiado pequeño cuando se añaden varias maletas grandes. La capacidad de pasajeros y la capacidad de equipaje tienen que funcionar al mismo tiempo.
  ```
- Protected tokens: None identified in this item.

### ITEM 898

- Source context: L343 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Omitir el número de vuelo
  ```
- Protected tokens: None identified in this item.

### ITEM 899

- Source context: L344 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El número de vuelo es lo que permite al servicio de recogida seguir los retrasos e identificar la terminal correcta.
  ```
- Protected tokens: None identified in this item.

### ITEM 900

- Source context: L347 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Introducir la terminal equivocada
  ```
- Protected tokens: None identified in this item.

### ITEM 901

- Source context: L348 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Incheon Terminal 1 y Terminal 2 tienen zonas de llegada diferentes. La terminal de la reserva debe coincidir con el vuelo de llegada.
  ```
- Protected tokens: `Incheon`, `Terminal 1`, `Terminal 2`

### ITEM 902

- Source context: L351 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Tratar la medianoche como si perteneciera al día anterior
  ```
- Protected tokens: None identified in this item.

### ITEM 903

- Source context: L352 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Una recogida justo después de medianoche pertenece a la nueva fecha del calendario. Es un error fácil cuando el vuelo sale un día y aterriza después de medianoche al día siguiente.
  ```
- Protected tokens: None identified in this item.

### ITEM 904

- Source context: L355 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Usar solo el nombre del hotel
  ```
- Protected tokens: None identified in this item.

### ITEM 905

- Source context: L356 - `p`
- Element/type: Body text
- Spanish:

  ```text
  Una dirección completa del hotel o alojamiento es más segura que depender de un nombre traducido de la propiedad, especialmente en hoteles pequeños y apartamentos.
  ```
- Protected tokens: None identified in this item.

### ITEM 906

- Source context: L359 - `h3`
- Element/type: H3
- Spanish:

  ```text
  Suponer que todas las reservas tienen espera ilimitada
  ```
- Protected tokens: None identified in this item.

### ITEM 907

- Source context: L360 - `p`
- Element/type: Body text
- Spanish:

  ```text
  El seguimiento del vuelo y la espera gratuita no son lo mismo. El periodo de espera incluido depende del producto realmente reservado.
  ```
- Protected tokens: None identified in this item.

### ITEM 908

- Source context: L369 - `h2`
- Element/type: H2
- Spanish:

  ```text
  Preguntas frecuentes
  ```
- Protected tokens: None identified in this item.

### ITEM 909

- Source context: L373 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Vale la pena un traslado privado para cuatro personas?
  ```
- Protected tokens: None identified in this item.

### ITEM 910

- Source context: L374 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  A veces. Cuatro tarifas separadas de transporte público pueden reducir la diferencia de precio, especialmente si al llegar a Seoul necesitas otro taxi por el equipaje o por el tramo final. La comparación útil es el coste total puerta a puerta, no solo la primera tarifa que ves.
  ```
- Protected tokens: `Seoul`

### ITEM 911

- Source context: L377 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puede un taxi regular llevar a cuatro personas y cuatro maletas grandes?
  ```
- Protected tokens: None identified in this item.

### ITEM 912

- Source context: L378 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No de forma fiable. La capacidad de pasajeros y el espacio del maletero son límites distintos. Cuando cada pasajero lleva una maleta grande, un taxi jumbo o un vehículo más grande reservado con antelación puede ser más práctico.
  ```
- Protected tokens: None identified in this item.

### ITEM 913

- Source context: L381 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Qué pasa si mi vuelo se retrasa?
  ```
- Protected tokens: None identified in this item.

### ITEM 914

- Source context: L382 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Muchos traslados reservados con antelación piden el número de vuelo y hacen seguimiento de los retrasos, pero el tiempo de espera gratuito varía según el proveedor y el producto. La regla que importa es la vigente en la reserva que hayas seleccionado.
  ```
- Protected tokens: None identified in this item.

### ITEM 915

- Source context: L385 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Dónde me encontraré con el conductor en Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 916

- Source context: L386 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  La organización exacta depende de la reserva. Entre los métodos habituales están un conductor esperando en la sala de llegadas con un cartel con tu nombre o un mensaje que te dirige a una salida o punto de encuentro concretos.
  ```
- Protected tokens: None identified in this item.

### ITEM 917

- Source context: L389 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Importa si llego a Terminal 1 o Terminal 2?
  ```
- Protected tokens: `Terminal 1`, `Terminal 2`

### ITEM 918

- Source context: L390 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Sí. Las terminales tienen zonas de llegada y puntos de recogida separados. Al hacer la reserva, utiliza la terminal indicada para tu vuelo de llegada.
  ```
- Protected tokens: None identified in this item.

### ITEM 919

- Source context: L393 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Un traslado privado es lo mismo que un call van en Korea?
  ```
- Protected tokens: `Korea`

### ITEM 920

- Source context: L394 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No. “Traslado privado” es un término internacional de reserva para un vehículo reservado para un solo grupo. “Call van” es un término coreano para un servicio comercial de furgonetas. Algunos vehículos de traslado desde el aeropuerto pueden pertenecer a esa categoría, pero los términos no son intercambiables.
  ```
- Protected tokens: None identified in this item.

### ITEM 921

- Source context: L397 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Necesito reservar con antelación el transporte desde Incheon Airport?
  ```
- Protected tokens: `Incheon Airport`

### ITEM 922

- Source context: L398 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  No en todos los viajes. Los viajeros solos y las parejas con equipaje manejable suelen tener opciones sencillas de tren, autobús o taxi. Reservar con antelación resulta más útil cuando el tamaño del vehículo, el espacio para equipaje, una llegada tardía o un grupo grande hacen más importante encontrar el vehículo adecuado al llegar.
  ```
- Protected tokens: None identified in this item.

### ITEM 923

- Source context: L401 - `summary`
- Element/type: FAQ question
- Spanish:

  ```text
  ¿Puedo reservar un traslado privado para una llegada nocturna?
  ```
- Protected tokens: None identified in this item.

### ITEM 924

- Source context: L402 - `p`
- Element/type: FAQ answer
- Spanish:

  ```text
  Muchos productos de traslado de Incheon Airport operan hasta tarde o las 24 horas, pero la disponibilidad y los posibles recargos nocturnos dependen del proveedor y del vehículo. La página de reserva para la fecha elegida debería mostrar las condiciones vigentes.
  ```
- Protected tokens: `Incheon Airport`

### ITEM 925

- Source context: L411 - `h2`
- Element/type: H2
- Spanish:

  ```text
  ¿Deberías reservar un traslado privado desde el aeropuerto?
  ```
- Protected tokens: None identified in this item.

### ITEM 926

- Source context: L413 - `p:nth-of-type(1)`
- Element/type: Body text
- Spanish:

  ```text
  Si viajas solo o en pareja con equipaje manejable y tu llegada todavía encaja con el horario del tren o del autobús del aeropuerto, normalmente cuesta justificar pagar por un vehículo privado.
  ```
- Protected tokens: None identified in this item.

### ITEM 927

- Source context: L414 - `p:nth-of-type(2)`
- Element/type: Body text
- Spanish:

  ```text
  La opción gana fuerza con una familia o grupo, varias maletas grandes, un cochecito, un viajero mayor, una hora de salida real del aeropuerto tardía o un hotel que deja una caminata incómoda desde la estación.
  ```
- Protected tokens: None identified in this item.

### ITEM 928

- Source context: L415 - `p:nth-of-type(3)`
- Element/type: Body text
- Spanish:

  ```text
  Cuando un traslado privado ya tiene sentido para el viaje, mira primero el vehículo y el límite de equipaje, y después las condiciones de espera y cancelación.
  ```
- Protected tokens: None identified in this item.

### ITEM 929

- Source context: L416 - `p:nth-of-type(4)`
- Element/type: Body text
- Spanish:

  ```text
  El logotipo de la OTA viene después de esos detalles.
  ```
- Protected tokens: None identified in this item.

---

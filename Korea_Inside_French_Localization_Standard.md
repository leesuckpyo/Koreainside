# Korea Inside — French Localization Standard

## Document metadata

- Status: ACTIVE / SPECIALIZED STANDARD
- Version: 1.0
- Language: French
- lang / hreflang: `fr`
- Future Production folder: `/fr/`
- Public language menu label: Français
- Effective date: 2026-09-28
- Higher authority: current direct user instruction, AGENTS.md, public content and navigation architecture standards.
- Purpose: French editorial and technical localization rules. Dated progress belongs in the French inventory and QA report.

## Authorized first-pass draft exception

The user explicitly authorizes Codex to read the current root English Production HTML and directly write a French FIRST-PASS LOCALIZATION DRAFT. This task-specific authorization takes precedence over the general localization standard's default prohibition on Codex drafting and its intermediate approval workflow.

Codex Draft ≠ Approved Public Copy ≠ CONTENT LOCKED ≠ Production Ready.

Final French public wording requires ChatGPT editorial review and user approval. A draft branch, commit, push, or completed drafting pass is not publication approval. Inventory stays at 0 COMPLETE / 58 MISSING / 3 EXCLUDE until the separate public approval, implementation, Production release, and QA process is complete.

## French editorial rules

Use natural standard French and address readers as vous. Read complete English paragraphs and sections before drafting. Preserve the traveler's decision, conditions and compromises while writing complete French sentences. Avoid English syntax, mixed English/French prose, mechanical substitutions, unnecessary nominalization, excessive promotional language and English-style title capitalization.

Preserve facts, numbers, dates, times, prices, room sizes, beds, occupancy, hotel and transport conditions, positive and negative judgments, recommendation strength and all qualifications. Retain the practical implications of luggage, station exits, transfers, the final walk, noise, check-in and booking conditions. Do not invent facts or imply a firsthand visit or stay.

Use Séoul, Corée du Sud, aéroport d’Incheon, hôtel, hébergement, chambre, métro, sortie, correspondance, taxi, bagages, consigne à bagages, ascenseur, paiement, espèces, carte de crédit and carte de débit where appropriate. Translate transfer according to context: correspondance for a connection and transfert for an airport service.

Preserve official names and brands, including Myeongdong, Hongdae, Seongsu, Insadong, Gangnam, Jamsil, Itaewon, Dongdaemun, Gongdeok, Mapo, AREX, KTX, T-money, WOWPASS, NAVER Map, KakaoMap, Korea Inside, Klook, Agoda, Expedia, Trip.com and Ubigi.

## Source and coverage

Root English HTML is the factual, structural and judgment source. Cloud French drafts and Japanese/Taiwan translations are not French copy sources.

Inventory every user-facing text node and language-dependent attribute, including metadata, headings, body, buttons, link labels, breadcrumbs, FAQs, alt text, captions, ARIA, visible data-labels, JSON-LD and inline-JavaScript UI strings. Preserve technical tokens and genuine proper names; distinguish them from untranslated English prose.

Read the resulting French again for grammar, natural phrasing and meaning. Regex residue checks support, but do not replace, editorial review.

## Technical boundaries

Only authorized French HTML and the named French Markdown records may change. Preserve element structure, section order, class, id, non-language data attributes, affiliate URLs, tracking, images, srcset candidates, JavaScript logic and schema structure.

Set lang=fr and the French self canonical. The home canonical is https://www.getkoreainside.com/fr/; other canonicals use /fr/FILENAME. Adapt relative asset paths for the nested directory. Route links between existing target siblings to /fr/. Keep external and affiliate targets unchanged; keep exclusions at their English targets.

Do not change root English, es/, ja/, zh-tw/, shared CSS/JS, sitemap, common navigation/footer wording or structure, or reciprocal hreflang in existing languages. The explicit common-UI protection is handled as a documented REVIEW_REQUIRED exception where it prevents complete French coverage. Required nested asset and target-link corrections do not authorize redesign or shared-file edits.

Do not edit images. Record English infographic candidates and locations for a later approved image-localization task.

## QA and unattended continuation

Verify source coverage, residue, structural invariants, numerical facts, recommendation meaning, affiliate/tracking equality, local assets and links, FAQ/schema consistency, Markdown residue and fabricated experience.

Record recoverable failures, safely retry once where useful, mark unresolved items REVIEW_REQUIRED and continue with the next page/batch. Commit and push safely completed batch files only after QA. Defer browser QA if unavailable; do not stop the whole run for browser or editorial-review limitations.

Stop the entire run only when safe continuation would risk the repository or existing work under the user's hard-stop conditions. Do not merge or push main, publish Production, change credentials, or declare draft copy approved.

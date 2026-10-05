// Deterministic SVG text-layer preparation; no automatic exports or HTML edits.
// Read-only: node images/Accommodation/infographic-localization-template-2026-10-02.mjs --check
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

export const templatePath = fileURLToPath(new URL('./infographic-localization-template-2026-10-02.json', import.meta.url));
const sha = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const escapeXml = s => s.replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&apos;' }[c]));

export function inspectTemplate(template, root) {
  const ids = new Set(), errors = [];
  for (const family of template.families) {
    if (sha(fs.readFileSync(path.join(root, family.source))) !== family.sha256) errors.push(family.id + ': reference drift');
  }
  for (const target of template.targets) {
    if (ids.has(target.id)) errors.push(target.id + ': duplicate ID');
    ids.add(target.id);
    const family = template.families.find(f => f.id === target.family);
    const [x, y, w, h] = target.box;
    if (!family || !target.english || !(w > 0 && h > 0) || x < 0 || y < 0 || x + w > family.width || y + h > family.height) errors.push(target.id + ': invalid geometry/source');
    const binding = target.thai;
    if (binding.status === 'APPROVED EXACT COPY') {
      if (!template.policy.sourceOfTruth.includes(binding.source)) errors.push(target.id + ': unapproved source');
      const lines = fs.readFileSync(path.join(root, binding.source), 'utf8').split(/\r?\n/);
      const expected = '- ' + String.fromCharCode(96) + binding.copyId + String.fromCharCode(96) + ' → ' + binding.value;
      if (lines[binding.line - 1] !== expected || !lines.some(s => s.includes('Status:') && s.includes('APPROVED PUBLIC COPY — CONTENT LOCKED'))) errors.push(target.id + ': approved Thai value/status drift');
    }
    if (binding.value === null && binding.status !== 'NEEDS THAI INFOGRAPHIC COPY') errors.push(target.id + ': invalid missing-copy state');
  }
  if (template.scope !== 'THAI ONLY') errors.push('Invalid scope');
  const missing = template.targets.filter(t => t.thai.value === null).map(t => t.id);
  return {
    errors,
    targetCounts: Object.fromEntries(template.families.map(f => [f.id, template.targets.filter(t => t.family === f.id).length])),
    missing,
    productionReady: false
  };
}

// Returns a DRAFT SVG REVIEW CANDIDATE, never a production asset or QA PASS.
// Requires complete approved ledger values, explicit exact line breaks, verified
// local fonts and a visually verified text-free PNG preserving original artwork.
// No English fallback, masking, truncation, clipping, shrinking or file writes.
export function renderCandidateSvg(template, familyId, plate, layout, root) {
  const inspection = inspectTemplate(template, root);
  if (inspection.errors.length) throw Error(inspection.errors.join('\n'));
  const family = template.families.find(f => f.id === familyId);
  if (!family) throw Error('Unknown family');
  const targets = template.targets.filter(t => t.family === familyId);
  const missing = targets.filter(t => !t.thai.value).map(t => t.id);
  if (missing.length) throw Error('NEEDS THAI INFOGRAPHIC COPY: ' + missing.join(', '));
  if (!plate || plate.approval !== 'VISUALLY VERIFIED TEXT-FREE MASTER' || !plate.reviewEvidence) throw Error('Text-free master preserving source artwork/crop has not been verified.');
  const bytes = fs.readFileSync(path.resolve(root, plate.path));
  if (sha(bytes) !== plate.sha256 || !bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])) || bytes.readUInt32BE(16) !== family.width || bytes.readUInt32BE(20) !== family.height) throw Error('Text-free plate hash/type/dimensions mismatch.');
  if (plate.sha256 === family.sha256) throw Error('Flattened reference cannot serve as a clean plate.');
  const fonts = new Map();
  const nodes = targets.map(target => {
    const style = layout?.[target.id], value = target.thai.value;
    if (!style || !Array.isArray(style.lines) || style.lines.join('') !== value || !(style.fontSize > 0) || !style.fontPath || !style.fontSha256 || !(style.lineHeight > 0)) throw Error(target.id + ': exact line breaks and verified font settings required');
    const font = fs.readFileSync(path.resolve(root, style.fontPath));
    if (sha(font) !== style.fontSha256) throw Error(target.id + ': font hash mismatch');
    if (!/\.(ttf|otf)$/i.test(style.fontPath)) throw Error(target.id + ': explicit TTF/OTF font required');
    const fontName = 'font_' + style.fontSha256;
    fonts.set(fontName, { bytes: font, format: /\.otf$/i.test(style.fontPath) ? 'opentype' : 'truetype' });
    const [x, y, w, h] = target.box;
    const mode = style.writingMode ?? 'horizontal-tb';
    if (!['horizontal-tb', 'vertical-rl', 'vertical-lr'].includes(mode)) throw Error(target.id + ': invalid writing mode');
    const align = style.align ?? 'left';
    if (!['left', 'center', 'right'].includes(align)) throw Error(target.id + ': invalid alignment');
    const color = style.color ?? '#101410';
    if (!/^#[0-9a-f]{6}$/i.test(color)) throw Error(target.id + ': explicit hex color required');
    const weight = style.weight ?? 400;
    if (!Number.isInteger(weight) || weight < 100 || weight > 900) throw Error(target.id + ': invalid font weight');
    const rotation = style.rotation ?? 0;
    if (!Number.isFinite(rotation)) throw Error(target.id + ': invalid rotation');
    const css = 'margin:0;padding:0;white-space:pre;overflow:visible;font-family:' + fontName + ';font-size:' + style.fontSize + 'px;line-height:' + style.lineHeight + ';color:' + color + ';font-weight:' + weight + ';text-align:' + align + ';writing-mode:' + mode + ';';
    return '<foreignObject data-copy-id="' + target.id + '" x="' + x + '" y="' + y + '" width="' + w + '" height="' + h + '" transform="rotate(' + rotation + ' ' + x + ' ' + y + ')"><div xmlns="http://www.w3.org/1999/xhtml" lang="th" style="' + css + '">' + style.lines.map(escapeXml).join('<br/>') + '</div></foreignObject>';
  });
  const fontCss = [...fonts].map(([name, font]) => '@font-face{font-family:' + name + ';src:url(data:font/' + (font.format === 'opentype' ? 'otf' : 'ttf') + ';base64,' + font.bytes.toString('base64') + ') format("' + font.format + '");}').join('');
  return '<svg xmlns="http://www.w3.org/2000/svg" width="' + family.width + '" height="' + family.height + '" viewBox="0 0 ' + family.width + ' ' + family.height + '"><metadata>DRAFT REVIEW CANDIDATE; visual/crop/overflow/glyph QA pending</metadata><style>' + fontCss + '</style><image width="' + family.width + '" height="' + family.height + '" href="data:image/png;base64,' + bytes.toString('base64') + '"/>' + nodes.join('') + '</svg>';
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.length !== 3 || process.argv[2] !== '--check') throw Error('Read-only CLI supports --check only.');
  const template = JSON.parse(fs.readFileSync(templatePath, 'utf8'));
  const root = path.resolve(path.dirname(templatePath), '../..');
  const result = inspectTemplate(template, root);
  console.log(JSON.stringify(result, null, 2));
  if (result.errors.length) process.exitCode = 1;
}

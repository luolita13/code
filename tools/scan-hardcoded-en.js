const fs = require('fs')
const path = require('path')

// Scan app-frontend Vue files for hardcoded English text in templates
const dir = path.resolve('apps/app-frontend/src')
const localePath = path.resolve('apps/app-frontend/src/locales/zh-CN/index.json')

function walk(d) {
  let r = []
  if (!fs.existsSync(d)) return r
  fs.readdirSync(d).forEach(f => {
    const p = path.join(d, f)
    if (fs.statSync(p).isDirectory()) r = r.concat(walk(p))
    else if (f.endsWith('.vue')) r.push(p)
  })
  return r
}

const files = walk(dir)
const issues = []

// Patterns that suggest hardcoded English in template (inside {{ }}, :placeholder, title, etc.)
// Exclude common Vue bindings, CSS classes, and technical strings
const skipPatterns = [
  /^\s*$/, // empty
  /^[\d.]+$/, // numbers
  /^[a-z-]+$/, // css-like (e.g. class names)
  /^\$/, // vue variables
  /^{/, // object
  /^\[/, // array
  /^@/, // event handlers
  /^:/, // vue bindings
  /^v-/, // vue directives
  /^(px|rem|em|%|vh|vw|deg|ms|s)$/, // units
  /^(true|false|null|undefined)$/, // keywords
  /^(div|span|p|h[1-6]|a|img|button|input|select|option|textarea|label|form|table|tr|td|th|ul|ol|li|nav|header|footer|main|section|article|aside)$/, // HTML tags
]

// Check for hardcoded strings in template {{ }} expressions
for (const f of files) {
  const text = fs.readFileSync(f, 'utf8')
  const relPath = path.relative(dir, f)

  // Find {{ 'string' }} or {{ "string" }} patterns
  const templateMatches = text.matchAll(/\{\{\s*['"]([A-Z][a-zA-Z\s,.'!?:;()-]+)['"]\s*\}\}/g)
  for (const m of templateMatches) {
    const str = m[1]
    if (str.length > 2 && !skipPatterns.some(p => p.test(str))) {
      issues.push({ file: relPath, type: 'template', text: str.trim() })
    }
  }

  // Find :placeholder="'Hardcoded string'" patterns
  const placeholderMatches = text.matchAll(/(?:placeholder|title|label|header|name)=['"]([A-Z][a-zA-Z\s,.'!?:;()-]+)['"]/g)
  for (const m of placeholderMatches) {
    const str = m[1]
    if (str.length > 2 && !skipPatterns.some(p => p.test(str))) {
      issues.push({ file: relPath, type: 'attribute', text: str.trim() })
    }
  }

  // Find >Hardcoded English text< in template (between tags)
  const tagContentMatches = text.matchAll(/>([A-Z][a-zA-Z\s,.'!?:;()-]{3,})<\//g)
  for (const m of tagContentMatches) {
    const str = m[1]
    // Skip if it looks like a component name or CSS class
    if (str.includes('__') || /^[A-Z][a-z]+[A-Z]/.test(str) || /^[a-z-]+$/.test(str)) continue
    if (!skipPatterns.some(p => p.test(str.trim()))) {
      issues.push({ file: relPath, type: 'tag-content', text: str.trim() })
    }
  }
}

console.log('Potential hardcoded English strings found:', issues.length)
const grouped = {}
for (const i of issues) {
  if (!grouped[i.file]) grouped[i.file] = []
  grouped[i.file].push(i)
}
for (const [file, items] of Object.entries(grouped)) {
  console.log('\n---', file, '---')
  for (const i of items) {
    console.log(`  [${i.type}] ${i.text}`)
  }
}

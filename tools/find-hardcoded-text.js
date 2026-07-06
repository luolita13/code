const fs = require('fs')
const path = require('path')

const dir = path.resolve('apps/app-frontend/src')

function walk(d) {
  let r = []
  for (const e of fs.readdirSync(d, { withFileTypes: true })) {
    const full = path.join(d, e.name)
    if (e.isDirectory()) r = r.concat(walk(full))
    else if (full.endsWith('.vue')) r.push(full)
  }
  return r
}

const files = walk(dir)
const out = []
const englishRe = /[A-Za-z]{2,}(?:\s+[A-Za-z]{2,})+/g
const excludeRe = /^(v-|@|:|#|\.)/

for (const f of files) {
  const text = fs.readFileSync(f, 'utf8')
  const templateMatch = text.match(/<template[\s\S]*?<\/template>/i)
  if (!templateMatch) continue
  const template = templateMatch[0]
    .replace(/<!--[\s\S]*?-->/g, '')
    .replace(/\{\{[\s\S]*?\}\}/g, ' ')
    .replace(/<script[\s\S]*?<\/script>/g, ' ')
    .replace(/<style[\s\S]*?<\/style>/g, ' ')

  // strip html tags
  const plain = template.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ')
  const matches = [...plain.matchAll(englishRe)]
  for (const m of matches) {
    const phrase = m[0].trim()
    if (phrase.length < 4) continue
    // skip class names with no spaces? we already require 2+ words
    out.push(`${path.relative(dir, f)}: ${phrase}`)
  }
}

fs.writeFileSync('tools/hardcoded-text.txt', out.join('\n'), 'utf8')
console.log('Found', out.length, 'candidates')

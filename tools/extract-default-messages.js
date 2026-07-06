const fs = require('fs')
const path = require('path')

const dir = path.resolve('apps/app-frontend/src')

function walk(d) {
  let r = []
  for (const e of fs.readdirSync(d, { withFileTypes: true })) {
    const full = path.join(d, e.name)
    if (e.isDirectory()) {
      r = r.concat(walk(full))
    } else if (/\.(vue|ts|tsx|js)$/.test(e.name)) {
      r.push(full)
    }
  }
  return r
}

const files = walk(dir)
const re = /id:\s*['"](app\.[^'"]+)['"]\s*[\s\S]*?defaultMessage:\s*(?:(['"])((?:[^\\2]|\\.)*?)\2)/g
const out = []
for (const f of files) {
  const text = fs.readFileSync(f, 'utf8')
  let m
  while ((m = re.exec(text)) !== null) {
    const id = m[1]
    const quote = m[2]
    let msg = m[3].replace(/\\n/g, '\n').replace(/\\"/g, '"').replace(/\\'/g, "'")
    out.push(`${id}\t${msg}\t${path.relative(dir, f)}`)
  }
}
fs.writeFileSync('tools/default-messages.tsv', out.join('\n'), 'utf8')
console.log('Extracted:', out.length)

const fs = require('fs')
const path = require('path')

const dir = path.resolve('apps/app-frontend/src')
const localePath = path.resolve('apps/app-frontend/src/locales/zh-CN/index.json')
const translations = Object.keys(JSON.parse(fs.readFileSync(localePath, 'utf8')))

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

const files = walk(dir).sort()
const re = /id:\s*['"](app\.[^'"]+)['"]/g
const ids = []
const idMap = {}
for (const f of files) {
  const text = fs.readFileSync(f, 'utf8')
  let m
  while ((m = re.exec(text)) !== null) {
    ids.push(m[1])
    idMap[m[1]] = idMap[m[1]] || []
    idMap[m[1]].push(f)
  }
}

const unique = [...new Set(ids)]
const missing = unique.filter((id) => !translations.includes(id))
const out = []
out.push('Files: ' + files.length)
out.push('Total ids: ' + ids.length + ' Unique: ' + unique.length + ' Missing: ' + missing.length)
for (const id of missing.sort()) {
  out.push(id + ' -> ' + idMap[id].slice(0, 3).map((x) => path.relative(dir, x)).join(', '))
}
fs.writeFileSync('tools/missing-i18n.txt', out.join('\n'), 'utf8')
console.log(out.join('\n'))

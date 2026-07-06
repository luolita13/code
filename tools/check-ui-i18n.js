const fs = require('fs')
const path = require('path')

function walk(d) {
  let r = []
  if (!fs.existsSync(d)) return r
  fs.readdirSync(d).forEach(f => {
    const p = path.join(d, f)
    if (fs.statSync(p).isDirectory()) r = r.concat(walk(p))
    else if (f.endsWith('.vue') || f.endsWith('.ts')) r.push(p)
  })
  return r
}

const dir = path.resolve('packages/ui/src')
const files = walk(dir)
const re = /id:\s*['"]([^'"]+)['"]/g
const ids = new Set()
for (const f of files) {
  const t = fs.readFileSync(f, 'utf8')
  let m
  while ((m = re.exec(t)) !== null) ids.add(m[1])
}
console.log('UI component ids:', ids.size)

const en = JSON.parse(fs.readFileSync('packages/ui/src/locales/en-US/index.json', 'utf8'))
const zh = JSON.parse(fs.readFileSync('packages/ui/src/locales/zh-CN/index.json', 'utf8'))

const needed = [...ids].filter(k => en[k])
const missing = needed.filter(k => !zh[k] || zh[k].message === en[k].message)
console.log('Needed by components:', needed.length, 'Missing translation:', missing.length)
missing.forEach(k => console.log(k + '|' + (en[k].message || '')))

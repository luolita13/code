const fs = require('fs')

const localePath = 'apps/app-frontend/src/locales/zh-CN/index.json'
const missingPath = 'tools/missing-translations.json'

const existing = JSON.parse(fs.readFileSync(localePath, 'utf8'))
const missing = JSON.parse(fs.readFileSync(missingPath, 'utf8'))

let added = 0
for (const [key, value] of Object.entries(missing)) {
  if (!(key in existing)) {
    existing[key] = value
    added++
  }
}

fs.writeFileSync(localePath, JSON.stringify(existing, null, 2) + '\n', 'utf8')
console.log(`Added ${added} new translation keys.`)

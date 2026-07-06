const fs = require('fs')
const path = require('path')

const en = JSON.parse(fs.readFileSync('packages/ui/src/locales/en-US/index.json', 'utf8'))
const zh = JSON.parse(fs.readFileSync('packages/ui/src/locales/zh-CN/index.json', 'utf8'))

// Keys needed by the App (excluding website-only keys like billing, affiliate, servers, form, payment)
const skipPrefixes = [
  'billing.', 'affiliate.', 'payment-method.', 'form.', 'servers.',
  'project.server.language.', 'project.server.region.',
  'servers.access-page.', 'servers.setup.', 'servers.manage.',
  'servers.notice.', 'servers.region.', 'search.server_content_type.',
  'search.filter_type.server_', 'project.settings.', 'project.about.',
  'project.versions.', 'report.', 'files.', 'external-project-license-status.',
  'settings.account.', 'settings.appearance.', 'settings.applications.',
  'settings.authorized-apps.', 'settings.billing.', 'settings.feature-flags.',
  'settings.pats.', 'settings.profile.', 'settings.sessions.',
  'project.download-count-tooltip', 'project.follower-count-tooltip',
  'project.online-player-count', 'project.recent-plays',
  'header.category.',
  'collections.label.private',
  'settings.language.platform.site',
  'label.rewards-program-terms-agreement',
]

const needed = Object.keys(en).filter(k => {
  if (skipPrefixes.some(p => k.startsWith(p))) return false
  // Only include keys that App components reference
  return true
})

const missing = needed.filter(k => !zh[k] || zh[k].message === en[k].message || zh[k].message === undefined)
console.log('App-relevant missing translations:', missing.length)

const output = {}
missing.forEach(k => {
  const msg = en[k]?.message || ''
  output[k] = { message: msg }
})

fs.writeFileSync('tools/ui-missing-translations.json', JSON.stringify(output, null, 2) + '\n', 'utf8')
console.log('Saved to tools/ui-missing-translations.json')

const fs = require('fs')
const path = require('path')

const localePath = path.resolve('apps/app-frontend/src/locales/zh-CN/index.json')
const existing = JSON.parse(fs.readFileSync(localePath, 'utf8'))

const newTranslations = {
  // InstanceItem.vue
  "app.instance-item.not-played-yet": { "message": "尚未游玩" },
  "app.instance-item.loading-modpack": { "message": "加载整合包..." },
  "app.instance-item.already-open": { "message": "实例已打开" },
  "app.instance-item.view-instance": { "message": "查看实例" },

  // Project Index.vue
  "app.project.more-options": { "message": "更多选项" },
  "app.project.open-in-browser": { "message": "在浏览器中打开" },
  "app.project.report": { "message": "举报" },
  "app.project.follow": { "message": "关注" },
  "app.project.unfollow": { "message": "取消关注" },
  "app.project.share": { "message": "分享" },

  // Project Version.vue
  "app.project.version.versions": { "message": "版本列表" },
  "app.project.version.installing": { "message": "安装中..." },
  "app.project.version.installed": { "message": "已安装" },
  "app.project.version.install": { "message": "安装" },
  "app.project.version.report": { "message": "举报" },
  "app.project.version.modrinth-website": { "message": "Modrinth 网站" },
  "app.project.version.changelog": { "message": "更新日志" },
  "app.project.version.files": { "message": "文件" },
  "app.project.version.primary": { "message": "主要" },
  "app.project.version.dependencies": { "message": "依赖" },
  "app.project.version.metadata": { "message": "元数据" },
  "app.project.version.release-channel": { "message": "发布通道" },
  "app.project.version.version-number": { "message": "版本号" },
  "app.project.version.loaders": { "message": "加载器" },
  "app.project.version.game-versions": { "message": "游戏版本" },
  "app.project.version.downloads": { "message": "下载量" },
  "app.project.version.publication-date": { "message": "发布日期" },
  "app.project.version.author": { "message": "作者" },
  "app.project.version.version-id": { "message": "版本 ID" },
  "app.project.version.dependency.subtitle": { "message": "版本 {versionNumber} 是 {dependencyType}" },
  "app.project.version.dependency.added-via-overrides": { "message": "通过覆盖添加" },

  // Project Versions.vue
  "app.project.versions.install": { "message": "安装" },
  "app.project.versions.swap-version": { "message": "切换版本" },
  "app.project.versions.more-options": { "message": "更多选项" },
  "app.project.versions.add-to-another-instance": { "message": "添加到其他实例" },
  "app.project.versions.open-in-browser": { "message": "在浏览器中打开" },

  // App.vue survey
  "app.survey.hey-there": { "message": "嗨，Modrinth 用户！" },
  "app.survey.description": { "message": "你介意回答几个关于 Modrinth App 使用体验的问题吗？" },
  "app.survey.feedback": { "message": "这些反馈将直接发送给 Modrinth 团队，帮助我们改进未来的更新！" },
  "app.survey.take": { "message": "参与调查" },
  "app.survey.no-thanks": { "message": "不用了" },

  // FriendsList.vue modal
  "friends.modal.view-friend-requests": { "message": "查看好友请求" },
  "friends.modal.no-pending-requests": { "message": "你目前没有待处理的好友请求 :C" },
  "friends.modal.sent-you-request": { "message": "{username} 向你发送了好友请求" },
  "friends.modal.you-sent-request": { "message": "你向 {username} 发送了好友请求" },
  "friends.modal.accept": { "message": "接受" },
  "friends.modal.ignore": { "message": "忽略" },
  "friends.modal.cancel": { "message": "取消" },
}

let added = 0
for (const [key, value] of Object.entries(newTranslations)) {
  if (!(key in existing)) {
    existing[key] = value
    added++
  }
}

// Sort keys alphabetically
const sorted = {}
Object.keys(existing).sort().forEach(k => { sorted[k] = existing[k] })

fs.writeFileSync(localePath, JSON.stringify(sorted, null, 2) + '\n', 'utf8')
console.log(`Added ${added} new translation keys. Total: ${Object.keys(sorted).length}`)

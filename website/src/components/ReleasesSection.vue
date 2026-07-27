<script setup>
import { ref, onMounted } from 'vue'

const releases = ref([])
const loading = ref(true)
const error = ref(false)

function renderBody(text) {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/^### (.+)$/gm, '<h4>$1</h4>')
    .replace(/^## (.+)$/gm, '<h3 class="release-section">$1</h3>')
    .replace(/^- (.+)$/gm, '<li>$1</li>')
    .replace(/(<li>.*<\/li>\n?)+/gs, (match) => `<ul>${match}</ul>`)
    .replace(/\n/g, '<br/>')
}

function formatSize(bytes) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1048576).toFixed(1) + ' MB'
}

onMounted(async () => {
  try {
    const res = await fetch('https://api.github.com/repos/CleanLauncher/Launcher/releases?per_page=10')
    if (!res.ok) throw new Error('Failed to fetch')
    const data = await res.json()
    releases.value = data.map(r => ({
      tag: r.tag_name,
      name: r.name || r.tag_name,
      date: new Date(r.published_at).toLocaleDateString('en-US', {
        year: 'numeric', month: 'short', day: 'numeric'
      }),
      body: r.body || '',
      url: r.html_url,
      assets: r.assets.map(a => ({
        name: a.name,
        size: formatSize(a.size),
        url: a.browser_download_url
      }))
    }))
  } catch {
    error.value = true
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <section id="releases" class="releases">
    <div class="container">
      <h2>Releases</h2>

      <div v-if="loading" class="loading">
        Loading releases...
      </div>

      <div v-else-if="error" class="loading error">
        Could not load releases.
        <a href="https://github.com/CleanLauncher/Launcher/releases" target="_blank" rel="noopener">View on GitHub</a>
      </div>

      <div v-else class="releases-list">
        <div v-for="release in releases" :key="release.tag" class="release-card">
          <div class="release-header">
            <div>
              <h3>{{ release.name }}</h3>
              <span class="release-date">{{ release.date }}</span>
            </div>
            <a :href="release.url" target="_blank" rel="noopener" class="release-link">
              View on GitHub
            </a>
          </div>

          <div v-if="release.body" class="release-body">
            <div v-html="renderBody(release.body)" />
          </div>

          <div v-if="release.assets.length" class="release-assets">
            <div
              v-for="asset in release.assets"
              :key="asset.name"
              class="asset"
            >
              <a :href="asset.url" target="_blank" rel="noopener" class="asset-name">
                {{ asset.name }}
              </a>
              <span class="asset-size">{{ asset.size }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.releases {
  padding: 100px 0;
  background: var(--bg-secondary);
}

h2 {
  text-align: center;
  font-size: 2.2rem;
  font-weight: 700;
  letter-spacing: -1px;
  margin-bottom: 48px;
}

.loading {
  text-align: center;
  color: var(--text-secondary);
  padding: 40px 0;
  font-size: 0.95rem;
}

.loading.error a {
  display: block;
  margin-top: 8px;
}

.releases-list {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.release-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 28px;
}

.release-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
  gap: 16px;
}

.release-header h3 {
  font-size: 1.2rem;
  font-weight: 700;
  margin-bottom: 4px;
}

.release-date {
  font-size: 0.82rem;
  color: var(--text-secondary);
}

.release-link {
  white-space: nowrap;
  font-size: 0.85rem;
}

.release-body {
  margin-bottom: 16px;
  font-size: 0.9rem;
  color: var(--text-secondary);
  line-height: 1.7;
}

.release-body :deep(h3.release-section) {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin: 16px 0 8px;
}

.release-body :deep(ul) {
  margin: 4px 0;
  padding-left: 20px;
}

.release-body :deep(li) {
  margin-bottom: 2px;
}

.release-assets {
  border-top: 1px solid var(--border);
  padding-top: 16px;
  display: flex;
  flex-wrap: wrap;
  gap: 8px 20px;
}

.asset {
  display: flex;
  align-items: center;
  gap: 8px;
}

.asset-name {
  font-size: 0.82rem;
  color: var(--text-secondary);
}

.asset-name:hover {
  color: var(--text-primary);
}

.asset-size {
  font-size: 0.75rem;
  color: var(--text-secondary);
  opacity: 0.6;
}

@media (max-width: 600px) {
  .release-header {
    flex-direction: column;
  }
}
</style>

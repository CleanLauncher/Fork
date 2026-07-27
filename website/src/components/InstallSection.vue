<script setup>
import { ref } from 'vue'

const activeTab = ref('linux')

const tabs = {
  linux: {
    label: 'Linux',
    commands: [
      { label: 'Flatpak', code: 'flatpak install com.cleanlauncher.Launcher' },
      { label: 'Snap', code: 'sudo snap install cleanlauncher' },
      { label: 'AppImage', code: '# Download from GitHub Releases\nchmod +x Launcher-*.AppImage\n./Launcher-*.AppImage' },
      { label: 'AUR (Arch)', code: 'yay -S launcher' }
    ]
  },
  windows: {
    label: 'Windows',
    commands: [
      { label: 'Installer', code: '# Download Launcher-Setup.exe from GitHub Releases' },
      { label: 'Portable', code: '# Download Launcher-portable.zip from GitHub Releases\n# Extract and run launcher.exe' }
    ]
  },
  macos: {
    label: 'macOS',
    commands: [
      { label: 'DMG', code: '# Download Launcher-macOS-*.dmg from GitHub Releases\n# Drag Launcher to Applications' },
      { label: 'ZIP', code: '# Download Launcher-macOS-*.zip from GitHub Releases\n# Extract and move to Applications' }
    ]
  }
}
</script>

<template>
  <section id="install" class="install">
    <div class="container">
      <h2>Install</h2>
      <p class="install-subtitle">Available for all major platforms</p>

      <div class="tabs">
        <button
          v-for="(tab, key) in tabs"
          :key="key"
          :class="['tab', { active: activeTab === key }]"
          @click="activeTab = key"
        >
          {{ tab.label }}
        </button>
      </div>

      <div class="install-cards">
        <div v-for="(cmd, idx) in tabs[activeTab].commands" :key="idx" class="install-card">
          <div class="install-card-label">{{ cmd.label }}</div>
          <pre class="install-card-code"><code>{{ cmd.code }}</code></pre>
        </div>
      </div>

      <p class="install-note">
        Or visit the
        <a href="https://github.com/CleanLauncher/Launcher/releases/latest" target="_blank" rel="noopener">Releases page</a>
        to download directly.
      </p>
    </div>
  </section>
</template>

<style scoped>
.install {
  padding: 100px 0;
  background: var(--bg-primary);
}

h2 {
  text-align: center;
  font-size: 2.2rem;
  font-weight: 700;
  letter-spacing: -1px;
  margin-bottom: 8px;
}

.install-subtitle {
  text-align: center;
  color: var(--text-secondary);
  margin-bottom: 40px;
}

.tabs {
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-bottom: 32px;
}

.tab {
  padding: 8px 24px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 0.9rem;
  cursor: pointer;
  transition: all 0.2s;
}

.tab:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}

.tab.active {
  background: var(--accent);
  color: #fff;
  border-color: var(--accent);
}

.install-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 16px;
  max-width: 800px;
  margin: 0 auto 32px;
}

.install-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
}

.install-card-label {
  padding: 12px 20px 0;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--accent);
}

.install-card-code {
  padding: 12px 20px 16px;
  margin: 0;
  overflow-x: auto;
  font-size: 0.82rem;
  line-height: 1.6;
  color: var(--text-secondary);
  background: transparent;
}

.install-note {
  text-align: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

@media (max-width: 600px) {
  .tabs {
    flex-wrap: wrap;
  }
}
</style>

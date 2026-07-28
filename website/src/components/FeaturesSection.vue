<script setup>
const features = [
  {
    title: 'Cross-Platform',
    description: 'Runs on Linux, Windows, and macOS. Build once, play everywhere.',
    icon: 'platform'
  },
  {
    title: 'Multiple Instances',
    description: 'Manage separate Minecraft instances with different versions, modloaders, and settings.',
    icon: 'instances'
  },
  {
    title: 'Mod Support',
    description: 'Install and manage mods from CurseForge and Modrinth with automatic dependency resolution.',
    icon: 'mods'
  },
  {
    title: 'Rust Core',
    description: 'High-performance Rust core library for HTTP, settings, and mod metadata handling.',
    icon: 'rust'
  },
  {
    title: 'Account Management',
    description: 'Support for Microsoft, offline, and other authentication methods.',
    icon: 'account'
  },
  {
    title: 'Open Source',
    description: 'Fully open source under the Apache 2.0 license. Community driven development.',
    icon: 'oss'
  }
]
</script>

<template>
  <section id="features" class="features">
    <div class="container">
      <h2>Features</h2>
      <div class="features-grid">
        <div v-for="feature in features" :key="feature.title" class="feature-card">
          <div class="feature-icon">
            <svg v-if="feature.icon === 'platform'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="3" width="20" height="14" rx="2"/>
              <line x1="8" y1="21" x2="16" y2="21"/>
              <line x1="12" y1="17" x2="12" y2="21"/>
            </svg>
            <svg v-else-if="feature.icon === 'instances'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="2" y="2" width="8" height="8" rx="1"/>
              <rect x="14" y="2" width="8" height="8" rx="1"/>
              <rect x="2" y="14" width="8" height="8" rx="1"/>
              <rect x="14" y="14" width="8" height="8" rx="1"/>
            </svg>
            <svg v-else-if="feature.icon === 'mods'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/>
              <path d="M2 17l10 5 10-5"/>
              <path d="M2 12l10 5 10-5"/>
            </svg>
            <svg v-else-if="feature.icon === 'rust'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="10"/>
              <path d="M8 12c0-2.2 1.8-4 4-4s4 1.8 4 4-1.8 4-4 4"/>
            </svg>
            <svg v-else-if="feature.icon === 'account'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
            <svg v-else-if="feature.icon === 'oss'" viewBox="0 0 24 24" width="28" height="28" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"/>
              <path d="M8 14s1.5 2 4 2 4-2 4-2"/>
              <line x1="9" y1="9" x2="9.01" y2="9"/>
              <line x1="15" y1="9" x2="15.01" y2="9"/>
            </svg>
          </div>
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.description }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.features {
  padding: 100px 0;
  background: var(--bg-secondary);
}

h2 {
  text-align: center;
  font-size: 2.2rem;
  font-weight: 700;
  margin-bottom: 56px;
  letter-spacing: -1px;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.feature-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 32px;
  transition: border-color 0.25s, transform 0.25s;
}

.feature-card:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.feature-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: rgba(233, 69, 96, 0.1);
  color: var(--accent);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
}

h3 {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 8px;
}

p {
  color: var(--text-secondary);
  font-size: 0.92rem;
  line-height: 1.6;
}

@media (max-width: 768px) {
  .features-grid {
    grid-template-columns: 1fr;
  }
}

@media (min-width: 769px) and (max-width: 1024px) {
  .features-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>

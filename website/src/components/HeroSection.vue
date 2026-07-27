<script setup>
import { ref, onMounted } from 'vue'

const version = ref('')

onMounted(async () => {
  try {
    const res = await fetch('https://api.github.com/repos/CleanLauncher/Launcher/releases/latest')
    if (res.ok) {
      const data = await res.json()
      version.value = data.tag_name
    }
  } catch {
    version.value = 'v111.0.4'
  }
})
</script>

<template>
  <section class="hero">
    <div class="container hero-inner">
      <div class="hero-badge" v-if="version">{{ version }}</div>
      <h1>Launcher</h1>
      <p class="hero-subtitle">
        A free, open-source, cross-platform Minecraft launcher.<br/>
        Built with Qt and Rust for speed and reliability.
      </p>
      <div class="hero-actions">
        <a href="#install" class="btn btn-primary">Download</a>
        <a href="https://github.com/CleanLauncher/Launcher" target="_blank" rel="noopener" class="btn btn-secondary">
          Source Code
        </a>
      </div>
      <div class="hero-platforms">
        <span>Linux</span>
        <span class="sep">/</span>
        <span>Windows</span>
        <span class="sep">/</span>
        <span>macOS</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.hero {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 120px 0 80px;
  background:
    radial-gradient(ellipse at 50% 0%, rgba(233, 69, 96, 0.08) 0%, transparent 60%),
    var(--bg-primary);
}

.hero-inner {
  max-width: 700px;
}

.hero-badge {
  display: inline-block;
  padding: 4px 14px;
  border: 1px solid var(--border);
  border-radius: 20px;
  font-size: 0.8rem;
  color: var(--accent);
  margin-bottom: 24px;
  font-family: monospace;
}

h1 {
  font-size: 4rem;
  font-weight: 800;
  letter-spacing: -2px;
  margin-bottom: 16px;
  background: linear-gradient(135deg, var(--text-primary) 0%, var(--accent) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-subtitle {
  font-size: 1.15rem;
  color: var(--text-secondary);
  margin-bottom: 36px;
  line-height: 1.7;
}

.hero-actions {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-bottom: 40px;
}

.btn {
  display: inline-flex;
  align-items: center;
  padding: 12px 28px;
  border-radius: 8px;
  font-size: 0.95rem;
  font-weight: 600;
  transition: transform 0.15s, box-shadow 0.2s;
}

.btn:hover {
  transform: translateY(-1px);
}

.btn-primary {
  background: var(--accent);
  color: #fff;
  box-shadow: 0 4px 20px rgba(233, 69, 96, 0.3);
}

.btn-primary:hover {
  color: #fff;
  box-shadow: 0 6px 28px rgba(233, 69, 96, 0.45);
}

.btn-secondary {
  border: 1px solid var(--border);
  color: var(--text-secondary);
}

.btn-secondary:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}

.hero-platforms {
  color: var(--text-secondary);
  font-size: 0.9rem;
  letter-spacing: 2px;
}

.sep {
  margin: 0 8px;
  opacity: 0.4;
}

@media (max-width: 600px) {
  h1 {
    font-size: 2.5rem;
  }
  .hero-subtitle {
    font-size: 1rem;
  }
  .hero-actions {
    flex-direction: column;
    align-items: center;
  }
}
</style>

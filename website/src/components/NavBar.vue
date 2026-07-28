<script setup>
import { ref } from 'vue'

const scrolled = ref(false)

if (typeof window !== 'undefined') {
  window.addEventListener('scroll', () => {
    scrolled.value = window.scrollY > 20
  })
}
</script>

<template>
  <nav :class="['navbar', { scrolled }]">
    <div class="container nav-inner">
      <a href="#" class="brand">
        <svg class="brand-icon" viewBox="0 0 64 64" width="32" height="32" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="64" height="64" rx="14" fill="#e94560"/>
          <path d="M20 44V20l24 12-24 12z" fill="#fff"/>
        </svg>
        <span>Launcher</span>
      </a>
      <div class="nav-links">
        <a href="#features">Features</a>
        <a href="#install">Install</a>
        <a href="#releases">Releases</a>
        <a href="https://github.com/CleanLauncher/Launcher" target="_blank" rel="noopener">GitHub</a>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  padding: 16px 0;
  transition: background 0.3s, box-shadow 0.3s, padding 0.3s;
}

.navbar.scrolled {
  background: rgba(15, 15, 26, 0.95);
  backdrop-filter: blur(12px);
  box-shadow: 0 1px 0 var(--border);
  padding: 10px 0;
}

.nav-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--text-primary);
  font-weight: 700;
  font-size: 1.15rem;
}

.brand-icon {
  flex-shrink: 0;
}

.nav-links {
  display: flex;
  gap: 28px;
}

.nav-links a {
  color: var(--text-secondary);
  font-size: 0.92rem;
  transition: color 0.2s;
}

.nav-links a:hover {
  color: var(--text-primary);
}

@media (max-width: 600px) {
  .nav-links {
    gap: 16px;
  }
  .nav-links a {
    font-size: 0.82rem;
  }
}
</style>

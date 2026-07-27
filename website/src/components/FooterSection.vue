<template>
  <footer class="footer">
    <div class="container footer-inner">
      <div class="footer-left">
        <span class="footer-brand">Launcher</span>
        <span class="footer-copy">
          Copyright (C) 2025-2026 Launcher Contributors.
          Licensed under Apache 2.0.
        </span>
      </div>
      <div class="footer-links">
        <a href="https://github.com/CleanLauncher/Launcher" target="_blank" rel="noopener">GitHub</a>
        <a href="https://github.com/CleanLauncher/Launcher/wiki" target="_blank" rel="noopener">Wiki</a>
        <a href="https://discord.gg/5kcBCvnbTp" target="_blank" rel="noopener">Discord</a>
        <a href="https://github.com/CleanLauncher/Launcher/issues" target="_blank" rel="noopener">Issues</a>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  padding: 32px 0;
  border-top: 1px solid var(--border);
  background: var(--bg-primary);
}

.footer-inner {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.footer-left {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.footer-brand {
  font-weight: 700;
  font-size: 0.95rem;
}

.footer-copy {
  font-size: 0.78rem;
  color: var(--text-secondary);
}

.footer-links {
  display: flex;
  gap: 24px;
}

.footer-links a {
  color: var(--text-secondary);
  font-size: 0.85rem;
  transition: color 0.2s;
}

.footer-links a:hover {
  color: var(--text-primary);
}

@media (max-width: 600px) {
  .footer-inner {
    flex-direction: column;
    text-align: center;
  }
  .footer-left {
    align-items: center;
  }
}
</style>

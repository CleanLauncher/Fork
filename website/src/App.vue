<script setup>
import NavBar from './components/NavBar.vue'
import HeroSection from './components/HeroSection.vue'
import FeaturesSection from './components/FeaturesSection.vue'
import ReleasesSection from './components/ReleasesSection.vue'
import InstallSection from './components/InstallSection.vue'
import FooterSection from './components/FooterSection.vue'
</script>

<template>
  <NavBar />
  <main>
    <HeroSection />
    <FeaturesSection />
    <InstallSection />
    <ReleasesSection />
  </main>
  <FooterSection />
</template>

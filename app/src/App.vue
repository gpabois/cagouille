<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
</script>

<template>
  <Navbar />
  <main class="container-fluid">
    <RouterView />
  </main>
</template>

<style scoped>
</style>

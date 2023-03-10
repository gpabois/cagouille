<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container-fluid">
      <a class="navbar-brand" href="#">Cagouille</a>
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <a class="nav-link active" aria-current="page" href="#">Home</a>
          </li>
          <li class="nav-item">
            <RouterLink :to="{name: 'aiots'}" class="nav-link">AIOTS</RouterLink>
          </li>
          
          <li class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
              Suivis
            </a>
            <ul class="dropdown-menu" aria-labelledby="navbarDropdown">
              <li class="dropdown-item">
                <RouterLink :to="{name: 'suivis_inspections'}">Inspections</RouterLink>
              </li>
              <li><a class="dropdown-item" href="#">Instructions</a></li>
              <li><a class="dropdown-item" href="#">Sanctions</a></li>
              <li><a class="dropdown-item" href="#">Plaintes</a></li>
            </ul>
          </li>
  
          <li class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
              Flux de travail
            </a>
            <ul class="dropdown-menu" aria-labelledby="navbarDropdown">
              <li class="dropdown-item">
                <RouterLink :to="{name: 'mes_taches'}">Mes tâches</RouterLink>
              </li>
              <li class="dropdown-item">
                <RouterLink :to="{name: 'rvats'}">RVAT</RouterLink>
              </li>
            </ul>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<style scoped>
</style>

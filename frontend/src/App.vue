<script setup lang="ts">
import { RouterView } from "vue-router";
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <div class="app">
    <Sidebar />
    <div class="content-background">
      <h2>{{ $route.name }}</h2>
      <div class="content-wrapper"><RouterView /></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "./styles/colors";
.app {
  display: flex;
  flex-wrap: nowrap;
  align-items: stretch;
}

.content-background {
  display: flex;
  flex-wrap: nowrap;
  flex-direction: column;
  box-sizing: border-box;
  padding: 1rem;
  flex-grow: 1;
  background-color: $accent;
  h2 {
    text-transform: capitalize;
    font-size: 1.5rem;
  }
}
.content-wrapper {
  margin-top: 0.5rem;
  box-sizing: border-box;
  border-radius: 1rem;
  width: 100%;
  height: 100%;
  background-color: white;
}
</style>

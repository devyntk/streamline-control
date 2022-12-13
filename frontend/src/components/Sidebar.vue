<script setup lang="ts">
import { RouterLink } from "vue-router";
import { ref } from "vue";

const open = ref(false);
</script>

<template>
  <nav class="sidebar" :class="{ open: open }">
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/about">About</RouterLink>
    <button @click="open = !open">{{ open ? "Close" : "Open" }}</button>
  </nav>
</template>

<style scoped lang="scss">
@import "../styles/colors";

.sidebar {
  width: 3rem;
  height: 100vh;
  background: #8c8c8c;
  transition: width 0.125s;

  &.open {
    width: 12rem;
  }
}
</style>

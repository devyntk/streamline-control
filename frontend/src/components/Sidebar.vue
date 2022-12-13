<script setup lang="ts">
import { RouterLink } from "vue-router";
import { ref } from "vue";

import { ArrowRightIcon, HomeIcon } from "@heroicons/vue/24/outline";

const open = ref(false);
</script>

<template>
  <nav class="sidebar" :class="{ open: open }">
    <div class="links">
      <h1 class="title">Streamline Control</h1>
      <RouterLink to="/">
        <HomeIcon />
        <div>Home</div>
      </RouterLink>
      <RouterLink to="/about"
        ><HomeIcon />
        <div>About</div></RouterLink
      >
    </div>
    <div class="actions">
      <div class="arrow" @click="open = !open">
        <ArrowRightIcon />
      </div>
    </div>
  </nav>
</template>

<style scoped lang="scss">
@import "../styles/colors";

$closed-width: 3rem;
$icon-margin: 0.75rem;

.sidebar {
  width: $closed-width;
  height: 100vh;
  background: $background-accent;
  transition: width 0.125s, margin 0.125s, height 0.125s;
  display: flex;
  flex-direction: column;
  color: white;

  .title {
    width: 0;
    height: 0;
    margin: 0;
    overflow: hidden;
    white-space: nowrap;
    font-size: 1.25rem;
  }
  a {
    height: 2.5rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    svg {
      box-sizing: border-box;
      padding: $icon-margin;
      width: $closed-width;
    }
    div {
      width: 0;
    }
  }

  .actions {
    margin-top: auto;
    display: flex;
    flex-direction: row-reverse;
    border-style: solid none;
    border-width: 0.05rem;
    .arrow {
      svg {
        transition: transform 0.125s;
        transform: rotate(0deg);
        padding: 0.5rem;
      }
      border-style: none solid;
      border-width: 0.05rem;
      display: flex;
      width: $closed-width;
      height: $closed-width;
      cursor: pointer;
    }
  }

  &.open {
    width: 12rem;
    .actions .arrow svg {
      transform: rotate(180deg);
    }
    .title {
      width: unset;
      margin: $icon-margin;
      height: unset;
    }
    a > div {
      width: unset;
    }
  }
}
</style>

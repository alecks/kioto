<template>
  <div>
    <Navigation />
    <router-view id="app"></router-view>
  </div>
</template>

<style lang="scss">
@import "~bootstrap/scss/bootstrap";
</style>

<script>
import Navigation from "@/components/Navigation.vue";

export default {
  name: "Home",
  components: {
    Navigation
  }
};
</script>

<template>
  <div>
    <Navigation />
    <router-view id="app"></router-view>
    <Footer />
  </div>
</template>

<style lang="scss">
@import "~bootstrap/scss/bootstrap";
</style>

<script>
import Navigation from "@/components/Navigation.vue";
import Footer from "@/components/Footer.vue";

export default {
  name: "Home",
  components: {
    Navigation,
    Footer
  }
};
</script>

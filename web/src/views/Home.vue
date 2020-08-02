<template>
  <div id="app">
    <main role="main">
      <section class="jumbotron text-center">
        <div class="container">
          <h1 class="jumbotron-heading">Kioto</h1>
          <p class="lead text-muted">
            Extensible calendars and schedules for Discord. That's it.
          </p>
          <a href="https://discord.com/x" class="btn btn-primary text-white">Invite Bot</a>
        </div>
      </section>

      <div class="py-5">
        <div class="container" id="features">
          <h1>Features</h1>
          <div class="row">
            <Card name="TODO" small="TODO" link="#">TODO</Card>
          </div>
        </div>
      </div>
    </main>

    <Footer v-bind:copyright="copyright" />
  </div>
</template>

<script>
import Footer from "@/components/Footer.vue";
import Card from "@/components/Card.vue";

export default {
  name: "Home",
  components: {
    Footer,
    Card
  }
};
</script>

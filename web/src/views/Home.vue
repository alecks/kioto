<template>
  <div id="app">
    <main role="main">
      <section class="jumbotron text-center">
        <div class="container">
          <h1 class="jumbotron-heading">Kioto</h1>
          <p class="lead text-muted">
            Extensible calendars and schedules for Discord. That's it.
          </p>
          <a href="https://discord.com/x" class="btn btn-primary text-white"
            >Invite Bot</a
          >
        </div>
      </section>

      <div class="py-5">
        <div class="container" id="features">
          <h1>Features</h1>
          <div class="row">
            <Card name="YAML" small="Interfaces/Workflows" link="#docs"
              >Use YAML to build extensive bot interfaces and workflows.
              Integrate restrictionless Go code.</Card
            >

            <Card name="Markdown" small="Interfaces/Documentation" link="#docs"
              >Create web documentation with Markdown and integrate it into
              Discord embeds.</Card
            >

            <Card name="Redis" small="Database" link="#docs"
              >Use Redis as an exceedingly fast key/value store out of the box.
              Store complex data types with JSON.</Card
            >
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

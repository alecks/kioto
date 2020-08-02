<template>
  <nav id="navigation" class="navbar navbar-expand-lg navbar-light bg-light">
    <a class="navbar-brand" href="#">
      Kioto
    </a>
    <button
      class="navbar-toggler"
      type="button"
      data-toggle="collapse"
      data-target="#navbar"
      aria-controls="navbar"
      aria-expanded="false"
      aria-label="Toggle navigation"
    >
      <span class="navbar-toggler-icon"></span>
    </button>

    <div class="collapse navbar-collapse" id="navbar">
      <ul class="navbar-nav ml-auto">
        <li class="nav-item">
          <a class="nav-link" href="https://docs.owo.gg">Documentation</a>
        </li>

        <li class="nav-item">
          <router-link class="nav-link btn btn-primary text-white" to="/login"
            >Login</router-link
          >
        </li>
      </ul>
    </div>
  </nav>
</template>

<script>
export default {
  name: "Navigation"
};
</script>
<style></style>

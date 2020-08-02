<template>
  <div class="footer">
    <footer class="bg-light text-muted">
      <div class="container">
        <div class="float-right">
          <p>
            <a href="#">Back to top</a>
          </p>
        </div>
        <p>
          Kioto is an
          <a href="https://github.com/fjah/kioto">open-source</a> project
          written in Rust.
        </p>
        <p>&copy; {{ new Date().getFullYear() }}; <b>BSD 3-clause</b>.</p>
      </div>
    </footer>
  </div>
</template>

<script>
export default {
  name: "Footer"
};
</script>

<style scoped lang="scss">
footer {
  padding-top: 3rem;
  padding-bottom: 3rem;

  p {
    margin-bottom: 0.25rem;
  }
}
</style>

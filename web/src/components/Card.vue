<template>
  <div id="card" class="col-md-4">
    <div class="box-shadow card mb-4 bg-light">
      <div class="card-body">
        <h5 class="card-title">
          <a :href="link">{{ name }}</a>
          <small class="text-muted float-right">{{ small }}</small>
        </h5>
        <p class="card-text">
          <slot></slot>
        </p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "Card",
  props: {
    name: String,
    small: String,
    link: String
  }
};
</script>

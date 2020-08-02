<template>
  <div id="card" class="col-md-4">
    <div class="box-shadow card mb-4 bg-light">
      <img class="card-img-top" :src="image" />
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
    image: String,
    link: String
  }
};
</script>

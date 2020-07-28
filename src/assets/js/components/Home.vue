<template>
    <div class="title">
        <h1>Seconds: {{ count }}</h1>
    </div>
</template>

<script>
  export default {
    name: "Home",
    data: () => ({
      count: 0
    }),
    beforeCreate() {
      console.log('Before Created')
    },
    created() {
      console.log('Created')
    },
    mounted() {
      this.onMount();
    },
    methods: {
      onMount() {
        console.log("Mounted");
        setInterval(() => {
          this.count++;
        }, 1000)
      }
    }
  }
</script>

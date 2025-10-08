<script setup lang="ts">
import {once} from "@tauri-apps/api/event";
import {useRouter} from "vue-router";
import {onMounted} from "vue";

const router = useRouter()

onMounted(async () => {
  await once('loaded', (event) => {
    router.push('Welcome');
  })
})

</script>

<template>

</template>

<style scoped lang="scss">

</style>

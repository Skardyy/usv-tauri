<script setup lang="ts">
    import block_comp from './components/block_comp.vue';
</script>

<template>
  <block_comp/>
  <div class="flex justify-center mt-28">
    <select name:zone_holder class="select select-s w-full mx-6 border rounded-md border-gray-500 pl-5 dark-select">
      <option value="LR">LR</option>
      <option value="AP">AP</option>
      <option value="SC">SC</option>
      <option value="LOO">LOO</option>
    </select>
  </div>
  <div class="mt-7 flex justify-evenly">
    <div class="flex">
      <input type="checkbox" class="checkbox" id="LR"/>
      <p>LR</p>
    </div>
    <div class="flex">
      <input type="checkbox" class="checkbox" id="AP"/>
      <p>AP</p>
    </div>
    <div class="flex">
      <input type="checkbox" class="checkbox" id="SC"/>
      <p>SC</p>
    </div>
    <div class="flex">
      <input type="checkbox" class="checkbox" id="LOO"/>
      <p>LOO</p>
    </div>   
  </div>
  <div class="w-screen">
    <button class="btn mx-6 mt-7">Apply</button>
  </div>
</template>

<style scoped>
.dark-select {
  @apply bg-transparent text-white
}
.dark-select option {
  @apply bg-foreground
}

.dark-select:focus {
  @apply border-pink-800 ring-transparent
}

.checkbox {
  @apply border-gray-300 mr-2 checkbox-md rounded-md transition-shadow duration-300
}

.checkbox:hover {
  @apply ring-8 ring-zinc-700 transition-shadow duration-300
}
</style>

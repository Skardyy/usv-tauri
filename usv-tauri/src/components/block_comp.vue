<script setup lang="ts">
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api';
    import { Block, BlockModule } from '../Utils/BlockModule';

    const block_mod = ref(new BlockModule([]))
    onMounted(async () => {
      const blocks = await invoke<Block[]>("get_blocks_mole")
      console.log(blocks)
      block_mod.value = new BlockModule(blocks)
    })

</script>

<template>
  <div class="flex flex-col justify-center text-center">
    <p class="text-white text-3xl mt-2" v-if="block_mod.current_block">{{ block_mod.current_block.zone_holder }}</p>
    <ul>
        <li v-if="block_mod.current_block" v-for="a in block_mod.current_block.actions">
        <div class="flex flex-row justify-between rounded-md my-3 bg-foreground mx-6">
            <p class="px-6 py-2">{{ a.word }}</p>
            <p class="px-6 py-2">{{ a.taunter }}</p>
        </div>
        </li>
    </ul>
    </div>
</template>

<style>
</style>

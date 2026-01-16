<script setup lang="ts">
import { onBeforeUnmount, onMounted } from 'vue';

defineOptions({ inheritAttrs: false });

defineProps<{
  isOpen: boolean,
  showContent?: boolean
}>()

const emit = defineEmits(['update:isOpen', 'update:showContent'])

const onKeyDown = (e: KeyboardEvent) => {
  if (e.key === "Escape") emit("update:isOpen", false)
}

onMounted(() => {
  window.addEventListener("keydown", onKeyDown)
})

onBeforeUnmount(() => {
  window.removeEventListener("keydown", onKeyDown)
})
</script>

<template>
  <slot name="trigger" />
  <transition 
    name="modal" 
    @after-enter='$emit("update:showContent", true)' 
    @before-leave='$emit("update:showContent", false)'
  >
    <div 
      @click.self='$emit("update:isOpen", false)' 
      v-if="isOpen" 
      v-bind="$attrs"
      :class="['fixed inset-0 flex items-center justify-center z-50 bg-gradient-to-bl from-primary/20 via-black/30 to-black/60 backdrop-blur-sm', $attrs.class]"
    >
      <transition name="content" appear>
        <div v-if="showContent">
          <slot name="content" />
        </div>
      </transition>
    </div>
  </transition>
</template>

<style scoped>
.modal-enter-active {
  transition: opacity 0.2s ease-out;
}

.modal-leave-active {
  transition: opacity 0.2s ease-in;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.content-enter-active {
  transition: all 0.25s ease-out;
}

.content-leave-active {
  transition: all 0.2s ease-in;
}

.content-enter-from {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}

.content-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(10px);
}
</style>
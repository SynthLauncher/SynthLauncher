<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from "vue"

const props = defineProps<{
  values: string[]
  selectedValue: string
}>()

const emit = defineEmits<{
  (e: "update:selectedValue", value: string): void;
}>();

const activeValue = ref<string>(props.selectedValue)

const items = ref<(HTMLElement | null)[]>([])
const indicator = ref<HTMLElement | null>(null)

const indicatorStyle = ref({})

function updateIndicator() {
  const activeIndex = props.values.indexOf(activeValue.value);
  if (activeIndex === -1) return

  const element = items.value[activeIndex]
  if (!element) return

  const { offsetLeft, offsetWidth } = element
  indicatorStyle.value = {
    left: `${offsetLeft}px`,
    width: `${offsetWidth}px`,
  }
}

onMounted(() => {
  nextTick(updateIndicator)
})

watch(activeValue, () => nextTick(updateIndicator))
</script>

<template>
  <div class="relative flex basis-0 w-fit gap-1 p-1 rounded-full bg-[#262729] select-none">
    <div 
      ref="indicator"
      class="absolute top-1 bottom-1 rounded-full bg-[#41a5e7]/20 shadow-sm transition-all duration-300 ease-in-out"
      :style="indicatorStyle" />

    <div 
      v-for="(value, i) in values" 
      :key="value" 
      @click="() => {
        activeValue = value
        emit('update:selectedValue', activeValue)
      }" 
      :ref="el => items[i] = el as HTMLElement"
      class="relative z-10 cursor-pointer py-1 px-4 rounded-full transition-all active:scale-95"
      :class="activeValue === value ? 'text-[#41a5e0]' : 'text-white'"
    >
      <h1 class="capitalize text-lg font-medium">
        {{ value }}
      </h1>
    </div>
  </div>
</template>

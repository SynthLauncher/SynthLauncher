<script setup lang="ts">
const emit = defineEmits(['update:modelValue'])

const toggle = () => {
    emit('update:modelValue', !props.modelValue)
}

const props = defineProps<{
    modelValue: boolean
}>()
</script>

<template>
    <button @click="toggle" :class="[
        'relative w-12 h-6 rounded-full border-2 transition-all duration-300',
        modelValue
            ? 'bg-secondary-accent border-secondary-accent'
            : 'bg-primary border-border'
    ]">
        <div :class="[
            'absolute top-0.5 w-4 h-4 bg-white rounded-full transition-transform duration-300 ease-in-out',
            modelValue ? 'translate-x-6' : 'translate-x-0.5'
        ]"></div>
    </button>
</template>
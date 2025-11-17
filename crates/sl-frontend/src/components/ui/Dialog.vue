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

    <transition name="modal" @after-enter='$emit("update:showContent", true)' @before-leave='$emit("update:showContent", false)'>
        <div @click.self='$emit("update:isOpen", false)' v-if="isOpen" v-bind="$attrs"
            :class="['fixed inset-0 flex items-center justify-center z-50', $attrs.class]">
            <transition name="content" appear>
                <slot name="content" />
            </transition>
        </div>
    </transition>
</template>

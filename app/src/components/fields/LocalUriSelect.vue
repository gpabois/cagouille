<script setup lang="ts">
import {ref, defineEmits, defineProps} from 'vue'

const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])

function change(e) {
    console.log(e.target.files[0])
}
</script>
<template>
    <input class="form-control" type="file" v-on:change="change">
</template>
<script setup lang="ts">
import AutocompleteQuery from '@/components/AutocompleteQuery.vue';
import { AUTOCOMPLETE } from '@/graphql/Communes.js';
import { defineEmits } from 'vue'

const emit = defineEmits(['update:modelValue'])
const elements = (data: any) => data.communes.edges;
</script>

<template>
    <AutocompleteQuery 
        @input="(e) => emit('update:modelValue', e)" 
        :query="AUTOCOMPLETE"
        :elements="elements" 
        :transform="(node) => ({'value': node.id, 'label': node.nom})"
    />
</template>

<style scoped>

</style>
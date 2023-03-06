<script setup lang="ts">
import AutocompleteQuery from './AutocompleteQuery.vue';
import { AUTOCOMPLETE } from '@/graphql/Aiots.js';
import { defineEmits, defineProps} from 'vue'

const emit = defineEmits(['update:modelValue'])
const props = defineProps(['modelValue'])
const elements = (data) => data.aiots.edges;
</script>

<template>
    <AutocompleteQuery 
        @input="(e) => emit('update:modelValue', e)" 
        :query="AUTOCOMPLETE"
        :modelValue="props.modelValue"
        :elements="elements" 
        :transform="(edge) => ({'value': edge.node.id, 'label': edge.node.libelle})"
    />
</template>
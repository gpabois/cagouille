<script setup lang="ts">
import AutocompleteQuery from './AutocompleteQuery.vue';
import { AUTOCOMPLETE as query } from '@/graphql/Groupes.js';
import { defineEmits, defineProps} from 'vue'

const emit = defineEmits(['update:modelValue'])
const props = defineProps(['modelValue'])
const elements = (data) => data.groups.edges;
</script>

<template>
    <AutocompleteQuery 
        @input="(e) => emit('update:modelValue', e)" 
        :query="query"
        :modelValue="props.modelValue"
        :elements="elements" 
        :transform="(edge) => ({'value': edge.node.id, 'label': edge.node.name})"
    />
</template>
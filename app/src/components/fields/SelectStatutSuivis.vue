<script setup lang="ts">
import SelectQuery from '@/components/fields/SelectQuery.vue';
import { TOUS } from '@/graphql/Suivis.js';
import { defineEmits, defineProps } from 'vue'

const props = defineProps(['value', 'required'])
const emit = defineEmits(['update:modelValue'])
</script>

<template>
    <SelectQuery 
        @update:modelValue="(e) => emit('update:modelValue', e)" 
        :query="TOUS"
        :modelValue="props.modelValue"
        :required="props.required"
        :elements="(data) => data.statusSuivis.edges" 
        :transform="(edge) => ({'value': edge.node.id, 'label': edge.node.nom})"
    />
</template>
<script setup lang="ts">
import SelectQuery from './SelectQuery.vue';
import { TOUT_TYPES } from '@/graphql/SuivisInspections.js';
import { defineEmits, defineProps } from 'vue'

const props = defineProps(['modelValue', 'required'])
const emit = defineEmits(['update:modelValue'])

const elements = (data) => data.aiots.edges;
const id = (edge) => edge.node.id;
const label = (edge) => edge.node.nom;
</script>

<template>
    <SelectQuery 
        @update:modelValue="(e) => emit('update:modelValue', e)" 
        :query="TOUT_TYPES"
        :value="props.modelValue"
        :required="props.required"
        :elements="(data) => data.typesInspections.edges" 
        :transform="(edge) => ({'value': edge.node.id, 'label': edge.node.nom})"
    />
</template>
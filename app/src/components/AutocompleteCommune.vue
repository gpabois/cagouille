<script setup lang="ts">
import AutocompleteQuery from './AutocompleteQuery.vue';
import { AUTOCOMPLETE_COMMUNE } from '../graphql/UtilsCommune.js';
import { defineEmits } from 'vue'

const emit = defineEmits(['input'])

const elements  = (data) => data.communes.edges;
const id        = (edge) => edge.node.id;
const label     = (edge) => edge.node.nom;

</script>

<template>
    <AutocompleteQuery 
        @input="(e) => emit('input', e)" 
        :query="AUTOCOMPLETE_COMMUNE"
        :elements="elements" 
        :id="id" 
        :label="label"
    />
</template>

<style scoped>

</style>
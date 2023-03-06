<script setup lang="ts">
import { ExportToCsv } from 'export-to-csv';
import {defineProps} from 'vue'
import { useApolloClient } from '@vue/apollo-composable'

const { resolveClient } = useApolloClient()
const client = resolveClient();

const {query, variables, elements, transform} = defineProps([
    'query', 
    'variables',
    'elements',
    'transform'
]);

function exportCsv(e) {
    client.writeQuery({query, variables});
}

</script>
<template>
    <button class="btn" @click="exportCsv">Export CSV</button>
</template>
<script setup lang="ts">
import Table from '../components/Table.vue';
import query from '../graphql/VueSuivisInspections.js';
import { AUTOCOMPLETE_AIOT } from '../graphql/UtilsAiot.js';
import { parseAndFormatDate, loadMoreMixin } from '../utils.js';
import {ref} from 'vue';
import { transform } from '@vue/compiler-core';

const loadMore = loadMoreMixin("suiviInspections");
const orderBy = ref('');
const filter = ref({});

const columns = [{
    id: 'nom',
    name: 'Nom',
    value: (row) => row.node.nom
}, {
    id: 'aiot',
    name: 'Aiot',
    value: (row) => row.node.aiot.nom,
    filter: {
        type: 'in',
        values: {
            type: 'query',
            query: AUTOCOMPLETE_AIOT,
            variables: (filter) => ({filter}),
            elements: (data) => { data.aiots.edges }
        },
        transform: (edge) => ({
            'id': edge.node.id,
            'label': edge.node.nom
        })
        
    }
}, {
    id: 'statut',
    name: 'Statut',
    value: (row) => row.node.statut ? row.node.statut.nom : "-"
}, {
    id: 'type',
    name: 'Type',
    value: (row) => row.node.type ? row.node.type.nom : "-"
}, {
    id: 'datePrevisionnelle',
    name: 'Date prévisionnelle',
    sortable: true,
    value: (row) => parseAndFormatDate(row.node.datePrevisionnelle)
}]

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.id}`).join(',');
}

function filterUpdated(filters) {

}
</script>

<template>
    <h1>Inspections</h1>
    <ApolloQuery :query="query" :variables="{orderBy, ...filter}">
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">               
                <Table 
                    :rows="data.suivisInspections.edges" 
                    :columns="columns" 
                    @sort="sortUpdated"
                    @filter="filterUpdated"
                >
                </Table>
                <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.suivisInspections.pageInfo.hasNextPage">Charger plus</button>
            </div>
        </template>
    </ApolloQuery>
</template>
<style scoped>
 revo-grid { height: 500px; } 
</style>
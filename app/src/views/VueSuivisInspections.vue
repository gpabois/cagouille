<script setup lang="ts">
import Table from '@/components/Table.vue';
import query from '@/graphql/VueSuivisInspections.js';
import { AUTOCOMPLETE as AUTOCOMPLETE_AIOT } from '@/graphql/Aiots.js';
import { parseAndFormatDate, loadMoreMixin } from '@/utils.js';
import {ref} from 'vue';
import NouveauSuiviInspection from '../components/suivis/inspections/Nouveau.vue'

const loadMore = loadMoreMixin("suiviInspections");
const orderBy = ref('');
const filter = ref({});
const displayCreateForm = ref(false);

const columns = [{
    id: 'nom',
    name: 'Nom',
    value: (row) => row.node.nom
}, {
    id: 'aiot',
    name: 'Aiot',
    value: (row) => row.node.aiot.nom
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
    <button class="btn btn-primary" type="button" @click="displayCreateForm=true">Nouveau</button>
    <ApolloQuery :query="query" :variables="{orderBy, ...filter}">
        <template v-slot="{ result: { loading, error, data }, query, refetch }">
            <div v-if="displayCreateForm">
                <div class="shadow p-4 mt-3 mb-3">
                    <NouveauSuiviInspection @created="refetch()"/>
                </div>
            </div>
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
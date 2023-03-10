<script setup lang="ts">
import {ref, reactive} from 'vue'
import { MES_TACHES as query } from '@/graphql/Moi.js';
import Table from '@/components/Table.vue';
import { loadMoreMixin } from '@/composables/relay.js';

const chargerPlusTachesRvat = loadMoreMixin({
    get: (data: any) => data.mesTaches.rvats,
    update: (data: any, {edges, pageInfo}) => ({
        ...data,
        mesTaches: {
            ...data.mesTaches,
            rvats: {
                edges,
                pageInfo
            }
        }
    })
});

const filters = reactive({})
const orderBy = ref(null);

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.filter_id}`).join(',');
}

function filterUpdated(filters) {}

const colonnesTachesRvat = [{
    id: 'id',
    name: '#',
    value: (edge) => edge.node.id
}, {
    id: 'nom_rvat',
    name: 'Nom',
    value: (edge) => edge.node.rvat.nom
}, {
    id: 'aiot',
    name: 'Aiot',
    value: (edge) => edge.node.rvat.aiot.libelle
}, {
    id: 'step',
    name: 'Etape',
    value: (edge) => edge.node.step
}, {
    id: 'deadline',
    name: 'Date limite',
    value: (edge) => edge.node.deadline || '-'
}]
</script>

<template>
    <div class="container-fluid m-3">
        <ApolloQuery :query="query" fetchPolicy="network-only">
            <template v-slot="{ result: { loading, error, data }, query, isLoading  }">
                <h1>Mes tâches</h1>
                <div v-if="isLoading" class="container spinner-border text-center" role="status">
                    <span class="sr-only"></span>
                </div>
                <div v-if="data && data.mesTaches.rvats.edges">
                    <Table                     
                        :rows="data.mesTaches.rvats.edges" 
                        :columns="colonnesTachesRvat"
                        @sort="sortUpdated"
                        @filter="filterUpdated"    
                    >
                        <template v-slot:row_id="{row}">
                            <RouterLink :to="{name: 'tache', params: {id: row.node.id}}">
                                {{ row.node.id }}
                            </RouterLink>
                        </template>
                    </Table>
                    <button class="btn btn-primary" @click="chargerPlusTachesRvat(query, data)" :disabled="!data.mesTaches.rvats.pageInfo.hasNextPage">Charger plus</button>
                </div>
                <div v-else>
                    No result :(
                </div>
            </template>
        </ApolloQuery>
    </div>
</template>

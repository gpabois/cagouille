<script setup lang="ts">
import {ref, reactive} from 'vue'
import { useRouter } from 'vue-router'
import { RECUPERER_TACHES as query } from '@/graphql/Taches.js';
import Table from '@/components/Table.vue';
import { loadMoreMixin } from '../utils.js';

const loadMore = loadMoreMixin('tasks');
const filters = reactive({})
const orderBy = ref(null);

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.filter_id}`).join(',');
}

function filterUpdated(filters) {}

const columns = [{
    id: 'id',
    name: '#',
    value: (edge) => edge.node.id
}, {
    id: 'flowClass',
    name: 'Type de flux',
    value: (edge) => edge.node.process.flowClass
},{
    id: 'step',
    name: 'Etape',
    value: (edge) => edge.node.step,
},{
    id: 'status',
    name: 'Statut',
    value: (edge) => edge.node.status
}, {
    id: 'message',
    name: 'Message',
    value: (edge) => edge.node.log
}]
</script>

<template>
    <div class="container-fluid m-3">
        <ApolloQuery :query="query" :variables="{...filters, orderBy}">
            <template v-slot="{ result: { loading, error, data }, query, isLoading  }">
                <h1>Tâches</h1>
                <div v-if="isLoading" class="container spinner-border text-center" role="status">
                    <span class="sr-only"></span>
                </div>
                <div v-if="data && data.tasks.edges">
                    <Table                     
                        :rows="data.tasks.edges" 
                        :columns="columns"
                        @sort="sortUpdated"
                        @filter="filterUpdated"    
                    >
                        <template v-slot:row_id="{row}">
                            <RouterLink :to="{name: 'tache', params: {id: row.node.id}}">
                                {{ row.node.id }}
                            </RouterLink>
                        </template>
                    </Table>
                    <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.tasks.pageInfo.hasNextPage">Charger plus</button>
                </div>
                <div v-else>
                    No result :(
                </div>
            </template>
        </ApolloQuery>
    </div>
</template>

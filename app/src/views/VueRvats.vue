<script setup lang="ts">
import {ref, reactive, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useMutation } from '@vue/apollo-composable'
import { SUPPRIMER, CREER, RECUPERER_RVATS as query } from '@/graphql/Rvats.js';
import Table from '@/components/Table.vue';
import { loadMoreMixin } from '../utils.js';

const router = useRouter();
const loadMore = loadMoreMixin('rvats');
const filters = reactive({})
const orderBy = ref(null);

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.filter_id}`).join(',');
}

function filterUpdated(filters) {}

const columns = [{
    id: 'id',
    name: '#',
    value: (edge) => edge.node.id,
    sortable: true
}, {
    id: 'nom',
    name: 'Nom',
    value: (edge) => edge.node.nom || "-"
}, {
    id: 'reference',
    name: "N° chrono",
    value: (edge) => edge.node.reference || "-"
}, {
    id: 'verifié',
    name: "Vérifié",
    value: (edge) => edge.node.verifie || "-"
}, {
    id: 'approuvé',
    name: "Approuvé",
    value: (edge) => edge.node.approuve || "-"
}, {
    id: 'dossier',
    name: "Dossier",
    value: (edge) => edge.node.uriTravail || edge.node.uriDefinitif || "-"
}, {
    id: 'actions',
    name: 'Actions',
    value: (edge) => null
}]

const { mutate: creerRvat, onDone: rvatCree } = useMutation(CREER);

rvatCree(function ({data}) {
    router.push({
        name: 'tache',
        params: {
            id: data.rvat.create.task.id
        }
    })
});
</script>

<template>
    <div class="container-fluid m-3">
        <button class="btn btn-primary" @click="creerRvat()">Créer RVAT</button>
        <ApolloQuery :query="query" :variables="{...filters, orderBy}" :pollInterval="100">
            <template v-slot="{ result: { loading, error, data }, query: {refetch} }">
                <h1>RVATS</h1>
                   
                <div v-if="loading" class="container spinner-border text-center" role="status">
                    <span class="sr-only"></span>
                </div>
                <div v-if="data && data.rvats.edges">
                    <Table :rows="data.rvats.edges" :columns="columns" @sort="sortUpdated" @filter="filterUpdated">
                        <template v-slot:row_actions="{row}">
                            <ApolloMutation :mutation="SUPPRIMER" :variables="{id: row.node.id}" :refetchQueries="[{query, variables: {...filters, orderBy}}]">
                                <template v-slot="{mutate: supprimerRvat, loading: deleting, error}">
                                    <button class="btn btn-danger" :disabled="deleting" @click="supprimerRvat({id: row.node.id})">
                                        Supprimer
                                    </button>
                                </template>
                            </ApolloMutation>
                        </template>
                    </Table>
                    <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.rvats.pageInfo.hasNextPage">Charger plus</button>
                </div>
                <div v-else>
                    No result :(
                </div>
            </template>
        </ApolloQuery>
    </div>
</template>

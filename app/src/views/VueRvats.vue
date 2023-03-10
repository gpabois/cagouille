<script setup lang="ts">
import {ref, reactive, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useMutation } from '@vue/apollo-composable'
import { SUPPRIMER, RECUPERER_RVATS as query } from '@/graphql/Rvats.js';
import Table from '@/components/Table.vue';
import NouveauRvat from '@/components/taches/Rvat/Nouveau.vue'
import { loadMoreMixin } from '../utils.js';

const loadMore = loadMoreMixin('rvats');
const filters = reactive({})
const orderBy = ref(null);
const afficherFormulaireCreation = ref(false);

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.filter_id}`).join(',');
}

function filterUpdated(filters) {}

const columns = [{
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
    value: (edge) => edge.node.verifie
}, {
    id: 'approuvé',
    name: "Approuvé",
    value: (edge) => edge.node.approuve
}, {
    id: 'transmis',
    name: "Transmis",
    value: (edge) => edge.node.transmis
}, {
    id: 'dossier',
    name: "Dossier",
    value: (edge) => ""
}, {
    id: 'actions',
    name: 'Actions',
    value: (edge) => null
}]

async function rvatCree(query) {
    afficherFormulaireCreation.value = false;
    await query.refetch();
}
</script>

<template>
    <div class="container-fluid mt-3">
        <ApolloQuery :query="query" :variables="{...filters, orderBy}">
            <template v-slot="{ result: { loading, error, data }, query}">
                <div class="container-fluid mb-3">
                    <div class="row">
                        <div class="col">
                            <h1>RVATS</h1>
                        </div>
                        <div class="col"></div>
                        <div class="col">
                            <button class="btn btn-primary" @click="afficherFormulaireCreation = true">Créer RVAT</button>
                        </div>
                    </div>
                </div>
                
                <NouveauRvat 
                    class="shadow-sm p-3 mb-3" 
                    v-if="afficherFormulaireCreation" 
                    @created="rvatCree(query)"/>
                
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
                        <template v-slot:row_dossier="{row: {node: {uriDefinitif, uriTravail}}}">
                            <a :href="uriDefinitif" v-if="uriDefinitif">
                                version définitive
                            </a>
                            <a :href="uriTravail" v-if="uriTravail">
                                Dossier
                            </a>
                        </template>
                        <template v-slot:row_verifié="{value: vérifié}">
                            <i class="bi bi-check2-square" v-if="vérifié"></i>
                            <i class="bi bi-x-square" v-else-if="vérifié==false"></i>
                            <i class="bi bi-square" v-else></i>
                        </template>
                        <template v-slot:row_approuvé="{value: approuvé}">
                            <i class="bi bi-check2-square" v-if="approuvé"></i>
                            <i class="bi bi-x-square" v-else-if="approuvé==false"></i>
                            <i class="bi bi-square" v-else></i>
                        </template>
                        <template v-slot:row_transmis="{value: transmis}">
                            <i class="bi bi-check2-square" v-if="transmis"></i>
                            <i class="bi bi-x-square" v-else-if="transmis==false"></i>
                            <i class="bi bi-square" v-else></i>
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

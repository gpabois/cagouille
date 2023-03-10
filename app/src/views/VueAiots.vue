<script setup lang="ts">
import {ref, reactive} from 'vue'
import { useRouter } from 'vue-router'
import query from '@/graphql/VueAiots.js';
import Table from '@/components/Table.vue';
import { loadMoreMixin } from '../utils.js';

const router = useRouter();
const loadMore = loadMoreMixin('aiots');
const filters = reactive({
    nom_Icontains: null,
    rubriquesIcpe_Rubrique_Code_Icontains: null,
    commune_Nom_Icontains: null,
    code_Icontains: null
})
const orderBy = ref(null);

function sortUpdated(sorts) {
    orderBy.value = sorts.map(s => `${s.sort == 'asc' ? '' : '-'}${s.filter_id}`).join(',');
}

function filterUpdated(filters) {
    
}

const columns = [{
    id: 'nom',
    name: 'Nom',
    value: (edge) => edge.node.nom,
    sortable: true
}, {
    id: 'code',
    name: 'Code AIOT',
    value: (edge) => edge.node.code
}, {
    id: 'commune',
    name: 'Commune',
    value: (edge) => edge.node.commune.nom,
    sortable: {'id': 'commune__nom'}
}, {
    id: 'departement',
    name: 'Département',
    value: (edge) => edge.node.commune.departement.nom
}, {
    id: 'region',
    name: 'Région',
    value: (edge) => edge.node.commune.departement.region.nom
}, {
    id: 'rubriques',
    name: "Rubriques ICPE", 
    value: (edge) => {
        return edge.node.rubriquesIcpe.edges.map((edge) => {
            return `${edge.node.rubrique.code} [${edge.node.rubrique.regime}]`
        })
    }
}]

</script>

<template>
    <div class="container-fluid m-3">
        <ApolloQuery :query="query" :variables="{...filters, orderBy}">
            <template v-slot="{ result: { loading, error, data }, query, isLoading  }">
                <h1>AIOTS</h1>
            
                <div class="container mb-3">
                    <form class="form-inline">
                        <div class="form-group">
                            <label for="nom_Icontains">
                            Nom
                            </label>
                            <input type="text" class="form-control" v-model="filters.nom_Icontains" id="nom_Icontains"/>
                        </div>
                        <div class="form-group">
                            <label for="code_Icontains">
                            Code AIOT
                            </label>
                            <input type="text" class="form-control" v-model="filters.code_Icontains" id="nom_Icontains"/>
                        </div>
                        <div class="form-group">
                            <label for="rubriquesIcpe_Rubrique_Code_Icontains">
                                Rubrique ICPE
                            </label>
                            <input 
                                id="rubriquesIcpe_Rubrique_Code_Icontains" 
                                class="form-control"
                                type="text" 
                                v-model="filters.rubriquesIcpe_Rubrique_Code_Icontains"/>
                        </div>
                        <div class="form-group">
                            <label for="commune_Nom_Icontains">
                                Commune
                            </label>
                            <input 
                                id="commune_Nom_Icontains" 
                                class="form-control"
                                type="text" 
                                v-model="filters.commune_Nom_Icontains"/>
                        </div>
                    </form>
                </div>
        
                <div v-if="isLoading || loading" class="container spinner-border text-center" role="status">
                    <span class="sr-only"></span>
                </div>
                <div v-if="data && data.aiots.edges">
                    <Table                     
                        :rows="data.aiots.edges" 
                        :columns="columns"
                        @sort="sortUpdated"
                        @filter="filterUpdated"    
                    >
                        <template v-slot:row_nom="{row}">
                            <RouterLink :to="{name: 'detail_aiot', params: {id: row.node.id}}">
                                {{ row.node.nom }}
                            </RouterLink>
                        </template>
                        <template v-slot:row_code="{value}">
                            <a :href="`https://gunenv.din.developpement-durable.gouv.fr/aiot/?searchTerms=${value}`">
                                {{ value }}
                            </a>
                        </template>
                        <template v-slot:row_rubriques="{value: rubriques}">
                            <ul class="list-group-flush" v-for="rubrique in rubriques" v-if="rubriques">
                                <li class="list-group-item">{{ rubrique }}</li>
                            </ul>
                            <div v-else></div>
                        </template>
                    </Table>
                    <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.aiots.pageInfo.hasNextPage">Charger plus</button>
                </div>
                <div v-else-if="!data && !isLoading">
                    No result :(
                </div>
            </template>
        </ApolloQuery>
    </div>
</template>

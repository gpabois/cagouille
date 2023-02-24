<script setup lang="ts">
import { useRouter } from 'vue-router'
import query from '../graphql/VueAiots.js';
import AutocompleteAiot from '../components/AutocompleteAiot.vue'

const router = useRouter();

async function loadMore(query: any, data: any) {
    await query.fetchMore({
        variables: {
            cursor: data.aiots.pageInfo.endCursor
        },
        updateQuery: (prevResult: any, {fetchMoreResult}) => {
            const newEdges = fetchMoreResult.aiots.edges;
            const pageInfo = fetchMoreResult.aiots.pageInfo;
            return newEdges.length ? {
                ...prevResult,
                aiots: {
                    ...prevResult.aiots,
                    edges: [...prevResult.aiots.edges, ...newEdges],
                    pageInfo,
                }
            } : prevResult;
        }
    })
}

function allerDetailAiot(id) {
    if(id) {
        router.push({
            name: 'detail_aiot',
            params: {
                id
            }
        })
    }
}

</script>

<template>
    <h1>AIOTS</h1>
    <AutocompleteAiot @input="allerDetailAiot" />
    <ApolloQuery :query="query">
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">
                <table class="table" v-if="data.aiots">
                    <thead>
                        <th>Nom</th>
                        <th>Code AIOT</th>
                        <th>Commune</th>
                        <th>Département</th>
                        <th>Région</th>
                    </thead>
                    <tbody>
                        <tr v-for="edge in data.aiots.edges">
                            <td><RouterLink :to="{name: 'detail_aiot', params: {id: edge.node.id}}">{{ edge.node.nom }}</RouterLink></td>
                            <td>{{ edge.node.code }}</td>
                            <td>{{ edge.node.commune.nom }}</td>
                            <td>{{ edge.node.commune.departement.nom }}</td>
                            <td>{{ edge.node.commune.departement.region.nom }}</td>
                        </tr>
                    </tbody>
                </table>
                <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.aiots.pageInfo.hasNextPage">Charger plus</button>
            </div>
        </template>
    </ApolloQuery>
</template>

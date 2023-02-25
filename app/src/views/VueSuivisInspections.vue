<script setup lang="ts">
import { useRouter } from 'vue-router'
import query from '../graphql/VueSuivisInspections.js';

const router = useRouter();

async function loadMore(query: any, data: any) {
    await query.fetchMore({
        variables: {
            cursor: data.suivisInspections.pageInfo.endCursor
        },
        updateQuery: (prevResult: any, {fetchMoreResult}) => {
            const newEdges = fetchMoreResult.suivisInspections.edges;
            const pageInfo = fetchMoreResult.suivisInspections.pageInfo;
            return newEdges.length ? {
                ...prevResult,
                suivisInspections: {
                    ...prevResult.suivisInspections,
                    edges: [...prevResult.suivisInspections.edges, ...newEdges],
                    pageInfo,
                }
            } : prevResult;
        }
    })
}
</script>

<template>
    <h1>Inspections</h1>
    <ApolloQuery :query="query">
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">
                <table class="table" v-if="data.suivisInspections">
                    <thead>
                        <th>Nom</th>
                        <th>AIOT</th>
                        <th>Statut</th>
                        <th>Type</th>
                    </thead>
                    <tbody>
                        <tr v-for="edge in data.suivisInspections.edges">
                            <td>
                                {{ edge.node.nom }}
                            </td>
                            <td>
                                <RouterLink :to="{name: 'detail_aiot', params: {id: edge.node.aiot.id}}">
                                    {{ edge.node.aiot.nom }}
                                </RouterLink>
                            </td>
                            <td>
                                {{ edge.node.statut.nom }}
                            </td>
                            <td>
                                {{ edge.node.type.nom }}
                            </td>
                        </tr>
                    </tbody>
                </table>
                <button class="btn btn-primary" @click="loadMore(query, data)" :disabled="!data.suivisInspections.pageInfo.hasNextPage">Charger plus</button>
            </div>
        </template>
    </ApolloQuery>
</template>

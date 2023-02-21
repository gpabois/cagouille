<script setup lang="ts">
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'

const AIOTS_QUERY = gql`
    query recupererAiots($cursor: String) {
        aiots(after: $cursor) {
            edges {
                node {
                    id, nom, code, commune {nom, abbv, departement {nom, region {nom}}}
                }
            },
            pageInfo {
                endCursor
                hasNextPage
            }
        }
    }
`;

const {result, fetchMore} = useQuery(AIOTS_QUERY, () => {});

function loadMore() {
    fetchMore({
        variables: {
            cursor: result.aiots.pageInfo.endCursor
        },
        updateQuery: (prevResult, {fetchMoreResult}) => {
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

</script>

<template>
    <h1>AIOTS</h1>
    {{ result }}

    
    <table class="table" v-if="result && result.aiots">
        <thead>
            <th>Nom</th>
            <th>Code AIOT</th>
            <th>Commune</th>
            <th>Département</th>
            <th>Région</th>
        </thead>
        <tbody>
            <tr v-for="edge in result.aiots.edges">
                <td>{{ edge.node.nom }}</td>
                <td>{{ edge.node.code }}</td>
                <td>{{ edge.node.commune.nom }}</td>
                <td>{{ edge.node.commune.departement.nom }}</td>
                <td>{{ edge.node.commune.departement.region.nom }}</td>
            </tr>
        </tbody>
    </table>

    <button class="btn btn-primary" @click="loadMore" :disabled="!result.aiots.pageInfo.hasNextPage">Charger plus</button>
</template>

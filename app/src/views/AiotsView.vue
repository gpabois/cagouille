<script setup lang="ts">
async function loadMore(query) {
    await query.fetchMore({
        variables: {
            cursor: query.data.aiots.pageInfo.endCursor
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
    <ApolloQuery :query="gql => gql`
        query RecupererAiots($cursor: String) {
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
    `">
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
                            <td>{{ edge.node.nom }}</td>
                            <td>{{ edge.node.code }}</td>
                            <td>{{ edge.node.commune.nom }}</td>
                            <td>{{ edge.node.commune.departement.nom }}</td>
                            <td>{{ edge.node.commune.departement.region.nom }}</td>
                        </tr>
                    </tbody>
                </table>
                <button class="btn btn-primary" @click="loadMore(query)" :disabled="!data.aiots.pageInfo.hasNextPage">Charger plus</button>

            </div>
        </template>
    </ApolloQuery>
</template>

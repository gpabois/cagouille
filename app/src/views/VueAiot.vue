<script setup lang="ts">

import { RECUPERER_AIOT } from '../graphql/VueAiot.js';

</script>

<template>
    <ApolloQuery :query="RECUPERER_AIOT" :variables="{id: $route.params.id}">
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">
                <h1>{{ data.aiot.nom }} à {{ data.aiot.commune.nom }}</h1>

                <h2>Rubriques ICPE</h2>
                <table class="table">
                    <thead>
                        <th>Rubrique</th>
                        <th>Libellé</th>
                        <th>Régime</th>
                    </thead>
                    <tbody>
                        <tr v-for="rubriqueIcpe in data.aiot.rubriquesIcpe.edges">
                            <td>{{ rubriqueIcpe.node.rubrique.code }}</td>
                            <td>{{ rubriqueIcpe.node.rubrique.libelle }}</td>
                            <td>{{ rubriqueIcpe.node.rubrique.regime }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </template>
    </ApolloQuery>
</template>

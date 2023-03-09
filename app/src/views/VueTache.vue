<script setup lang="ts">

import { RECUPERER_TACHE as query } from '@/graphql/Taches.js';

import RvatStart from '@/components/taches/Rvat/Start.vue';
import RvatVerifier from '@/components/taches/Rvat/Verifier.vue';
import RvatApprouver from '@/components/taches/Rvat/Approuver.vue';
import RvatTransmettre from '@/components/taches/Rvat/Transmettre.vue';

const router = {
    Rvat: {
        start: RvatStart,
        verifier: RvatVerifier,
        approuver: RvatApprouver,
        transmettre: RvatTransmettre
    }
}

function getComponent(task) {
    if(router[task.process.flowClass] && router[task.process.flowClass][task.step]) {
        return router[task.process.flowClass][task.step];
    }
    return null;
}

</script>

<template>
    <ApolloQuery :query="query" :variables="{id: $route.params.id}">
        <template v-slot="{ result: { loading, error, data }, query: {refetch} }">
            <div v-if="data">
                <template v-if="getComponent(data.task) && data.task.status == 'STALL'">
                    <component 
                        :is="getComponent(data.task)" 
                        :task="data.task" 
                        @done="refetch()"/>
                </template>
                <template v-else>
                    <h1>Tâche {{ data.task.id }}</h1>

                    Statut: {{ data.task.status }}
                </template>
            </div>
        </template>
    </ApolloQuery>
</template>

<script setup lang="ts">

import { RECUPERER_TACHE as query } from '@/graphql/Taches.js';
import RvatVerifier from '@/components/taches/Rvat/Verifier.vue';
import RvatApprouver from '@/components/taches/Rvat/Approuver.vue';
import RvatTransmettre from '@/components/taches/Rvat/Transmettre.vue';

const router = {
    rvat: {
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

async function tacheExecutee(query) {
    console.log("Tâche éxecutée");
    await query.refetch();
}

</script>

<template>
    <ApolloQuery :query="query" :variables="{id: $route.params.id}">
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">
                <template v-if="getComponent(data.task) !== null && data.task.status == 'STALL'">
                    <component 
                        :is="getComponent(data.task)" 
                        :task="data.task" 
                        @done="tacheExecutee(query)"/>
                </template>
                <template v-else>
                    <h1>Tâche {{ data.task.id }}</h1>
                    Flux: {{ data.task.process.flowClass }}
                    Etape: {{ data.task.step }}
                    Statut: {{ data.task.status }}
                </template>
            </div>
        </template>
    </ApolloQuery>
</template>

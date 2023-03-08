<script setup lang="ts">

import { RECUPERER_TACHE as query } from '@/graphql/Taches.js';

import RvatStart from '@/components/taches/Rvat/Start.vue';

const router = {
    Rvat: {
        start: RvatStart
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
        <template v-slot="{ result: { loading, error, data }, query }">
            <div v-if="data">
                <h1>{{ data.task.process.flowClass }} {{ data.task.step }} à {{ data.task.id }}</h1>
                
                <template v-if="getComponent(data.task)">
                    <component :is="getComponent(data.task)" :task="data.task">
                    </component>
                </template>
            </div>
        </template>
    </ApolloQuery>
</template>

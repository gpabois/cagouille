<script setup lang="ts">
import {ref, defineProps, defineEmits} from 'vue'
import {TRANSMETTRE as MUTATION} from '@/graphql/Rvats.js'
import AutocompleteGroupe from '@/components/fields/AutocompleteGroupe.vue'
import AutocompleteAiot from '@/components/fields/AutocompleteAiot.vue';

var props = defineProps(['task']);
var emit = defineEmits(['done']);
var input = ref({
    task: props.task.id,
    reference: null,
    uriDefinitif: null
})

</script>
<template>
    <div class="container m-3">
        <h2>Transmettre le RVAT</h2>
        <ApolloMutation :mutation="MUTATION" :variables="{input}" @done="emit('done', $event)">
            <template v-slot="{mutate, loading, error}">
                <form>
                    {{ error }}
                    <div class="form-group">
                        <label>N° Chrono</label>
                        <input type="text" id="checkbox" v-model="input.reference" class="form-control">
                    </div>
                    
                    <div class="form-group" v-if="input.approuve">
                        <label>Chemin vers le document en version définitive</label>
                        <input type="text" v-model="input.uriDefinitif" class="form-control"/>
                    </div>

                    <button class="btn btn-primary" @click="mutate()">
                        Transmettre
                    </button>
                </form>
            </template>
        </ApolloMutation>
    </div>
</template>
<script setup lang="ts">
import {ref, defineProps, defineEmits, watchEffect, reactive} from 'vue'
import {TRANSMETTRE as MUTATION} from '@/graphql/Rvats.js'

var props = defineProps(['task']);
var emit = defineEmits(['done']);
var input = reactive({
    task: props.task.id,
    reference: null,
    uriDefinitif: null
})

var uriFragment = ref(null)

watchEffect(() => {
    input.uriDefinitif = `file:///${uriFragment.value}`;
});

</script>
<template>
    <div class="container m-3">
        <h2>Transmettre le RVAT</h2>
        <ApolloMutation :mutation="MUTATION" :variables="{input}" @done="emit('done', $event)">
            <template v-slot="{mutate, loading, error}">
                    {{ error }}
                    <div class="form-group">
                        <label>N° Chrono</label>
                        <input type="text" id="checkbox" v-model="input.reference" class="form-control">
                    </div>
                    
                    <div class="form-group mb-3">
                        <label>Chemin vers le document en version définitive</label>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <div class="input-group-text">file:///</div>
                            </div>
                            <input type="text" v-model="uriFragment" class="form-control"/>
                        </div>
                    </div>

                    <button class="btn btn-primary" @click="mutate()">
                        Transmettre
                    </button>
            </template>
        </ApolloMutation>
    </div>
</template>
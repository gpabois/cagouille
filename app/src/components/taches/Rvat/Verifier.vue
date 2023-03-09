<script setup lang="ts">
import {ref, defineProps, defineEmits} from 'vue';
import {VERIFIER as MUTATION} from '@/graphql/Rvats.js';

var props = defineProps(['task']);
var emit = defineEmits(['done']);
var input = ref({
    task: props.task.id,
    verifie: null,
    commentaireVerificateur: null
});

var onDone = function($event: any) {
    console.log($event);
}

var onError = function($event: any) {
    console.log($event);
}
</script>

<template>
    <div class="container m-3">
        <h2>Vérifier le RVAT</h2>
        <ApolloMutation :mutation="MUTATION" :variables="{input}" @done="emit('done', $event)">
            <template v-slot="{mutate, loading, error, gqlError}">
                <div v-if="error">{{ {...error} }}</div>
                <div class="form-check">
                    <label class="form-check-label">Vérifié ?</label>
                    <input type="checkbox" id="checkbox" v-model="input.verifie" class="form-check-input">
                </div>
                
                <div class="form-group mb-3" v-if="input.verifie">
                    <label>Commentaire pour le rédacteur</label>
                    <textarea type="text" v-model="input.commentaireVerificateur" class="form-control">
                    </textarea>
                </div>

                <button class="btn btn-primary" @click="mutate()" :disabled="loading">
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" v-if="loading"></span>Transmettre
                </button>
            </template>
        </ApolloMutation>
    </div>
</template>
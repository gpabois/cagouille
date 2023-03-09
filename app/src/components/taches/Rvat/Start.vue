<script setup lang="ts">
import {ref, defineProps, defineEmits} from 'vue';
import {START as MUTATION} from '@/graphql/Rvats.js';
import AutocompleteGroupe from '@/components/fields/AutocompleteGroupe.vue'
import AutocompleteAiot from '@/components/fields/AutocompleteAiot.vue';

var props = defineProps(['task']);
var emit = defineEmits(['done']);
var input = ref({
    task: props.task.id,
    aiot: null,
    uriTravail: null,
    verificateur: null,
    approbateur: null,
    administratif: null,
    dateLimiteVerification: null,
    dateLimiteApprobation: null
});
</script>

<template>
    <div class="container m-3">
        <h2>Préparer le RVAT</h2>
        <ApolloMutation :mutation="MUTATION" :variables="{input}" @done="emit('done', $event)">
            <template v-slot="{mutate, loading, error}">
                {{ error }}
                <div class="form-group">
                    <label>Nom</label>
                    <input type="text" v-model="input.nom" class="form-control"/>
                </div>
                
                <div class="form-group">
                    <label>Lien vers le RVAT</label>
                    <input type="text" v-model="input.uriTravail" class="form-control"/>
                </div>
                
                <div class="form-group">
                    <label>AIOT</label>
                    <AutocompleteAiot v-model="input.aiot"/>
                </div>

                <div class="form-group">
                    <label>Vérificateur</label>
                    <AutocompleteGroupe v-model="input.verificateur"/>
                </div>

                <div class="form-group">
                    <label>Date limite vérificateur</label>
                    <input type="date" class="form-control" v-model="input.dateLimiteVerification"/>
                </div>
                
                <div class="form-group">
                    <label>Approbateur</label>
                    <AutocompleteGroupe v-model="input.approbateur"/>
                </div>

                <div class="form-group">
                    <label>Date limite approbateur</label>
                    <input type="date" class="form-control" v-model="input.dateLimiteApprobation"/>
                </div>

                <div class="form-group mb-3">
                    <label>Administratif</label>
                    <AutocompleteGroupe v-model="input.administratif"/>
                </div>

                <button class="btn btn-primary" @click="mutate()">
                    Transmettre
                </button>
            </template>
        </ApolloMutation>
    </div>
</template>
<script setup lang="ts">
import {reactive, defineProps} from 'vue'
import { useMutation } from '@vue/apollo-composable'
import {START as MUTATION} from '@/graphql/Rvat.js'
import AutocompleteGroupe from '@/components/fields/AutocompleteGroupe.vue'
import AutocompleteAiot from '@/components/fields/AutocompleteAiot.vue';

var props = defineProps(['task'])

var input = reactive({
    task: props.task.id,
    aiot: null,
    nom: null,
    uriTravail: null,
    verificateur: null,
    approbateur: null,
    administratif: null,
    dateLimiteVerification: null,
    dateLimiteApprobation: null
})

const { mutate } = useMutation(MUTATION, {variables: {input}, errorPolicy: 'ignore'});

</script>
<template>
    <div class="container m-3">
        <h2>Execution</h2>
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

    </div>
</template>
<script setup lang="ts">
import { reactive, defineEmits } from 'vue';
import { NOUVEAU } from '@/graphql/SuivisInspections.js';
import SelectStatutSuivis from '@/components/fields/SelectStatutSuivis.vue';
import SelectTypeInstruction from '@/components/fields/SelectTypeInstruction.vue';
import AutocompleteAiot from '@/components/fields/AutocompleteAiot.vue';

const input = reactive({
    'nom': null,
    'aiot': null,
    'type': null,
    'statut': null,
    'datePrevisionnelle': null,
    'datePreparation': null,
    'dateInspection': null,
    'dateRapport': null
});

const emit = defineEmits(['created'])

</script>

<template>
    <div>
        <ApolloMutation :mutation="NOUVEAU" :variables="{input}">
            <template v-slot="{mutate, loading, error}">
                <form>
                    {{ error }}
                    <div class="form-group mb-3">
                        <label>Nom</label>
                        <input class="form-control" v-model="input.nom"/>
                    </div>
                    <div class="form-group mb-3">
                        <label>AIOT</label>
                        <AutocompleteAiot v-model="input.aiot" />
                    </div>
                    <div class="form-group mb-3">
                        <label>Statut</label>
                        <SelectStatutSuivis v-model="input.statut" />
                    </div>
                    <div class="form-group mb-3">
                        <label>Types</label>
                        <SelectTypeInstruction v-model="input.type" />
                    </div>
                    <div class="form-group mb-3">
                        <label>Date prévisionnelle</label>
                        <input type="date" class="form-control" v-model="input.datePrevisionnelle"/>
                    </div>
                    <div class="form-group mb-3">
                        <label>Date préparation</label>
                        <input type="date" class="form-control" v-model="input.datePreparation"/>
                    </div>
                    <div class="form-group mb-3">
                        <label>Date inspection</label>
                        <input type="date" class="form-control" v-model="input.dateInspection"/>
                    </div>
                    <div class="form-group mb-3">
                        <label>Date rapport</label>
                        <input type="date" class="form-control" v-model="input.dateRapport"/>
                    </div>
                    
                    <button class="btn btn-primary" :disabled="loading" @click="mutate()">Ajouter</button>
                </form>
            </template>
        </ApolloMutation>
    </div>
</template>
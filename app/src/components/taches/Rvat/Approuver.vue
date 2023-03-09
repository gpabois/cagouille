<script setup lang="ts">
import {ref, defineProps, defineEmits} from 'vue';
import {APPROUVER as MUTATION} from '@/graphql/Rvats.js';

var props = defineProps(['task']);
var emit = defineEmits(['done']);
var input = ref({
    task: props.task.id,
    approuve: null,
    commentaireApprobateur: null
});
</script>

<template>
    <div class="container m-3">
        <h2>Approuver le RVAT</h2>
        <ApolloMutation :mutation="MUTATION" :variables="{input}" @done="emit('done', $event)">
            <template v-slot="{mutate, loading, error}">
                <form>
                    {{ error }}
                    <div class="form-group">
                        <label>Approuvé ?</label>
                        <input type="checkbox" id="checkbox" v-model="input.approuve" class="form-control">
                    </div>
                    
                    <div class="form-group" v-if="input.approuve">
                        <label>Commentaire pour le rédacteur :</label>
                        <textarea v-model="input.commentaireApprobateur" class="form-control">
                        </textarea>
                    </div>

                    <button class="btn btn-primary" @click="mutate()">
                        Transmettre
                    </button>
                </form>
            </template>
        </ApolloMutation>
    </div>
</template>
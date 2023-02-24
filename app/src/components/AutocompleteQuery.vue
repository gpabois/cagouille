<script setup lang="ts">
import { ref, defineEmits, defineProps } from 'vue'

const props = defineProps([
    'query',
    'elements',
    'label',
    'id'
])

const filter = ref('');
const filterBuffer = ref ('');
const displayResults = ref(false);
const debounce = ref(false);

const emit = defineEmits(['input'])

function onSelected(e) {
    filterBuffer.value = e.target.dataset["label"];
    displayResults.value = false;
    debounce.value = true;
    emit('input', e.target.dataset["id"]);
}

function onInputModified(e) {
    emit('input', null)
    
    if(debounce.value) {
        debounce.value = false;
        return;
    }

    if (filterBuffer.value.length >= 3) 
    {
        displayResults.value = true;
        filter.value = filterBuffer.value;
    }
}
</script>

<template>
    <div class="autocomplete-form">
        <input class="form-control" type="text" id="autocomplete-search" v-model="filterBuffer" @input="onInputModified">
        <ApolloQuery :query="props.query" :variables="{filter}">
            <template v-slot="{ result: { loading, error, data }, query }">
                <ul class="list-group list-group-flush" 
                    v-if="displayResults && data && props.elements(data).length"
                >
                    <li class="list-group-item list-group-item-action" 
                        :data-id="props.id(element)"
                        :data-label="props.label(element)"
                        v-for="element in props.elements(data)" 
                        @click="onSelected">
                        {{ props.label(element) }}
                    </li>
                </ul>
            </template>
        </ApolloQuery>
    </div>
</template>
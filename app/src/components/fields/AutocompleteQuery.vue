<script setup lang="ts">
import { ref, defineEmits, defineProps } from 'vue'

const props = defineProps([
    'query',
    'elements',
    'transform',
    'value'
])

const filter = ref('');
const filterBuffer = ref ('');
const displayResults = ref(false);
const debounce = ref(false);

const emit = defineEmits(['input']);

const onSelected = function (e) {
    filterBuffer.value = e.target.dataset["label"];
    displayResults.value = false;
    debounce.value = true;
    emit('input', e.target.dataset["value"]);
};

const onInputModified = function (_) {
    emit('input', null);
    
    if(debounce.value) {
        debounce.value = false;
        return;
    }

    if (filterBuffer.value.length >= 3) 
    {
        displayResults.value = true;
        filter.value = filterBuffer.value;
    }
};

const value = (element) =>  props.transform(element).value;
const label = (element) => props.transform(element).label;
</script>

<template>
    <div class="autocomplete-form">
        <input 
            class="form-control" 
            type="text" 
            id="autocomplete-search" 
            v-model="filterBuffer" 
            @input="onInputModified"
        >
        <ApolloQuery :query="props.query" :variables="{filter}">
            <template v-slot="{ result: { loading, error, data }, query }">
                <ul class="list-group list-group-flush" 
                    v-if="displayResults && data && props.elements(data).length"
                >
                    <li class="list-group-item list-group-item-action"
                        v-for="element in props.elements(data)"  
                        :data-value="value(element)"
                        :data-label="label(element)"
                        @click="onSelected">
                        {{ label(element) }}
                    </li>
                </ul>
            </template>
        </ApolloQuery>
    </div>
</template>
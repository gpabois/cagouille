<script setup lang="ts">
import { transform } from '@vue/compiler-core';
import { ref, defineEmits, defineProps, onUpdated } from 'vue'

const props = defineProps([
    'query',
    'elements',
    'transform',
    'required',
    'modelValue'
])

function onUpdated($event) {
    emit('update:modelValue', $event.target.value)
}

const emit = defineEmits(['update:modelValue'])

const value = (element: any) => props.transform(element).value
const label = (element: any) => props.transform(element).label
</script>

<template>
    <ApolloQuery :query="props.query">
            <template v-slot="{ result: { loading, error, data }, query }">
                <select 
                    class="form-select" 
                    @input="onUpdated"
                >
                    <option v-if="!required" :value="null">----</option>
                    <option v-if="data" v-for="element in elements(data)" :value="value(element)">
                        {{ label(element) }}
                    </option>
                </select>
            </template>
        </ApolloQuery>
</template>
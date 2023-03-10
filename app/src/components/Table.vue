<script setup lang="ts">
import { defineProps, defineEmits, reactive } from 'vue';
import AutocompleteQuery from '@/components/fields/AutocompleteQuery.vue';

const props = defineProps(['columns', 'rows']);

const order = reactive([]);
const filter = reactive([]);

const emit = defineEmits(['order', 'filter']);

function sortUpdated(col) {
    const stateIdx = sortStateIndex(col);
    
    if(stateIdx < 0) {
        var sort = {
            'id':   col.id,
            'filter_id': col.id,
            'sort': 'asc'
        };
        
        if (col.sortable.id) {
            sort['filter_id'] = col.sortable.id
        }
        
        order.push(sort);
    } else {
        if (order[stateIdx].sort == 'asc') {
            order[stateIdx].sort = 'desc';
        } else {
            order.splice(stateIdx, 1);
        }
    }

    emit('sort', order);
}

function sortState(col) {
    const stateIdx = sortStateIndex(col);
    if(stateIdx < 0) {
        return null
    } else {
        return order[stateIdx].sort
    }
}

function sortStateIndex(col) {
    return order.findIndex((e) => e.id == col.id)
}

const filterInIdx = (col) => filter.findIndex(f => f.type == 'in' && f.id == col.id);

function filterSelectedIn(col) {
    const idx = filterInIdx(col);

    if (idx >= 0) {
        return filter[idx].values;
    } else {
        return []
    }
}

function filterAddIn(col, element) 
{
    if(element == null)
        return;

    const idx = filterInIdx(col);

    if (idx >= 0) {
        filter[idx].values.push(element);
    } else {
        filter.push({
            'type': 'in',
            'id': col.id,
            'values': reactive([element])
        })
    }

    emit('filter', filter);
}

function filterRemoveIn(col, element) {
    const idx = filterInIdx(col);

    if (idx >= 0) {
        const elIdx = filter[idx].values.findIndex(el => el.id == element.id);

        if(elIdx >= 0) {
            filter[idx].values.splice(elIdx, 1);
        }

        if(filter[idx].values.length == 0) {
            filter.splice(idx, 1);
        }

        console.log(filter[idx]);
    }

    emit('filter', filter);
}

</script>
<template>
    <div class="table-responsive-md">
        <table class="table" v-if="props.rows">
            <thead>
                <th v-for="col in props.columns">
                    {{ col.name }} 

                    <span v-if="col.sortable" @click="sortUpdated(col)">
                        <i class="bi bi-sort-alpha-down" v-if="sortState(col) == 'asc'"></i>
                        <i class="bi bi-sort-alpha-up" v-else-if="sortState(col) == 'desc'"></i>
                        <i class="bi bi-sort-up-alt" v-else></i>
                    </span>
                    
                    <div v-if="col.filter">
                        <i class="bi bi-three-dots"></i>
                        <div v-if="col.filter.type == 'in'">
                            <AutocompleteQuery 
                                v-if="col.filter.values.type == 'query'"
                                :query="col.filter.values.query"
                                :elements="col.filter.values.elements"
                                :transform="col.filter.transform"
                                @input="element => filterAddIn(col, element)"
                            />
                            <ul class="list-group list-group-flush">
                                <li 
                                    v-for="element in filterSelectedIn(col)"
                                    class="list-group-item list-group-item-action" >
                                    <input 
                                        class="form-check-input me-1" 
                                        type="checkbox" 
                                        checked
                                        @click="filterRemoveIn(col, element)" />
                                    {{ element.label }}
                                </li>
                            </ul>
                        </div>
                    </div>
                </th>
            </thead>
            <tbody>
                <tr v-for="row in props.rows">
                    <td v-for="col in props.columns">
                        <slot :name="`row_${col.id}`" :row="row" :value="col.value(row)">
                            {{ col.value(row) }}
                        </slot>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>
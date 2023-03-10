import { ref, onMounted, onUnmounted } from 'vue'

export function loadMoreMixin({get, update}) {
    return async function(query, data) {
        root = get(data)
        await query.fetchMore({
            variables: {
                cursor: root.pageInfo.endCursor
            },
            updateQuery: (prevResult, {fetchMoreResult}) => {
                const newEdges = get_root_node(fetchMoreResult).edges;
                const currentEdges = node.edges;
                const newPageInfo = get_root_node(fetchMoreResult).pageInfo;
                return newEdges.length ? update({
                    ...prevResult,
                    suivisInspections: {
                        ...prevResult[dataKey],
                        edges: [...prevResult[dataKey].edges, ...newEdges],
                        pageInfo,
                    }
                }, {currentEdges, newEdges, edges: [...currentEdges, newEdges], pageInfo: newPageInfo, newPageInfo}) : prevResult;
            }
        })
    }
}
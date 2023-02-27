import moment from 'moment';

export function parseDate(strDate: string) {
   return moment(strDate)
}

export function parseAndFormatDate(strDate: string){
    var date = parseDate(strDate);

    if (date !== undefined || date !== null) {
        return date.format('DD/MM/YYYY');
    } else {
        return "";
    }
}

export const loadMoreMixin = (options) => async function(query, data) {
    const dataKey = options.data;
    await query.fetchMore({
        variables: {
            cursor: data[dataKey].pageInfo.endCursor
        },
        updateQuery: (prevResult: any, {fetchMoreResult}) => {
            const newEdges = fetchMoreResult[dataKey].edges;
            const pageInfo = fetchMoreResult[dataKey].pageInfo;
            return newEdges.length ? {
                ...prevResult,
                suivisInspections: {
                    ...prevResult[dataKey],
                    edges: [...prevResult[dataKey].edges, ...newEdges],
                    pageInfo,
                }
            } : prevResult;
        }
    })
}
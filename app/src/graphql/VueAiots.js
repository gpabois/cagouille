import gql from 'graphql-tag'

export default gql`
query RecupererAiots($cursor: String) {
    aiots(after: $cursor) {
        edges {
            node {
                id, nom, code, commune {nom, abbv, departement {nom, region {nom}}}
            }
        },
        pageInfo {
            endCursor
            hasNextPage
        }
    }
}  
`

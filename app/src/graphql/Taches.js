import gql from 'graphql-tag'

export const RECUPERER_TACHE = gql`query GetTask($id: ID!) {
    task(id: $id) {
        id,
        step,
        status,
        log,
        process {
            id, 
            flowClass
        }
    }
}`;

export const RECUPERER_TACHES = gql`query GetTask($cursor: String) {
    tasks(after: $cursor) {
        edges {
            node {
                id,
                status,
                step,
                log,
                process {
                    id, 
                    status,
                    flowClass
                }
            }
        },
        pageInfo {
            endCursor,
            hasNextPage
        }
    }
}`;

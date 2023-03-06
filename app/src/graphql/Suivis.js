import gql from 'graphql-tag'

export const TOUS = gql`query RecupererStatusSuivis {
    statusSuivis {
        edges {
            node {
                id, 
                nom
            }
        }
    }
}
`;

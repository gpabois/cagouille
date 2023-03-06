import gql from 'graphql-tag'

export const AUTOCOMPLETE = gql`
    query RecupererCommunes($filter: String) {
        communes(nom_Istartswith: $filter) {
            edges {
                node {
                    id, nom, codePostal
                }
            }
        }
    }  
`;

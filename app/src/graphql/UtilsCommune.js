import gql from 'graphql-tag'

export const AUTOCOMPLETE_COMMUNE = gql`
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

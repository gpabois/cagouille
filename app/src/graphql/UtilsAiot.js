import gql from 'graphql-tag'

export const AUTOCOMPLETE_AIOT = gql`
    query RecupererAiots($filter: String) {
        aiots(nom_Istartswith: $filter) {
            edges {
                node {
                    id, nom, code, commune {nom, abbv, departement {nom, region {nom}}}
                }
            }
        }
    }  
`;

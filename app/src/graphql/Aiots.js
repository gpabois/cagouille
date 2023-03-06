import gql from 'graphql-tag'

export const AUTOCOMPLETE = gql`
    query RecupererAiots($filter: String) {
        aiots(nom_Istartswith: $filter) {
            edges {
                node {
                    id, nom, libelle, code, commune {nom, abbv, departement {nom, region {nom}}}
                }
            }
        }
    }  
`;

import gql from 'graphql-tag'

export const RECUPERER_AIOT = gql`
    query RecupererAiot($id: ID!) {
        aiot(id: $id) {
            id, 
            nom,
            commune {
                nom, 
                codePostal
            },
            rubriquesIcpe {
                edges {
                    node {
                        rubrique {
                            code, libelle, regime
                        }
                    }
                }
            }
        }
    }  
`;

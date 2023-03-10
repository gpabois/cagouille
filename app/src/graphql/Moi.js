import gql from 'graphql-tag'

export const MES_TACHES = gql`
    query MesTaches {
        mesTaches {
            rvats {
                pageInfo {
                    hasNextPage,
                    endCursor
                },
                edges {
                    node {
                        id, 
                        step,
                        deadline,
                        rvat {
                            id, 
                            nom,
                            aiot {
                                libelle
                            }
                        }
                    }
                }
            }
        }
    }  
`;
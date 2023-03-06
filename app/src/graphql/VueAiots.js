import gql from 'graphql-tag'

export default gql`
query RecupererAiots(
        $cursor: String, 
        $orderBy: String,
        $nom_Icontains: String, 
        $code_Icontains: String,
        $rubriquesIcpe_Rubrique_Code_Icontains: String,
        $commune_Nom_Icontains: String) {
    aiots(
        orderBy: $orderBy,
        after: $cursor,
        code_Icontains: $code_Icontains,
        nom_Icontains: $nom_Icontains,
        rubriquesIcpe_Rubrique_Code_Icontains: $rubriquesIcpe_Rubrique_Code_Icontains,
        commune_Nom_Icontains: $commune_Nom_Icontains
    ) {
        edges {
            node {
                id, nom, code, commune {nom, abbv, departement {nom, region {nom}}},
                rubriquesIcpe {
                    edges {
                        node {
                        rubrique {
                            code, 
                            regime, 
                            description
                        }
                        }
                    }
                }
            }
        },
        pageInfo {
            endCursor
            hasNextPage
        }
    }
}  
`

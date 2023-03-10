import gql from 'graphql-tag'

export const CREER = gql`
mutation CreerRvat($input: CreateRvatInput!) {
    rvat {
        create(input: $input) {
            ok,
            errors {
              field, 
              messages
            }
        }
    }
}  
`;

export const SUPPRIMER = gql`
mutation SupprimerRvat($id: GlobalID!) {
    rvat {
        delete (id: $id) {
            ok
        }
    }
}
`;

export const RECUPERER_RVATS = gql`
query RecupererRvats($cursor: String) {
    rvats(after: $cursor) {
        edges {
            node {
                id,
                nom,
                uriTravail,
                uriDefinitif,
                reference,
                verifie,
                approuve,
                transmis,
                transmisLe,
                verificateur { name },
                approbateur { name },
                aiot {
                    id,
                    nom,
                    code
                    commune {
                        nom
                    }
                }
            }
        },
        pageInfo {
            endCursor,
            hasNextPage
        }
    }
}
`;

export const VERIFIER = gql`
    mutation Verifier($input: VerifierRvatInput!) {
        rvat {
            verifier(input: $input) {
                ok,
                errors {
                    field, 
                    messages
                }
            }
        }
    }  
`;

export const APPROUVER = gql`
    mutation Approuver($input: ApprouverRvatInput!) {
        rvat {
            approuver(input: $input) {
                ok,
                errors {
                    field, 
                    messages
                }
            }
        }
    }  
`;

export const TRANSMETTRE = gql`
    mutation Transmettre($input: TransmettreRvatInput!) {
        rvat {
            transmettre(input: $input) {
                ok,
                errors {
                    field, 
                    messages
                }
            }
        }
    }  
`;
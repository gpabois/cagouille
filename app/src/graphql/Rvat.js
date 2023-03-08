import gql from 'graphql-tag'

export const START = gql`
    mutation Start($input: startInput!) {
        rvat {
            start(input: $input) {
                ok,
                errors
            }
        }
    }  
`;

export const VERIFIER = gql`
    mutation Verifier($input: verifierInput!) {
        rvat {
            verifier(input: $input) {
                ok,
                errors
            }
        }
    }  
`;

export const APPROUVER = gql`
    mutation Approuver($input: approuverInput!) {
        rvat {
            approuver(input: $input) {
                ok,
                errors
            }
        }
    }  
`;

export const TRANSMETTRE = gql`
    mutation Transmettre($input: transmettreInput!) {
        rvat {
            transmettre(input: $input) {
                ok,
                errors
            }
        }
    }  
`;
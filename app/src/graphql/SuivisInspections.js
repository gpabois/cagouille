import gql from 'graphql-tag'

export const NOUVEAU = gql`
mutation AjouterSuiviInspection($input: AjouterSuiviInspectionInput!) {
    ajouterSuiviInspection(input: $input) {
        suiviInspection {
            id
        }
    }
}`;

export const TOUT_TYPES = gql`
query RecupererTypesInstructions {
    typesInspections {
        edges {
            node {
                id, nom
            }
        }
    }
}
`;
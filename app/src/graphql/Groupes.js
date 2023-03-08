import gql from 'graphql-tag'

export const AUTOCOMPLETE = gql`
    query AutocompleteGroups($filter: String) {
        groups(name_Icontains: $filter) {
            edges {
                node {
                    id, 
                    name
                }
            }
        }
    }  
`;
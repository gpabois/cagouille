import gql from 'graphql-tag'

export default gql`
query RecupererSuivisInspections($cursor: String, $orderBy: String) {
  suivisInspections(after: $cursor, orderBy: $orderBy) {
    edges {
      node {
        id, 
        nom, 
        statut { nom },
        type { nom },
        dateRapport, 
        dateInspection, 
        datePreparation, 
        datePublication, 
        datePrevisionnelle,
        aiot {
          id, nom, code
        }
      }
    },
    pageInfo {
        endCursor,
        hasNextPage
    }
  }
} 
`

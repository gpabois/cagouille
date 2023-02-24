import graphene
from aiots.schema import Query as AiotQuery
from aiots.schema import Mutation as AiotMutation
from suivis.schema import Query as SuivisQuery
from accounts.schema import Query as AccountsQuery

class Query(AiotQuery, SuivisQuery, AccountsQuery):
    pass

class Mutation(AiotMutation):
    pass

schema = graphene.Schema(query=Query, mutation=Mutation)

import graphene
from aiots.schema import Query as AiotQuery
from aiots.schema import Mutation as AiotMutation
from suivis.schema import Query as SuivisQuery
from suivis.schema import Mutation as SuivisMutation
from accounts.schema import Query as AccountsQuery

class Query(AiotQuery, SuivisQuery, AccountsQuery):
    pass

class Mutation(AiotMutation, SuivisMutation):
    pass

schema = graphene.Schema(query=Query, mutation=Mutation)

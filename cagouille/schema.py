import graphene
from aiots.schema import Query as AiotQuery
from accounts.schema import Query as AccountsQuery

class Query(AiotQuery, AccountsQuery):
    pass

schema = graphene.Schema(query=Query)

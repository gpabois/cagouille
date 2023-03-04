import graphene
from aiots.schema import Query as AiotQuery
from aiots.schema import Mutation as AiotMutation
from suivis.schema import Query as SuivisQuery
from suivis.schema import Mutation as SuivisMutation
from accounts.schema import Query as AccountsQuery
from workflow_engine.schema import Mutation as WorkflowMutation
from workflow_engine.schema import Query as WorkflowQuery

class Query(AiotQuery, SuivisQuery, AccountsQuery, WorkflowQuery):
    pass

class Mutation(AiotMutation, SuivisMutation, WorkflowMutation):
    pass

schema = graphene.Schema(query=Query, mutation=Mutation)

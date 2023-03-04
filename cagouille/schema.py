import graphene
from aiots.schema import Query as AiotQuery
from aiots.schema import Mutation as AiotMutation
from suivis.schema import Query as SuivisQuery
from suivis.schema import Mutation as SuivisMutation
from accounts.schema import Query as AccountsQuery
from workflow_engine.schema import Mutation as WorkflowEngineMutation
from workflow_engine.schema import Query as WorkflowEngineQuery
from workflows.schema import Mutation as WorkflowsMutation
from workflows.schema import Query as WorkflowsQuery

class Query(AiotQuery, SuivisQuery, AccountsQuery, WorkflowEngineQuery, WorkflowsQuery):
    pass

class Mutation(AiotMutation, SuivisMutation, WorkflowEngineMutation, WorkflowsMutation):
    pass

schema = graphene.Schema(query=Query, mutation=Mutation)

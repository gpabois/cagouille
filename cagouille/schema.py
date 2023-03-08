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

from graphene_django.debug import DjangoDebug

class Query(AiotQuery, SuivisQuery, AccountsQuery, WorkflowEngineQuery, WorkflowsQuery):
    debug = graphene.Field(DjangoDebug, name='debug')

class Mutation(AiotMutation, SuivisMutation, WorkflowEngineMutation, WorkflowsMutation):
    debug = graphene.Field(DjangoDebug, name='debug')

class ExceptionMiddleware(object):
    def resolve(self, next, root, info, **args):
        try:
            return next(root, info, **args)   
        except Exception as e:
            print(e)
            return None

schema = graphene.Schema(
    query=Query, 
    mutation=Mutation
)

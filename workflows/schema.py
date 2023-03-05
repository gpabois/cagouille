from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene import relay, ObjectType, ID, Field, Mutation
from graphene_plus import GlobalID

from workflow_engine.tasks import spawn_flow
from workflow_engine.schema import Process, Task, flow_mutation

from . import flows
from . import models

class Rvat(DjangoObjectType):
    class Meta:
        model = models.Rvat
        interfaces = (relay.Node,)
        filter_fields = ('id', 'nom', 'uri_travail')

class Query(ObjectType):
    rvats = DjangoFilterConnectionField(Rvat)

class Mutation(ObjectType):
    rvat = Field(
        flow_mutation(
            flows.Rvat, 
            Rvat
        )
    )
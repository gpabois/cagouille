from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene import relay, ObjectType, ID, Field, Mutation, Boolean
from graphene_plus import GlobalID
from graphql_relay import from_global_id

from workflow_engine.tasks import spawn_flow
from workflow_engine.schema import Process, Task, flow_mutation
from django.db import transaction
from . import flows
from . import models

class Rvat(DjangoObjectType):
    class Meta:
        model = models.Rvat
        interfaces = (relay.Node,)
        filter_fields = (
            'id', 
            'nom', 
            'reference',
            'uri_travail', 
            'uri_definitif', 
            'redacteur', 
            'verificateur', 
            'approbateur', 
            'aiot', 
            'verifie', 
            'approuve'
        )

class Query(ObjectType):
    rvats = DjangoFilterConnectionField(Rvat)

class SupprimerRvat(Mutation):
    ok = Boolean()

    class Meta:
        name = "DeleteRvat"
    
    class Arguments:
        id = GlobalID(required=True)
    
    @classmethod
    @transaction.atomic
    def mutate(cls, root, info, **input):
        rvat = models.Rvat.objects.get(pk=from_global_id(input.get('id'))[1])
        rvat.process.delete()
        rvat.delete()
        return cls(ok=True)

RvatMutations = flow_mutation(
    flows.Rvat, 
    Rvat,
    delete=SupprimerRvat.Field()
)

class Mutation(ObjectType):
    rvat = Field(RvatMutations)

    @staticmethod
    def resolve_rvat(root, info):
        return RvatMutations()


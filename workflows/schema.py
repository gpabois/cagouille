from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Scalar
from graphene_plus import GlobalID
from workflow_engine.tasks import spawn_flow
from workflow_engine.schema import Process, Task

from . import flows
from . import models

class Rvat(DjangoObjectType):
    class Meta:
        model = models.Rvat
        interfaces = (relay.Node,)
        filter_fields = ('id', 'nom', 'uri_travail')

class CreerRvat(Mutation):
    class Arguments:
        pass

    process = Field(Process)
    task = Field(Task)

    @classmethod
    def mutate(cls, root, info, id):
        process, task = spawn_flow(flows.Rvat, user=info.context.user)
        return cls(process=process, task=task)

class Query(ObjectType):
    rvats = DjangoFilterConnectionField(Rvat)

class Mutation(ObjectType):
    creer_rvat = CreerRvat.Field()
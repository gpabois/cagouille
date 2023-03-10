from graphene_django import DjangoObjectType, DjangoListField
from graphene_django.filter import DjangoFilterConnectionField
from graphene import relay, ObjectType, ID, Field, Mutation, Boolean, String
from graphene_plus import GlobalID
from graphql_relay import from_global_id, to_global_id

from django.db import transaction
from django.db.models import Q

from workflow_engine.tasks import spawn_flow
from workflow_engine.schema import Process, Task, flow_mutation
from workflow_engine import models as wfe_models

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

class TacheRvat(DjangoObjectType):
    class Meta:
        model = wfe_models.Task
        interfaces = (relay.Node,)
        filter_fields = '__all__'
    
    rvat = Field(Rvat)
    id = ID(required=True)

    def resolve_id(self, info):
        return to_global_id("Task", self.id)

    def resolve_rvat(self, info):
        return models.Rvat.objects.get(process=self.process)

class MesTaches(ObjectType):
    rvats = DjangoFilterConnectionField(TacheRvat)
    
    @staticmethod
    def resolve_rvats(parent, info):
        current_user = info.context.user
        return wfe_models.Task.objects.filter(
            Q(
                Q(assigned_to_group__user=current_user) | Q(assigned_to_user=current_user)
            )
            &
            Q(process__flow_class='rvat')
            &
            Q(status='stall')
        )

class Query(ObjectType):
    rvats = DjangoFilterConnectionField(Rvat)
    mes_taches = Field(MesTaches)

    @staticmethod
    def resolve_mes_taches(parent, info):
        return MesTaches()

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



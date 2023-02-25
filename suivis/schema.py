from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation
from graphql_relay.node.node import from_global_id
from . import models

class StatutSuivi(DjangoObjectType):
    class Meta:
        model = models.StatutSuivi
        interfaces = (relay.Node,)        
        filter_fields  = ('id', 'nom')

class TypeInspection(DjangoObjectType):
    class Meta:
        model = models.TypeInspection
        interfaces = (relay.Node,)    
        filter_fields  = ('id', 'nom')

class SuiviInspection(DjangoObjectType):
    class Meta:
        model = models.SuiviInspection
        interfaces = (relay.Node,)
        filter_fields  = ('id', 'nom')

class Query(ObjectType):
    status_suivis = DjangoFilterConnectionField(StatutSuivi)
    suivis_inspections = DjangoFilterConnectionField(SuiviInspection)
    types_inspections = DjangoFilterConnectionField(TypeInspection)
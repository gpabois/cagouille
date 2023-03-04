from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Scalar
from graphene_plus import GlobalID
from . import models

class Rvat(DjangoObjectType):
    class Meta:
        model = models.Rvat
        interfaces = (relay.Node,)
        fields = ('id', 'nom', 'uri_travail')

class Query(ObjectType):
    rvats = DjangoFilterConnectionField(Rvat)
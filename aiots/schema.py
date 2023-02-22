from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene import relay, ObjectType

from . import models

class Region(DjangoObjectType):
    class Meta:
        model = models.Region
        interfaces = (relay.Node,)
        filter_fields  = ('id', 'nom')

class Departement(DjangoObjectType):
    class Meta:
        model = models.Departement
        interfaces = (relay.Node, )
        filter_fields  = ('id', 'nom')

class Commune(DjangoObjectType):
    class Meta:
        model = models.Commune
        interfaces = (relay.Node, )
        filter_fields  = {
            'nom': ['exact', 'icontains', 'istartswith'],
            'code_postal': ['exact', 'icontains', 'istartswith'],
            'departement__nom': ['exact', 'icontains', 'istartswith']
        }

class Aiot(DjangoObjectType):
    class Meta:
        model = models.Aiot
        interfaces = (relay.Node, )
        filter_fields  = {
            'nom': ['exact', 'icontains', 'istartswith'],
            'code': ['exact', 'icontains', 'istartswith'],
            'commune__nom': ['exact', 'icontains', 'istartswith'],
            'commune__departement__nom': ['exact', 'icontains', 'istartswith'],
            'commune__departement__region__nom': ['exact', 'icontains', 'istartswith'],
            'rubriques_icpe__rubrique__code': ['exact', 'icontains', 'istartswith']
        }
        
class RubriqueIcpeAiot(DjangoObjectType):
    class Meta:
        model = models.RubriqueIcpeAiot
        interfaces = (relay.Node, )

class RubriqueIcpe(DjangoObjectType):
    class Meta:
        model = models.RubriqueIcpe
        interfaces = (relay.Node, )

class Query(ObjectType):
    regions = DjangoFilterConnectionField(Region)
    departements = DjangoFilterConnectionField(Departement)
    communes = DjangoFilterConnectionField(Commune)
    
    aiots = DjangoFilterConnectionField(Aiot)
    aiot = relay.Node.Field(Aiot)

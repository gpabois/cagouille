from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Scalar
from graphql_relay.node.node import from_global_id
from . import models

class GlobalID(Scalar):
    serialize = coerce_int
    parse_value = coerce_int

    @staticmethod
    def parse_literal(ast, _variables=None):
        if isinstance(ast, IntValueNode):
            return int(ast.value)
        return Undefined

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

class CreerAiot(relay.ClientIDMutation):
    class Input:
        nom = String(required=True)
        code = String(required=True)
        commune_global_id = String(name="commune", required=True)
    
    aiot = Field(Aiot)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        commune_id = from_global_id(input['commune_global_id'])
        input['commune'] = models.Commune.objects.get(id=commune_id.id)
        del input['commune_global_id']
        aiot = models.Aiot(**input)
        aiot.save()
        return cls(aiot=aiot)

class SupprimerAiot(Mutation):
    ok = Boolean()

    class Arguments:
        id = ID()
    
    @classmethod
    def mutate(cls, root, info, id):
        id = from_global_id(id).id
        aiot = models.Aiot.objects.get(pk=id)
        aiot.delete()
        return cls(ok=True)


class ModifierAiot(relay.ClientIDMutation):
    class Input:
        id = ID(required=True)
        commune_id = ID()
    
    aiot = Field(Aiot)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        aiot = models.Aiot.objects.get(pk=input.id)

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

class Mutation(ObjectType):
    modifier_aiot = ModifierAiot.Field()
    creer_aiot = CreerAiot.Field()
    supprimer_aiot = SupprimerAiot.Field()
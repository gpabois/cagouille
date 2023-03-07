import django_filters
import graphene
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Scalar
from graphene_plus import GlobalID
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

class FiltreAiot(django_filters.FilterSet):
    order_by = django_filters.OrderingFilter(
        fields=(
            'nom',
            'commune__nom'
        )
    )
    
    class Meta:
        model = models.Aiot
        fields = {
            'nom': ['exact', 'icontains', 'istartswith'],
            'code': ['exact', 'icontains', 'istartswith'],
            'commune__nom': ['exact', 'icontains', 'istartswith'],
            'rubriques_icpe__rubrique__code': ['exact', 'icontains', 'istartswith'],
        }

import suivis.schema
import suivis.models
class Aiot(DjangoObjectType):

    libelle = graphene.String()
    
    suivis_inspections = DjangoFilterConnectionField(
        suivis.schema.SuiviInspection,
        order_by=String(required=False)
    )

    class Meta:
        model = models.Aiot
        interfaces = (relay.Node, )
        filterset_class = FiltreAiot

    @classmethod
    def get_queryset(cls, queryset, info):
        return queryset.distinct()

    def resolve_libelle(self, info):
        return str(self)

    def resolve_suivis_inspections(self, info, **kwargs):
        print(kwargs)
        query = suivis.models.SuiviInspection.objects.filter(aiot_id=self.id)
        
        if "order_by" in kwargs:
            query.order_by(kwargs['order_by'])
        
        return query
class CreerAiot(relay.ClientIDMutation):
    class Input:
        nom = String(required=True)
        code = String(required=True)
        commune_id = GlobalID(name="commune", required=True)
    
    aiot = Field(Aiot)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        aiot = models.Aiot(**input)
        aiot.save()
        return cls(aiot=aiot)

class SupprimerAiot(Mutation):
    ok = Boolean()

    class Arguments:
        id = GlobalID()
    
    @classmethod
    def mutate(cls, root, info, id):
        aiot = models.Aiot.objects.get(pk=id)
        aiot.delete()
        return cls(ok=True)

class ModifierAiot(relay.ClientIDMutation):
    class Input:
        id = ID(required=True)
        commune_id = GlobalID(name="commune")
    
    aiot = Field(Aiot)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        aiot = models.Aiot.objects.get(pk=input.id)
        
        del input['id']
        
        for k, v in input.items():
            setattr(aiot, k, v)
        
        if input:
            aiot.save()
        
        return cls(aiot=aiot)
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
    
    aiots = DjangoFilterConnectionField(Aiot, max_limit=None)
    aiot = relay.Node.Field(Aiot)

class Mutation(ObjectType):
    modifier_aiot = ModifierAiot.Field()
    creer_aiot = CreerAiot.Field()
    supprimer_aiot = SupprimerAiot.Field()
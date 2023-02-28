from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation
from graphql_relay.node.node import from_global_id
import django_filters
from . import models
from aiots import models as aiots_models

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

class FiltreSuiviInspection(django_filters.FilterSet):
    aiot__in = django_filters.filters.ModelMultipleChoiceFilter(
        field_name='aiot',
        queryset=aiots_models.Aiot.objects.all()
    )

    order_by = django_filters.OrderingFilter(
        fields=(
            'nom',
            'statut',
            'type',
            'date_previsionnelle'
        )
    )
    class Meta:
        model = models.SuiviInspection
        fields = [
            'nom', 
            'aiot', 
            'statut', 
            'type', 
            'date_previsionnelle'
        ]

class SuiviInspection(DjangoObjectType):
    class Meta:
        model = models.SuiviInspection
        interfaces = (relay.Node,)
        filterset_class = FiltreSuiviInspection


class AjouterSuiviInspection(relay.ClientIDMutation):
    class Input:
        nom = String(required=True)
        code = String(required=True)
        commune_global_id = String(name="commune", required=True)
    
    suivi_inspection = Field(SuiviInspection)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        commune_id = from_global_id(input['commune_global_id'])
        input['commune'] = models.Commune.objects.get(id=commune_id.id)
        del input['commune_global_id']
        aiot = models.Aiot(**input)
        aiot.save()
        return cls(aiot=aiot)

class Query(ObjectType):
    status_suivis = DjangoFilterConnectionField(StatutSuivi)
    suivis_inspections = DjangoFilterConnectionField(SuiviInspection)
    types_inspections = DjangoFilterConnectionField(TypeInspection)

class Mutation(ObjectType):
    ajouter_suivi_inspection = AjouterSuiviInspection.Field()
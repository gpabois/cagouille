from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Date
from graphene_plus import GlobalID
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
        nom                 = String(required=True)
        aiot_id             = GlobalID(name="aiot", required=True)
        type_id             = GlobalID(name="type")
        statut_id           = GlobalID(name="statut")
        date_previsionnelle = Date()
        date_preparation    = Date()
        date_inspection     = Date()
        date_rapport        = Date()
    
    suivi_inspection = Field(SuiviInspection)

    @classmethod
    def mutate_and_get_payload(cls, root, info, **input):
        try:
            suivi_inspection = models.SuiviInspection(**input)
            suivi_inspection.save()
            return cls(suivi_inspection=suivi_inspection)
        except Exception as e:
            print(e)

class ModifierSuiviInspection(relay.ClientIDMutation):
    class Input:
        id          = GlobalID(required=True)
        nom         = String()
        aiot_id     = GlobalID(name="aiot")
        type_id     = GlobalID(name="type")
        statut_id   = GlobalID(name="statut")
        date_previsionnelle = Date()
        date_preparation    = Date()
        date_inspection     = Date()
        date_rapport        = Date()
        date_publication    = Date()
    
    suivi_inspection = Field(SuiviInspection)

    @classmethod
    def mutate_and_get_payload(cls, root, info, id, **input):
        suivi = models.SuiviInspection.objects.get(pk=id)
        for k, v in input.items():
            setattr(suivi, k, v)

        if input:
            suivi.save()
        
        return cls(suivi_inspection=suivi_inspection)


class SupprimerSuiviInspection(Mutation):
    ok = Boolean()

    class Arguments:
        id = GlobalID()
    
    @classmethod
    def mutate(cls, root, info, id):
        aiot = models.SuiviInspection.objects.get(pk=id)
        aiot.delete()
        return cls(ok=True)

class Query(ObjectType):
    status_suivis = DjangoFilterConnectionField(StatutSuivi)
    suivis_inspections = DjangoFilterConnectionField(SuiviInspection)
    types_inspections = DjangoFilterConnectionField(TypeInspection)

class Mutation(ObjectType):
    ajouter_suivi_inspection = AjouterSuiviInspection.Field()
    modifier_suivi_inspection = ModifierSuiviInspection.Field()
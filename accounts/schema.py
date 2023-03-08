from graphene import Field, ObjectType
import django_filters
from django.contrib.auth import get_user_model
from django.contrib.auth.models import Group
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene import relay

class User(DjangoObjectType):
    class Meta:
        model = get_user_model()

class FiltreGroup(django_filters.FilterSet):
    order_by = django_filters.OrderingFilter(
        fields=(
            'name',
        )
    )
    
    class Meta:
        model = Group
        fields = {
            'name': ['exact', 'icontains', 'istartswith']
        }

class GroupType(DjangoObjectType):
    class Meta:
        model = Group
        interfaces = (relay.Node,)
        filterset_class = FiltreGroup

class Query(ObjectType):
    moi = Field(User)
    groups = DjangoFilterConnectionField(GroupType)

    def resolve_moi(self, info):
        user = info.context.user
        
        if user.is_anonymous:
            raise Exception('Not logged in!')
        
        return user
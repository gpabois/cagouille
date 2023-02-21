from django.contrib.auth import get_user_model
import graphene
from graphene_django import DjangoObjectType

class User(DjangoObjectType):
    class Meta:
        model = get_user_model()


class Query(graphene.ObjectType):
    moi = graphene.Field(User)

    def resolve_moi(self, info):
        user = info.context.user
        if user.is_anonymous:
            raise Exception('Not logged in!')
        return user
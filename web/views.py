from django.shortcuts import render
from django.views.generic import TemplateView
from django.contrib.auth.mixins import LoginRequiredMixin
from graphene_django.views import GraphQLView

# Create your views here.
class IndexView(LoginRequiredMixin, TemplateView):
    template_name = "web/index.html"

class PrivateGraphQLView(LoginRequiredMixin, GraphQLView):
    pass
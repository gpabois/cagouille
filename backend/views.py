from rest_framework.renderers import JSONRenderer, TemplateHTMLRenderer
from rest_framework import viewsets

from django.shortcuts import render, reverse
from django_filters.views import FilterView
from django_tables2.views import SingleTableMixin
from django.views.generic import TemplateView
from django.shortcuts import get_object_or_404

from . import models, forms, signals, tables, serializers

class HomeView(TemplateView):
    template_name = "cagouille/home.html"

class InspectionTrackerViewSet(viewsets.ModelViewSet):
    queryset = models.InspectionTracker.objects.all()
    serializer_class = serializers.InspectionTrackerSerializer

class AiotViewSet(viewsets.ModelViewSet):
    queryset = models.AIOT.objects.all()
    serializer_class = serializers.AiotSerializer


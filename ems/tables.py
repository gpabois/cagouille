import django_tables2 as tables
from . import models 

class DocumentTable(tables.Table):
    class Meta:
        model = models.Document

class InspectionTrackerTable(tables.Table):
    class Meta:
        model = models.TrackerInspection
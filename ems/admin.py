from django.contrib import admin
from . import models
# Register your models here.

admin.site.register(models.Region)
admin.site.register(models.Departement)
admin.site.register(models.Commune)

admin.site.register(models.AIOT)
admin.site.register(models.TrackerStatus)
admin.site.register(models.TrackerInstruction)
admin.site.register(models.TrackerInspection)
admin.site.register(models.TrackerInspectionType)
from django.contrib import admin
from . import models
# Register your models here.
admin.site.register(models.Region)
admin.site.register(models.Departement)
admin.site.register(models.Commune)

admin.site.register(models.AIOT)

# Trackers
admin.site.register(models.TrackerStatus)
admin.site.register(models.InstructionTracker)
admin.site.register(models.InstructionTrackerType)
admin.site.register(models.InspectionTracker)
admin.site.register(models.InspectionTrackerType)
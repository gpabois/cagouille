from django.contrib import admin
from . import models

# Register your models here.
admin.site.register(models.StatutSuivi)
admin.site.register(models.SuiviInspection)
admin.site.register(models.TypeInspection)
admin.site.register(models.SuiviInstruction)
admin.site.register(models.TypeInstruction)

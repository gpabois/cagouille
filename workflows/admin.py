from django.contrib import admin
from workflow_engine.tasks import spawn_flow

from . import models
from . import flows 
# Register your models here.


@admin.action(description='Créer un RVAT')
def creer_rvat(modeladmin, request, queryset):
    process, task = spawn_flow(flows.Rvat, user=request.user)

class RvatAdmin(admin.ModelAdmin):
    actions = [creer_rvat]

admin.site.register(models.Rvat, RvatAdmin)
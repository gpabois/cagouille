from django.contrib import admin
from . import models

# Register your models here.
admin.site.register(models.Region)
admin.site.register(models.Departement)
admin.site.register(models.Commune)
admin.site.register(models.Aiot)
admin.site.register(models.RubriqueIcpeAiot)
admin.site.register(models.RubriqueIcpe)
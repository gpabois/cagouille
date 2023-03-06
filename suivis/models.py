from django.db import models
from django.contrib.auth.models import Group, User
import mptt
from aiots import models as aiots_models
from mptt.models import MPTTModel, TreeForeignKey
from polymorphic.models import PolymorphicModel

# Tracker
class StatutSuivi(models.Model):
    nom = models.CharField(max_length=255)

    def __str__(self):
        return self.nom

class BaseSuivi(PolymorphicModel):
    nom         = models.CharField(max_length=255)
    statut      = models.ForeignKey(StatutSuivi, on_delete=models.SET_NULL, null=True, blank=True)
    aiot        = models.ForeignKey(aiots_models.Aiot, on_delete=models.CASCADE, null=True)
    
    responsable = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    entite_responsable = models.ForeignKey(Group, on_delete=models.SET_NULL, null=True)

    def __str__(self):
        return "{} - {}".format(self.nom, self.aiot)

class TypeInspection(models.Model):
    nom = models.CharField(max_length=255)

    def __str__(self):
        return self.nom

class SuiviInspection(BaseSuivi):
    type = models.ForeignKey(TypeInspection, on_delete=models.SET_NULL, null=True)
    date_previsionnelle = models.DateTimeField(null=True)
    date_preparation = models.DateTimeField(null=True)
    date_inspection = models.DateTimeField(null=True)
    date_rapport = models.DateTimeField(null=True)
    date_publication = models.DateTimeField(null=True)

class TypeInstruction(models.Model):
    nom = models.CharField(max_length=255)

    def __str__(self):
        return self.nom

class SuiviInstruction(BaseSuivi):
    type = models.ForeignKey(TypeInstruction, on_delete=models.SET_NULL, null=True)
    
class BaseLogSuivi(PolymorphicModel):
    suivi = models.ForeignKey(BaseSuivi, on_delete=models.CASCADE, null=False)
    created_at = models.DateTimeField(auto_now_add=True)

class FluxActiviteSuivi(BaseLogSuivi):
    acteur = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    verbe = models.CharField(max_length=255)
    cible = models.CharField(max_length=255)

class MessageSuivi(BaseLogSuivi):
    user = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    message = models.TextField()

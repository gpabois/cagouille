from django.db import models
from workflow_engine.models import WorkflowContext
from django.contrib.auth.models import User, Group
from aiots.models import Aiot
# Create your models here.

class Rvat(WorkflowContext):
    nom = models.CharField(max_length=255)
    rvat_au_regional = models.BooleanField(default=False)
    
    verifie = models.BooleanField(default=False)
    date_limite_verification = models.DateField(null=True)
    commentaire_verificateur = models.TextField(null=True)
    
    approuve = models.BooleanField(default=False)
    date_limite_approbation = models.DateField(null=True)
    commentaire_approbateur = models.TextField()
    
    approuve_regional = models.BooleanField(default=False)
    date_limite_approbation_regional = models.DateField(null=True)
    commentaire_regional = models.TextField()

    redacteur = models.ForeignKey(User, on_delete=models.SET_NULL, null=True, related_name="rvats")
    verificateur = models.ForeignKey(Group, on_delete=models.SET_NULL, null=True, related_name="rvats_verifies")
    approbateur = models.ForeignKey(Group, on_delete=models.SET_NULL, null=True, related_name="rvats_approuves")

    administratif = models.ForeignKey(Group, on_delete=models.SET_NULL, null=True, related_name="rvats_administres")

    aiot = models.ForeignKey(Aiot, null=True, on_delete=models.SET_NULL, related_name="rvats")
    
    uri_travail = models.CharField(max_length=255)
    uri_travail_regional = models.CharField(max_length=255)
    uri_definitif = models.CharField(max_length=255)
    
    reference_regional = models.CharField(max_length=255, null=True)
    reference = models.CharField(max_length=255)
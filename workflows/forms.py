from django import models, forms
from .models import Rvat

class FormulairePreparationRvat(forms.ModelForm):
   class Meta:
        model = Rvat
        fields = (
            'verificateur',
            'redacteur',
            'approbateur',
            'administratif',
            'aiot',
            'uri_travail',
            'date_limite_verification',
            'date_limite_approbation',
            'date_limite_approbation_regional',
            'rvat_au_niveau_regional',
        )  

class FormulaireVerificateurRvat(forms.ModelForm):
   class Meta:
        model = Rvat
        fields = ('verifie', 'commentaire_verificateur')

class FormulaireApprobateurRvat(forms.ModelForm):
   class Meta:
        model = Rvat
        fields = ('approuve', 'commentaire_approbateur')

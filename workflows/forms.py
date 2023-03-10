from django import forms
from .models import Rvat

class FormulairePreparationRvat(forms.ModelForm):
   class Meta:
        model = Rvat
        fields = (
            'nom',
            'aiot',
            
            'verificateur',
            'approbateur',
            'administratif',

            'uri_travail',
            'date_limite_verification',
            'date_limite_approbation'
        )  

class FormulaireVerificateurRvat(forms.ModelForm):
   commentaire_verificateur = forms.CharField(required=False)
   class Meta:
        model = Rvat
        fields = ('verifie', 'commentaire_verificateur')

class FormulaireApprobateurRvat(forms.ModelForm):
   commentaire_approbateur = forms.CharField(required=False)
   class Meta:
        model = Rvat
        fields = ('approuve', 'commentaire_approbateur')

class FormulaireTransmettre(forms.ModelForm):
   class Meta:
        model = Rvat
        fields = ('reference', 'uri_definitif')

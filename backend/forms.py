from django import forms
from django.db import transaction

from . import models

class UploadNewDocumentForm(forms.Form):
    obj = forms.FileField()

    def save(self):
        with transaction.atomic():
            doc = models.Document()
            version = models.DocumentVersion(document=doc, version=1)
            version.obj = self.cleaned_data['obj']
            doc.name = version.oname

            doc.save()
            version.save()
            return doc


class NewCabinetForm(forms.ModelForm):
    class Meta:
        model = models.CabinetNode
        fields = ['nom', 'parent']


class NewInspectionTrackerForm(forms.ModelForm):
    class Meta:
        model = models.InspectionTracker
        fields = ['nom', 'aiot', 'status', 'responsable', 'entite_responsable', 'type']




        


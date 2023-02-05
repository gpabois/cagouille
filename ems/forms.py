from django import forms
from django.db import transaction

from .models import Document, DocumentVersion

class UploadNewDocumentForm(forms.Form):
    obj = forms.FileField()

    def save(self):
        with transaction.atomic():
            doc = Document()
            version = DocumentVersion(document=doc, version=1)
            version.obj = self.cleaned_data['obj']
            doc.name = version.oname

            doc.save()
            version.save()
            return doc
            

        


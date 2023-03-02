from django import models, forms
from .models import Task
from .engine import ENGINE

class ContextForm(forms.ModelForm):
    task = models.ModelChoiceField(queryset=Task.objects.all())
    
    def _get_context(self):
        return self.meta.model.objects.get(process=self.cleaned_data['task'].process)

    def save(self, *args, **kwargs):
        self.pk = self._get_context().id
        return super().save(*args, **kwargs)
        
        
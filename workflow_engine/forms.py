from django.db import models
from django import forms
from .models import Task
from .engine import ENGINE

class ContextForm(forms.ModelForm):
    task = forms.ModelChoiceField(queryset=Task.objects.all())
    
    def get_context(self):
        task = self.cleaned_data['task']
        return self.Meta.model.objects.get(process=task.process)

    def clean(self, *args, **kwargs):
        self.instance = self.get_context()
        return super().clean(*args, **kwargs)
        
        
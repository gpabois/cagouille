from workflow_engine.forms import ContextForm
from workflow_engine_tests import models

class SimpleForm(ContextForm):
    class Meta:
        model = models.SimpleContext
        fields = ['approval_decision']

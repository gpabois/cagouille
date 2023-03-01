from django.db import models
from workflow_engine.models import WorkflowContext

class SimpleContext(WorkflowContext):
    approval_decision = models.BooleanField(default=False)
    approved = models.BooleanField(default=False)
    
    class Meta:
        managed = True
        app_label = 'workflow_engine_tests'
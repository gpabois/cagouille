from django.db import models
from workflow_engine import WorkflowContext
# Create your models here.

class ContexteRvat(WorkflowContext):
    verifie         = models.BooleanField(default=False)
    approuve        = models.BooleanField(default=False)
    approuve_spr    = models.BooleanField(default=False)

    redacteur       = models.ForeignKey(User)
    verificateur    = models.ForeignKey(Group, on_delete=models.CASCADE)
    approbateur     = models.ForeignKey(Group, on_delete=models.CASCADE)
    administratif   = models.ForeignKey(Group, on_delete=models.CASCADE)

    aiot = models.ForeignKey(aiots_models.Aiot, null=True, related_name="rvats")
from .engine import                 WorkflowContext
from django.db                      import models
from django.contrib.auth.models     import Group, User
from aiots                          import models as aiots_models

class Process:
    flow_class = models.CharField(max_length=255)
    status = models.CharField(choices=(
        'init',
        'running',
        'aborted',
        'failed'
        'done'
    ))

class Task:
    process = models.ForeignKey(Process)
    step = models.CharField(max_length=255)
    status = models.CharField(choices=(
        'init',
        'idling',
        'aborted',
        'failed',
        'done'
    ))
    log = models.TextField()

class ContexteRvat(WorkflowContext):
    verifie         = models.BooleanField(default=False)
    approuve        = models.BooleanField(default=False)
    approuve_spr    = models.BooleanField(default=False)

    redacteur       = models.ForeignKey(User)
    verificateur    = models.ForeignKey(Group, on_delete=models.CASCADE)
    approbateur     = models.ForeignKey(Group, on_delete=models.CASCADE)
    administratif   = models.ForeignKey(Group, on_delete=models.CASCADE)

    aiot = models.ForeignKey(aiots_models.Aiot, null=True, related_name="rvats")

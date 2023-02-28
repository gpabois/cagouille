from django.db                      import models
from django.contrib.auth.models     import Group, User

class Process(models.Model):
    flow_class = models.CharField(max_length=255)
    status = models.CharField(choices=(
        'init',
        'running',
        'aborted',
        'failed'
        'done'
    ))

class Task(models.Model):
    process = models.ForeignKey(Process, on_delete=models.CASCADE)
    step    = models.CharField(max_length=255)
    status  = models.CharField(choices=(
        'init',
        'idling',
        'aborted',
        'failed',
        'done'
    ))
    log = models.TextField()

class WorkflowContext(models.Model):
    process = models.ForeignKey(Process, on_delete=models.CASCADE)
    class Meta:
        abstract = True


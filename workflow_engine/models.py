from django.db import models
from django.contrib.auth.models import User

class Process(models.Model):
    flow_class = models.CharField(max_length=255)
    status = models.CharField(max_length=20, default='init', choices=(
        ('init', 'Initialised'),
        ('running', 'Running'),
        ('aborted', 'Aborted'),
        ('failed', 'Failed'),
        ('done', 'Done')
    ))

    def __str__(self):
        return "{}({})".format(self.flow_class, self.pk)

    def aborted(self):
        self.status = 'aborted'

    def failed(self, error):
        self.status = 'failed'
        self.log = str(error)

    def done(self):
        self.status = 'done'

class Task(models.Model):
    assigned_to = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    process = models.ForeignKey(Process, on_delete=models.CASCADE)
    step    = models.CharField(max_length=255)
    status  = models.CharField(max_length=20, default='init', choices=(
        ('init', 'Initalised'),
        ('ready', 'Ready'),
        ('stall', 'Stall'),
        ('aborted', 'Aborted'),
        ('failed', 'Failed'),
        ('done', 'Done'),
        ('closed', 'Closed')
    ))

    def __str__(self):
        return "{}::{}({}) [{}]".format(str(self.process), self.step, self.pk, self.status)

    log = models.TextField()
    def ready(self):
        self.status = 'ready'

    def closed(self):
        self.closed = 'closed'

    def aborted(self):
        self.status = 'aborted'

    def failed(self, error):
        self.status = 'failed'
        self.log = str(error)

    def done(self):
        self.status = 'done'

class WorkflowContext(models.Model):
    process = models.ForeignKey(Process, on_delete=models.CASCADE)
    class Meta:
        abstract = True


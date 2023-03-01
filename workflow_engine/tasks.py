from .engine import ENGINE
from .models import Task
from celery import shared_task

@shared_task
def activate(task_id, **input):
    task = Task.objects.get(pk=task_id)
    return ENGINE.activate(task, **input)

def spawn_flow(flow, context):
    return ENGINE.spawn_process(flow, context)
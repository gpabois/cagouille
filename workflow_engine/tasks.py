from .engine import ENGINE
from .models import Task
from celery import shared_task
from celery.utils.log import get_task_logger

logger = get_task_logger(__name__)

@shared_task
def activate(task_id, **options):
    logger.info("Activating task {}".format(str(task_id)))
    task = Task.objects.get(id=int(task_id))
    return ENGINE.activate(task, **options)

def submit(task, data, **options):
    return ENGINE.submit(task, data, **options)

def spawn_flow(flow, context, **kwargs):
    return ENGINE.spawn_flow(flow, **kwargs)
from collections import OrderedDict
from django import test
from django.apps import apps
from django.conf import settings
from django.core.management import call_command
from django.db import connection
from itertools import chain

from workflow_engine.tasks import spawn_flow, activate
from workflow_engine.status import STALL, DONE, FAILED

def wait_for_task_status(task, activation_job, status):
    """
        Wait until a task has reached a specific status.
    """

    activation = activation_job.get()
    
    if activation.task.status == status:
        return activation.task
    else:
        return wait_for_task_status(*activation.get_next_by_task(task), status)

def wait_for_end_or_stall(activation):
    if activation.task.process.status == DONE:
        return []
    elif activation.task.status in (STALL, FAILED):
        return [activation.task]
    else:
        return list(chain(*[wait_for_end_or_stall(activation_job.get()) for _, activation_job in activation]))

# Create your tests here.
class TestCase(test.TestCase):
    def setUp(self):
        self.modify_settings(INSTALLED_APPS={"append", "workflow_engine_tests"})
        self.modify_settings(CELERY_TASK_ALWAYS_EAGER=True)

class SimpleTestCase(TestCase):
    @test.override_settings(CELERY_TASK_ALWAYS_EAGER=True)
    def test_no_error(self):
        from .flows import SimpleFlow
        from .models import SimpleContext

        # Test branching then

        # Spawn a new workflow
        process, start, activation_job = spawn_flow(SimpleFlow,  SimpleContext())

        # Run until a task is stalling (requiring an user action)
        [user_action] = wait_for_end_or_stall(activation_job.get())
        assert user_action.status == STALL

        # Run until we reach the end
        result = wait_for_end_or_stall(activate(user_action.id, approval=True))
        assert result == [], str(result)

    @test.override_settings(CELERY_TASK_ALWAYS_EAGER=True)
    def test_branching(self):
        from .flows import SimpleFlow
        from .models import SimpleContext

        # Test branching then

        # Spawn a new workflow
        process, start, activation_job = spawn_flow(SimpleFlow,  SimpleContext())

        # Run until a task is stalling (requiring an user action)
        [user_action] = wait_for_end_or_stall(activation_job.get())
        assert user_action.status == STALL

        # Run until we reach the end
        result = wait_for_end_or_stall(activate(user_action.id, approval=True))
        assert result == [], str(result)
        
        context = SimpleContext.objects.get(process=process)
        assert context.approved

        # Spawn a new workflow
        process, start, activation_job = spawn_flow(SimpleFlow,  SimpleContext())

        # Test branching else

        # Run until a task is stalling (requiring an user action)
        [user_action] = wait_for_end_or_stall(activation_job.get())
        assert user_action.status == STALL

        # Run until we reach the end
        result = wait_for_end_or_stall(activate(user_action.id, approval=False))
        assert result == [], str(result)
        
        context = SimpleContext.objects.get(process=process)
        assert not context.approved
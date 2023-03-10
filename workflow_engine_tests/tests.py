from itertools import chain

from django import test

from workflow_engine.models import Process
from workflow_engine.tasks import spawn_flow, activate, submit
from workflow_engine.status import STALL, DONE, FAILED, SUBMITTED, CLOSED

from celery.result import AsyncResult
from celery.contrib.testing.worker import start_worker
from celery.contrib.testing.app import TestApp

from cagouille.celery import app

def wait_for_end_or_stall(task):
    task.wait(timeout=3.0, propagate=True)

    if task.process.status == DONE:
        return []
    
    elif task.status == CLOSED:
        return []

    elif task.status in (STALL, FAILED):
        return [task]
    
    else:
        following_tasks = list(chain(*[wait_for_end_or_stall(task) for task in list(task.followings.all()) + [task]]))
        return following_tasks

# Create your tests here.
class SimpleTestCase(test.SimpleTestCase):
    databases = '__all__'
    
    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        TestApp()
        cls.celery_worker = start_worker(app, perform_ping_check=False)
        cls.celery_worker.__enter__()

    @classmethod
    def tearDownClass(cls):
        super().tearDownClass()
        cls.celery_worker.__exit__(None, None, None)

    def test_no_error_eager(self):
        from .flows import SimpleFlow
        from .models import SimpleContext

        # Test branching then

        # Spawn a new workflow
        process, start = spawn_flow(SimpleFlow,  SimpleContext(), eager=True)

        # Run until a task is stalling (requiring an user action)
        [user_action] = wait_for_end_or_stall(start)
        assert user_action.status == STALL

        # Submit data
        result = submit(user_action, {'approval_decision': True}, eager=True)

        assert isinstance(result, SimpleContext), result.errors
        assert result.approval_decision == True, result.approval_decision
        assert user_action.status == SUBMITTED, user_action.status
        
        context = process.get_context()
        assert context.approved, context.approved

        process = Process.objects.get(pk=process.id)
        assert process.status == DONE, process.status

    def test_no_error(self):
        from .flows import SimpleFlow
        from .models import SimpleContext

        # Test branching then

        # Spawn a new workflow
        process, start = spawn_flow(SimpleFlow,  SimpleContext())

        # Run until a task is stalling (requiring an user action)
        [user_action] = wait_for_end_or_stall(start)
        assert user_action.status == STALL

        # Submit data
        result = submit(user_action, {'approval_decision': True})

        assert isinstance(result, SimpleContext), result.errors
        assert result.approval_decision == True, result.approval_decision
        assert user_action.status == SUBMITTED, user_action.status
        
        wait_for_end_or_stall(user_action)

        context = process.get_context()
        assert context.approved, context.approved

        process.refresh_from_db()
        assert process.status == DONE, process.status
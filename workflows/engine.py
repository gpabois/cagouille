from typing import Type
from .tasks import BaseTaskType
from .models import Task, Process

def workflow(cls):
    ENGINE.register(cls)
    return cls

class WorkflowMeta:   
    def __setattr__(self, name, value):
        if issubclass(value, BaseTaskType):
            self.steps[name] = value
        setattr(self, name, value)

class Workflow(metaclass=WorkflowMetaclass):
    __metaclass__ = WorkflowMeta
    
    context_class = None
    steps = {}

    def __init__(self):
        pass

    def get_step(self, step):
        pass

    def get_context(self, process):
        return self.context_class.objects.get(process=process)

class Engine:
    def __init__(self):
        self.flows = {}

    def register(self, cls):
        self.flows[cls.__name__] = cls

    def get_flow(self, flow_class):
        return self.flows(flow_class)
    
    def spawn_task(self, flow, process, step_name, **input):
        pass

    def run(self, task: models.Task, **input):
        flow = self.get_flow(task.process.flow_class)
        context = flow.get_context(taks.process)
        step = flow.get_step(task.step)
        
        try:
            result = flow(step)
            task.statut = 'done'
        except Exception as e:
            task.statut = 'failed'
            task.log = str(e)
        finally:
            task.save()

class WorkflowContext:
    process = models.ForeignKey(models.Process)

ENGINE = Engine()
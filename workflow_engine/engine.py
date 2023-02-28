from typing  import Type
from .models import Task, Process

class Engine:
    def __init__(self):
        self.flows = {}

    def register(self, cls):
        self.flows[cls.__name__] = cls

    def get_flow(self, flow_class):
        return self.flows(flow_class)
    
    def spawn_task(self, flow, process, step_name, **input):
        pass

    def run(self, task: Task, **input):
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

ENGINE = Engine()
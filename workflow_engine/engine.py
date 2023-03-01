from typing  import Type
from django.db import transaction
from .models import Task, Process

class Engine:
    def __init__(self):
        self.flows = {}

    def register(self, cls):
        self.flows[cls.__name__] = cls

    def flow(self, flow_class):
        return self.flows[flow_class]
    
    @transaction.atomic
    def spawn_process(self, flow, context):
        if flow.__name__ not in self.flows:
            self.register(flow)

        flow_class = flow.__name__

        process = Process(flow_class=flow_class)
        process.save()
        context.process = process
        context.save()
        return process, *self.spawn_task('start', process)
    
    def spawn_task(self, step, process):
        from .tasks import activate
        flow = self.flow(process.flow_class)
        node = flow.node(step)
        task = Task(process=process, step=step)
        task.save()
        return task, activate.delay(task.id)

    def activate(self, task: Task, **input):       
        try:
            flow = self.flow(task.process.flow_class)
            context = flow.context(task.process)
            node = flow.node(task.step)
           
            return node(context=context, engine=self, task=task, process=task.process, **input)

        except Exception as e:
            task.failed(e)
            task.process.failed(e)
            raise e
        
        finally:
            task.save()
            task.process.save()

ENGINE = Engine()
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

    def context(self, process):
        flow = self.flow(process.flow_class)
        return flow.context(process)        
    
    def spawn_process(self, flow, context, **options):
        if flow.__name__ not in self.flows:
            self.register(flow)

        flow_class = flow.__name__

        process = Process(flow_class=flow_class)
        process.save()
        
        context.process = process
        context.save()

        return process, self.spawn_task('start', process, **options)
    
    def spawn_task(self, step, process, previous=None, **options):
        from .tasks import activate
        
        flow = self.flow(process.flow_class)
        node = flow.node(step)
        
        task = Task(process=process, step=step)
        task.previous = previous
        task.save()
        
        if "eager" in options:
            activate(task.id, **options)
        
        else:
            job = activate.delay(task.id, **options)
            task.current_job = job.task_id
            task.save()
            job.forget()
            
        return task

    def submit(self, task, data, **options):
        from .nodes import node_activation
        
        try:
            flow    = self.flow(task.process.flow_class)
            context = flow.context(task.process)
            node    = flow.node(task.step)
            data['task'] = task

            with node_activation(task, self, **options) as activation:
                return node.submit(
                    activation=activation,
                    data=data
                )

        except Exception as e:
            task.failed(e)
            task.process.failed(e)
            task.save()
            task.process.save()     
            raise e

    def activate(self, task: Task, **options):       
        from .nodes import node_activation
        try:
            flow    = self.flow(task.process.flow_class)
            context = flow.context(task.process)
            node    = flow.node(task.step)
            
            with node_activation(task, self, **options) as activation:
                node(
                    context=context, 
                    activation=activation,
                    **options
                )

        except Exception as e:
            task.failed(e)
            task.process.failed(e)
            raise e
        
        finally:
            task.save()
            task.process.save()

ENGINE = Engine()
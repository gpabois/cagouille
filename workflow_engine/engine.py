from typing  import Type
from django.db import transaction
from .models import Task, Process

class SpawnFlowResult:
    def __init__(self):
        self.errors = None
        self.context = None

    @property
    def is_ok(self):
        return not self.errors
    
    def set_errors(self, errors):
        self.errors = errors

    def set_context(self, context):
        self.context = context

    def set_process(self, process):
        self.process = process
    
    def set_task(self, task):
        self.task = task

class Engine:
    def __init__(self):
        self.flows = {}

    def register(self, cls):
        self.flows[cls.get_name()] = cls

    def flow(self, flow):
        if isinstance(flow, str):
            return self.flows[flow]
        else:
            if flow.get_name() not in self.flows:
                self.register(flow)   
            
            return self.flow(flow.get_name())         

    def context(self, process):
        flow = self.flow(process.flow_class)
        return flow.context(process)        
    
    def spawn_flow(self, flow, **kwargs):
        result = SpawnFlowResult()
        flow = self.flow(flow)

        process = Process(flow_class=flow.get_name())

        if "user" in kwargs:
            process.created_by = kwargs['user']
            
        process.save()
        result.set_process(process)

        flow.context_factory(context_class=flow.context_class, result=result, process=process, **kwargs)
        
        if result.is_ok:
            self.spawn_task("start", process, result=result, **kwargs)
        
        return result
            
    def spawn_task(self, step, process, previous=None, **kwargs):
        from .tasks import activate
        
        flow = self.flow(process.flow_class)
        node = flow.node(step)
        
        task = Task(process=process, step=step)
        
        if "user" in kwargs:
            task.assigned_to_user = kwargs['user']
            del kwargs['user']
            
        task.previous = previous
        task.save()
        
        if "eager" in kwargs:
            activate(task.id, eager=True)
        
        else:
            def on_commit():
                job = activate.delay(task.id, eager=False)
                task.current_job = job.task_id
                task.save()
                job.forget()

            transaction.on_commit(on_commit)
        
        if "result" in kwargs:
            kwargs['result'].set_task(task)

        return task

    def submit(self, task, data, **options):
        from .nodes import node_activation
        
        try:
            flow    = self.flow(task.process.flow_class)
            context = flow.context(task.process)
            node    = flow.node(task.step)

            if "user" in options:
                task.done_by = options['user']
                del options['user']

            with node_activation(task, self, **options) as activation:
                return node.submit(
                    activation=activation,
                    context=context,
                    data=data
                )

        except Exception as e:
            task.failed(e)
            task.save()
            task.process.failed(e)
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
            task.save()
            task.process.save()
            raise e

ENGINE = Engine()
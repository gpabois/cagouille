from .tasks import activate
from .status import READY, INIT, DONE, CLOSED, STALL, FAILED, ABORTED, SUBMITTED
from . import signals
from contextlib import contextmanager
from django.db import transaction

@contextmanager
def node_activation(task, engine, **options):
    activation = NodeActivation(task, engine, **options)

    try:
        yield activation
    
    except Exception as e:
        activation.failed(e)
        raise e
    
    finally:
        activation.commit()

class NodeActivation:
    def __init__(self, task, engine, **options):
        self.engine = engine
        self.task = task
        self.act_result = None
        self._reactivate = False
        self._tasks = []
        self.options = options

    def __iter__(self):
        return iter(self.nexts)

    def commit(self):
        self.task.save()
        self.task.process.save()

        if self.task.status in (READY, SUBMITTED):
            if "eager" in self.options:
                activate(
                    self.task.id, 
                    **self.options
                )
            else:
                def on_commit():
                    job = activate.delay(
                        self.task.id, 
                        **self.options
                    ) 
                    self.task.current_job = job.task_id
                    self.task.save()
                    job.forget()
                
                transaction.on_commit(on_commit)

        for spawn in self._tasks:
            spawn()

    def spawn_task(self, step):
        self._tasks.append(
            lambda: self.engine.spawn_task(
                step, 
                self.task.process, 
                previous=self.task,
                **self.options
            )
        )

    def close_workflow(self):
        self.task.done()
        self.task.process.done()

    def can_be_activated(self):
        return self.task.status in (READY, STALL, SUBMITTED)

    def is_entering(self):
        return self.task.status == INIT

    def is_leaving(self):
        return self.task.status == DONE

    def is_running(self):
        return self.task.status == READY
    
    def submitted(self):
        self.task.status = SUBMITTED

    def done(self):
        self.task.status = DONE
    
    def aborted(self):
        self.task.status = ABORTED
        self.process.status = ABORTED
    
    def failed(self, error):
        self.task.process.status = FAILED
        self.task.status = FAILED
        self.task.log = str(error)
    
    def ready(self):
        self.task.status = READY
    
    def stall(self):
        self.task.status = STALL
    
    def close(self):
        self.task.status = CLOSED

class BaseNode:
    def __init__(self, **options):
        if 'enter' in options:
            self.enter = options['enter']
        else:
            self.enter = None
        
        if 'leave' in options:
            self.leave = options['leave']
        else:
            self.leave = None

    def on_paired_with_flow(self, flow_class):
        pass

    def __call__(self, activation, **input):
        if activation.is_entering():
            self.on_entering(activation, **input)
            activation.ready()          

        if activation.can_be_activated():
            self.activate(
                activation=activation, 
                **input
            )

        if activation.is_leaving():
            self.on_leaving(activation, **input)
            activation.close()

        return activation

    def on_entering(self, activation, **input):
        signals.entering_task.send(sender=self.__class__, task=activation.task)
        
        if self.enter:
            self.enter(activation, **input)

    def on_leaving(self, activation, **input):
        signals.leaving_task.send(sender=self.__class__, task=activation.task)
        if self.leave:
            self.leave(activation, **input)

class If(BaseNode):
    def __init__(self, predicate, sthen, selse, **options):
        super().__init__(**options)
        self.predicate = predicate
        self.sthen = sthen
        self.selse = selse

    def activate(self, activation, **input):
        if self.predicate(**input):
            activation.spawn_task(self.sthen)
        else:
            activation.spawn_task(self.selse)

        activation.done()

class Branch(BaseNode):
    def __init__(self, default, **branches):
        super().__init__(**branches)
        self.default = default
        self.branches = branches
    
    def activate(self, activation, **input):
        for branch, predicate in self.branches.items():
            if predicate(activation, **input):
                activation.spawn_task(branch)
                activation.done()
                return
        
        activation.spawn_task(self.default)
        activation.done()

class Job(BaseNode):
    def __init__(self, fn, next, **options):
        super().__init__(**options)
        self.fn = fn
        self.next = next
    
    def activate(self, activation, **input):
        self.fn(**input)
        activation.spawn_task(self.next)
        activation.done()

class UserAction(BaseNode):
    def __init__(self, form_class, next, **options):
        super().__init__(**options)
        self.form_class = form_class
        self.next = next
        self.plugins = [] if "plugins" not in options else options['plugins']  

    def on_paired_with_flow(self, flow_class):
        self.flow_class = flow_class

        for plugin in self.plugins:
            plugin(self)

    def submit(self, activation, data, **options):
        form = self.form_class(
            data, 
            instance=activation.task.process.get_context()
        )
        form.task = activation.task

        if form.is_valid():
            context = form.save()
            activation.submitted()

            return context
        else:
            return form

    def activate(self, activation, **input):
        if activation.task.status == READY:
            activation.task.status = STALL
        
        elif activation.task.status == SUBMITTED:
            activation.done()
            activation.spawn_task(self.next)     

class End(BaseNode):
    def activate(self, activation, **input):
        activation.close_workflow()

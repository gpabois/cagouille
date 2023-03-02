from .tasks import activate
from .status import READY, INIT, DONE, CLOSED, STALL, FAILED, ABORTED, SUBMITTED
from . import signals
from contextlib import contextmanager

@contextmanager
def node_activation(task, engine):
    activation = NodeActivation(task, engine)

    try:
        yield activation
    
    except Exception as e:
        activation.failed(e)
        raise e
    
    finally:
        activation.commit()

class NodeActivation:
    def __init__(self, task, engine):
        self.engine = engine
        self.task = task

        self.act_result = None
        self.nexts = []

    def __iter__(self):
        return iter(self.nexts)

    def commit(self):
        self.task.save()
        self.task.process.save()

    def get_next_by_task(self, task):
        return next(filter(lambda n: n[0].id == task.id, self.nexts))

    def reactivate(self):
        self.nexts.append(
            (self.task, activate.delay(self.task.id))
        )

    def set_activation_result(self, result):
        self.act_result = result
    
    def get_activation_result(self):
        return self.act_result

    def spawn_task(self, step):
        task, job = self.engine.spawn_task(step, self.task.process)
        self.nexts.append((task, job))

    def close_workflow(self):
        self.task.process.done()
        self.task.done()

    def can_be_activated(self):
        return self.task.status in (READY, STALL, SUBMITTED)

    def is_entering(self):
        return self.task.status == INIT

    def is_leaving(self):
        return self.task.status == DONE

    def is_running(self):
        return self.task.status == READY
    
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

    def __call__(self, activation, **input):
        if activation.is_entering():
            self.on_entering(activation, **input)
            activation.ready()          

        if activation.can_be_activated():
            self.activate(activation=activation, **input)

        if activation.is_leaving():
            self.on_leaving(**input)
            activation.close()

        return activation

    def on_entering(self, activation, **input):
        signals.entering_task(sender=self.__class__, task=activation.task)
        
        if self.enter:
            self.enter(**input)

    def on_leaving(self, **input):
        signals.leaving_task(sender=self.__class__, task=activation.task)
        if self.leave:
            self.leave(**input)

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

class Job(BaseNode):
    def __init__(self, fn, next, **options):
        super().__init__(**options)
        self.fn = fn
        self.next = next
    
    def activate(self, activation, **input):
        activation.set_activation_result(self.fn(**input))
        activation.spawn_task(self.next)
        activation.done()

class UserActionResult:
    def __init__(self):
        self.ok = False
        self.result = None
    
    def valid(self, result=None):
        self.ok = True
        self.result = result
    
    def invalid(self, result=None):
        self.ok = False
        self.result = result

class UserActionForm(BaseNode):
    def __init__(self, form_class, next, **options):
       self.form_class = form_class
       self.next = next
       
    def __create_meta_mutation(self):
        attrs = {}
        
        if getattr(self, 'form_class'):
            attrs['form_class'] = self.form_class

        return type("Meta", (), attrs)

    @staticmethod
    def __perform_mutate(return_name, return_field):
        def wrapper(self, form, info):
            context = form.save()
            # Change task status
            task = form.cleaned_data['task']
            task.status = SUBMITTED
            task.save()
            # Fire and forget
            activate(form.cleaned_data['task'].id)
            # Return mutation node
            return self.cls(**{
                return_name: return_field(context)
            })

        return wrapper

    def as_mutation(self, return_name, return_field):
        from graphene_django import DjangoModelFormMutation

        type_name = "".join(list(map(lambda n: n.capitalize(), [self.flow.name, self.name])))
        
        return type(
            type_name, 
            (DjangoModelFormMutation,), 
            {
                'Meta': self._create_meta_mutation(),
                return_name: return_field,
                'perform_mutate': UserActionForm.__perform_mutate()
            }
        )

    def submit(self, **data):
        form = self.form_class(data)
        
        if form.is_valid():
            return form.save()
        else:
            return None

    def activate(self, activation, **input):
        if activation.task.status == READY:
            activation.task.status = STALL
        elif activation.task.status == SUBMITTED:
            activation.done()
            activation.spawn_task(self.next)     

class UserAction(BaseNode):
    def __init__(self, fn, next, **options):
        super().__init__(**options)
        
        if 'form_class' in options:
            self.form_class = form_class

        self.fn = fn
        self.next = next     


    def activate(self, activation, **input):
        if activation.task.status == READY:
            activation.task.status = STALL
        else:
            decision = UserActionResult()
            
            activation.set_activation_result(
                self.fn(decision=decision, **input)
            )
            
            if decision.ok:
                activation.done()
                activation.spawn_task(self.next)


class End(BaseNode):
    def activate(self, activation, **input):
        activation.close_workflow()

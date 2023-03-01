from .tasks import activate

class NodeActivation:
    def __init__(self, task, engine):
        self.engine = engine
        self.task = task

        self.act_result = None
        self.nexts = []

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

    def __call__(self, task, process, engine, **input):
        activation = NodeActivation(task, engine)
        
        if task.status == 'init':
            self.on_entering(**input)
            task.ready()
        
        elif task.status == 'done':
            self.on_leaving(**input)
            task.closed()
        
        elif task.status in ('ready', 'stall'):
            self.activate(activation=activation, task=task, **input)
        
        task.save()
        process.save()

        # Schedule the execution.
        if task.status not in ('closed', 'stall', 'failed', 'aborted'):
            activation.reactivate()

        return activation

    def on_entering(self, **input):
        if self.enter:
            self.enter(**input)

    def on_leaving(self, **input):
        if self.leave:
            self.leave(**input)

class If(BaseNode):
    def __init__(self, predicate, sthen, selse, **options):
        super().__init__(**options)
        self.sthen = sthen
        self.selse = selse

    def activate(self, activation, **input):
        if self.predicate(**input):
            activation.spawn_task(self.sthen)
        else:
            activation.spawn_task(self.selse)

class Job(BaseNode):
    def __init__(self, fn, next, **options):
        super().__init__(**options)
        self.fn = fn
        self.next = next
    
    def activate(self, activation, task, process, **input):
        activation.set_activation_result(self.fn(**input))
        activation.spawn_task(self.next)

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

class UserAction(BaseNode):
    def __init__(self, fn, next, **options):
        super().__init__(**options)
        
        self.fn = fn
        self.next = next     

    def activate(self, activation, **input):
        if activation.task.status == 'ready':
            activation.task.status = 'stall'
        else:
            decision = UserActionResult()
            
            activation.set_activation_result(
                self.fn(decision=decision, **input)
            )
            
            if decision.ok:
                activation.task.status = 'done'
                activation.spawn_task(self.next)

class End(BaseNode):
    def __call__(self, task, process, **input):
        task.done()
        process.done()

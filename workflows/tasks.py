class GoTo:
    def __init__(self, step):
        self.step = step

    def __call__(self, engine, flow, task):
        pass

class BaseTaskType:
    def __init__(self, **options):
        if 'enter' in options:
            self.enter = options['enter']
        else:
            self.enter = None
        
        if 'leave' in options:
            self.leave = options['leave']
        else:
            self.leave = None

    def __call__(self, task, **input):
        raise NotImplementedError()
    
    def on_entering(self, **input):
        if self.enter:
            self.enter(**input)

    def on_leaving(self, **input):
        if self.leave:
            self.leave(**input)

class If(BaseTaskType):
    def __init__(self, predicate, then, _else, **options):
        BaseTaskType.__init__(self, **options)
        self.then = then
        self._else = _else

    def __call__(self, **input):
        if self.predicate(**input):
            return GoTo(self.then)
        else:
            return GoTo(self._else)

class UserAction(BaseTaskType):
    def __init__(self, fn, next, **options):
        BaseTaskType.__init__(self, **options)
        
        self.fn = fn
        self.next = next     
        self.enter = enter
        self.leave = leave

    def on_entering(self, **input):
        super().on_entering(**input)

    def __call__(self, **input):
        self.fn(**input)
        return GoTo(self.next)

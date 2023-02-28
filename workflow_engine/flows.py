from .nodes  import BaseNode
from .engine import ENGINE

class WorkflowMeta(type):   
    def __init__(cls, name, bases, attrs, abstract=False):
        super().__init__(name, bases, attrs)
        if not abstract:
            ENGINE.register(cls)

    def __setattr__(self, name, value):
        if issubclass(value, BaseNode):
            self.steps[name] = value
        setattr(self, name, value)

class Workflow(metaclass=WorkflowMeta, abstract=True):
    
    context_class = None
    steps = {}

    def __init__(self):
        pass

    def get_step(self, step):
        pass

    def get_context(self, process):
        return self.context_class.objects.get(process=process)
from .nodes  import BaseNode, End
from .engine import ENGINE

class SelfClassAttribute:
    def __init__(self, name):
        self.name = name
    
    def __call__(self, cls):
        return getattr(cls, self.name)

class SelfObject:
    def __getattr__(self, name):
        return SelfClassAttribute(name)
    
    def resolve(self, target, source):
        for name, value in {**target.__dict__}.items():
            if isinstance(value, SelfClassAttribute):
                setattr(target, name, value(source))
        
        return target

Self = SelfObject()

class WorkflowMeta(type):   
    def __new__(cls, name, bases, attrs, abstract=False):
        cls = super().__new__(cls, name, bases, attrs)

        for key, value in attrs.items():
            if isinstance(value, BaseNode):
                cls.steps[key] = Self.resolve(value, cls)  
                value.flow = cls
                value.name = key
                value.on_paired_with_flow(cls)

        if not abstract:
            ENGINE.register(cls)
        
        return cls

class Workflow(metaclass=WorkflowMeta, abstract=True):
    context_class = None
    steps = {
        'end': End()
    }

    @classmethod
    def node(cls, step):
        return cls.steps[step]

    @classmethod
    def context(cls, process):
        return cls.context_class.objects.get(process=process)
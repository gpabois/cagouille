from workflow_engine.flows import Workflow, Self
from workflow_engine import nodes
from .models import SimpleContext

class SimpleFlow(Workflow):
    context_class = SimpleContext

    start = nodes.UserAction(Self.fn_start, next='check_approval')
    check_approval = nodes.If(Self.fn_check_approval, sthen='approve', selse='reject')
    approve = nodes.Job(Self.fn_approve, next='end')
    reject = nodes.Job(Self.fn_reject, next='end')

    @staticmethod
    def fn_start(decision, context, approval):
        context.approval_decision = approval
        context.save()
        decision.valid()
    
    @staticmethod
    def fn_check_approval(context, **kwargs):
        return context.approval_decision
    
    @staticmethod
    def fn_approve(context, **kwargs):
        context.approved = True
        context.save()
    
    @staticmethod
    def fn_reject(context, **kwargs):
        context.approved = False
        context.save()
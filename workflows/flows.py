from workflow_engine.flows import Workflow, Self
from workflow_engine import nodes
from workflow_engine.schema import UserActionMutation


from .models import Rvat
from . import schema
from . import forms
from . import signals

class RvatFlow(Workflow):
    name = 'rvat'
    context_class=Rvat

    start = nodes.UserAction(
        forms.FormulairePreparationRvat,
        next="verifier", 
        plugins=[
            UserActionMutation('rvat', schema.Rvat)
        ]
    )

    verifier = nodes.UserAction(
        forms.FormulaireVerificateurRvat,
        next="statuer_verification",
        enter=Self.enter_verification,
        plugins=[
            UserActionMutation('rvat', schema.Rvat)
        ]
    )

    statuer_verification = nodes.If(Self.statuer_verification, "approuver", "correction_verification")
    correction_verification = nodes.UserAction(
        forms.FormulairePreparationRvat,
        next="verifier", 
        enter=Self.enter_correction_verification,
        plugins=[
            UserActionMutation('rvat', schema.Rvat)
        ]
    )
    
    approuver = nodes.UserAction(
        forms.FormulaireApprobateurRvat,
        enter=Self.enter_approbation,
        next="statuer_approbation",
        plugins=[
            UserActionMutation('rvat', schema.Rvat)
        ]
    )
    statuer_approbation = nodes.If(Self.f_statuer_verification, "approuver", "correction_verification")
    correction_approbation = nodes.UserAction(Self.f_start, next="verifier", enter=Self.enter_correction_verification)

    @staticmethod
    def f_start(activation, context, decision, redacteur, **input):
        context.redacteur = redacteur
        activation.task.assigned_to = context.redacteur
        decision.valid()

    @staticmethod
    def f_verifier(activation, context, **input):
        decision.valid()

    @staticmethod
    def enter_verification(activation, context, **input):
        activation.task.assigned_to_group = context.verificateur
        signals.rvat_a_verifier(sender=RvatFlow, task=activation.task)
    
    @staticmethod
    def f_statuer_verification(activation, context, **input):
        return context.verifie

    @staticmethod
    def enter_correction_verification(activation, context, **input):
        pass
    
    @staticmethod
    def f_approuver(activation, context, **input):
        pass

    @staticmethod
    def enter_approbation(activation, context, **input):
        activation.task.assigned_to_group = context.approbateur
        signals.rvat_a_approuver(sender=RvatFlow, task=activation.task)




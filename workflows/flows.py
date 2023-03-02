from workflow_engine.flows import Workflow, Self
from workflow_engine import nodes


from .models import Rvat
from . import forms
from . import signals

class RvatFlow(Workflow):
    context_class=Rvat

    start = nodes.UserAction(
        Self.f_start, 
        next="verifier", 
        form_class=forms.FormulairePreparationRvat
    )

    verifier = nodes.UserAction(
        Self.f_verifier, 
        next="statuer_verification",
        enter=Self.enter_verification,
        form_class=forms.FormulaireVerificateurRvat
    )

    statuer_verification = nodes.If(Self.statuer_verification, "approuver", "correction_verification")
    correction_verification = nodes.UserAction(
        Self.f_start, 
        next="verifier", 
        enter=Self.enter_correction_verification
    )
    
    approuver = nodes.UserAction(
        Self.f_approuver, 
        enter=Self.enter_approbation,
        next="statuer_approbation",
        form_class=forms.FormulaireApprobateurRvat
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
        activation.task.assigned_to = context.verificateur.user_set.first()
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




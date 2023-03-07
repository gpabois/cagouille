from workflow_engine.flows import Workflow, Self
from workflow_engine import nodes

from . import models
from . import forms
from . import signals

def poursuivre(etape):
    def wrapper(activation, context, **kwargs):
        return not getattr(context, etape)
    return wrapper

class Rvat(Workflow):
    name = 'rvat'
    context_class=models.Rvat

    start = nodes.UserAction(
        forms.FormulairePreparationRvat,
        next="verifier",
        enter=Self.enter_preparation
    )

    verifier = nodes.UserAction(
        forms.FormulaireVerificateurRvat,
        next="poursuivre",
        enter=Self.enter_verifier
    )

    approuver = nodes.UserAction(
        forms.FormulaireApprobateurRvat,
        enter=Self.enter_approuver,
        next="poursuivre"
    )
    
    correction = nodes.UserAction(
        forms.FormulairePreparationRvat,
        next="renvoyer",
        enter=Self.enter_correction
    )

    poursuivre = nodes.Branch(
        'transmettre',
        verifier=poursuivre('verifie'),
        approuver=poursuivre('approuve')
    )

    transmettre = nodes.UserAction(
        forms.FormulaireTransmettre,
        next="end",
        enter=Self.enter_transmettre
    )
    
    @staticmethod
    def enter_preparation(activation, context, **input):
        signals.rvat_a_preparer(sender="RVAT", task=activation.task)

    @staticmethod
    def enter_verifier(activation, context, **input):
        activation.task.assigned_to_group = context.verificateur
        signals.rvat_a_verifier(sender="RVAT", task=activation.task)

    @staticmethod
    def enter_approuver(activation, context, **input):
        activation.task.assigned_to_group = context.approbateur
        signals.rvat_a_approuver(sender="RVAT", task=activation.task)

    @staticmethod
    def enter_transmettre(activation, context, **input):
        activation.task.assigned_to_group = context.administratif
        signals.rvat_a_transmettre(sender="RVAR", task=activation.task)
    
    @staticmethod
    def enter_correction(activation, context, **input):
        activation.task.assigned_to_group = context.redacteur





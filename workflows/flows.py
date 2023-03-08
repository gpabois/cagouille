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
        enter=Self.enter_preparation,
        leave=Self.leave_preparation
    )

    verifier = nodes.UserAction(
        forms.FormulaireVerificateurRvat,
        next="poursuivre",
        enter=Self.enter_verifier,
        leave=Self.leave_verifier
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
        enter=Self.enter_transmettre,
        leave=Self.leave_transmettre
    )
    
    @staticmethod
    def enter_preparation(activation, context, **input):
        signals.rvat_a_preparer.send(sender="RVAT", task=activation.task)

    @staticmethod
    def leave_preparation(activation, context, **input):
        context.redacteur = activation.task.assigned_to_user
        context.save()

    @staticmethod
    def enter_verifier(activation, context, **input):
        activation.task.assigned_to_group = context.verificateur
        signals.rvat_a_verifier.send(sender="RVAT", task=activation.task)

    @staticmethod
    def leave_verifier(activation, context, **input):
        if context.verifie:
            signals.rvat_verifie.send(sender="RVAT", task=activation.task)
        else:
            signals.rvat_rejet_verification.send(sender="RVAT", task=activation.task)

    @staticmethod
    def enter_approuver(activation, context, **input):
        activation.task.assigned_to_group = context.approbateur
        signals.rvat_a_approuver.send(sender="RVAT", task=activation.task)

    @staticmethod
    def leave_approuver(activation, context, **input):
        if context.approuve:
            signals.rvat_approuve.send(sender="RVAT", task=activation.task)
        else:
            signals.rvat_rejet_approbation.send(sender="RVAT", task=activation.task)

    
    @staticmethod
    def enter_correction(activation, context, **input):
        activation.task.assigned_to_group = context.redacteur
        activation.rvat_a_corriger.send(sender="RVAT", task=activation.task)

    @staticmethod
    def enter_transmettre(activation, context, **input):
        activation.task.assigned_to_group = context.administratif
        signals.rvat_a_transmettre.send(sender="RVAT", task=activation.task)

    @staticmethod
    def leave_transmettre(activation, context, **input):
         signals.rvat_transmis.send(sender="RVAT", task=activation.task)






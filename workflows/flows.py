from workflow_engine.flows import Workflow, Self, FormBasedContextFactory
from workflow_engine import nodes
import datetime

from . import models
from . import forms
from . import signals

def poursuivre(etape):
    def wrapper(activation, context, **kwargs):
        return not getattr(context, etape)
    return wrapper

def aller_vers_cloture_procedure_gun(activation, context, **kwargs):
    return context.lien_procedure_gun is not None

class Rvat(Workflow):
    name = 'rvat'
    context_class=models.Rvat
    context_factory = FormBasedContextFactory(forms.FormulairePreparationRvat)

    start = nodes.Branch("verifier", leave=Self.leave_start)

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

    verifier_cloture = nodes.Branch(
        'end'
    )

    @staticmethod
    def leave_start(activation, context, **input):
        context.redacteur = activation.task.process.created_by
        context.save()

    @staticmethod
    def enter_verifier(activation, context, **input):
        activation.task.deadline = context.date_limite_verification
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
        activation.task.deadline = context.date_limite_approbation
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
         context.transmis = True
         context.transmis_le = datetime.date.today()
         context.save()






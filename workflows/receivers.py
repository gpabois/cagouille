from django.dispatch import receiver
from django.conf import settings
from templated_email import send_templated_mail
from . import signals

@receiver(signals.rvat_a_preparer)
def notifier_preparer_rvat(sender, task, **kwargs):
    user = task.assigned_to_user
    context = task.process.get_context()
    
    if user.email:
        send_templated_mail(
            template_name="rvat_a_preparer",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[user.email],
            context={
                'user': user,
                'task': task,
                'context': context
            }
        )

@receiver(signals.rvat_a_verifier)
def notifier_verifier_rvat(sender, task, **kwargs):
    group = task.assigned_to_group
    context = task.process.get_context()

    if group:
        for user in group.user_set.all():
            if user.email:
                send_templated_mail(
                    template_name="rvat_a_verifier",
                    from_email=settings.EMAIL_HOST_USER,
                    recipient_list=[user.email],
                    context={
                        'user': user,
                        'task': task,
                        'context': context
                    }
                )

@receiver(signals.rvat_rejet_verification)
def notifier_rejet_verification_rvat(sender, task, **kwargs):
    context = task.process.get_context()
    
    if context.redacteur.email:
        send_templated_mail(
            template_name="rvat_rejet_verification",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[context.redacteur.email],
            context={
                'user': context.redacteur,
                'task': task,
                'context': context
            }
        )


@receiver(signals.rvat_verifie)
def notifier_verifie(sender, task, **kwargs):
    context = task.process.get_context()
    
    if context.redacteur.email:
        send_templated_mail(
            template_name="rvat_verifie",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[context.redacteur.email],
            context={
                'user': context.redacteur,
                'task': task,
                'context': context
            }
        )


@receiver(signals.rvat_a_approuver)
def notifier_approuver_rvat(sender, task, **kwargs):
    group = task.assigned_to_group
    context = task.process.get_context()

    if group:
        for user in group.user_set.all():
            if user.email:
                send_templated_mail(
                    template_name="rvat_a_approuver",
                    from_email=settings.EMAIL_HOST_USER,
                    recipient_list=[user.email],
                    context={
                        'user': user,
                        'task': task,
                        'context': context
                    }
                )

@receiver(signals.rvat_rejet_approbation)
def notifier_rejet_approbation_rvat(sender, task, **kwargs):
    context = task.process.get_context()
    
    if context.redacteur.email:
        send_templated_mail(
            template_name="rvat_rejet_approbation",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[context.redacteur.email],
            context={
                'user': context.redacteur,
                'task': task,
                'context': context
            }
        )


@receiver(signals.rvat_approuve)
def notifier_approuve(sender, task, **kwargs):
    context = task.process.get_context()
    
    if context.redacteur.email:
        send_templated_mail(
            template_name="rvat_approuve",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[context.redacteur.email],
            context={
                'user': context.redacteur,
                'task': task,
                'context': context
            }
        )
        
@receiver(signals.rvat_a_transmettre)
def notifier_transmettre_rvat(sender, task, **kwargs):
    group = task.assigned_to_group
    context = task.process.get_context()

    if group:
        for user in group.user_set.all():
            if user.email:
                send_templated_mail(
                    template_name="rvat_a_transmettre",
                    from_email=settings.EMAIL_HOST_USER,
                    recipient_list=[user.email],
                    context={
                        'user': user,
                        'task': task,
                        'context': context
                    }
                )

@receiver(signals.rvat_transmis)
def notifier_transmis_rvat(sender, task, **kwargs):
    context = task.process.get_context()
    
    if context.redacteur.email:
        send_templated_mail(
            template_name="rvat_transmis",
            from_email=settings.EMAIL_HOST_USER,
            recipient_list=[context.redacteur.email],
            context={
                'user': context.redacteur,
                'task': task,
                'context': context
            }
        )
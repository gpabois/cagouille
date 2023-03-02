import django.dispatch

rvat_verifie = django.dispatch.Signal()
rvat_approuve = django.dispatch.Signal()
rvat_rejet_approbation = django.dispatch.Signal()

rvat_a_preparer = django.dispatch.Signal()
rvat_a_approuver = django.dispatch.Signal()
rvat_a_verifier = django.dispatch.Signal()
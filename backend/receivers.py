from django.dispatch import receiver

from . import signals
from .services import add_document_to_cabinets_by_ids

@receiver(signals.new_document_created)
def store_in_cabinets(sender, **kwargs):
    doc = kwargs['document']

    if 'request' in kwargs:
        request = kwargs['request']
        cabinets = request.GET.get('cabinets', '')
        
        if not cabinets:
            return

        cabinets_ids = list(map(int, cabinets.split(',')))
        add_document_to_cabinets_by_ids(doc, cabinets_ids)

from .models import CabinetNode
from django.db import transaction

def add_document_to_cabinets_by_ids(doc, cabinets_ids):
    """
        Add a document to a set of cabinets, by their ids.
    """
    with transaction.atomic():
        for cabinet in CabinetNode.filter(id__in=cabinets_ids):
            doc_node = CabinetNode.new_document_node(doc, cabinet)
            doc_node.save()

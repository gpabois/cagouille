from django.db import models
from django.contrib.auth.models import Group, User
import mptt
from mptt.models import MPTTModel, TreeForeignKey
from polymorphic.models import PolymorphicModel

from aiots.models import aiot_models
from documents.models import doc_models

# Cabinet file system
class CabinetNode(MPTTModel):
    aiot     = models.ForeignKey(aiot_models.AIOT, on_delete=models.SET_NULL, null=True)
    document = models.ForeignKey(doc_models.Document, on_delete=models.CASCADE, null=True)
    nom      = models.CharField(max_length=255)
    parent   = TreeForeignKey('self', on_delete=models.CASCADE, null=True, blank=True, related_name='children')

    class MPTTMeta:
        order_insertion_by = ['nom']

    @staticmethod
    def new_document_node(document, parent):
        node = CabinetNode()
        node.document = document
        node.parent = parent
        return node
    
    @staticmethod
    def new_cabinet_node(name, parent):
        node = CabinetNode()
        node.name = name
        node.parent = parent
        return node

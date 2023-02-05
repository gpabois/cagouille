from django.db import models
from tree_queries.models import TreeNode
from polymorphic.models import PolymorphicModel

# Create your models here.

# AIOT-Related 
class Region(models.Model):
    nom = models.CharField(max_length=255, null=False)

class Departement(models.Model):
    departement = models.ForeignKey(Region, on_delete=models.CASCADE)
    nom = models.CharField(max_length=255, null=False)

class Commune(models.Model):
    departement = models.ForeignKey(Departement, on_delete=models.CASCADE)
    nom = models.CharField(max_length=255, null=False)
    abbv = models.CharField(max_length=10, null=False)

class AIOT(models.Model):
    commune = models.ForeignKey(Commune, on_delete=models.SET_NULL, null=True)
    code = models.CharField(max_length=255)
    nom = models.CharField(max_length=255)

# Metadata models
class MetadataGroup(models.Model):
    pass

# EDMS Core
def object_directory_path(instance, filename):
    import hashlib
    h = hashlib.new('sha256')
    
    instance.obj.open() 
    h.update(instance.obj.read())

    h = h.hexdigest()

    instance.oname = filename
    instance.oid = h

    return "objects/{0}".format(h)

class Document(models.Model):
    name = models.CharField(max_length=255)
    uploaded_at = models.DateTimeField(auto_now_add=True)

class DocumentVersion(models.Model):
    document = models.ForeignKey(Document, on_delete=models.CASCADE, null=False)
    version = models.IntegerField()
    uploaded_at = models.DateTimeField(auto_now_add=True)

    obj = models.FileField(upload_to=object_directory_path)
    oname = models.CharField(max_length=255)
    oid = models.CharField(max_length=1024)

# Tracker
class TrackerStatus(models.Model):
    nom = models.CharField(max_length=255)

class Tracker(PolymorphicModel):
    nom     = models.CharField(max_length=255)
    status  = models.ForeignKey(TrackerStatus, on_delete=models.SET_NULL, null=True)
    aiot    = models.ForeignKey(AIOT, on_delete=models.CASCADE, null=True)

class TrackerInspection(Tracker):
    pass

class TrackerInstruction(Tracker):
    pass

# Cabinet file system
class CabinetNode(TreeNode):
    aiot = models.ForeignKey(AIOT, on_delete=models.SET_NULL, null=True)
    document = models.ForeignKey(Document, on_delete=models.CASCADE, null=True)
    nom = models.CharField(max_length=255)

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

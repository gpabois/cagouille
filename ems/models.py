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
    commune = models.ForeignKey(Commune, on_delete=models.set_null)
    code = models.CharField(max_length=255)
    nom = models.CharField(max_length=255)

# Cabinet file system
class CabinetNode(TreeNode):
    document = models.ForeignKey(Document, on_delete=models.CASCADE, null=True)
    nom = models.CharField(max_length=255)

# Metadata models
class MetadataGroup(models.Model):
    pass

# EDMS Core
def object_directory_path(instance, filename):
    import hashlib
    h = hashlib.new('sha256')
    
    with data in instance.file.open():
        h.update(data.read())

    h = h.hexdigest()

    instance.oname = filename
    instance.oid = h

    return "objects/{0}".format(h)

class Document(models.Model):
    name = models.CharField(max_length=255)

class DocumentRevision(models.Model):
    document = models.ForeignKey(Document, on_delete=models.CASCADE, null=False)
    version = models.IntegerField()
    uploaded_at = models.DateTimeField(auto_now_add=True)

    obj = models.ModelField(upload_to=object_directory_path)
    oname = models.CharField(max_length=255)
    oid = models.CharField(max_length=1024)

# Tracker
class TrackerStatus(models.Model):
    nom = models.CharField(max_length=255)

class Tracker(PolymorphicModel):
    nom     = models.CharField(max_length=255)
    status  = models.ForeignKey(TrackerStatus, on_delete=models.set_null, null=False)
    aiot    = models.ForeignKey(AIOT, on_delete=models.CASCADE, null=True)

class TrackerInspection(Tracker):
    pass

class TrackerInstruction(Tracker):
    pass



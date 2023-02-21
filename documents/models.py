from django.db import models
from django.contrib.auth.models import Group, User
import mptt
from mptt.models import MPTTModel, TreeForeignKey
from polymorphic.models import PolymorphicModel

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

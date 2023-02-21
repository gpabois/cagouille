from django.db import models
from django.contrib.auth.models import Group, User
import mptt
from mptt.models import MPTTModel, TreeForeignKey
from polymorphic.models import PolymorphicModel

# Tracker
class TrackerStatus(models.Model):
    nom = models.CharField(max_length=255)

class Tracker(PolymorphicModel):
    nom         = models.CharField(max_length=255)
    status      = models.ForeignKey(TrackerStatus, on_delete=models.SET_NULL, null=True, blank=True)
    aiot        = models.ForeignKey(AIOT, on_delete=models.CASCADE, null=True)
    
    responsable = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    entite_responsable = models.ForeignKey(Group, on_delete=models.SET_NULL, null=True)

    def __str__(self):
        return "{} - {}".format(self.nom, self.aiot)

class InspectionTrackerType(models.Model):
    nom = models.CharField(max_length=255)

    def __str__(self):
        return self.nom

class InspectionTracker(Tracker):
    type = models.ForeignKey(InspectionTrackerType, on_delete=models.SET_NULL, null=True)

class InstructionTrackerType(models.Model):
    nom = models.CharField(max_length=255)

    def __str__(self):
        return self.nom

class InstructionTracker(Tracker):
    type = models.ForeignKey(InstructionTrackerType, on_delete=models.SET_NULL, null=True)
    
class TrackerLog(PolymorphicModel):
    tracker = models.ForeignKey(Tracker, on_delete=models.CASCADE, null=False)
    created_at = models.DateTimeField(auto_now_add=True)

class TrackerActivityStream(TrackerLog):
    actor = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    verb = models.CharField(max_length=255)
    target = models.CharField(max_length=255)

class TrackerMessage(TrackerLog):
    user = models.ForeignKey(User, on_delete=models.SET_NULL, null=True)
    message = models.TextField()

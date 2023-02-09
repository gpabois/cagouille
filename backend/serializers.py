from rest_framework import serializers
from . import models

class AiotSerializer(serializers.ModelSerializer):
    class Meta:
        model = models.AIOT
        fields = '__all__'

class InspectionTrackerSerializer(serializers.ModelSerializer):
    aiot = AiotSerializer(read_only=True)

    class Meta:
        model = models.InspectionTracker
        fields = ['id', 'nom', 'status', 'aiot', 'responsable', 'entite_responsable', 'type']
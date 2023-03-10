from django.db import models
from django.contrib.auth.models import Group, User
import mptt
from mptt.models import MPTTModel, TreeForeignKey
from polymorphic.models import PolymorphicModel

# Create your models here.
#TreeForeignKey(Group, on_delete=models.CASCADE, blank=True, null=True).contribute_to_class(Group, 'parent')
#models.PositiveIntegerField(default=0, editable=False, db_index=True).contribute_to_class(Group, 'level')
#models.PositiveIntegerField(default=0, editable=False, db_index=True).contribute_to_class(Group, 'lft')
#models.PositiveIntegerField(default=0, editable=False, db_index=True).contribute_to_class(Group, 'rght')
#models.PositiveIntegerField(default=0, editable=False, db_index=True).contribute_to_class(Group, 'tree_id')
#mptt.register(Group, order_insertion_by=['name'])

# AIOT-Related 
class RegionManager(models.Manager):
    def get_by_natural_key(self, code, nom):
        return self.get(code=code, nom=nom)

class Region(models.Model):
    nom = models.CharField(max_length=255, null=False)
    code = models.CharField(max_length=255, null=False)

    objects = RegionManager()

    class Meta:
        unique_together = [['code', 'nom']]

    def natural_key(self):
        return (self.code, self.nom)

    def __str__(self):
        return self.nom

class DepartementManager(models.Manager):
    def get_by_natural_key(self, code, nom):
        return self.get(code=code, nom=nom)  

class Departement(models.Model):
    region  = models.ForeignKey(Region, on_delete=models.CASCADE)
    nom     = models.CharField(max_length=255, null=False)
    code    = models.CharField(max_length=50, null=False)
    
    objects = DepartementManager()

    class Meta:
        unique_together = [['code', 'nom']]

    def natural_key(self):
        return (self.code, self.nom)

    def __str__(self):
        return "{} - {}".format(self.nom, self.code)

class CommuneManager(models.Manager):
    def get_by_natural_key(self, code_postal, nom):
        return self.get(code_postal=code_postal, nom=nom)   

class Commune(models.Model):
    departement = models.ForeignKey(Departement, on_delete=models.CASCADE)
    nom = models.CharField(max_length=255, null=False)
    abbv = models.CharField(max_length=50, null=False)
    code_insee = models.CharField(max_length=50, null=True)
    code_postal = models.CharField(max_length=50, null=False)
    
    objects = CommuneManager()
    
    class Meta:
        unique_together = [['code_postal', 'nom']]

    def natural_key(self):
        return (self.code_postal, self.nom)

    def __str__(self):
        return self.nom

class StatutAiotManager(models.Manager):
    def get_by_natural_key(self, libelle):
        return self.get(libelle=libelle)      

class StatutAiot(models.Model):
    libelle = models.CharField(max_length=255)

    def natural_key(self):
        return (self.libelle,)

    def __str__(self):
        return self.libelle

class AiotManager(models.Manager):
    def get_by_natural_key(self, code):
        return self.get(code=code)  
      
class Aiot(models.Model):
    statut = models.ForeignKey(StatutAiot, on_delete=models.SET_NULL, null=True)
    commune = models.ForeignKey(Commune, on_delete=models.SET_NULL, null=True)
    code = models.CharField(max_length=255)
    nom = models.CharField(max_length=255)

    adresse_site = models.CharField(max_length=255, null=True)
    complement_adresse_site = models.CharField(max_length=255, null=True)
    courriel = models.CharField(max_length=255, null=True)
    
    ied = models.BooleanField(default=False)
    
    a_poi = models.BooleanField(default=False)
    date_poi = models.DateTimeField(null=True)

    objects = AiotManager()

    class Meta:
        unique_together = [['code']]

    @property
    def identifiant(self):
        return str(self)

    def natural_key(self):
        return self.code

    def __str__(self):
        return "{} [{}#{}]".format(
            self.nom,
            self.commune.abbv,
            self.code[-4:]
        )

class RubriqueIcpeAiot(models.Model):
    aiot = models.ForeignKey(Aiot, on_delete=models.CASCADE, related_name="rubriques_icpe")
    rubrique = models.ForeignKey('RubriqueIcpe', on_delete=models.CASCADE)
    date_autorisation = models.DateField(null=True)
    date_entree_en_vigueur = models.DateField(null=True)
    date_cessation = models.DateField(null=True)

REGIMES = (
    ('A', 'Autorisation'),
    ('E', 'Enregistrement'),
    ('DC', 'Déclaration avec contrôle périodique'),
    ('D', 'Déclaration'),
    ('NC', 'Non-classé')
)

class RubriqueIcpe(MPTTModel):
    parent = TreeForeignKey('self', on_delete=models.CASCADE, null=True, blank=True, related_name='children')
    code = models.CharField(max_length=255)
    regime = models.CharField(max_length=2, choices=REGIMES, default='NC')
    libelle = models.TextField()
    description = models.TextField()
    
    def __str__(self):
        return self.code 
    class MPTTMeta:
        order_insertion_by = ['code']
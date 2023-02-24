import csv

from django.core.management.base import BaseCommand, CommandError
from suivis import models
from suivis import services

class Command(BaseCommand):
    help = "Synchronise les inspections"

    def add_arguments(self, parser):
        parser.add_argument('chemins_fichiers_csv', nargs='+', type=str)

    def handle(self, *args, **options):
        for chemin_fichier_csv in options['chemins_fichiers_csv']:
            with open(chemin_fichier_csv, 'r') as csv_file:
                for suivi in services.gun_synchroniser_inspections(iter(csv.DictReader(csv_file))):
                    self.stdout.write("Synchronisé {}".format(str(suivi)))

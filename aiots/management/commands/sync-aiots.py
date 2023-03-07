from django.core.management.base import BaseCommand, CommandError
from aiots import models
from aiots import services
class Command(BaseCommand):
    
    help = "Synchronise les AIOTS avec Géorisques"

    def add_arguments(self, parser):
        parser.add_argument('mode', help="Le mode de synchronisation georisques / gun")
        parser.add_argument('--fichier', help='Spécifie un fichier pour synchroniser')
        parser.add_argument('--code-region', help='Spécifie une région de France')
        parser.add_argument('--code-departement', help='Spécifie un département de France')

    def handle(self, *args, **options):
        if options['mode'] == 'georisques':
            for aiot in services.synchroniser_avec_georisques(**options):
                self.stdout.write("Synchronisé {}".format(str(aiot)))
        else:
            for aiot in services.synchroniser_avec_fichier_gun(**options):
                self.stdout.write("Synchronisé {}".format(str(aiot)))
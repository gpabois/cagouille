from django.db import transaction
from datetime import datetime
from django.db import transaction
from suivis import models as suivis_models
from aiots import models as aiots_models

import requests, itertools, time

DATE_FORMAT = "%d/%m/%Y"

def parse_date(date):
    try:
        return datetime.strptime(date, DATE_FORMAT)
    except ValueError as e:
        return None

def get_type_insp(nature):
    try:
        return suivis_models.TypeInspection.objects.get(nom=nature)
    except suivis_models.TypeInspection.DoesNotExist:
        type = suivis_models.TypeInspection(nom=nature)
        type.save()
        return type

def get_statut_suivi(etat):
    try:
        return suivis_models.StatutSuivi.objects.get(nom=etat)
    except suivis_models.StatutSuivi.DoesNotExist:
        statut = suivis_models.StatutSuivi(nom=etat)
        statut.save()
        return statut
    
@transaction.atomic
def gun_synchroniser_inspections(inspections):
    for insp in inspections:
        code_aiot = insp['Code AIOT']
        nom       = insp['Procédure']
        type      = get_type_insp(insp["Nature"])
        statut    = get_statut_suivi(insp['Etat'])

        date_previsionnelle = parse_date(insp['Date prévisionnelle'])
        date_preparation = parse_date(insp['Préparation'])
        date_inspection = parse_date(insp['Inspection'])
        date_rapport = parse_date(insp['Rapport'])
        date_publication = parse_date(insp['Publication'])
        
        try:
            aiot = aiots_models.Aiot.objects.get(code=code_aiot)   

            try:
                suivi = suivis_models.SuiviInspection.objects.get(nom=nom, aiot=aiot)
            except suivis_models.SuiviInspection.DoesNotExist:
                suivi =  suivis_models.SuiviInspection(nom=nom, aiot=aiot)
            
            suivi.statut = statut
            suivi.type = type
            suivi.date_previsionnelle = date_previsionnelle
            suivi.date_preparation = date_preparation
            suivi.date_inspection = date_inspection
            suivi.date_rapport = date_rapport
            suivi.date_publication = date_publication

            suivi.save()
            yield suivi
        except aiots_models.Aiot.DoesNotExist:
            pass


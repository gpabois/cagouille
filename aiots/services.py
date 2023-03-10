from django.conf import settings
from aiots import models
from django.db import transaction
from django.db.models import Q

import requests, itertools, time, csv

IIC_API_URI = "{}/installations_classees".format(settings.GEORISQUE_API_URL)

def batched_it(iterable, n):
    "Batch data into iterators of length n. The last batch may be shorter."
    # batched('ABCDEFG', 3) --> ABC DEF G
    if n < 1:
        raise ValueError('n must be at least one')
    it = iter(iterable)
    while True:
        chunk_it = itertools.islice(it, n)
        try:
            first_el = next(chunk_it)
        except StopIteration:
            return
        yield itertools.chain((first_el,), chunk_it)

def georisques_boucle_recuperation_installations_classees(**params):
    resp = requests.get(url=IIC_API_URI, params=params).json()
    
    yield from iter(resp['data'])
    page = 1
    while resp['data']:
        page += 1
        resp = requests.get(url=IIC_API_URI, params={**params, "page": page}).json()
        yield from resp['data']

def georisques_recuperer_installations_classees(**filter):
    if "code_region" in filter and filter["code_region"]:
        code_insees = map(
            lambda commune: commune.code_insee,
            models.Commune.objects.filter(departement__region__code=filter['code_region'])
        )
        filter['code_insee'] = code_insees
        del filter['code_region']
    
    if "code_departement" in filter and filter["code_departement"]:
        code_insees = map(
            lambda commune: commune.code_insee,
            models.Commune.objects.filter(departement__code=filter['code_departement'])
        )
        filter['code_insee'] = code_insees
        del filter['code_departement']     
    
    if filter['code_insee']:
        for batch_code_insee in batched_it(filter['code_insee'], 9):
            yield from georisques_boucle_recuperation_installations_classees(
                code_insee=",".join(batch_code_insee)
            )

def recuperer_rubrique_icpe(rubrique_data):
    code = rubrique_data["numeroRubrique"]
    regime = rubrique_data["regimeAutoriseAlinea"] if "regimeAutoriseAlinea" in rubrique_data else "NC"
    regime_map = {
        "Autorisation": "A",
        "Enregistrement": "E",
        "D??claration avec contr??le": "DC",
        "D??claration": "D",
        "NC": "NC"
    }
    regime = regime_map[regime]

    if "alinea" in rubrique_data:
        code = "{}-{}".format(code, rubrique_data["alinea"].replace(".", "-"))

    try:
        return models.RubriqueIcpe.objects.get(code=code)
    except models.RubriqueIcpe.DoesNotExist as e:
        rubrique = models.RubriqueIcpe(
            code=code,
            libelle=rubrique_data["nature"],
            regime=regime
        )
        rubrique.save()
        return rubrique

def synchroniser_statut_aiot(statut):
    try:
        return models.StatutAiot.objects.get(libelle=statut)
    except models.StatutAiot.DoesNotExist:
        statut = models.StatutAiot(libelle=statut)
        statut.save()
        return statut

def synchroniser_ligne_fichier_gun(row):
    ied         = row['Statut IED'] == 'Oui'
    courriel    = row['Courriel ??change admin'] if '@' in row['Courriel ??change admin'] else None
    code   = row['Code AIOT']
    adresse_site = row['Adresse site']
    complement_adresse_site = row['Compl??ment adresse site'] if row['Compl??ment adresse site'] != '-' else None
    statut = synchroniser_statut_aiot(row["Etat de l'activit??"])
    a_poi = row['Existence POI'] == 'Oui'
    date_poi =  None

    try:
        aiot = models.Aiot.objects.get(Q(code__contains=code))
        
        aiot.ied = ied
        aiot.courriel = courriel
        aiot.adresse_site = adresse_site
        aiot.complement_adresse_site = complement_adresse_site
        aiot.a_poi = a_poi
        aiot.date_poi = date_poi

        aiot.save()
        return aiot
    except models.Aiot.MultipleObjectsReturned as e:
        return None
    except models.Aiot.DoesNotExist:
        return None

@transaction.atomic
def synchroniser_avec_fichier_gun(**options):
    fichier = options['fichier']
    with open(fichier, "r") as file:
        for row in csv.DictReader(file, delimiter=";"):
            aiot = synchroniser_ligne_fichier_gun(row)
            if aiot:
                yield aiot

@transaction.atomic
def synchroniser_avec_georisques(**filter):
    for aiot_data in georisques_recuperer_installations_classees(**filter):
        code_aiot = aiot_data['codeAIOT']

        try:
            aiot = models.Aiot.objects.get_by_natural_key(code=code_aiot)
        except models.Aiot.DoesNotExist as e:
            aiot = models.Aiot()

        aiot.nom = aiot_data['raisonSociale']
        aiot.code = aiot_data['codeAIOT']
        aiot.commune = models.Commune.objects.filter(code_insee=aiot_data['codeInsee']).first()
        aiot.save()
        # Synchronise les rubriques
        for rubrique_data in aiot_data["rubriques"]:
            rubrique = recuperer_rubrique_icpe(rubrique_data)
            try:
                rubrique_icpe = models.RubriqueIcpeAiot.objects.get(aiot=aiot, rubrique=rubrique)
            
            except models.RubriqueIcpeAiot.DoesNotExist:
                rubrique_icpe = models.RubriqueIcpeAiot(
                    aiot=aiot,
                    rubrique=recuperer_rubrique_icpe(rubrique_data)
                )
            
            rubrique_icpe.save()


        yield aiot


    
    

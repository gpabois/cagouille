from django.urls import path

from . import views

urlpatterns = [
    path('documents', views.index_documents, name="index_documents"),
    path('document/new', views.upload_new_document, name='upload_new_document'),
]
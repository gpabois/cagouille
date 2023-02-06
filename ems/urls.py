from django.urls import path
from django.conf.urls.i18n import i18n_patterns

from . import views

urlpatterns = [
    path('documents', views.DocumentIndexView.as_view(), name="list-documents"),
    path('document/new', views.upload_new_document, name='new_document'),
    path('inspections/trackers', views.InspectionTrackerIndexView.as_view(), name='index_inspections_trackers'),

    path('cabinets', views.CabinetListView.as_view(), name="list-cabinets"),
    path('cabinets/new', views.CabinetNewView.as_view(), name="new-cabinet"),
    path('cabinets/<int:pk>', views.CabinetDetailView.as_view(), name="cabinet_detail")
]
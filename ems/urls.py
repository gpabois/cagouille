from django.urls import path, include
from django.conf.urls.i18n import i18n_patterns
from rest_framework.routers import DefaultRouter

from . import views

router = DefaultRouter()
router.register(r'aiots', views.AiotViewSet, basename='aiot')
router.register(r'inspections/trackers', views.InspectionTrackerViewSet, basename='inspection-tracker')

urlpatterns = [
    path('api/', include(router.urls)),
    path('documents', views.DocumentIndexView.as_view(), name="list-documents"),
    path('document/new', views.upload_new_document, name='new_document'),
    
    path('inspections/trackers', views.InspectionTrackerIndexView.as_view(), name='list-inspections-trackers'),
    path('inspections/trackers/new', views.InspectionTrackerNewView.as_view(), name='new-inspection-tracker'),
    path('cabinets', views.CabinetListView.as_view(), name="list-cabinets"),

    path('cabinets/new', views.CabinetNewView.as_view(), name="new-cabinet"),
    path('cabinets/<int:pk>', views.CabinetDetailView.as_view(), name="cabinet_detail")
]
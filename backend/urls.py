from django.urls import path, include
from django.conf.urls.i18n import i18n_patterns
from rest_framework.routers import DefaultRouter

from . import views

router = DefaultRouter()
router.register(r'aiots', views.AiotViewSet, basename='aiot')
router.register(r'inspections/trackers', views.InspectionTrackerViewSet, basename='inspection-tracker')

urlpatterns = [
    path('', views.HomeView.as_view(), name='home'),
    path('api/', include(router.urls)),
]
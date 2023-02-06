from django.shortcuts import render
from django_filters.views import FilterView
from django_tables2.views import SingleTableMixin
from django.views.generic import ListView, DetailView, FormView
from django.shortcuts import get_object_or_404

from . import models, forms, signals, tables

# Create your views here.
def upload_new_document(request):
    if request.method == 'POST':
        form = forms.UploadNewDocumentForm(request.POST, request.FILES)
        if form.is_valid():
            doc = form.save()
            signals.new_document_created.send(sender="upload_new_document", document=doc, request=request)
    else:
        form = forms.UploadNewDocumentForm()
    
    return render(request, 'ems/document/upload.html', {'form': form})

class CabinetNewView(FormView):
    template_name = "ems/cabinets/new.html"
    form_class = forms.NewCabinetForm

class CabinetListView(ListView):
    queryset = models.CabinetNode.objects.filter(parent=None).order_by('-nom')
    context_object_name = "cabinets"
    template_name = "ems/cabinets/list.html"    

class CabinetDetailView(DetailView):
    model = models.CabinetNode
    context_object_name = "cabinet"
    template_name = "ems/cabinets/detail.html"

class DocumentIndexView(SingleTableMixin, FilterView):
    table_class = tables.DocumentTable
    model = models.Document
    template_name = "ems/documents/list.html"

class InspectionTrackerIndexView(SingleTableMixin, FilterView):
    table_class = tables.InspectionTrackerTable
    model = models.TrackerInspection
    template_name = "ems/inspections/trackers/list.html"

def new_inspection_tracker(request):
    if request.method == 'POST':
        form = forms.NewInspectionTrackerForm(request.POST)
        if form.is_valid():
            form.save()
    else:
        form = forms.NewInspectionTrackerForm()
    
    return render(request,'ems/inspections/trackers/new.html', {'form': form})
from django.shortcuts import render
from django.core.paginator import Paginator

from .forms import UploadNewDocumentForm
from .signals import new_document_uploaded
from .models import Document

# Create your views here.
def upload_new_document(request):
    if request.method == 'POST':
        form = UploadNewDocumentForm(request.POST, request.FILES)
        if form.is_valid():
            doc = form.save()
            new_document_uploaded.send(sender="upload_new_document", document=doc, request=request)
    else:
        form = UploadNewDocumentForm()
    
    return render(request, 'ems/document/upload.html', {'form': form})

def index_documents(request):
    documents = Document.objects.all()
    paginator = Paginator(documents, 25)
    page = request.GET.get('page')
    documents = paginator.get_page(page)

    return render(request, 'ems/document/index.html', {'documents': Document.objects.all()})   
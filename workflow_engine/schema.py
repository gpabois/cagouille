from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene import relay, ObjectType, ID, Field, String, Int, Boolean, Mutation, Scalar
from graphene_plus import GlobalID
from . import models

class Task(DjangoObjectType):
    class Meta:
        model = models.Process
        filter_fields = ('status', )
        interfaces = (relay.Node,)

class Process(DjangoObjectType):
    class Meta:
        model = models.Task
        filter_fields = ('status', )
        interfaces = (relay.Node,)

class UserActionMutation:
    def __init__(self, return_field_name, return_field_type):
        self.return_field_name = return_field_name
        self.return_field_type = return_field_type

    def __create_meta_mutation(self,  node):
        attrs = {}
        
        if getattr(node, 'form_class'):
            attrs['form_class'] = node.form_class

        return type("Meta", (), attrs)

    def __perform_mutate(self):
        def wrapper(self, form, info):
            task = form.cleaned_data['task']
            options = {}
            
            if getattr(info.context, 'user'):
                options['user'] = info.context.user

            context = submit(context, form.cleaned_data, **options)

            return node.__class__(**{
                self.return_field_name: self.return_field_type(context)
            })

        return wrapper

    def add_mutation(self, node):
        flow_name = node.flow.__name__.lower()
        if hasattr(node.flow, 'name'):
            flow_name = node.flow.name

        type_name = "".join(list(map(lambda n: n.capitalize(), [flow_name, node.name])))
        
        mutation_type = type(
            type_name, 
            (DjangoModelFormMutation,), 
            {
                'name': "_".join([flow_name, node.name.lower()]),
                'Meta': self.__create_meta_mutation(node),
                self.return_field_name: self.return_field_type,
                'perform_mutate': self.__perform_mutate()
            }
        )

        setattr(Mutation, mutation_type.name, mutation_type.Field())
        return mutation_type

    def __call__(self, node):
        self.add_mutation(node)

class Query(ObjectType):
    processes = DjangoFilterConnectionField(Process)
    tasks = DjangoFilterConnectionField(Task)
    process = relay.Node.Field(Process)
    task = relay.Node.Field(Task)

class Mutation(ObjectType):
    pass
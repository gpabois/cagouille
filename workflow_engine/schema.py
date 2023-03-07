import graphene
from graphene import ObjectType, Field, Mutation, relay, String, Boolean
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from django.db.models import Q

from . import tasks
from . import models
from . import nodes

class User(DjangoObjectType):
    class Meta:
        model = models.User
        filter_fields = ('id', 'username', 'email')
        interfaces = (relay.Node,)

class Group(DjangoObjectType):
    class Meta:
        model = models.Group
        filter_fields = ('id',)
        interfaces = (relay.Node,)

class Task(DjangoObjectType):
    class Meta:
        model = models.Task
        filter_fields = ('id', 'status', 'process')
        interfaces = (relay.Node,)
    
    assigned_to_user = Field(User)
    assigned_to_group = Field(Group)

class MyTask(DjangoObjectType):
    class Meta:
        model = models.Task
        interfaces = (relay.Node, )
        filter_fields = ('id', 'status', 'process')

class Process(DjangoObjectType):
    class Meta:
        model = models.Process
        filter_fields = ('status',)
        interfaces = (relay.Node,)

def __create_mutation(flow):
    class CreateFlow(graphene.Mutation):
        class Arguments:
            pass

        ok = Boolean()
        process = Field(Process)
        task = Field(Task)
        error = String()

        @classmethod
        def mutate(cls, root, info):
            try:
                process, task = tasks.spawn_flow(flow, flow.context_class(), user=info.context.user)
                return cls(process=process, task=task, ok=True)
            except Exception as e:
                import traceback
                traceback.print_exc()
                return cls(error=str(e), ok=False)

    return CreateFlow

def flow_mutation(flow, context_type):
    fields = {
        'create': __create_mutation(flow).Field()
    }
    
    for step, node in flow.steps.items():
        if isinstance(node, nodes.UserAction):
            field = __as_task_mutation(node, context_type)
            fields[field.name] = field.Field()

    return type("{}Mutations".format(flow.__name__), (ObjectType,), fields)

def __create_meta_task_mutation(node):
    attrs = {}
    
    if getattr(node, 'form_class'):
        attrs['form_class'] = node.form_class

    return type("Meta", (), attrs)

def __perform_task_mutate():
    def wrapper(self, form, info):
        task = form.cleaned_data['task']
        options = {}
        
        if getattr(info.context, 'user'):
            options['user'] = info.context.user

        context = submit(context, form.cleaned_data, **options)

        return node.__class__(**{
            'context': context
        })

    return wrapper

def __as_task_mutation(node, context_type):
    type_name = node.name
    
    mutation_type = type(
        type_name, 
        (DjangoModelFormMutation,), 
        {
            'name':             node.name,
            'context':          Field(context_type),
            'Meta':             __create_meta_task_mutation(node),
            'perform_mutate':   __perform_task_mutate()
        }
    )   

    return mutation_type 

class Query(ObjectType):
    processes = DjangoFilterConnectionField(Process)
    tasks = DjangoFilterConnectionField(Task)
    my_tasks = DjangoFilterConnectionField(MyTask)
    process = relay.Node.Field(Process)
    task = relay.Node.Field(Task)

    def resolve_my_tasks(self, info):
        if info.context.user.is_anonymous:
            return Task.objects.none()
        else:
            return Task.objects.filter(
                Q(assigned_to_user=info.context.user) 
                | 
                Q(assigned_to_group__user=info.context.user)
            )

class Mutation(ObjectType):
    pass
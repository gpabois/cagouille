import graphene
from graphene import ObjectType, Field, Mutation, relay, String, Boolean
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_plus import GlobalID
from django.db.models import Q
from django.forms import Form

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
            field = __as_task_mutation(flow, node, context_type)
            fields[field.name] = field.Field()

    return type("{}Mutations".format(flow.__name__), (ObjectType,), fields)

def __as_task_mutation(flow, node, context_type):
    type_name = node.name
    
    class TaskMutation(DjangoModelFormMutation):
        name = type_name
        context = Field(context_type)
        ok = Boolean()
        
        class Input:
            task = GlobalID()
            
        class Meta:
            form_class = node.form_class
            name = type_name
       
        @classmethod
        def mutate_and_get_payload(cls, root, info, **data):
            form = cls.get_form(root, info, **data)
            print(data)
            task = models.Task.objects.get(pk=data['task'])
            options = {}
            
            if getattr(info.context, 'user'):
                options['user'] = info.context.user
            
            result = tasks.submit(task, data, **options)
            
            if isinstance(result, Form):
                form = result
                errors = ErrorType.from_errors(form.errors)
                _set_errors_flag_to_context(info)

                return cls(errors=errors, ok=False, **form.data)
            else:
                context = result
                cls(context=context, ok=False)

    return TaskMutation

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
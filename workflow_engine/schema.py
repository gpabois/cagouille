import graphene
from graphene import ObjectType, Field, Mutation, relay, String, Boolean
from graphene_django.types import ErrorType
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_django.constants import MUTATION_ERRORS_FLAG
from graphene_django import DjangoObjectType
from graphene_django.filter import DjangoFilterConnectionField
from graphene_django.forms.mutation import DjangoModelFormMutation
from graphene_plus import GlobalID
import django.db.models
from django.db.models import Q
from django.forms import Form
from graphql_relay import from_global_id

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

def _replace_global_ids_with_model_ids(form_class, data):
    model = form_class._meta.model
    
    for field, value in data.items():
        if not hasattr(model, field):
            continue

        model_field = model._meta.get_field(field)

        if isinstance(model_field, (django.db.models.AutoField, django.db.models.ForeignKey)):
            _, data[field] = from_global_id(value)

    return data

def _set_errors_flag_to_context(info):
    # This is not ideal but necessary to keep the response errors empty
    if info and info.context:
        setattr(info.context, MUTATION_ERRORS_FLAG, True)

def _create_mutation(flow, context_type):
    if flow.context_factory.requires_form_submission:
        form_class = flow.context_factory.form_class
        
        class FormBasedCreateFlow(DjangoModelFormMutation):
            context = Field(context_type)
            process = Field(Process)
            task = Field(Task)
            ok = Boolean()
                
            class Meta:
                form_class = flow.context_factory.form_class
                name = "Create{}".format(flow.get_name().capitalize())

            @classmethod
            def mutate_and_get_payload(cls, root, info, **data):
                try:
                    data = _replace_global_ids_with_model_ids(cls._meta.form_class, data)
                    form_kwargs = cls.get_form_kwargs(root, info, **data)
                    
                    result = tasks.spawn_flow(
                        flow, 
                        flow.context_class(), 
                        user=info.context.user, 
                        data=form_kwargs
                    )

                    if result.is_ok:
                        return cls(
                            ok=True, 
                            process=result.process, 
                            task=result.task, 
                            context=result.context, 
                            errors=[]
                        )
                    else:
                        errors = ErrorType.from_errors(result.errors)
                        _set_errors_flag_to_context(info)
                        return cls(errors=errors, ok=False)   
                except Exception as e:
                    import traceback
                    traceback.print_exc()
                    raise e
        return FormBasedCreateFlow
    else:
        class CreateFlow(graphene.Mutation):
            class Meta:
                name = "Create{}".format(flow.name.capitalize())

            ok = Boolean()
            process = Field(Process)
            task = Field(Task)
            error = String()

            @classmethod
            def mutate(cls, root, info):
                try:
                    result = tasks.spawn_flow(flow, flow.context_class(), user=info.context.user)
                    if result.is_ok:
                        return cls(process=process, task=task, ok=True)
                    else:
                        errors = ErrorType.from_errors(result.errors)
                        _set_errors_flag_to_context(info)
                        return cls(errors=result.errors)
                except Exception as e:
                    import traceback
                    traceback.print_exc()
                    return cls(errors=[str(e)], ok=False)
        return CreateFlow

def flow_mutation(flow, context_type, **fields):
    fields = {
        **fields,
        'create': _create_mutation(flow, context_type).Field()
    }
    
    for step, node in flow.steps.items():
        if isinstance(node, nodes.UserAction):
            field = _create_task_mutation(flow, node, context_type)
            fields[field.name] = field.Field()

    return type("{}Mutations".format(flow.__name__), (ObjectType,), fields)

def _create_task_mutation(flow, node, context_type):
    type_name = node.name
    
    class TaskMutation(DjangoModelFormMutation):
        name = type_name
        context = Field(context_type)
        ok = Boolean()
        
        class Input:
            task = GlobalID()
            
        class Meta:
            form_class = node.form_class
            name = "{}{}".format(type_name.capitalize(), flow.name.capitalize())
       
        @classmethod
        def mutate_and_get_payload(cls, root, info, **data):
            data = _replace_global_ids_with_model_ids(cls._meta.form_class, data)
            form = cls.get_form(root, info, **data)
            task = models.Task.objects.get(pk=from_global_id(data['task'])[1])
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
                return cls(errors=[], context=context, ok=True)

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
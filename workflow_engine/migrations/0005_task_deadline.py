# Generated by Django 4.1.6 on 2023-03-10 13:22

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('workflow_engine', '0004_alter_process_created_at_alter_task_created_at'),
    ]

    operations = [
        migrations.AddField(
            model_name='task',
            name='deadline',
            field=models.DateField(null=True),
        ),
    ]

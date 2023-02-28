from graphene import Scalar
from graphql_relay.node.node import from_global_id
from graphql import Undefined

from graphql.language.ast import (
    BooleanValueNode,
    FloatValueNode,
    IntValueNode,
    StringValueNode,
)

class GlobalID(Scalar):
    serialize = str
    parse_value = str

    @staticmethod
    def parse_literal(ast, _variables=None):
        if isinstance(ast, IntValueNode):
            return ast.value
        
        elif isinstance(ast, StringValueNode):
            return int(from_global_id(ast.value).id)
        
        return Undefined
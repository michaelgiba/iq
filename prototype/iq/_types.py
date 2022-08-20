from dataclasses import dataclass
from typing import Any, Union


@dataclass
class Layer:
    pass


@dataclass
class Operator:
    pass


@dataclass
class OperationNode:
    lhs: 'ASTNode'
    rhs: 'ASTNode'
    op: Operator 


@dataclass
class LiteralNode:
    value: Any


ASTNode = Union[OperationNode, LiteralNode]

@dataclass
class AST:
    root: ASTNode


@dataclass
class Token:
    pass

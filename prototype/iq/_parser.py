from ._types import Token, AST, LiteralNode
from typing import List


def ast_from_tokens(tokens: List[Token]) -> AST:
    return AST(root=LiteralNode(None))

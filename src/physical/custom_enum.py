"""# custom_enum
A custom, tiny implementation of the `Enum` class, because Codon wouldn't compile the standard library's `enum.py` file.
It's basically just a fancy dict parser, but it was necessary.

# Classes

## Enum[T]
The whole reason we're here.

## EnumMember[T]
A variant of the enum with a value and a name.
"""


class EnumMember[T]:
    name: str
    value: T
    """# EnumMember[T]
    A class holding a name and value field, used as each variant of the Enum.

    # Attributes

    ## name: str
    The name of the variant.

    ## value: T
    The value of the variant.

    # Methods

    ## __init__(self, name: str, value: T)
    Initializes member.
    """

    def __init__(self, name: str, value: T):
        self.name = name
        self.value = value


class Enum[T]:
    """# Enum
    Enherit to create an Enum.
    Any attribute that doesn't start with `__` becomes an EnumMember.
    """
    
    def __init__(self, values: dict[str, T]):
        for (name, value) in values.items():
            self.__setattr__(name, EnumMember(name, value))
    

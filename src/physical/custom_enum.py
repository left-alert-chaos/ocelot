"""# custom_enum
A custom, tiny implementation of the `Enum` class, because Codon wouldn't compile the standard library's `enum.py` file.
It's basically just a fancy dict parser, but it was necessary.

# Classes

## Enum[T]
The whole reason we're here.
Can be used identically to std Enum.

## EnumMember[T]
A variant of the enum with a value and a name.
"""


class Enum[T]:
    def __init__(self, name: str, value: T):
        self.name = name
        self.value = value

    def __init_subclass__(cls):
        for (name, value) in cls.__dict__.items():
            cls.__dict__[name] = cls(name, value)


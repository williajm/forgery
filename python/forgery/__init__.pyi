"""Type stubs for the forgery package."""

from forgery._forgery import Faker as Faker

__all__: list[str]
__version__: str

# Default Faker instance
fake: Faker

# Module-level convenience functions
def seed(value: int) -> None:
    """Seed the default Faker instance for deterministic output.

    Args:
        value: The seed value.
    """
    ...

def name() -> str:
    """Generate a single random full name.

    Returns:
        A full name (first + last).
    """
    ...

def names(n: int) -> list[str]:
    """Generate a batch of random full names.

    Args:
        n: Number of names to generate.

    Returns:
        A list of full names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def first_name() -> str:
    """Generate a single random first name.

    Returns:
        A first name.
    """
    ...

def first_names(n: int) -> list[str]:
    """Generate a batch of random first names.

    Args:
        n: Number of first names to generate.

    Returns:
        A list of first names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def last_name() -> str:
    """Generate a single random last name.

    Returns:
        A last name.
    """
    ...

def last_names(n: int) -> list[str]:
    """Generate a batch of random last names.

    Args:
        n: Number of last names to generate.

    Returns:
        A list of last names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def email() -> str:
    """Generate a single random email address.

    Returns:
        An email address.
    """
    ...

def emails(n: int) -> list[str]:
    """Generate a batch of random email addresses.

    Args:
        n: Number of emails to generate.

    Returns:
        A list of email addresses.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def integer(min: int = 0, max: int = 100) -> int:
    """Generate a single random integer within a range.

    Args:
        min: Minimum value (inclusive). Default: 0.
        max: Maximum value (inclusive). Default: 100.

    Returns:
        A random integer.

    Raises:
        ValueError: If min > max.
    """
    ...

def integers(n: int, min: int = 0, max: int = 100) -> list[int]:
    """Generate a batch of random integers within a range.

    Args:
        n: Number of integers to generate.
        min: Minimum value (inclusive). Default: 0.
        max: Maximum value (inclusive). Default: 100.

    Returns:
        A list of random integers.

    Raises:
        ValueError: If min > max or n exceeds the maximum batch size (10 million).
    """
    ...

def uuid() -> str:
    """Generate a single random UUID (version 4).

    Returns:
        A UUID string.
    """
    ...

def uuids(n: int) -> list[str]:
    """Generate a batch of random UUIDs (version 4).

    Args:
        n: Number of UUIDs to generate.

    Returns:
        A list of UUID strings.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

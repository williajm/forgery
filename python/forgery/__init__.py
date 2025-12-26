"""forgery - Fake data at the speed of Rust.

A high-performance fake data generation library for Python, powered by Rust.
Designed to be 50-100x faster than Faker for batch operations.

Example:
    >>> from forgery import fake
    >>> fake.seed(42)
    >>> names = fake.names(1000)  # Generate 1000 names in one call
    >>> email = fake.email()  # Single value convenience method

    >>> from forgery import Faker
    >>> my_fake = Faker()
    >>> my_fake.seed(123)
    >>> my_fake.emails(100)
"""

from forgery._forgery import Faker

__all__ = [
    "Faker",
    "email",
    "emails",
    "fake",
    "first_name",
    "first_names",
    "integer",
    "integers",
    "last_name",
    "last_names",
    "name",
    "names",
    "seed",
    "uuid",
    "uuids",
]

__version__ = "0.1.0"

# Default Faker instance for convenient access
fake: Faker = Faker()


def seed(value: int) -> None:
    """Seed the default Faker instance for deterministic output.

    Args:
        value: The seed value.

    Example:
        >>> from forgery import seed, names
        >>> seed(42)
        >>> names1 = names(10)
        >>> seed(42)
        >>> names2 = names(10)
        >>> assert names1 == names2
    """
    fake.seed(value)


def name() -> str:
    """Generate a single random full name.

    Returns:
        A full name (first + last).

    Example:
        >>> from forgery import name
        >>> print(name())
        John Smith
    """
    return fake.name()


def names(n: int) -> list[str]:
    """Generate a batch of random full names.

    Args:
        n: Number of names to generate.

    Returns:
        A list of full names.

    Example:
        >>> from forgery import names
        >>> batch = names(1000)
        >>> len(batch)
        1000
    """
    return fake.names(n)


def first_name() -> str:
    """Generate a single random first name.

    Returns:
        A first name.
    """
    return fake.first_name()


def first_names(n: int) -> list[str]:
    """Generate a batch of random first names.

    Args:
        n: Number of first names to generate.

    Returns:
        A list of first names.
    """
    return fake.first_names(n)


def last_name() -> str:
    """Generate a single random last name.

    Returns:
        A last name.
    """
    return fake.last_name()


def last_names(n: int) -> list[str]:
    """Generate a batch of random last names.

    Args:
        n: Number of last names to generate.

    Returns:
        A list of last names.
    """
    return fake.last_names(n)


def email() -> str:
    """Generate a single random email address.

    Returns:
        An email address.

    Example:
        >>> from forgery import email
        >>> print(email())
        john123@gmail.com
    """
    return fake.email()


def emails(n: int) -> list[str]:
    """Generate a batch of random email addresses.

    Args:
        n: Number of emails to generate.

    Returns:
        A list of email addresses.
    """
    return fake.emails(n)


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
    return fake.integer(min, max)


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
    return fake.integers(n, min, max)


def uuid() -> str:
    """Generate a single random UUID (version 4).

    Returns:
        A UUID string.

    Example:
        >>> from forgery import uuid
        >>> print(uuid())
        a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d
    """
    return fake.uuid()


def uuids(n: int) -> list[str]:
    """Generate a batch of random UUIDs (version 4).

    Args:
        n: Number of UUIDs to generate.

    Returns:
        A list of UUID strings.
    """
    return fake.uuids(n)

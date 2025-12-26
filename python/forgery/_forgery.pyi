"""Type stubs for the Rust extension module."""

class Faker:
    """A fake data generator with its own random state.

    Each instance maintains independent RNG state, allowing for deterministic
    generation when seeded. The default locale is "en_US".

    Example:
        >>> from forgery import Faker
        >>> fake = Faker()
        >>> fake.seed(42)
        >>> names = fake.names(100)
    """

    def __init__(self, locale: str = "en_US") -> None:
        """Create a new Faker instance with the specified locale.

        Args:
            locale: The locale for generated data (default: "en_US").
        """
        ...

    def seed(self, value: int) -> None:
        """Seed the random number generator for deterministic output.

        Args:
            value: The seed value.
        """
        ...

    # Name generators
    def name(self) -> str:
        """Generate a single random full name."""
        ...

    def names(self, n: int) -> list[str]:
        """Generate a batch of random full names.

        Args:
            n: Number of names to generate.
        """
        ...

    def first_name(self) -> str:
        """Generate a single random first name."""
        ...

    def first_names(self, n: int) -> list[str]:
        """Generate a batch of random first names.

        Args:
            n: Number of first names to generate.
        """
        ...

    def last_name(self) -> str:
        """Generate a single random last name."""
        ...

    def last_names(self, n: int) -> list[str]:
        """Generate a batch of random last names.

        Args:
            n: Number of last names to generate.
        """
        ...

    # Internet generators
    def email(self) -> str:
        """Generate a single random email address."""
        ...

    def emails(self, n: int) -> list[str]:
        """Generate a batch of random email addresses.

        Args:
            n: Number of emails to generate.
        """
        ...

    # Number generators
    def integer(self, min: int = 0, max: int = 100) -> int:
        """Generate a single random integer within a range.

        Args:
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).
        """
        ...

    def integers(self, n: int, min: int = 0, max: int = 100) -> list[int]:
        """Generate a batch of random integers within a range.

        Args:
            n: Number of integers to generate.
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).
        """
        ...

    # Identifier generators
    def uuid(self) -> str:
        """Generate a single random UUID (version 4)."""
        ...

    def uuids(self, n: int) -> list[str]:
        """Generate a batch of random UUIDs (version 4).

        Args:
            n: Number of UUIDs to generate.
        """
        ...

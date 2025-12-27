"""Type stubs for the Rust extension module."""

import builtins

# Records schema types (matching forgery/__init__.pyi for consistency)
FieldValue = str | int | float | tuple[int, int, int]
SimpleType = str
IntRangeSpec = tuple[str, int, int]
FloatRangeSpec = tuple[str, float, float]
TextSpec = tuple[str, int, int]
DateRangeSpec = tuple[str, str, str]
ChoiceSpec = tuple[str, list[str]]
FieldSpec = SimpleType | IntRangeSpec | FloatRangeSpec | TextSpec | DateRangeSpec | ChoiceSpec
Schema = dict[str, FieldSpec]

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

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    def first_name(self) -> str:
        """Generate a single random first name."""
        ...

    def first_names(self, n: int) -> list[str]:
        """Generate a batch of random first names.

        Args:
            n: Number of first names to generate.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    def last_name(self) -> str:
        """Generate a single random last name."""
        ...

    def last_names(self, n: int) -> list[str]:
        """Generate a batch of random last names.

        Args:
            n: Number of last names to generate.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
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

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    # Number generators
    def integer(self, min: int = 0, max: int = 100) -> int:
        """Generate a single random integer within a range.

        Args:
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).

        Raises:
            ValueError: If min > max.
        """
        ...

    def integers(self, n: int, min: int = 0, max: int = 100) -> list[int]:
        """Generate a batch of random integers within a range.

        Args:
            n: Number of integers to generate.
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).

        Raises:
            ValueError: If min > max or n exceeds the maximum batch size (10 million).
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

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    def md5(self) -> str:
        """Generate a single random MD5 hash."""
        ...

    def md5s(self, n: int) -> list[str]:
        """Generate a batch of random MD5 hashes."""
        ...

    def sha256(self) -> str:
        """Generate a single random SHA256 hash."""
        ...

    def sha256s(self, n: int) -> list[str]:
        """Generate a batch of random SHA256 hashes."""
        ...

    # Float generators
    def float(self, min: builtins.float = 0.0, max: builtins.float = 1.0) -> builtins.float:
        """Generate a single random float within a range."""
        ...

    def floats(
        self, n: int, min: builtins.float = 0.0, max: builtins.float = 1.0
    ) -> list[builtins.float]:
        """Generate a batch of random floats within a range."""
        ...

    # Color generators
    def color(self) -> str:
        """Generate a single random color name."""
        ...

    def colors(self, n: int) -> list[str]:
        """Generate a batch of random color names."""
        ...

    def hex_color(self) -> str:
        """Generate a single random hex color (#RRGGBB)."""
        ...

    def hex_colors(self, n: int) -> list[str]:
        """Generate a batch of random hex colors."""
        ...

    def rgb_color(self) -> tuple[int, int, int]:
        """Generate a single random RGB color tuple."""
        ...

    def rgb_colors(self, n: int) -> list[tuple[int, int, int]]:
        """Generate a batch of random RGB color tuples."""
        ...

    # DateTime generators
    def date(self, start: str = "2000-01-01", end: str = "2030-12-31") -> str:
        """Generate a single random date (YYYY-MM-DD format)."""
        ...

    def dates(self, n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
        """Generate a batch of random dates."""
        ...

    def date_of_birth(self, min_age: int = 18, max_age: int = 80) -> str:
        """Generate a single random date of birth."""
        ...

    def dates_of_birth(self, n: int, min_age: int = 18, max_age: int = 80) -> list[str]:
        """Generate a batch of random dates of birth."""
        ...

    def datetime(self, start: str = "2000-01-01", end: str = "2030-12-31") -> str:
        """Generate a single random datetime (ISO 8601 format)."""
        ...

    def datetimes(self, n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
        """Generate a batch of random datetimes."""
        ...

    # Text generators
    def sentence(self, word_count: int = 10) -> str:
        """Generate a single random sentence."""
        ...

    def sentences(self, n: int, word_count: int = 10) -> list[str]:
        """Generate a batch of random sentences."""
        ...

    def paragraph(self, sentence_count: int = 5) -> str:
        """Generate a single random paragraph."""
        ...

    def paragraphs(self, n: int, sentence_count: int = 5) -> list[str]:
        """Generate a batch of random paragraphs."""
        ...

    def text(self, min_chars: int = 50, max_chars: int = 200) -> str:
        """Generate a single random text block."""
        ...

    def texts(self, n: int, min_chars: int = 50, max_chars: int = 200) -> list[str]:
        """Generate a batch of random text blocks."""
        ...

    # Address generators
    def street_address(self) -> str:
        """Generate a single random street address."""
        ...

    def street_addresses(self, n: int) -> list[str]:
        """Generate a batch of random street addresses."""
        ...

    def city(self) -> str:
        """Generate a single random city name."""
        ...

    def cities(self, n: int) -> list[str]:
        """Generate a batch of random city names."""
        ...

    def state(self) -> str:
        """Generate a single random state name."""
        ...

    def states(self, n: int) -> list[str]:
        """Generate a batch of random state names."""
        ...

    def country(self) -> str:
        """Generate a single random country name."""
        ...

    def countries(self, n: int) -> list[str]:
        """Generate a batch of random country names."""
        ...

    def zip_code(self) -> str:
        """Generate a single random zip code."""
        ...

    def zip_codes(self, n: int) -> list[str]:
        """Generate a batch of random zip codes."""
        ...

    def address(self) -> str:
        """Generate a single random full address."""
        ...

    def addresses(self, n: int) -> list[str]:
        """Generate a batch of random full addresses."""
        ...

    # Phone generators
    def phone_number(self) -> str:
        """Generate a single random phone number."""
        ...

    def phone_numbers(self, n: int) -> list[str]:
        """Generate a batch of random phone numbers."""
        ...

    # Company generators
    def company(self) -> str:
        """Generate a single random company name."""
        ...

    def companies(self, n: int) -> list[str]:
        """Generate a batch of random company names."""
        ...

    def job(self) -> str:
        """Generate a single random job title."""
        ...

    def jobs(self, n: int) -> list[str]:
        """Generate a batch of random job titles."""
        ...

    def catch_phrase(self) -> str:
        """Generate a single random catch phrase."""
        ...

    def catch_phrases(self, n: int) -> list[str]:
        """Generate a batch of random catch phrases."""
        ...

    # Network generators
    def url(self) -> str:
        """Generate a single random URL."""
        ...

    def urls(self, n: int) -> list[str]:
        """Generate a batch of random URLs."""
        ...

    def domain_name(self) -> str:
        """Generate a single random domain name."""
        ...

    def domain_names(self, n: int) -> list[str]:
        """Generate a batch of random domain names."""
        ...

    def ipv4(self) -> str:
        """Generate a single random IPv4 address."""
        ...

    def ipv4s(self, n: int) -> list[str]:
        """Generate a batch of random IPv4 addresses."""
        ...

    def ipv6(self) -> str:
        """Generate a single random IPv6 address."""
        ...

    def ipv6s(self, n: int) -> list[str]:
        """Generate a batch of random IPv6 addresses."""
        ...

    def mac_address(self) -> str:
        """Generate a single random MAC address."""
        ...

    def mac_addresses(self, n: int) -> list[str]:
        """Generate a batch of random MAC addresses."""
        ...

    # Email variants
    def safe_email(self) -> str:
        """Generate a single random safe email (example.com/org/net)."""
        ...

    def safe_emails(self, n: int) -> list[str]:
        """Generate a batch of random safe emails."""
        ...

    def free_email(self) -> str:
        """Generate a single random free email (gmail.com, etc.)."""
        ...

    def free_emails(self, n: int) -> list[str]:
        """Generate a batch of random free emails."""
        ...

    # Finance generators
    def credit_card(self) -> str:
        """Generate a single random credit card number with valid Luhn checksum."""
        ...

    def credit_cards(self, n: int) -> list[str]:
        """Generate a batch of random credit card numbers."""
        ...

    def iban(self) -> str:
        """Generate a single random IBAN with valid checksum."""
        ...

    def ibans(self, n: int) -> list[str]:
        """Generate a batch of random IBANs."""
        ...

    # Records generators
    def records(self, n: int, schema: Schema) -> list[dict[str, FieldValue]]:
        """Generate structured records based on a schema.

        The schema is a dictionary mapping field names to type specifications:
        - Simple types: "name", "email", "uuid", "int", "float", etc.
        - Integer range: ("int", min, max)
        - Float range: ("float", min, max)
        - Text with limits: ("text", min_chars, max_chars)
        - Date range: ("date", start, end)
        - Choice: ("choice", ["option1", "option2", ...])

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.

        Returns:
            A list of dictionaries, each containing the generated fields.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...

    def records_tuples(self, n: int, schema: Schema) -> list[tuple[FieldValue, ...]]:
        """Generate structured records as tuples based on a schema.

        This is faster than records() since it avoids creating dictionaries.
        Values are returned in alphabetical order of the schema keys.

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.

        Returns:
            A list of tuples, each containing values in alphabetical key order.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...
